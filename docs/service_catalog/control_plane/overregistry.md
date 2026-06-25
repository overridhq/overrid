# Overregistry Implementation Plan

## Objective

Store versioned resource, workload, package, provider, node capability, purpose tag, and catalog records.

## First Build Phase

[Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md).

## Dependencies

- Overpass owner identities.
- Overtenant scope.
- Overgate command admission.
- Shared manifest schemas.

## Development Order

1. Implement resource, workload, and package manifest records.
2. Add immutable versioning for accepted manifests.
3. Add provider and node capability records.
4. Add schema version metadata.
5. Add purpose tag and public catalog records in federation phases.

## Contracts And Interfaces

- Manifest API.
- Provider and node records.
- Capability records.
- Schema version refs.
- Catalog query API.
- App monetization policy refs, accepted publisher terms versions, ORU-only attestations, and bypass enforcement state.

## Detailed SDS

- [Overregistry SDS](../../sds/control_plane/overregistry.md)

## Sub-Build Plan

- [SUB BUILD PLAN #12 - Overregistry](../../build_plan/sub_build_plan_012_overregistry.md)

## Design Alignment

- Treat Overregistry as the immutable versioned fact store for manifests, providers, nodes, capabilities, packages, native apps, and catalog records.
- Require Overgate admission, owner identity, tenant scope, schema version, content hash, trace id, and audit refs for accepted records.
- Make accepted manifest content immutable; corrections and updates create new versions with predecessor links.
- Distinguish claimed provider/node facts from verified facts so Oververify and challenge services can attach evidence later.
- Require downstream services to cite registry ids, versions, content hashes, and schema versions for replay.
- Preserve ORU-only monetization attestations and payment-bypass enforcement state for app, native service, and catalog records.

## Validation

- Manifest updates create new versions.
- Scheduler and policy can replay decisions from registry facts.
- Invalid or unsigned manifest changes are rejected.
- Monetized app records retain accepted terms-policy refs and are suspended or hidden when external payment bypass is detected.

## Handoff

Overregistry feeds Overguard, Oversched, Overpack, Oververify, federation, and public app catalogs.
