# Phase 8: Data, Storage, and Namespace Platform

## Objective

Build the broader application substrate: durable state, objects, private storage, names, routes, and discovery.

This phase turns Overrid from a workload runner into a platform that apps can use for stateful products.

## Depends On

- Phase 7 grid-resident backbone.
- Trust and policy controls.
- ORU accounting.
- Package/artifact storage needs from prior phases.

## Build Order

1. Define data classes and ownership rules.
2. Build Overbase v0 for structured state.
3. Build Overstore v0 for objects, artifacts, models, and media.
4. Build Overvault v0 for encrypted private state and secrets.
5. Build universal namespace records.
6. Bind Overasset records to namespace, storage, route, and entitlement references.
7. Connect namespace to Overmesh route resolution.
8. Add anti-squatting, delegation, transfer, and disputes.
9. Prove a simple stateful app.

## Workstream 1: Data Classes

Define classes:

- Public.
- Public low-sensitivity.
- Tenant private.
- User private.
- Organization private.
- Secret-bearing.
- Regulated.
- System-service.
- Grant-funded public-interest.

Policy, placement, backup, and cache rules must use these classes.

## Workstream 2: Overbase V0

Build structured data primitives:

- Document collections.
- Key-value collections.
- Event streams.
- Vector indexes.
- Index lifecycle.
- Consistency levels.
- Sharding plan.
- Replication plan.
- Backup and recovery.
- Tenant and app boundaries.

Overbase should first serve app state and Docdex-like index metadata. It should not try to replace every database pattern immediately.

## Workstream 3: Overstore V0

Build object storage for:

- Content-addressed objects.
- Package artifacts.
- Model artifacts.
- Media files.
- Dataset chunks.
- Snapshots.
- Backups.
- Logs.

Add chunking, checksums, replication or erasure coding, repair jobs, retention policy, and access policy.

## Workstream 4: Overvault V0

Build encrypted private storage for:

- User private records.
- Organization private records.
- Secrets.
- Escrowed records.
- Sensitive app state.
- Key policy metadata.

Overvault must keep access decisions explicit. Apps should not read private state just because they can call a storage API.

## Workstream 5: Universal Namespace

Create names for:

- People.
- Organizations.
- Apps.
- Services.
- Agents.
- Swarms.
- Tags.
- Assets.
- Routes.
- Native app pages.

Namespace records should support ownership, delegation, transfer, route binding, verification markers, and dispute state.

## Workstream 6: Overasset Namespace Bindings

Connect Overasset records to platform primitives:

- Namespace ownership references.
- Storage entitlement references.
- App/service ownership references.
- Dataset, model, media, and package rights metadata.
- Transfer, revocation, dispute, and correction state.

These records are utility references for Overrid operations. They must not become NFT-style speculative assets.

## Workstream 7: Overmesh Route Resolution

Resolve namespace entries to:

- Service endpoints.
- App routes.
- API routes.
- Storage refs.
- Asset records.
- Identity records.

Route changes must be auditable and policy checked.

## Workstream 8: Abuse And Disputes

Add controls for:

- Impersonation.
- Squatting.
- Misleading names.
- Abandoned names.
- Unauthorized transfer.
- Route hijack.
- Trademark or community dispute where applicable.

Central AI or stewardship systems should rely on evidence and appeal paths, not unreviewable automatic deletion.

## Validation

- A simple app stores structured state in Overbase.
- The app stores artifacts in Overstore.
- The app stores private settings or secrets in Overvault.
- A namespace resolves to app route, identity, service, and asset records.
- Overasset records bind to namespace/storage references without bypassing policy.
- Unauthorized cross-tenant data access is denied.
- Route update emits audit events.

## Exit Gate

Phase 8 is complete when an app can use Overbase, Overstore, Overvault, namespace, and Overmesh as normal platform primitives.

## Handoff To Phase 9

Phase 9 uses these primitives to make app deployment intent-driven through Overpack.
