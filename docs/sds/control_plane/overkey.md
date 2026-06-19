SDS #9

# Overkey SDS

## Purpose

Manage API credentials, signing keys, delegated access, rotation, revocation, and secret references.

Overkey is the credential and key metadata authority for Overrid. It gives Overgate, SDK, CLI, native apps, services, and operators a consistent way to enroll credentials, verify signatures, rotate access, revoke compromised keys, and reference protected secrets without storing raw private material in service records.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [overkey.md](../../service_catalog/control_plane/overkey.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md), [Phase 8: Data, Storage, and Namespace Platform](../../build_plan/phase_08_data_storage_namespace_platform.md) |
| Sub-build plan | [SUB BUILD PLAN #9 - Overkey](../../build_plan/sub_build_plan_009_overkey.md) |

## Service Family

- Family: Control plane.
- Owning layer: Credential lifecycle, signature metadata, delegated access, and secret references.
- Primary data scope: credential records, public key records, API key hashes, service-account key records, revocation records, rotation records, verification logs, and secret refs.
- First build phase from service plan: Overkey-lite in [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md); broader key services in [Phase 8: Data, Storage, and Namespace Platform](../../build_plan/phase_08_data_storage_namespace_platform.md).

## Problem Statement

Overrid cannot rely on hardcoded secrets, informal API keys, or one-off signing implementations. Every command admission, service callback, native app integration, AI assistant request, and mobile service needs verifiable credential state. Overkey provides that state without becoming a raw secret vault, allowing credentials to rotate and revoke while preserving audit history.

## Goals

- Maintain key and credential metadata for people, organizations, nodes, apps, native services, service accounts, and system services.
- Support API key enrollment, signing public key enrollment, service-account key records, and secret-reference links.
- Provide fast verification material to Overgate without exposing private keys.
- Record rotation, revocation, expiry, last-used metadata, trust scope, and tenant scope.
- Preserve audit history for credential lifecycle changes.
- Integrate with Overvault for protected secret material and with Overpass/Overtenant for identity and scope checks.
- Avoid any hardcoded development secret path in production flows.

## Non-Goals

- Do not store raw private keys, seed phrases, passwords, or unencrypted secret values.
- Do not replace Overvault as the protected secret store.
- Do not make authorization decisions that belong to Overtenant or Overguard.
- Do not allow credentials to bypass Overgate admission.
- Do not turn key metadata into transferable speculative assets.
- Do not encode charge tables, customer-count assumptions, or economic projections.

## Primary Actors And Clients

- Overgate verifying signatures and API keys.
- SDK and CLI enrolling and using credential references.
- Native apps and mobile clients using app credentials or delegated user credentials.
- Node agents and workers using service-account credentials.
- Operators rotating or revoking credentials.
- Overvault resolving protected secret refs.
- Overpass and Overtenant validating identity and scope.

## Dependencies

- Overpass identities for credential subjects and actor references.
- Overtenant scope for tenant membership, ownership, and service-account permission.
- Overgate authentication and admission flow.
- Overvault for protected secrets and secret material.
- Shared schema package for credential, key, signature, revocation, audit, and error objects.
- Overwatch event log for lifecycle evidence.
- Overguard for policy constraints on credential class, scope, and delegation once available.

## Owned Responsibilities

Overkey owns:

- Credential records and public verification metadata.
- API key hashes and key identifiers.
- Signing public key records and algorithm metadata.
- Service-account credential records.
- Delegated access credential metadata.
- Rotation plans, active key versions, predecessor/successor links, and grace windows.
- Revocation records and revocation reason codes.
- Last-used metadata supplied by Overgate.
- Secret-reference contracts with Overvault.
- Verification helper APIs for Overgate and trusted internal services.

Overkey does not own raw secret storage, business authorization, tenant membership, or request admission.

## Data Model

The first implementation should define these records:

- `credential_record`: credential id, subject id, subject type, tenant id, credential class, status, allowed use, created by, created time, expiry time, and audit refs.
- `api_key_record`: credential id, key id, hash algorithm, key hash, prefix hint, scope, status, last used time, and rotation group.
- `public_key_record`: credential id, key id, algorithm, public key material or JWK ref, canonicalization version, allowed signature use, not-before time, not-after time, and status.
- `service_account_key`: service account id, tenant id, allowed services, allowed command classes, rotation policy, and status.
- `delegation_record`: delegator id, delegate id, tenant id, allowed scopes, expiry, revocation state, and evidence refs.
- `rotation_record`: rotation id, old credential id, new credential id, grace window, initiated by, state, reason, and audit refs.
- `revocation_record`: credential id, revoked by, revoked time, reason code, effective time, affected command classes, and audit refs.
- `verification_result`: credential id, key version, verified or denied state, reason code, replay window result, and dependency version refs.
- `secret_ref`: Overvault ref, secret class, allowed resolver services, rotation policy, and access audit refs.

Status values must be explicit: `pending`, `active`, `rotating`, `suspended`, `revoked`, `expired`, and `tombstoned`.

## API Surface

Phase 1 should keep the API small:

- `POST /v1/credentials/api-keys`: enroll an API key hash and metadata.
- `POST /v1/credentials/signing-keys`: enroll a signing public key.
- `POST /v1/credentials/service-accounts`: create or attach service-account credentials.
- `POST /v1/credentials/{credential_id}/rotate`: start a rotation plan.
- `POST /v1/credentials/{credential_id}/revoke`: revoke a credential with reason and evidence.
- `GET /v1/credentials/{credential_id}`: read caller-visible credential metadata.
- `POST /v1/verify/signature`: internal verification helper for Overgate.
- `POST /v1/verify/api-key`: internal API key verification helper for Overgate.
- `POST /v1/usage/last-used`: internal last-used update from Overgate.
- `GET /v1/healthz` and `GET /v1/readyz`: liveness and dependency readiness.

Admin APIs must be signed, tenant-scoped, and audited. Internal verification APIs must be restricted to service accounts.

## Event Surface

Overkey should emit these Overwatch-compatible events:

- `overkey.credential_requested`: enrollment request received.
- `overkey.credential_created`: credential metadata created.
- `overkey.credential_activated`: credential becomes usable.
- `overkey.credential_rotation_started`: rotation plan opened.
- `overkey.credential_rotated`: successor credential activated and linked.
- `overkey.credential_suspended`: credential temporarily blocked.
- `overkey.credential_revoked`: credential permanently denied.
- `overkey.credential_expired`: expiry reached.
- `overkey.verification_denied`: verification failed with stable reason code.
- `overkey.secret_ref_bound`: credential or service account bound to an Overvault secret ref.

Events must not include raw secret values or private keys.

## Core Workflow

1. Overgate submits a signed verification request using a service account.
2. Overkey loads credential metadata by credential id or key id.
3. Overkey checks status, expiry, tenant scope, allowed use, rotation window, revocation state, and subject identity refs.
4. Overkey validates signature or API key hash using public metadata.
5. Overkey returns a typed verification result with reason code, subject refs, key version, and audit refs.
6. Overgate completes actor and tenant admission with Overpass and Overtenant.
7. Overgate reports last-used metadata after successful admission.

Enrollment, rotation, and revocation use the same signed-command path through Overgate and produce lifecycle events.

## State Machine

Credential lifecycle state:

1. `pending`: credential request exists but is not usable.
2. `active`: credential can verify allowed requests.
3. `rotating`: replacement exists or is being enrolled while old credential may remain valid inside a grace window.
4. `suspended`: credential is temporarily denied pending operator, policy, or owner action.
5. `revoked`: credential is permanently denied from its effective time.
6. `expired`: credential passed its not-after or expiry time.
7. `tombstoned`: credential metadata remains for audit but cannot be restored.

Verification result state:

1. `verified`: all checks passed.
2. `denied`: credential, signature, scope, expiry, revocation, tenant, or algorithm check failed.
3. `blocked`: dependency or policy state prevents a reliable decision.

Credential history must be append-only. Rotation must link old and new credential records instead of overwriting identity.

## Policy And Security

- Private keys and raw secrets are never stored in Overkey records.
- API keys are stored only as strong hashes with non-secret prefixes for lookup.
- Verification APIs are internal and require service-account authentication.
- Credential scope must include tenant, subject, allowed use, and command class boundaries.
- Revoked credentials must fail verification immediately after revocation becomes effective.
- Rotation grace windows must be explicit and auditable.
- Weak algorithms, unknown canonicalization versions, and missing expiry metadata should be denied by default.
- Test credentials must be environment-scoped and impossible to use against production endpoints.
- Delegated credentials require delegator, delegate, scope, expiry, and revocation evidence.

## Metering And Accounting

Overkey is not an accounting service. It should:

- Emit usage-relevant events for verification volume, credential lifecycle operations, and operator actions.
- Preserve actor, tenant, service account, and command-class refs so Overmeter can attribute control-plane usage later.
- Avoid direct ORU or Seal Ledger mutation.
- Provide credential refs needed by services that later perform chargeable work.
- Keep lifecycle APIs independent from charge assumptions.

## Observability And Operations

Overkey should expose:

- Credential counts by status, type, tenant, and subject type.
- Verification success and denial counts by reason code.
- Revocation and rotation latency metrics.
- Expiring credential reports.
- Last-used views for stale credentials.
- Audit search hooks through Overwatch.
- Readiness checks for Overpass, Overtenant, Overvault, schema package, and local storage.
- Migration tooling for algorithm upgrades and schema changes.

## Failure Modes And Recovery

- Unknown credential id: deny with stable reason code.
- Revoked, suspended, expired, or tombstoned credential: deny.
- Public key algorithm unsupported: deny and require rotation.
- Overvault unavailable during secret-ref binding: keep credential pending or blocked.
- Overpass subject missing or suspended: deny verification or block activation.
- Rotation successor invalid: keep old credential state unchanged unless revoked by policy.
- Last-used update fails: do not fail admission if verification already succeeded; retry telemetry update with audit-safe queueing.
- Compromised credential: revoke, emit audit event, invalidate verification cache, and expose affected tenant/subject refs to operators.

## Validation Plan

The service implementation plan lists these specific validation requirements:

- Revoked credentials cannot authenticate.
- Rotated credentials preserve audit history.
- Services verify signatures from key records rather than hardcoded secrets.

Additional SDS-level validation:

- Contract tests cover enrollment, verification, rotation, revocation, last-used update, and secret-ref binding.
- Signature tests cover algorithm, canonicalization, body hash, timestamp, replay window, tenant, and command-class checks.
- API key tests prove raw key values are not persisted and wrong keys cannot verify.
- Rotation tests prove grace windows and predecessor/successor links are auditable.
- Tenant isolation tests prove one tenant cannot inspect or revoke another tenant's credentials.
- Dependency-failure tests prove fail-closed verification for missing identity, tenant, or key metadata.

## Build Breakdown

1. Define credential, API key hash, signing public key, revocation, rotation, and verification schemas.
2. Implement Overkey-lite storage and enrollment for Phase 1 service accounts and SDK/CLI keys.
3. Add internal verification APIs for Overgate.
4. Add revocation and rotation metadata with audit events.
5. Add last-used updates and operator read models.
6. Integrate Overvault secret refs for protected material in later phases.
7. Add delegated access, advanced key classes, and namespace-aware key services in [Phase 8: Data, Storage, and Namespace Platform](../../build_plan/phase_08_data_storage_namespace_platform.md).

The Phase 1 build must remove hardcoded dev secrets from the synthetic command flow.

## Handoff And Downstream Use

Overkey supports Overgate, Overrun secret mounts, Overvault access, SDK, CLI, native apps, mobile services, and signed operator actions.

Downstream services should request verification through Overgate or approved internal service accounts. If a new credential class is needed, update this SDS, the shared schema package, the service implementation plan, and the build-plan crosswalk together.

## Open Design Questions

Resolved decisions:

- The first seed-hardware implementation should approve Ed25519 for signed command envelopes, service-account signatures, node enrollment signatures, and operator/admin commands. BLAKE3 should be used for canonical body hashes, key fingerprints, and evidence/hash refs; rustls/mTLS protects transport but does not replace command signatures. Phase 1 should not add RSA, ECDSA, JWT-only, or external KMS-specific signing as required command-authority paths. Future algorithms require schema-versioned `algorithm`, `canonicalization_version`, fixtures, compatibility tests, and explicit migration records before they can verify production commands.
- Key lookup should use explicit non-secret identifiers rather than raw key material. Signed requests carry `tenant_id`, `credential_id` or `key_id`, `algorithm`, `key_version`, `canonicalization_version`, timestamp, replay window fields, and the canonical body hash. Public signing keys are looked up by tenant-scoped credential/key id plus status/version; API keys use a short non-secret prefix only for routing and a server-side strong lookup hash for verification. Logs, traces, denial events, and audit exports may include ids, prefixes, fingerprints, and reason codes, but never raw API keys, bearer values, private keys, or reusable secret material.
- Production operator/admin credentials, emergency revocation credentials, system-service credentials, node enrollment keys, service-account keys that can affect queue execution, vault access, accounting, rights, payout, namespace, or policy state, and later mobile/native-app recovery credentials require a recorded protection class. Acceptable production protection classes are host keychain, hardware-backed key, TPM/secure-enclave-style signer, or another Overkey/Overvault-approved non-exporting signer. Phase 1 may use local file-backed test keys only for loopback development and seed smoke tests, and those credentials must be environment-scoped, visibly marked as test-only, and impossible to use against production endpoints.
- Verification caches are an optimization only. Once revocation support is active, positive verification cache entries must be keyed by tenant, credential id, key version, allowed use, command class, canonicalization version, and revocation epoch, and should live for at most 30 seconds for ordinary Phase 1 credentials. High-risk credentials such as operator/admin, emergency, system-service, vault, accounting, rights, payout, namespace, and policy-override credentials should either bypass positive caching or use a maximum 5-second cache. Denial and unknown-key caches should be short, at most 5 seconds, to avoid pinning recovery. Rotation, suspension, revocation, expiry, tenant suspension, or algorithm deprecation must bump the relevant credential/revocation epoch and invalidate Overgate caches immediately; if invalidation or Overwatch evidence is unavailable, high-risk verification fails closed and ordinary verification falls back to a fresh Overkey lookup.
- Emergency credential revocation at grid scale uses a signed, idempotent break-glass command through Overgate, issued by an operator/admin credential with the required protection class and role. The command records target credential ids or scopes, tenant and subject refs, reason code, effective time, affected command classes, incident/evidence refs, and expected current state. Overkey writes an append-only `revocation_record`, emits `overkey.credential_revoked` and cache-invalidation events to Overwatch, bumps the revocation epoch, and exposes propagation status for Overgate, Overvault, Overqueue, Oversched, Overcell, and relevant system services. Until propagation is confirmed, protected commands using affected credentials fail closed; operators get a traceable revocation report, affected-subject inventory, and follow-up rotation/re-enrollment tasks rather than direct mutable database edits.
