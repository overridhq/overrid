SDS #14

# Overtenant SDS

## Purpose

Build tenant, subtenant, role, quota, suspension, private-swarm, white-label, and offboarding boundaries.

Overtenant is the authority for tenant scope and role-based control. It tells Overgate, Overregistry, Overqueue, Overguard, accounting services, federation services, and native apps which tenant owns an object, which identities can act in that tenant, which roles apply, whether the tenant is active, and which quota or budget scope applies.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [overtenant.md](../../service_catalog/control_plane/overtenant.md) |
| Sub-build plan | [SUB BUILD PLAN #14 - Overtenant](../../build_plan/sub_build_plan_014_overtenant.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md) |

## Service Family

- Family: Control plane.
- Owning layer: Tenant boundary, membership, role binding, quota scope, suspension, private-swarm, and offboarding authority.
- Primary data scope: tenant records, membership records, role bindings, subtenant links, quota refs, budget refs, suspension records, private-swarm refs, white-label refs, and offboarding records.
- First build phase from service plan: [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md).

## Problem Statement

Overrid needs a tenant boundary before any resource, manifest, queue item, key, ledger ref, native app object, or AI assistant request can be trusted. Without Overtenant, services will guess authorization from identity alone, mix tenant data, and make suspension or offboarding impossible to enforce. Overtenant makes every tenant-scoped action explicit and replayable.

## Goals

- Create and manage tenant records and lifecycle state.
- Manage membership and role bindings between Overpass identities and tenants.
- Provide fast tenant, role, quota-scope, and suspension checks to Overgate and control-plane services.
- Ensure every tenant-scoped object carries a tenant id unless the protocol marks it as global metadata.
- Support subtenant, private-swarm, white-label, quota, budget, and offboarding refs without bloating Phase 1.
- Emit audit evidence for tenant creation, role changes, suspension, and offboarding.
- Keep tenant scope compatible with ORU, Seal Ledger, native apps, federation, and grid-resident backbone operations.

## Non-Goals

- Do not create identity records; Overpass owns identity.
- Do not store credentials; Overkey owns credentials.
- Do not settle quotas or budget usage; Overmeter, ORU, Seal Ledger, and Overbill own later accounting.
- Do not decide workload policy beyond tenant scope and role checks; Overguard owns policy.
- Do not delete tenant history during offboarding.
- Do not encode pricing, customer-count, or market assumptions.

## Primary Actors And Clients

- Tenant owners and administrators.
- Overgate checking role and tenant state during admission.
- Overpass resolving identity refs.
- Overkey checking service-account scope.
- Overregistry attaching tenant scope to manifests and provider records.
- Overqueue enforcing suspended or blocked tenant state.
- Overguard applying policy to tenant-scoped work.
- Seal Ledger, Overbill, and wallet services attaching accounting refs.
- Federation and native apps using tenant boundaries for organizations, communities, and public utilities.
- Operators performing suspension, recovery, and offboarding actions.

## Dependencies

- Overpass identity records.
- Shared tenant schemas and role binding schemas.
- Overgate command context and signed admin actions.
- Overwatch event log for tenant audit evidence.
- Overkey for service-account and operator credential refs.
- Overguard for policy constraints once available.
- Overmeter, ORU, Seal Ledger, and Overbill for quota and budget refs in later phases.

## Owned Responsibilities

Overtenant owns:

- Tenant lifecycle state.
- Tenant membership and role binding records.
- Tenant owner and administrator refs.
- Subtenant relationships and inheritance rules.
- Quota scope and budget reference metadata.
- Suspension, read-only, hold, and recovery state.
- Private-swarm and white-label metadata refs.
- Offboarding records and cleanup obligations.
- Tenant-scoped authorization check APIs.

Overtenant does not own identity proof, credentials, workload policy, queue state, registry content, ledger finality, or native app business records.

## Data Model

The first implementation should define these records:

- `tenant_record`: tenant id, tenant type, owner identity id, parent tenant id, lifecycle state, display handle ref, created by, created time, updated time, and audit refs.
- `membership_record`: membership id, tenant id, identity id, membership state, invited by, accepted time, suspended time, and audit refs.
- `role_binding`: binding id, tenant id, identity id or service account id, role id, scope, grantor id, effective time, expiry time, state, and audit refs.
- `role_definition`: role id, tenant scope, permission set, inherited roles, admin flag, service-account flag, and compatibility version.
- `quota_scope_ref`: tenant id, quota class, budget or ORU account ref, policy ref, effective state, and audit refs.
- `suspension_record`: tenant id, suspension class, reason code, initiated by, effective time, read-only flag, blocked command classes, appeal refs, and recovery refs.
- `subtenant_link`: parent tenant id, child tenant id, inheritance rule, isolation rule, billing scope ref, and audit refs.
- `private_swarm_ref`: tenant id, resource pool ref, allowed node refs, trust class, policy refs, and state.
- `offboarding_record`: tenant id, requested by, reason code, freeze state, export refs, deletion refs, retention refs, cleanup tasks, and final state.

Tenant states must include `pending`, `active`, `read_only`, `suspended`, `offboarding`, `closed`, and `tombstoned`.

## API Surface

Phase 1 should expose:

- `POST /v1/tenants`: create a tenant through an admitted command.
- `GET /v1/tenants/{tenant_id}`: read tenant state visible to the caller.
- `POST /v1/tenants/{tenant_id}/memberships`: invite or add a member.
- `POST /v1/tenants/{tenant_id}/role-bindings`: grant or update a role binding.
- `DELETE /v1/tenants/{tenant_id}/role-bindings/{binding_id}`: revoke a role binding through an audited command.
- `GET /v1/tenants/{tenant_id}/memberships`: list filtered memberships.
- `POST /v1/tenants/{tenant_id}/state`: transition tenant state, including read-only or suspension.
- `POST /v1/internal/tenant-check`: internal check for Overgate and services, returning tenant state, membership, role, quota scope, and reason codes.
- `POST /v1/tenants/{tenant_id}/offboarding`: start offboarding once later cleanup services exist.

Internal checks must be read-only, bounded, and safe to cache with explicit invalidation events.

## Event Surface

Overtenant should emit:

- `overtenant.tenant_created`: tenant record created.
- `overtenant.tenant_state_changed`: lifecycle state changed.
- `overtenant.member_invited`: membership invitation created.
- `overtenant.member_added`: membership became active.
- `overtenant.member_removed`: membership revoked or closed.
- `overtenant.role_granted`: role binding created.
- `overtenant.role_revoked`: role binding revoked.
- `overtenant.quota_scope_changed`: quota or budget ref changed.
- `overtenant.suspension_applied`: tenant or member suspended.
- `overtenant.suspension_released`: suspension lifted.
- `overtenant.offboarding_started`: tenant entered offboarding.
- `overtenant.offboarding_completed`: tenant closed with retained evidence refs.

Events must include tenant id, actor id, target identity or binding refs, reason code, prior state, next state, trace id, and audit refs.

## Core Workflow

1. Overgate admits a signed tenant command.
2. Overtenant validates tenant schema, actor identity, owner refs, role authority, and idempotency context.
3. Overtenant writes tenant, membership, role, quota, suspension, or offboarding state.
4. Overtenant emits Overwatch events for every mutating transition.
5. Overgate and downstream services call tenant-check APIs before protected mutations.
6. Suspension or read-only events invalidate cached tenant decisions.
7. Offboarding coordinates export, retention, cleanup, accounting holds, and route closure through later services.

## State Machine

Tenant lifecycle:

1. `pending`: tenant exists but cannot run protected workloads.
2. `active`: tenant can act according to roles, policy, quota, and credentials.
3. `read_only`: tenant can read allowed records but cannot create new protected mutations except recovery or offboarding actions.
4. `suspended`: protected mutations are blocked by policy, operator action, dispute, or abuse response.
5. `offboarding`: tenant is being exported, cleaned, retained, or closed.
6. `closed`: tenant is no longer active but evidence and required records remain.
7. `tombstoned`: tenant id is permanently reserved for audit and cannot be reused.

Membership lifecycle:

1. `invited`
2. `active`
3. `suspended`
4. `revoked`
5. `expired`

Role binding lifecycle:

1. `proposed`
2. `active`
3. `suspended`
4. `revoked`
5. `expired`

No tenant, membership, or role history may be overwritten in place.

## Policy And Security

- Every tenant-scoped object must carry tenant id unless Protocol Core marks it as global metadata.
- Tenant checks must distinguish owner, admin, member, service account, system service, and external viewer roles.
- Suspended and read-only tenants must be blocked at Overgate before side effects.
- Role grants and revocations require signed commands and audit evidence.
- Service accounts must be tenant-scoped unless explicitly global system-service identities.
- Tenant cache entries in Overgate and services must expire or invalidate on tenant events.
- Offboarding must preserve legal, audit, accounting, and dispute evidence while removing or closing user-visible active surfaces.
- Private-swarm and white-label metadata must not create hidden bypasses around policy or accounting.

## Metering And Accounting

Overtenant does not bill or settle. It should:

- Provide tenant, quota scope, budget refs, ORU account refs, and accounting owner refs to metering and billing services.
- Emit usage-relevant events for tenant creation, role changes, suspension, offboarding, and quota-scope changes.
- Preserve tenant refs for ORU, Seal Ledger, Overbill, Overgrant, provider payout, and wallet records.
- Avoid direct ledger mutation.
- Keep native-service economics structural by exposing tenant and app refs, not charge logic.

## Observability And Operations

Overtenant should expose:

- Tenant counts by state and type.
- Membership and role-binding counts by tenant.
- Suspended, read-only, offboarding, and closed tenant reports.
- Role-drift and orphaned-service-account reports.
- Cache invalidation streams for Overgate and services.
- Audit search hooks through Overwatch.
- Offboarding progress and blocked cleanup views.
- Migration tooling for role definitions and tenant hierarchy changes.

## Failure Modes And Recovery

- Missing owner identity: reject tenant creation.
- Actor lacks role authority: deny before state mutation.
- Duplicate idempotency key with same request hash: return prior result.
- Duplicate idempotency key with conflicting role change: reject with conflict reason.
- Overwatch unavailable: fail closed for role, suspension, and lifecycle mutations unless an audited buffer is active.
- Tenant suspended during queued workload: Overqueue or Overgate must block new protected work and leave existing state queryable.
- Role accidentally revoked: require signed recovery action and evidence.
- Offboarding cleanup partially fails: keep tenant in `offboarding` with blocked tasks and reason codes.

## Validation Plan

The service implementation plan lists these specific validation requirements:

- Every tenant-scoped object carries tenant context.
- Role checks are enforced through Overgate and Overguard.
- Suspended tenants cannot submit workloads.

Additional SDS-level validation:

- Contract tests cover tenant create/read/state, membership, role grant, role revoke, internal tenant check, suspension, and offboarding APIs.
- Tenant isolation tests prove objects cannot be read or mutated across tenant boundaries.
- Role tests cover owner, admin, member, service account, system service, expired role, and revoked role cases.
- Suspension tests prove Overgate denies protected mutations and Overqueue blocks new ready work.
- Cache invalidation tests prove role and suspension changes invalidate stale authorization.
- Offboarding tests prove history is retained while active surfaces close.

## Build Breakdown

1. Define tenant, membership, role binding, role definition, quota scope, suspension, subtenant, private-swarm, and offboarding schemas.
2. Implement tenant creation and lifecycle state.
3. Add membership and role binding APIs.
4. Add internal tenant-check API for Overgate, Overregistry, Overqueue, and Overkey.
5. Add suspension and read-only state enforcement.
6. Add quota and budget refs for later accounting phases.
7. Add private-swarm, white-label, subtenant, and offboarding records as later phases require them.

The Phase 1 exit gate is tenant creation, membership, role check, and suspension enforcement for the signed synthetic workload flow.

## Handoff And Downstream Use

Overtenant is required by Overgate, Overguard, Overregistry, Overqueue, Overkey, Seal Ledger, Overbill, federation, and native apps.

Downstream services should ask Overtenant for tenant and role checks through explicit APIs or cached event-fed projections. If a service needs a new tenant scope, role type, or offboarding hook, update this SDS, the service implementation plan, shared schemas, and the build-plan crosswalk together.

## Open Design Questions

- Phase 1 must implement `owner`, `admin`, `developer`, `viewer`, and `service_account` role classes. `owner` is required for tenant creation, owner transfer, recovery, and offboarding approval; `admin` manages memberships, ordinary role bindings, quota refs, and suspension proposals; `developer` can submit manifests and workload commands but cannot manage membership; `viewer` can read permitted status, audit summaries, and tenant metadata without mutation; and `service_account` is a non-human role bound to credential refs, command classes, and service scopes. `operator` is not a normal tenant membership role in Phase 1; it is a system or operator identity class used only for signed, audited safety actions such as suspension, recovery, emergency offboarding, or break-glass maintenance.
- Before a mature event bus exists, tenant-check caching should use Overtenant-owned revisions and a lightweight invalidation cursor instead of external queue or pub/sub products. Every tenant, membership, role, quota, and suspension mutation increments a tenant authorization revision, writes an Overwatch-compatible event, and returns the affected tenant, identity, role binding, service account, and command-class invalidation refs. Overgate and internal services may cache tenant-check responses only when the cache key includes tenant id, actor or service-account id, role set, command class, and revision. A stale, missing, or unknown cursor must bypass cache and call Overtenant live; high-risk mutating commands fail closed if live tenant state cannot be checked.
- Subtenants are allowed only as an acyclic single-parent tree until later governance work explicitly expands the model. Child tenants must have their own tenant id, lifecycle state, role bindings, service accounts, quota scope, suspension state, and audit refs. Role inheritance is default-deny: parent authority does not automatically grant access inside the child, child roles never grant access upward, and cross-sibling access is denied unless an explicit delegated role binding names the target tenant and command class. Budget, ORU account, private-swarm, and white-label inheritance must be expressed as explicit refs, not implied by tree position.
- Offboarding must retain evidence needed to prove what happened without retaining user-visible data as active product state. Required retained records include the tenant id and tombstone, lifecycle and offboarding records, signed command refs, actor and role-binding history, suspension and dispute refs, export/delete manifests, retention class, cleanup task states, route and namespace closure refs, quota/budget/accounting refs, Overwatch event refs, and hashes or refs proving export or deletion completion. Raw user content, private payloads, and secrets should be removed, vaulted, redacted, or reduced to hashes/refs according to the relevant Overstore, Overvault, accounting, compliance, and dispute-retention rules.
- Tenant states block capabilities separately. `pending` blocks protected reads, workload queueing, scheduling, and accounting settlement except setup reads and setup mutations by the creator or admin path. `active` allows reads, writes, queueing, scheduling, and accounting according to role, policy, quota, and credential checks. `read_only` allows permitted reads, audit views, exports, accounting views, and recovery or offboarding actions, but blocks ordinary writes, new queue items, and new scheduling. `suspended` blocks ordinary reads of sensitive tenant surfaces, all ordinary writes, new queueing, and scheduling; only appeal, recovery, operator review, audit, dispute, and required accounting hold/correction paths remain available. `offboarding` allows export, retention, cleanup, dispute, and accounting-finalization work only. `closed` allows authorized audit, compliance, dispute, and accounting lookups but no active product reads, writes, queueing, or scheduling. `tombstoned` preserves only reserved-id, audit, dispute, and historical accounting refs and cannot be reactivated.
