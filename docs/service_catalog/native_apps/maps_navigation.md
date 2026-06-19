# Maps and Navigation Implementation Plan

## Objective

Build native maps, local discovery, routes, places, business listings, community map layers, and privacy-preserving location controls.

## First Build Phase

[Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md) after privacy and abuse controls mature.

## Dependencies

- Overbase.
- Overstore.
- Overpass namespace.
- Directory listings.
- Search engine.
- Overvault.

## Development Order

1. Define place, route, and map layer records.
2. Integrate business and organization listings.
3. Add local discovery and search.
4. Add privacy-preserving location settings.
5. Add offline or cached map areas where feasible.

## Contracts And Interfaces

- Place schema.
- Route schema.
- Map layer schema.
- Location privacy policy.

## Validation

- User location preferences are enforced.
- Directory/search results can appear on maps.
- Offline/cached data respects policy and freshness.

## Handoff

Maps integrate directory, search, messaging, local discovery, and mobile clients.

## Detailed SDS

The detailed design contract is [Maps and Navigation SDS](../../sds/native_apps/maps_navigation.md).

## Design Alignment

- Treat Maps and Navigation as the privacy-preserving place, route, layer, local-discovery, and offline-area utility, not as a surveillance or advertising surface.
- Require place records, route requests/results, map layer refs, location permission records, offline area manifests, correction reports, handoff refs, and usage refs.
- Enforce explicit, revocable, purpose-bound location permissions for current, exact, background, route, saved-place, and offline use.
- Directory Listings remains the listing authority, Search Engine remains the broad ranking authority, and Messaging Center remains the communication authority.
