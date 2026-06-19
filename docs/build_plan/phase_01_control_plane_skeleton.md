# Phase 1: Control-Plane Skeleton

## Objective

Build the minimum Overrid control plane that can accept identities, tenants, resources, manifests, commands, keys, audit events, and queued work.

The goal is not to run real workloads yet. The goal is to make every future workload enter the system through a controlled, signed, tenant-aware, auditable path.

## Depends On

- Phase 0 shared schemas.
- Local stack and integration harness.
- API conventions for signatures, idempotency, trace ids, and audit events.

## Build Order

1. Overpass-lite identity records.
2. Overtenant tenant and role boundaries.
3. Overkey-lite credential and signing records.
4. Overgate API ingress and command envelope validation.
5. Overregistry manifest storage.
6. Overwatch append-only event log.
7. Overqueue persistent pending work.
8. End-to-end synthetic command flow.

## Workstream 1: Overpass-Lite

Build identity primitives for:

- People.
- Organizations.
- Nodes.
- Apps.
- Native services.
- Service accounts.
- System services.

The first version should support lifecycle states: pending, active, disabled, suspended, and tombstoned/deleted-marker history. Do not erase identity history when suspension or deletion-like actions happen; preserve auditability.

## Workstream 2: Overtenant

Build tenant boundaries before any resource is visible:

- Tenant creation.
- Tenant membership.
- Roles and role bindings.
- Quota scope.
- Suspension state.
- Tenant audit context.

Every object created after this point must carry a tenant id unless it is explicitly global protocol metadata.

## Workstream 3: Overkey-Lite

Build key and credential records:

- API keys for SDK/CLI.
- Signing public keys.
- Service account keys.
- Key rotation metadata.
- Revocation state.
- Last-used metadata.

The first implementation can be simple, but all command signatures should verify against key records rather than hardcoded development secrets.

## Workstream 4: Overgate

Overgate is the first API ingress:

- Authenticate actor.
- Validate tenant context.
- Validate request schema.
- Verify command signature where required.
- Enforce idempotency keys.
- Assign trace id.
- Emit audit event.
- Forward accepted commands to the internal command handler.

Overgate should return denials with stable reason codes.

## Workstream 5: Overregistry

Overregistry stores declared facts before execution exists:

- Resource manifests.
- Workload manifests.
- Package manifests.
- Provider records.
- Native app records.
- Schema versions.
- Capability record drafts.

Every manifest should be versioned, signed or linked to a signed command, and immutable after acceptance. Updates create new versions.

## Workstream 6: Overwatch Event Log

Build the append-only operational event log:

- Request accepted.
- Request denied.
- Identity created.
- Tenant created.
- Key created or revoked.
- Manifest accepted.
- Queue item created.
- State transition applied.

This is the beginning of replayable governance. Every later trust, billing, dispute, and central AI decision depends on evidence produced here.

## Workstream 7: Overqueue Skeleton

Create persistent pending work:

- Queue item id.
- Tenant id.
- Workload manifest ref.
- Priority.
- Requested resource card.
- Retry metadata.
- Current state.
- Dead-letter state.
- Audit event refs.

No execution happens yet. A synthetic workload only needs to reach pending state.

## Validation

- Tenant admin can create tenant, identity, credential, resource manifest, and signed workload command.
- Duplicate idempotency key returns the original outcome or a deterministic duplicate rejection.
- Invalid signature is denied before side effects.
- Synthetic workload reaches pending queue state.
- Audit chain can be followed from API request to queue item.

## Exit Gate

Phase 1 is complete when the system can accept a signed tenant-scoped workload command, validate it, persist its manifest, place it into durable queue state, and prove the command path through audit events.

## Handoff To Phase 2

Phase 2 uses identity, tenant, key, manifest, and resource records to register real founder hardware as the seed private swarm.
