# Universal Namespace Service Implementation Plan

## Objective

Create human-readable names for people, organizations, apps, services, agents, swarms, communities, tags, assets, and routes.

## First Build Phase

[Phase 8: Data, Storage, and Namespace Platform](../../build_plan/phase_08_data_storage_namespace_platform.md).

## Dependencies

- Overpass identities.
- Overasset rights.
- Seal Ledger ownership evidence.
- Overmesh route resolution.

## Development Order

1. Define namespace record schema.
2. Add ownership, delegation, transfer, and route binding.
3. Add verification markers.
4. Add dispute records for impersonation, squatting, and route hijack.
5. Integrate namespace lookup with messaging, search, maps, and app routes.

## Contracts And Interfaces

- Namespace API.
- Route binding API.
- Delegation and transfer records.
- Dispute hook contract.

## Detailed SDS

The detailed design contract lives in [Universal Namespace Service SDS](../../sds/data_storage_namespace/universal_namespace_service.md).

## Design Alignment

- Treat the namespace service as the readable naming and route-binding layer, not the identity authority, asset-right system, ledger, object store, or private network.
- Store normalized names, owner refs, target refs, route bindings, delegation grants, transfers, verification markers, disputes, tombstones, and resolution cache entries.
- Use Overpass for identity refs, Overasset and Seal Ledger for rights/evidence refs, Overmesh for route resolution, and Overclaim for disputes.
- Keep public resolution privacy-aware so private identities, routes, storage refs, and secret-bearing endpoints are not leaked.

## Validation

- A name resolves to the right identity, app, service, route, or asset.
- Unauthorized transfers are denied.
- Route changes emit audit events.

## Handoff

The namespace service becomes the human and AI-facing address layer for Overrid.
