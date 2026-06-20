SDS #29

# Overvault SDS

## Purpose

Provide secure storage and access control for sensitive material: secrets, encrypted private records, key policy metadata, secret refs, mount leases, access decisions, escrowed records, and protected app state.

Overvault is the vault boundary. It keeps private material behind explicit access decisions and audited release paths. It does not replace Overkey identity/key metadata, Overbase structured state, or Overstore object persistence.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [overvault.md](../../service_catalog/data_storage_namespace/overvault.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 8: Data, Storage, and Namespace Platform](../../build_plan/phase_08_data_storage_namespace_platform.md) |
| Sub-build plan | [SUB BUILD PLAN #29 - Overvault](../../build_plan/sub_build_plan_029_overvault.md) |

## Service Family

- Family: Data, storage, and namespace
- Owning layer: Secret refs, encrypted private records, vault access decisions, mount leases, rotation, revocation, and escrow controls
- Primary data scope: secret records, secret versions, encrypted records, access policies, access grants, access requests, mount leases, key policy refs, rotation jobs, revocation records, escrow records, and access audit refs
- First build phase from service plan: [Phase 8: Data, Storage, and Namespace Platform](../../build_plan/phase_08_data_storage_namespace_platform.md), with minimal secret references earlier for Overrun

## Problem Statement

Overrid workloads and native apps need secrets and private state, but normal app databases and object stores should not become places where secret material leaks into logs, indexes, backups, or result records. Overrun also needs a way to mount secrets for workloads without exposing raw values to the scheduler, queue, or audit stream.

Overvault provides that boundary. It makes every secret release, mount, read, rotation, revocation, and escrow operation policy-checked and auditable.

## Goals

- Store secret and encrypted-record metadata with versioned policy refs.
- Issue secret refs and mount leases without exposing secret values to unrelated services.
- Support encrypted private records for user, organization, app, and regulated scopes.
- Keep access decisions explicit, replayable, and visible to Overwatch.
- Integrate with Overkey for credential and key metadata while keeping responsibilities separate.
- Support secret rotation, revocation, ttl, expiry, and emergency quarantine.
- Provide minimal Phase 3 secret refs for Overrun, then full Phase 8 vault behavior.

## Non-Goals

- Do not own user identity or credential enrollment. Overpass and Overkey own those boundaries.
- Do not act as general structured-state storage. Overbase owns app state.
- Do not act as general object storage. Overstore owns object bytes and replicas.
- Do not expose raw secret values in logs, audit events, scheduler records, run results, or diagnostics.
- Do not allow apps to read secrets only because they can call a storage API.
- Do not create opaque privileged bypasses for operators; emergency access still requires signed policy and evidence.

## Primary Actors And Clients

- Overrun, requesting policy-approved secret mount leases for workloads.
- Native apps, storing and reading private settings, credentials, tokens, and sensitive app records.
- Personal AI and encrypted Docdex RAG, accessing private context refs only when authorized.
- Overkey, providing credential and key metadata refs.
- Overguard, deciding access, data class, retention, workload, and emergency policy.
- Overbase and Overstore, storing refs to private records or encrypted objects without owning vault access.
- Admin/developer UI, CLI, and SDK, managing vault entries through signed actions.
- Overwatch and Overmeter, consuming audit and usage refs.

## Dependencies

- [Overkey](../control_plane/overkey.md) for credential refs, signing refs, key metadata, rotation signals, and service-account credential state.
- [Overpass](../control_plane/overpass.md) for identity refs, stable account refs, and namespace-linked subjects.
- [Overtenant](../control_plane/overtenant.md) for tenant, membership, role, suspension, and offboarding context.
- [Overguard](../trust_policy_verification/overguard.md) for data-class, access, workload, emergency, retention, and escrow policy decisions.
- [Overwatch](../control_plane/overwatch.md) for append-only access evidence and incident refs.
- [Overmeter](../execution_scheduling/overmeter.md) for raw vault operation usage.
- [Overbase](overbase.md) for structured refs to private app records where the encrypted payload remains vault-controlled.
- [Overstore](overstore.md) for encrypted object bytes when large private objects need content-addressed storage.
- [Overmesh](../execution_scheduling/overmesh.md) for secure node or service delivery paths where required.

## Owned Responsibilities

Overvault owns:

- Secret record, secret version, encrypted private record, and vault policy refs.
- Access request, access decision, access grant, and denial records.
- Workload secret mount lease creation, ttl enforcement, and revocation.
- Secret rotation, version retirement, revocation, quarantine, and deletion tombstones.
- Escrowed record workflows where disputes, compliance, or regulated workflows require controlled release.
- Redaction rules for logs, diagnostics, audit events, and result records.
- Vault usage events for reads, writes, mount leases, rotations, revocations, and denied accesses.

Overvault must keep the access decision separate from the caller. A caller can request access; only policy and valid identity/tenant context can grant it.

## Data Model

The first implementation should define:

- `secret_record`: secret id, tenant id, owner ref, app/service scope, secret kind, data class, current version, allowed subjects, retention policy, rotation policy, and state.
- `secret_version`: secret id, version id, encrypted payload ref, key policy ref, created-by ref, created-at, activation time, expiry, revoked-at, and checksum.
- `encrypted_record`: record id, subject scope, encrypted payload ref or bounded encrypted value, schema ref, data class, retention policy, and searchability flag.
- `vault_access_policy`: policy id, subject refs, allowed actor/service refs, workload class, data class, purpose, ttl, release mode, and emergency rule.
- `access_request`: request id, caller ref, target secret/record, purpose, workload/run refs, tenant/app refs, requested ttl, and trace id.
- `access_decision`: request id, decision, policy refs, reason code, allowed fields, denied fields, ttl, redaction rule, and audit refs.
- `access_grant`: grant id, target ref, grantee ref, scope, ttl, allowed operation, issuance channel, revocation state, and audit refs.
- `mount_lease`: lease id, run id, node id, secret refs, mount path policy, process scope, ttl, renewal rule, revocation hook, and cleanup evidence.
- `rotation_job`: target secret, source version, target version, rotation reason, rollout state, dependent refs, rollback eligibility, and completion evidence.
- `revocation_record`: target ref, reason, actor/policy refs, effective time, dependent grants, cleanup state, and evidence refs.
- `escrow_record`: escrow id, protected subject, release policy, required evidence, requester refs, review state, release result, and audit refs.

Common envelope fields:

- `id`, `tenant_id`, `app_id`, `actor_id` or service account.
- `trace_id`, `idempotency_key`, `state`, `created_at`, `updated_at`, `policy_refs`, and `audit_refs`.

## API Surface

The v0 API should make access explicit:

- `POST /overvault/secrets`: create a secret record and first encrypted version.
- `POST /overvault/secrets/{secret_id}/versions`: add a rotated or replacement version.
- `GET /overvault/secrets/{secret_id}`: read metadata only, with redaction.
- `POST /overvault/access-requests`: request access to a secret or encrypted record.
- `POST /overvault/access-requests/{request_id}/decide`: internal policy-decision handoff from Overguard.
- `POST /overvault/grants`: create a bounded access grant after an allowed decision.
- `POST /overvault/mount-leases`: issue a workload mount lease for Overrun.
- `POST /overvault/mount-leases/{lease_id}/revoke`: revoke a mount lease and trigger cleanup.
- `POST /overvault/secrets/{secret_id}/rotate`: start a rotation job.
- `POST /overvault/secrets/{secret_id}/revoke`: revoke a secret or version.
- `POST /overvault/encrypted-records`: store a private encrypted record.
- `GET /overvault/encrypted-records/{record_id}`: read a private encrypted record through policy.
- `POST /overvault/escrow`: create or request an escrowed release workflow.

API requirements:

- Metadata reads must not expose secret values or sensitive encrypted payloads.
- Release APIs must return through approved delivery channels, not normal logs or audit payloads.
- Mount leases require run, node, workload, tenant, data-class, and policy refs.
- Access grants and mount leases must have ttl and revocation behavior.
- Emergency access must create high-severity Overwatch evidence.

## Event Surface

- `overvault.secret_created`
- `overvault.secret_version_added`
- `overvault.access_requested`
- `overvault.access_allowed`
- `overvault.access_denied`
- `overvault.grant_issued`
- `overvault.grant_revoked`
- `overvault.mount_lease_issued`
- `overvault.mount_lease_revoked`
- `overvault.secret_rotated`
- `overvault.secret_revoked`
- `overvault.encrypted_record_written`
- `overvault.escrow_requested`
- `overvault.escrow_released`
- `overvault.quarantine_applied`

Events must include target refs, actor/service refs, policy refs, reason codes, ttl, and trace refs. They must never include raw secret values or decrypted private payloads.

## Core Workflow

1. Actor or service creates a secret or encrypted record through Overgate and Overvault.
2. Overvault validates identity, tenant, app/service scope, schema, data class, and policy refs.
3. Secret payload is encrypted and stored as a version with key policy refs.
4. A caller requests access with purpose, subject, workload/run refs, ttl, and trace id.
5. Overguard evaluates the request and returns allow/deny with reason codes and policy refs.
6. Overvault issues an access grant or mount lease only when policy allows it.
7. Overrun or authorized app receives the secret through the approved channel and scope.
8. Grants and mount leases expire or are revoked; cleanup evidence is recorded.
9. Rotation, revocation, quarantine, escrow, and deletion workflows preserve audit and retention refs.

## State Machine

Secret lifecycle:

1. `draft`: metadata is being assembled.
2. `active`: current version can be requested under policy.
3. `versioning`: new version is being added or activated.
4. `rotating`: rotation job is active.
5. `revoking`: revocation is propagating to grants and mount leases.
6. `revoked`: secret or version cannot be newly released.
7. `expired`: ttl or retention policy ended normal use.
8. `quarantined`: access is blocked by incident, policy, or dispute.
9. `escrowed`: release requires escrow workflow.
10. `tombstoned`: logical deletion is recorded.
11. `purged`: physical cleanup is complete where policy allows.

Access grant lifecycle:

1. `requested`
2. `policy_checked`
3. `allowed`
4. `issued`
5. `active`
6. `expired`
7. `revoked`
8. `denied`

Secret versions and access decisions are append-only. Revocation creates new records and does not erase prior access evidence.

## Policy And Security

- Raw secret values must not appear in API logs, scheduler records, run results, Overwatch payloads, or diagnostics.
- Every release requires actor/service identity, tenant/app scope, purpose, data class, ttl, and policy refs.
- Mount leases must be scoped to a run, node, process scope, path policy, ttl, and cleanup behavior.
- Secret-bearing workloads must not run unless Overguard and Overvault both allow the mount.
- Emergency access requires explicit policy, signed operator or stewardship action, high-severity audit evidence, and post-action review.
- Key metadata and credential lifecycle come from Overkey; Overvault controls encrypted secret material and access records.
- Backups must preserve encrypted payloads and key policy refs without broadening access.
- Tenant offboarding must revoke grants and mount leases before normal private data access is removed.

## Metering And Accounting

Overvault emits raw usage events; it does not bill:

- Secret writes, version writes, encrypted record writes, access requests, access decisions, grants, mount leases, rotations, revocations, escrow workflows, and denied requests.
- Link usage to tenant, app/service, actor/service account, data class, target kind, policy refs, and operation state.
- Denied and emergency operations should be visible for audit and risk review, not hidden.
- Overvault never mutates ORU balances, Seal Ledger entries, provider payouts, or external payment systems.

## Observability And Operations

- Operators need counts for active secrets, active grants, active mount leases, denied requests, rotations due, revoked grants, quarantined records, escrow workflows, and emergency accesses.
- Health checks should cover encryption/decryption path, key policy refs, Overkey, Overguard, Overwatch, Overmeter, storage backend, and mount-lease delivery path.
- Audit views must show who requested access, why, which policy allowed or denied it, and when grants expired or were revoked.
- Redaction checks should run continuously against logs and diagnostics.
- Rotation jobs need progress, dependency refs, rollback state, and failed-dependent cleanup visibility.

## Failure Modes And Recovery

- Missing identity, tenant, purpose, or data class: deny before side effects.
- Policy ambiguity: deny or block; never release by default.
- Key policy unavailable: block access and record dependency failure.
- Secret version corruption: quarantine version, preserve metadata, and require rotation or restore.
- Grant delivery failure: keep grant pending or revoke according to policy; do not retry with broader scope.
- Mount cleanup failure: revoke lease, notify Overrun/Overwatch, and schedule cleanup review.
- Rotation failure: keep prior active version if policy allows, or quarantine target if compromise is suspected.
- Emergency access misuse: quarantine target, revoke grants, and create incident refs.

## Validation Plan

The service implementation plan lists these requirements:

- Secrets are never exposed to workloads without policy approval.
- Access decisions are auditable.
- Private app state remains tenant/user scoped.

Additional SDS-level validation:

- Contract tests for secret create, version, metadata read, access request, access decision, grant, mount lease, revoke, rotate, encrypted record, and escrow APIs.
- Redaction tests across API logs, Overwatch events, Overrun logs, run results, diagnostics, and export paths.
- Tenant/user/app isolation tests for private records and secret refs.
- Mount lease tests for valid, expired, revoked, wrong-node, wrong-run, and cleanup-failure cases.
- Rotation and revocation tests covering dependent grants and mount leases.
- Emergency access and escrow tests with evidence and post-action review refs.
- Usage emission tests for allowed, denied, revoked, rotated, and emergency operations.

## Build Breakdown

1. Define secret, secret version, encrypted record, access policy, access request, decision, grant, mount lease, rotation, revocation, and escrow schemas.
2. Implement metadata-only secret refs and encrypted payload versioning.
3. Add access request and Overguard decision flow.
4. Add bounded access grants and workload mount leases for Overrun.
5. Add rotation, revocation, ttl expiry, and cleanup hooks.
6. Add encrypted private record API for native apps and personal AI.
7. Add escrow and emergency access workflows.
8. Add continuous redaction and audit validation checks.

## Handoff And Downstream Use

Overvault supports Overrun secret mounts, personal AI private context, workspace private settings, messaging protected data, mobile app secrets, regulated workflows, and native app private records. Downstream services should store Overvault refs and access-decision refs, not raw private material.

## Open Design Questions

Resolved decisions:

- The first founder-hardware deployment must use Overvault-owned envelope encryption before persistence or delivery. Secret values and private records use per-secret, per-record, or tightly scoped data-encryption keys, while key-encryption and release authority stay in founder-controlled custody such as a host keychain, TPM/secure-enclave-style signer, hardware-backed key, or other Overkey/Overvault-approved non-exporting signer. File-backed keys are allowed only for loopback development and explicit Phase 3 stub profiles. Overkey stores credential metadata, public verification material, key policy refs, rotation refs, and revocation refs; it does not store raw private material. Overvault stores encrypted payload refs, encryption envelope refs, BLAKE3 fingerprints/checksums, access policy refs, rotation state, and Overwatch evidence. External Vault/KMS-style systems may be benchmark or bridge adapters later, but they are not the Overvault product boundary.
- Before full grid-resident Overvault is available, Overrun may receive secrets only through an explicit `founder_local_secret_ref` delivery profile mediated by Overcell on founder hardware. The delivery channel is a signed, lease-bound, node-local mount or file-descriptor style handoff with ttl, least-scope process visibility, redaction policy, unmount/zeroization evidence, cleanup evidence, and a recorded migration ref to Overvault. Raw secret values must never enter Overqueue, Oversched, Overwatch payloads, scheduler reason records, run results, logs, diagnostics, environment dumps, or normal artifact storage. Public/provider pools, regulated workloads, third-party secrets, and assignments whose policy requires Overvault must fail Overrun preflight rather than silently falling back to this stub.
- Central AI stewardship may request or recommend emergency access, but it cannot self-authorize secret release or receive raw vault material by default. Emergency release requires a signed break-glass command through Overgate, an Overguard emergency policy decision, a protected operator/steward credential class, explicit target refs, purpose, data class, ttl, allowed fields, redaction profile, incident/evidence refs, and high-severity Overwatch evidence. Severe or privacy-invasive releases require steward or appeal-body review unless a narrowly defined automated safety policy already permits the exact action. Every emergency grant expires, creates post-action review work, supports correction/retraction records, and must be visible to the owning service and dispute/compliance workflow.
- Searchable private encrypted records must use separate, access-scoped search projections rather than indexing raw private content into shared indexes. By default, encrypted private records are not full-text searchable. When search is required, the owning app or authorized service creates schema-declared projections containing only permitted refs, hashes, redacted fields, tokenized terms, or embeddings whose data class and access scope match the source record. Overbase/vector indexes and any encrypted Docdex RAG indexes must pre-filter by tenant, app, actor/service account, data class, grant, revocation state, and purpose before exposing candidates. Embeddings derived from private content are themselves private/secret-bearing artifacts; they must not be reused in global indexes, cross-tenant search, public discovery, or central AI analysis without explicit policy and evidence refs. Revocation, retention expiry, and offboarding must tombstone or rebuild the projection.
- Regulated workloads are not admitted until Overvault supports escrow workflows for regulated evidence holds, dispute/incident evidence preservation, compliance-bound release requests, user or organization recovery, and emergency safety access. Each escrow workflow must define protected subject refs, required evidence, requester and approver roles, release policy, redaction rules, appeal path, retention or legal-hold behavior, ttl, revocation behavior, and Overwatch audit refs before any release. Central AI may analyze scoped evidence packages or recommend a release path, but final release still belongs to Overvault plus the owning compliance, dispute, stewardship, or recovery workflow. Missing escrow policy, missing appeal path, or missing redaction profile is a deny/block condition for regulated workload admission.
