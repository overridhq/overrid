SDS #12

# Overregistry SDS

## Purpose

Store versioned resource, workload, package, provider, node capability, purpose tag, and catalog records.

Overregistry is the declared-facts store for Overrid. It keeps accepted manifests and capability records immutable by version so policy, scheduling, verification, packaging, federation, and native app catalogs can replay decisions from the same facts that existed when a command was accepted.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [overregistry.md](../../service_catalog/control_plane/overregistry.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md) |
| Sub-build plan | [SUB BUILD PLAN #12 - Overregistry](../../build_plan/sub_build_plan_012_overregistry.md) |

## Service Family

- Family: Control plane.
- Owning layer: Manifest, provider, capability, package, and catalog facts.
- Primary data scope: resource manifests, workload manifests, package manifests, provider records, node capability records, native app records, schema version refs, catalog entries, and purpose tag refs.
- First build phase from service plan: [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md).

## Problem Statement

Overrid cannot schedule or govern work from mutable claims scattered across services. It needs a versioned registry of what resources exist, what workloads ask for, which packages are approved, which providers and nodes claim which capabilities, and which schema version defined those facts. Overregistry turns admitted declarations into immutable records that later decisions can cite.

## Goals

- Accept resource, workload, package, provider, node capability, and native app manifests through signed commands.
- Make accepted manifests immutable; updates create new versions with predecessor links.
- Store schema version refs and validation evidence for every accepted record.
- Provide replayable facts for Overguard policy, Oversched placement, Overpack validation, Oververify evidence, federation, and public catalogs.
- Preserve owner identity, tenant scope, trace id, command id, and audit refs for every record.
- Keep catalog records structural and non-speculative; public visibility is a policy and tenant decision, not a registry shortcut.
- Store accepted ORU-only monetization terms-policy refs, publisher attestations, and payment-bypass enforcement state for monetized apps.

## Non-Goals

- Do not execute packages or workloads.
- Do not decide policy admission; Overguard owns policy.
- Do not choose placement; Oversched owns placement.
- Do not certify trust by itself; Oververify and challenge services own verification evidence.
- Do not store raw secrets, private input payloads, or mutable hidden overrides inside accepted manifests.
- Do not encode pricing, customer-count, or market assumptions.

## Primary Actors And Clients

- Overgate submitting admitted manifest commands.
- SDK, CLI, admin UI, and developer tooling creating manifests.
- Overguard replaying policy decisions from registry facts.
- Oversched reading workload and capability facts.
- Overpack and package validators reading package refs and provenance.
- Oververify and challenge services attaching evidence to provider and node records.
- Native app and federation services reading catalog-visible records.
- Operators reviewing manifest history and rejected declarations.

## Dependencies

- Overgate command admission and idempotency refs.
- Overpass owner identities for people, organizations, nodes, apps, native services, service accounts, and system services.
- Overtenant tenant scope, role bindings, suspension state, and catalog visibility boundaries.
- Shared manifest schemas from the shared schema package.
- Overwatch event log for manifest lifecycle evidence.
- Overkey for signed command and package-signing credential refs.
- Overguard, Oververify, Overpack, Oversched, and federation services as downstream consumers.

## Owned Responsibilities

Overregistry owns:

- Versioned manifest record storage.
- Manifest validation result refs and schema version refs.
- Provider and node capability records.
- Package manifest records and provenance refs.
- Native app, service, and catalog records.
- Purpose tag and public catalog refs in later federation phases.
- Immutability rules for accepted versions.
- Query APIs for current, historical, tenant-visible, and catalog-visible records.

Overregistry does not own policy decisions, execution state, resource leases, provider payouts, or accounting finality.

## Data Model

The first implementation should define these records:

- `manifest_record`: manifest id, manifest type, version, tenant id, owner identity id, command id, trace id, schema version, content hash, validation state, visibility, predecessor version, successor version, and audit refs.
- `workload_manifest_record`: workload class, package ref, resource card, input refs, output refs, data sensitivity, egress policy, secrets policy, retry policy, timeout, and policy refs.
- `resource_manifest_record`: provider id, node id, resource class, region or locality, capacity summary, trust class, availability state, capability refs, and verification refs.
- `package_manifest_record`: package id, package version, artifact refs, provenance refs, dependency lock refs, SBOM refs, signature refs, runtime class, and validator refs.
- `provider_record`: provider identity id, tenant or federation scope, onboarding state, verification refs, dispute refs, and eligibility refs.
- `node_capability_record`: node id, hardware class, CPU, memory, GPU, storage, network, benchmark refs, observed capability, claimed capability, and last verified time.
- `native_app_record`: app id, owner id, service refs, catalog visibility, data classes, required APIs, usage refs, ORU-only monetization policy ref, accepted publisher terms version, external-checkout absence attestation, monetization suspension state, payout-hold refs, and policy refs.
- `schema_version_ref`: schema family, schema version, validator version, compatibility class, and migration notes.
- `catalog_entry`: record ref, catalog scope, title or handle ref, purpose tag refs, visibility state, review refs, and takedown refs.

Accepted records must be immutable by content hash. Corrections create new records or new versions.

## API Surface

Phase 1 should expose:

- `POST /v1/manifests`: submit a signed manifest command through Overgate.
- `GET /v1/manifests/{manifest_id}`: read current caller-visible manifest record.
- `GET /v1/manifests/{manifest_id}/versions`: list allowed historical versions.
- `POST /v1/manifests/{manifest_id}/versions`: submit a replacement version that links to the prior accepted version.
- `GET /v1/providers/{provider_id}`: read provider facts visible to the caller.
- `GET /v1/nodes/{node_id}/capabilities`: read allowed node capability facts.
- `GET /v1/packages/{package_id}/versions/{version}`: read package manifest and provenance refs.
- `GET /v1/catalog`: query public or tenant-visible catalog records.
- `GET /v1/admin/registry/records/{record_id}`: operator read with audit and role filtering.

Mutating APIs require Overgate admission, tenant context, owner identity, idempotency key, trace id, schema validation, and Overwatch events.

## Event Surface

Overregistry should emit:

- `overregistry.manifest_submitted`: manifest command accepted for validation.
- `overregistry.manifest_validated`: schema and reference checks passed.
- `overregistry.manifest_rejected`: manifest failed validation or authorization.
- `overregistry.manifest_accepted`: immutable version became active.
- `overregistry.manifest_superseded`: newer version replaced a prior active version.
- `overregistry.provider_registered`: provider record created or versioned.
- `overregistry.node_capability_recorded`: capability record accepted.
- `overregistry.package_recorded`: package manifest accepted.
- `overregistry.catalog_entry_changed`: catalog visibility or refs changed.
- `overregistry.record_deprecated`: record remains readable but should not be used for new work.

Events should include record refs, content hashes, schema versions, owner refs, tenant refs, trace ids, and reason codes. Private manifest data should be represented by refs when possible.

## Core Workflow

1. Overgate admits a signed manifest command and forwards it to Overregistry.
2. Overregistry checks tenant scope, owner identity, manifest type, schema version, and idempotency context.
3. Overregistry validates the manifest using shared schemas and referenced records.
4. Accepted records are written with content hash, version, predecessor link, and audit refs.
5. Updates create new versions; prior accepted versions remain readable and replayable.
6. Downstream services cite registry record ids and versions in decisions.
7. Catalog visibility is updated only through explicit commands and policy refs.

## State Machine

Registry record lifecycle:

1. `submitted`: signed command reached Overregistry.
2. `validating`: schema, owner, tenant, and reference checks are running.
3. `rejected`: record failed validation or authorization before acceptance.
4. `accepted`: immutable version is active for new decisions.
5. `superseded`: newer accepted version replaced this version for new decisions.
6. `deprecated`: version remains valid for replay but should not be selected for new work.
7. `suspended`: policy or tenant action blocks new use while preserving history.
8. `revoked`: record cannot be used for new work because the fact is invalid, unsafe, or withdrawn.
9. `archived`: record is retained for audit and replay but hidden from normal queries.

Content of an accepted version cannot be edited. State transitions add metadata and events.

## Policy And Security

- Manifest mutation requires owner identity, tenant scope, and admitted command evidence.
- Accepted records are immutable by content hash and version.
- Reads must filter by tenant, catalog visibility, role, and data class.
- Package and workload manifests must use secret refs, not raw secrets.
- Monetized app records must preserve ORU-only monetization attestations and must not accept app-level external checkout fields as valid payment facts.
- Provider and node capability records must distinguish claimed facts from verified facts.
- A suspended tenant, revoked owner, or invalid package ref must block new accepted versions.
- Catalog-visible records require explicit visibility state and may be taken down or hidden without deleting audit history.
- Downstream services must cite registry versions so decisions can be replayed.

## Metering And Accounting

Overregistry is not an accounting service. It should:

- Emit usage-relevant events for manifest creation, updates, catalog changes, package records, and capability records.
- Preserve tenant, owner, app, provider, node, package, workload, and trace refs for Overmeter attribution.
- Provide registry version refs to usage, ORU, Seal Ledger, dispute, and receipt records.
- Provide monetization policy refs, accepted terms versions, and bypass enforcement state to Overbill, ORU Account Service, Overguard, Provider Payout Service, Search, Directory, and Native App Catalog.
- Avoid direct ledger mutation or payment integration.
- Keep native-service catalog records compatible with near-cost public utility operation without embedding charge assumptions.

## Observability And Operations

Overregistry should expose:

- Record counts by type, tenant, owner, state, schema version, and visibility.
- Rejected manifest counts by reason code.
- Current versus historical version views.
- Stale provider and node capability reports.
- Package provenance coverage reports.
- Catalog visibility and takedown views.
- Audit search hooks through Overwatch.
- Migration tooling for schema version changes and compatibility backfills.

## Failure Modes And Recovery

- Invalid schema: reject before acceptance and keep validation evidence.
- Duplicate manifest version with same content hash: return prior accepted version.
- Duplicate version with conflicting content hash: reject with conflict reason.
- Missing owner identity or tenant scope: reject before write.
- Downstream consumer sees superseded version: allow replay but block selection for new work if state requires it.
- Package later found unsafe: mark revoked or suspended and emit events without deleting prior decisions.
- Overwatch unavailable: fail closed for accepted-version writes unless a documented buffer is active.
- Schema migration bug: retain old version and compatibility report before accepting new schema family.

## Validation Plan

The service implementation plan lists these specific validation requirements:

- Manifest updates create new versions.
- Scheduler and policy can replay decisions from registry facts.
- Invalid or unsigned manifest changes are rejected.

Additional SDS-level validation:

- Contract tests cover manifest submit, read, version list, supersede, reject, provider record, node capability, package record, and catalog query APIs.
- Immutability tests prove accepted content cannot be edited in place.
- Tenant isolation tests prove private records are not visible across tenants.
- Replay tests reconstruct policy and scheduler inputs from exact registry versions.
- Schema version tests cover compatible, deprecated, and rejected manifests.
- Package and secret-ref tests prove raw secrets are not accepted in manifests.

## Build Breakdown

1. Define registry record, manifest, version, provider, node capability, package, schema ref, native app, and catalog schemas.
2. Implement manifest submission through Overgate-admitted commands.
3. Add strict validation and immutable accepted-version writes.
4. Add current and historical read APIs with tenant filtering.
5. Add provider and node capability records.
6. Add package manifest and provenance refs.
7. Add catalog and purpose tag records in federation and native app phases.
8. Add migration and compatibility reporting for schema changes.

The Phase 1 exit gate is enough registry fact storage for a signed synthetic workload to reference accepted manifests and reach Overqueue.

## Handoff And Downstream Use

Overregistry feeds Overguard, Oversched, Overpack, Oververify, Overqueue, federation, deployment planning, native app catalogs, and public app catalogs.

Downstream services should cite registry ids, versions, content hashes, and schema versions in decisions. If a service needs a new manifest family or catalog visibility rule, update this SDS, the service implementation plan, shared schemas, and the build-plan crosswalk together.

## Open Design Questions

Resolved decisions:

- Phase 1 must define schemas for workload, resource, package, provider, node capability, native app, and schema-ref records so later phases do not invent incompatible registry shapes. The Phase 1 exit gate, however, only requires accepted workload, resource, package, provider, and schema-ref records that let a signed synthetic workload reference an accepted manifest and reach Overqueue. Node capability records may be writable as claimed or draft capability facts once Phase 2 hardware discovery begins, and native app records remain reserved/minimal until Phase 12 catalog and native-service workflows need them.
- Public catalog-safe fields are limited to explicitly published catalog metadata: record ref, manifest family, active version, compatibility class, schema family/version, public title or handle ref, purpose tag refs, owner display ref, visibility state, review/takedown refs, high-level capability class, public app/service refs, and redacted audit or evidence refs. Tenant ids, raw owner identity ids, command ids, trace ids, private audit refs, artifact refs, input/output refs, package dependency/provenance details, node ids, resource locality/capacity details, policy refs, secret refs, verification evidence, dispute refs, and any non-public content hash or record payload stay tenant-private by default unless an explicit catalog command and policy decision publish a redacted view.
- Manifest schema compatibility follows the shared schema package rule: before external clients depend on a manifest family, Phase 1 draft manifests may migrate to the current accepted schema with explicit migration evidence. After SDKs, adapters, mobile clients, native apps, or real execution depend on a stable family, services accept the current stable major plus one previous stable major for new submissions when the compatibility report says the older family is safe. All historical versions remain readable for replay indefinitely through registry/Overwatch evidence, while deprecated, revoked, security-sensitive, accounting-sensitive, signing, policy, secret, ownership, and namespace-breaking schemas are blocked for new use with stable reason codes instead of silent downgrade behavior.
- Before Phase 3 real execution, every executable package manifest must carry package id and version, immutable artifact refs, BLAKE3/content hashes, signature refs, signer or builder identity refs, source/build refs, dependency lock refs, SBOM refs where the runtime class needs them, base image/module refs for OCI or WASI-style execution, runtime contract refs, permission declarations, validation report refs, policy compatibility refs, and the schema/validator/ruleset versions that produced those facts. Phase 3 may use local or stubbed artifact storage behind Overrid-shaped refs, but Overregistry must still store refs and hashes as native Overrid package facts rather than treating a container image name or external registry URL as sufficient provenance.
- Registry revocation propagates by appending a revocation transition and Overwatch event for the exact registry id/version/content hash, then requiring downstream services to re-check registry state at queue readiness, scheduler fetch, lease creation, and runner start. Queued work that references a revoked workload, package, provider, node, or schema version moves to blocked, retry-wait, cancellation, or dead-letter state with user-correctable reason codes when a new signed manifest can fix it. Scheduled, leased, or starting work must cancel or release the lease before execution unless Overguard grants an explicit recovery path. Running work uses severity-based policy: security or secret-risk revocations stop execution and preserve usage/evidence up to termination; lower-risk deprecations may allow the current run to finish while blocking retries and new scheduling. Completed work is never rewritten; it receives revocation/dispute/accounting follow-up refs so audit replay still uses the facts that were active when the work was accepted.
