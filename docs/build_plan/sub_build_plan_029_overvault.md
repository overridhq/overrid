# SUB BUILD PLAN #29 - Overvault

Attached SDS: [docs/sds/data_storage_namespace/overvault.md](../sds/data_storage_namespace/overvault.md)

## Purpose

This sub-build plan turns SDS #29 into an implementation sequence for Overvault. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Overvault is the native vault boundary for secrets, encrypted private records, key policy refs, access requests, access decisions, grants, mount leases, rotation, revocation, escrow, emergency access, redaction, and protected app state. It is not identity, credential metadata, structured app state, object persistence, policy authority, audit storage, accounting mutation, or a conventional Vault/KMS product boundary.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #29: Overvault](../sds/data_storage_namespace/overvault.md) | Controls Overvault purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machine, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Overvault service plan](../service_catalog/data_storage_namespace/overvault.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical build order from master Phase 0 through master Phase 13. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared schemas, canonical JSON/JSON Schema discipline, signed envelopes, idempotency, trace ids, stable reason codes, local fixtures, deterministic harnesses, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overgate, Overpass, Overtenant, Overkey, Overwatch, Overqueue, identity, tenant, key, command, credential, and audit primitives that Overvault consumes. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies Overrun secret-ref consumers and the narrow founder-local secret delivery stub that must migrate into full Phase 8 Overvault behavior. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard data-class, access, workload, egress, emergency, retention, escrow, dispute, and verification policies consumed by Overvault. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Consumes Overvault raw usage through Overmeter while keeping rollups, pricing, ledger mutation, billing, settlement, payout, and external payment rails outside Overvault. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies protected system-service operation, backup/restore patterns, failover/recovery expectations, migration tooling, and grid-resident service readiness prerequisites. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Controls Overvault's first full build point as the private encrypted-state and secret-storage member of the native data/storage/namespace platform. |
| [Phase 9: Overpack Deployment Platform](phase_09_overpack_deployment_platform.md) | Supplies application-intent secret refs, deployment-time private settings, package/runtime secret requirements, and provisioning handoffs without moving deployment semantics into Overvault. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Enforces deny-by-default public-provider behavior for private, regulated, and secret-bearing workloads and prevents silent fallback to public placement. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Consumes Overvault private records, app credentials, personal AI private context refs, encrypted RAG access scopes, workspace private settings, messaging protected data, and mobile app secrets. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies retention, compliance hold, incident, threat-model, audit-export, migration, PIP, and governance hardening for vault contracts, escrow, emergency access, and protected release review. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #29 first full build work aligned to master Phase 8, with minimal Phase 3 secret-ref stubs and later native-app/governance gates. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf, Ed25519 signatures, BLAKE3/content hashes, signed envelopes, native Overrid service boundaries, encryption-before-placement, and no conventional vault/KMS product boundary. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 3, 4, 5, 7, 8, 11, 12, and 13 | Attach SDS #29, freeze Overvault scope, preserve Phase 8 as the first full build point, and record prerequisite plus later public-provider/native/governance gates. |
| 2 | Master Phases 0, 1, 4, and 8 | Build Rust contracts, JSON Schemas, envelope metadata, redaction contracts, state machines, fixtures, and harnesses before vault side effects. |
| 3 | Master Phases 1, 4, 7, and 8 | Implement secret records, secret versions, encrypted-record metadata, envelope persistence, metadata-only reads, and append-only version history. |
| 4 | Master Phases 1, 4, 5, 8, and 13 | Implement explicit access requests, Overguard decisions, bounded grants, denial records, audit refs, and raw usage events. |
| 5 | Master Phases 3, 4, 7, 8, and 11 | Implement minimal Phase 3 founder-local secret refs and full Phase 8 mount leases for Overrun without exposing raw secrets to queues, schedulers, logs, or public pools. |
| 6 | Master Phases 4, 5, 8, and 13 | Implement rotation, revocation, ttl expiry, quarantine, tombstones, cleanup hooks, and dependency evidence for grants and mount leases. |
| 7 | Master Phases 6, 8, 12, and 13 | Implement encrypted private records, access-scoped search projections, personal AI/private RAG refs, native app private data, and projection tombstone/rebuild paths. |
| 8 | Master Phases 4, 8, 12, and 13 | Implement escrow, emergency access, break-glass, regulated evidence holds, appeal/review paths, and post-action correction records. |
| 9 | Master Phases 5, 7, 8, 9, 12, and 13 | Integrate Overwatch, Overmeter, Overkey, Overguard, Overrun, Overbase, Overstore, native apps, SDK/CLI/admin surfaces, operations dashboards, and backup/restore handoffs. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, tech-stack alignment, queue state, progress evidence, negative controls, and implementation handoff gates. |

## Tech Stack Guardrails

- Overvault core is a Rust service/module using shared contract types, Tokio for async rotation/revocation/cleanup workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Overvault contracts use canonical JSON plus JSON Schema for docs-facing examples, fixtures, access policies, requests, decisions, grants, mount leases, escrow workflows, emergency records, events, reason codes, and replay reports. Compact Protobuf contracts may be added only through the shared contract layer where needed.
- Mutating calls use signed envelopes, tenant/app context, actor or service-account identity, idempotency keys, trace ids, schema versions, stable reason codes, data-class refs, policy refs, ttl, and append-only audit events.
- Ed25519 is used where signatures are required. BLAKE3/content fingerprints are used for encrypted payload refs, envelope integrity, redacted replay bundles, fixture commitments, and evidence checksums.
- The v0 founder-hardware backend uses Overvault-owned envelope encryption before persistence or delivery. Raw private material stays behind non-exporting founder-controlled custody where available; file-backed keys are allowed only for loopback development and explicit stub profiles.
- Overkey owns identity, credential metadata, public verification material, key policy refs, rotation refs, and revocation refs. Overvault owns encrypted payload refs, envelope refs, access records, release decisions, grants, leases, and protected material lifecycle.
- External Vault/KMS-style systems may be benchmark baselines or bridge adapters later, but they are not the Overvault product boundary.
- Raw secret values and decrypted private payloads must never enter Overqueue, Oversched, Overwatch payloads, Overmeter events, run results, logs, diagnostics, normal artifact storage, normal object listings, or shared indexes.
- PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFT, pricing, revenue, customer-count, or external-payment assumptions must not become Overvault's product boundary.
- TypeScript is limited to generated bindings and operator/developer UI surfaces. The core vault runtime, redaction checks, release workers, and contract validation stay Rust-first.

## Phase 1: SDS Attachment, Vault Scope, And Phase Gates

### Work Items

- **1.1 Attach the build plan to SDS #29.**
  - Design: Link this document from the numbered Overvault SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/data_storage_namespace/overvault.md`, `docs/service_catalog/data_storage_namespace/overvault.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #29 returns both the Overvault SDS and this sub-build plan.

- **1.2 Freeze Overvault as the vault authority.**
  - Design: Record that Overvault owns secret records, secret versions, encrypted private records, vault access policies, access requests, access decisions, grants, mount leases, rotation jobs, revocation records, escrow records, redaction rules, tombstones, and vault usage facts.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms Overvault does not own identity, credential enrollment, structured app state, object bytes, package semantics, namespace ownership, policy rule authoring, audit storage, ORU balances, Seal Ledger entries, invoices, payouts, or billing state.

- **1.3 Preserve master Phase 8 as the first full build point.**
  - Design: Keep full implementation in master Phase 8 because Overvault depends on signed identity/tenant/request/audit rails, policy controls, raw usage emission, encrypted storage primitives, and protected grid readiness.
  - Output: Phase-gate note that Phase 3 may use only the explicit founder-local secret-ref stub while Phase 8 proves native vault behavior.
  - Validation: Review proves this plan does not move full Overvault into earlier master phases and does not change the master Phase 0 through Phase 13 order.

- **1.4 Carry forward resolved SDS decisions.**
  - Design: Preserve SDS #29 decisions for Overvault-owned envelope encryption, Overkey metadata separation, founder-local secret refs mediated by Overcell, explicit emergency access review, access-scoped private search projections, and escrow requirements before regulated workload admission.
  - Output: Resolved-decision checklist tied to SDS #29 open-question answers.
  - Validation: Review rejects external Vault/KMS product boundaries, raw secret fallback into scheduler paths, self-authorized central AI release, shared private indexes, and regulated workload admission without escrow policy.

- **1.5 Define vault authority boundaries.**
  - Design: Create a boundary matrix for Overgate, Overpass, Overtenant, Overkey, Overguard, Overwatch, Overmeter, Overrun, Overcell, Overmesh, Overbase, Overstore, Overclaim, Oververify, Challenge Task Service, native apps, personal AI, SDK, CLI, admin UI, compliance services, and governance services.
  - Output: Boundary matrix listing consumed refs, emitted refs, denial behavior, retry owner, redaction profile, release channel, and replay evidence for each dependency.
  - Validation: Review confirms every dependency has an owning service and Overvault receives/release private facts through explicit APIs, grants, leases, and evidence refs rather than privileged direct storage access.

## Phase 2: Rust Contracts, Schemas, Envelopes, And Fixtures

### Work Items

- **2.1 Create the Overvault Rust contract module.**
  - Design: Add contract types for secret records, secret versions, encrypted records, access policies, access requests, access decisions, access grants, mount leases, rotation jobs, revocation records, escrow records, emergency access records, redaction policies, tombstones, usage refs, and reason codes.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, state enums, API error types, reason-code mapping, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms Overvault contracts remain separate from Overkey, Overbase, Overstore, Overguard, Overwatch, Overmeter, and accounting logic.

- **2.2 Define canonical JSON Schemas and examples.**
  - Design: Add versioned schemas for secret create, secret version add, encrypted record write, metadata read view, access request, access decision, access grant, mount lease, revoke, rotate, escrow request, emergency release, redaction report, quarantine, tombstone, purge report, and API errors.
  - Output: Schema files, valid examples, invalid examples, schema index, generated validators, and compatibility notes.
  - Validation: Schema tests reject missing tenant, app, actor/service account, trace, idempotency, data class, purpose, ttl, policy refs, key policy refs, target refs, audit refs, and redaction profile where required.

- **2.3 Define the envelope encryption metadata model.**
  - Design: Model encrypted payload refs, envelope refs, key policy refs, non-exporting signer refs, data-encryption key scope, key-encryption authority, BLAKE3 fingerprints/checksums, rotation refs, revocation refs, and development-only file-key constraints.
  - Output: Envelope schema, key-policy compatibility checks, fingerprint contract, dev-stub profile, and migration notes from founder-local profiles to full Overvault.
  - Validation: Security tests prove raw private material, raw data-encryption keys, and decrypted payloads are not serializable into normal API responses, logs, events, fixtures, or replay reports.

- **2.4 Define lifecycle and state transitions.**
  - Design: Model draft, active, versioning, rotating, revoking, revoked, expired, quarantined, escrowed, tombstoned, and purged secret states plus requested, policy_checked, allowed, issued, active, expired, revoked, and denied access-grant states.
  - Output: State transition table, legal transition rules, terminal/overlay state semantics, reason codes, and event payload refs.
  - Validation: State tests reject release before policy decision, grant issuance without ttl, secret purge before retention eligibility, normal reads from quarantined records, and deletion that erases prior access evidence.

- **2.5 Create deterministic fixtures and harness scenarios.**
  - Design: Build fixtures for secret create, version add, metadata read, access request, allowed decision, denied decision, grant issuance, mount lease, expired lease, revoked lease, wrong-node lease, rotation, revocation, encrypted record, private projection, escrow request, emergency access, quarantine, and redaction failures.
  - Output: Fixture directory, expected API responses, event payloads, usage facts, audit refs, redacted diagnostics, and replay hashes.
  - Validation: Fixture tests produce stable output and prove ambiguous or missing facts fail closed as denied, blocked, expired, quarantined, or review-required states.

## Phase 3: Secret Records, Versions, Encrypted Records, And Metadata Reads

### Work Items

- **3.1 Implement secret record creation.**
  - Design: Accept tenant/app/owner refs, secret kind, data class, allowed subject refs, retention policy, rotation policy, current state, key policy refs, policy refs, idempotency key, trace id, and redaction profile.
  - Output: `POST /overvault/secrets`, secret record, first version intent, metadata-only response, and `overvault.secret_created` event.
  - Validation: Secret-create tests reject missing tenant/app refs, unauthorized actor, missing data class, unsupported retention, missing key policy refs, invalid allowed subjects, and duplicate incompatible idempotency keys.

- **3.2 Implement secret version add and activation.**
  - Design: Add encrypted payload refs, envelope refs, key policy refs, version id, created-by refs, activation time, expiry, checksum/fingerprint, previous-version refs, rollback eligibility, and append-only activation evidence.
  - Output: `POST /overvault/secrets/{secret_id}/versions`, version record, activation report, compatibility checks, and `overvault.secret_version_added` event.
  - Validation: Version tests reject plaintext payload storage, missing envelope refs, invalid key policy state, incompatible data class, activation before policy allow, and silent overwrite of prior version evidence.

- **3.3 Implement metadata-only secret reads.**
  - Design: Return redacted secret metadata, current version label, state, allowed operation hints, rotation due status, policy refs, retention state, and denied/not-found-safe behavior without exposing secret values.
  - Output: `GET /overvault/secrets/{secret_id}`, redacted metadata view, field-level redaction matrix, and audit refs.
  - Validation: Metadata tests prove ordinary reads do not expose raw secret values, decrypted payloads, private checksums where disallowed, key material, non-exporting signer internals, envelope internals, or cross-tenant existence signals.

- **3.4 Implement encrypted private record writes.**
  - Design: Store encrypted record metadata, subject scope, encrypted payload ref or bounded encrypted value, schema ref, data class, retention policy, searchability flag, owner refs, and policy refs.
  - Output: `POST /overvault/encrypted-records`, encrypted record, policy admission report, redacted metadata response, and `overvault.encrypted_record_written` event.
  - Validation: Encrypted-record tests reject plaintext private payloads, missing schema refs, unsupported data class, overbroad subject scope, missing retention policy, and attempts to make private content globally searchable.

- **3.5 Emit write and metadata evidence.**
  - Design: Emit raw usage for secret writes, version writes, encrypted record writes, metadata reads, rejected writes, redaction decisions, encryption checks, and idempotent retries without billing semantics.
  - Output: Overmeter raw usage event contract, Overwatch audit mapping, source refs, replay bundle fields, and redacted diagnostics.
  - Validation: Evidence tests prove Overvault never mutates ORU balances, Seal Ledger entries, invoices, pricing, payouts, settlement state, or external payment systems.

## Phase 4: Access Requests, Policy Decisions, Grants, And Denials

### Work Items

- **4.1 Implement explicit access requests.**
  - Design: Accept caller refs, target secret or encrypted record, purpose, workload/run refs, tenant/app refs, requested ttl, data class, delivery channel, trace id, and idempotency key before any release side effect.
  - Output: `POST /overvault/access-requests`, access request record, pending decision state, and `overvault.access_requested` event.
  - Validation: Request tests reject missing identity, tenant, purpose, data class, target ref, ttl, workload/run refs where required, and requests that attempt direct secret value return.

- **4.2 Integrate Overguard decision handoff.**
  - Design: Send policy input bundles with subject, actor/service account, tenant/app, data class, purpose, workload class, emergency flag, ttl, release mode, requested fields, and prior grants/revocations.
  - Output: `POST /overvault/access-requests/{request_id}/decide`, decision record, reason codes, policy refs, allowed fields, denied fields, redaction rule, ttl, and audit refs.
  - Validation: Decision tests prove missing policy, policy ambiguity, stale policy, mismatched tenant/app, and dependency failure deny or block instead of releasing by default.

- **4.3 Implement bounded access grants.**
  - Design: Issue grants only after an allowed decision with target ref, grantee ref, operation scope, allowed fields, ttl, release channel, revocation state, policy refs, and audit refs.
  - Output: `POST /overvault/grants`, access grant record, grant lifecycle, delivery authorization ref, and `overvault.grant_issued` event.
  - Validation: Grant tests reject denied decisions, expired decisions, missing ttl, broader scope than decision, wrong grantee, wrong tenant, quarantined target, and release through unapproved channels.

- **4.4 Implement denial and redacted error behavior.**
  - Design: Record denials with reason codes, policy refs, redaction hints, operator-visible diagnostics, safe user-facing responses, and replayable evidence without exposing target existence where policy forbids it.
  - Output: Denial record, `overvault.access_denied` event, redacted API errors, safe retry hints, and audit refs.
  - Validation: Denial tests prove unauthorized callers cannot distinguish absent, unauthorized, revoked, quarantined, or policy-blocked targets unless policy allows that visibility.

- **4.5 Emit access and grant usage.**
  - Design: Emit raw usage for access requests, policy decisions, grants, denials, expired decisions, revoked grants, dependency failures, emergency flags, and redaction work.
  - Output: Usage/audit event mapping, grant replay report, denial replay report, and redacted diagnostics.
  - Validation: Evidence tests prove access logs support disputes and post-action review without exposing raw secret values, decrypted payloads, key material, or private target details.

## Phase 5: Overrun Secret Refs, Mount Leases, Delivery, And Cleanup

### Work Items

- **5.1 Implement the Phase 3 founder-local secret-ref bridge.**
  - Design: Support only explicit `founder_local_secret_ref` delivery mediated by Overcell on founder hardware with signed lease-bound node-local handoff, ttl, least-scope process visibility, redaction policy, cleanup evidence, and migration ref to full Overvault.
  - Output: Stub profile schema, Overcell handoff contract, migration ref, allowed workload-class gate, and denial reasons for unsupported scopes.
  - Validation: Bridge tests prove public/provider pools, regulated workloads, third-party secrets, and policies requiring full Overvault fail Overrun preflight instead of silently falling back to the stub.

- **5.2 Implement full mount lease issuance.**
  - Design: Create leases with run id, node id, process scope, path policy, target secret refs, allowed operations, ttl, renewal rule, revocation hook, cleanup behavior, and policy refs.
  - Output: `POST /overvault/mount-leases`, mount lease record, delivery authorization bundle, lease lifecycle, and `overvault.mount_lease_issued` event.
  - Validation: Lease tests reject wrong node, wrong run, missing workload policy, expired grant, incompatible process scope, unsupported path policy, public-node secret-bearing placement, and lease issuance without cleanup behavior.

- **5.3 Implement approved delivery channels.**
  - Design: Deliver secrets only through signed, lease-bound, node-local mount or file-descriptor style channels with scoped process visibility, zeroization, redaction checks, and no normal API value return.
  - Output: Delivery channel contract, Overrun client binding, channel state events, cleanup evidence refs, and redacted diagnostics.
  - Validation: Delivery tests prove raw secret values do not enter Overqueue, Oversched, Overwatch payloads, scheduler reason records, run results, logs, diagnostics, environment dumps, or normal artifact storage.

- **5.4 Implement lease renewal, expiry, and revocation.**
  - Design: Support ttl countdown, renewal policy, revoked state, active process cleanup, Overrun callback, Overwatch evidence, dependency failure behavior, and cleanup review scheduling.
  - Output: `POST /overvault/mount-leases/{lease_id}/revoke`, renewal contract, expiry worker, cleanup worker, and `overvault.mount_lease_revoked` event.
  - Validation: Lease lifecycle tests cover valid, expired, revoked, wrong-node, wrong-run, cleanup failure, renewal denied, and dependency outage paths with distinct auditable states.

- **5.5 Emit mount and cleanup evidence.**
  - Design: Emit raw usage for lease creation, delivery attempts, renewals, expiry, revocation, cleanup work, zeroization evidence, delivery failures, and denied preflight.
  - Output: Overmeter usage contract, Overwatch evidence mapping, cleanup replay bundle, and redacted diagnostics.
  - Validation: Replay tests reconstruct lease issuance, delivery, expiry, revocation, cleanup success, and cleanup failure without exposing raw secret values or private node internals.

## Phase 6: Rotation, Revocation, TTL, Quarantine, And Tombstones

### Work Items

- **6.1 Implement rotation jobs.**
  - Design: Create rotation jobs with target secret, source version, target version, rotation reason, rollout state, dependent refs, rollback eligibility, ttl impact, affected grants/leases, and completion evidence.
  - Output: `POST /overvault/secrets/{secret_id}/rotate`, rotation job record, state transitions, dependency report, and `overvault.secret_rotated` event.
  - Validation: Rotation tests cover normal rotation, dependency failure, stale grant cleanup, failed rollout, rollback-eligible rotation, compromise-suspected rotation, and evidence replay.

- **6.2 Implement revocation records.**
  - Design: Revoke secrets, versions, grants, leases, encrypted records, or escrow releases with reason, actor/policy refs, effective time, dependent grants, cleanup state, quarantine decision, and evidence refs.
  - Output: `POST /overvault/secrets/{secret_id}/revoke`, revocation record, dependent cleanup queue, denial behavior, and `overvault.secret_revoked` event.
  - Validation: Revocation tests prove no new releases occur after revocation, existing leases are cleaned up or escalated, access decisions remain append-only, and prior audit evidence is preserved.

- **6.3 Implement ttl expiry and retention behavior.**
  - Design: Expire grants, leases, versions, records, and escrow releases according to ttl and retention policy while preserving audit refs, cleanup state, and tombstone eligibility.
  - Output: Expiry worker, retention state fields, tombstone workflow, purge eligibility report, and redacted retention diagnostics.
  - Validation: Expiry tests block access after ttl, preserve required evidence during retention, reject purge under hold, and create tombstones instead of silent deletion.

- **6.4 Implement quarantine workflows.**
  - Design: Quarantine compromised, disputed, corrupted, policy-denied, incident-linked, emergency-misused, or ambiguous targets by blocking release, revoking grants/leases, preserving evidence, and creating remediation refs.
  - Output: Quarantine record, `overvault.quarantine_applied` event, remediation state, incident/dispute refs, and review workflow.
  - Validation: Quarantine tests prove quarantine overrides normal grants and mount leases until cleared by policy and preserves evidence for incident, dispute, compliance, or governance review.

- **6.5 Emit lifecycle evidence.**
  - Design: Record rotation, revocation, ttl expiry, tombstone, purge attempts, quarantine, dependency cleanup, failed cleanup, and retained evidence without storing raw private material.
  - Output: Overmeter usage mapping, Overwatch evidence mapping, lifecycle replay report, and redacted operator diagnostics.
  - Validation: Evidence tests prove lifecycle audits can explain who changed access, why, when it took effect, and which dependencies were cleaned up without exposing secrets or decrypted payloads.

## Phase 7: Encrypted Private Records, Search Projections, And Native App Use

### Work Items

- **7.1 Implement private encrypted record reads.**
  - Design: Read private encrypted records only through policy-approved access requests, field scopes, redaction profile, ttl, grant refs, and approved delivery or response channels.
  - Output: `GET /overvault/encrypted-records/{record_id}`, private record read contract, redacted metadata, field filter, and access evidence refs.
  - Validation: Read tests reject direct storage reads, missing grants, revoked grants, expired ttl, wrong actor, wrong app, cross-tenant scope, and broad field release.

- **7.2 Implement access-scoped search projections.**
  - Design: By default, encrypted private records are not full-text searchable; when search is required, create schema-declared projections containing only permitted refs, hashes, redacted fields, tokenized terms, or embeddings scoped to data class and access grant.
  - Output: Projection schema, projection lifecycle, projection policy refs, tombstone/rebuild worker, and projection metadata view.
  - Validation: Projection tests prove raw private content is never indexed into shared indexes and private embeddings are treated as private or secret-bearing artifacts.

- **7.3 Integrate Overbase and encrypted Docdex RAG access filters.**
  - Design: Require tenant, app, actor/service account, data class, grant, revocation state, purpose, and policy filters before Overbase/vector indexes or encrypted Docdex RAG indexes expose candidates.
  - Output: Access-filter contract, candidate prefilter API, revocation tombstone path, retention expiry rebuild path, and RAG handoff notes.
  - Validation: RAG tests prove no global index, cross-tenant search, public discovery, or central AI analysis reuses private embeddings without explicit policy and evidence refs.

- **7.4 Support native app private data profiles.**
  - Design: Define profiles for personal AI private context, workspace private settings, messaging protected data, mobile app secrets, directory private refs, search private preferences, and regulated app records.
  - Output: Native-app profile matrix, required refs, default retention, release modes, projection policy, and redaction notes.
  - Validation: Native-app tests prove apps use Overvault refs and access decisions instead of direct private storage APIs, raw values in Overbase, or hidden external secret stores.

- **7.5 Emit private-record and projection evidence.**
  - Design: Emit raw usage for private record writes, reads, projection writes, projection rebuilds, projection tombstones, denied candidate exposure, redaction work, and revocation propagation.
  - Output: Usage/audit event mapping, projection replay bundle, access-filter replay report, and redacted diagnostics.
  - Validation: Evidence tests prove owners can audit private search behavior without exposing private content, private embeddings, or redacted projection fields.

## Phase 8: Escrow, Emergency Access, Regulated Workflows, And Review

### Work Items

- **8.1 Implement escrow records.**
  - Design: Create escrow workflows with protected subject refs, required evidence, requester refs, approver roles, release policy, allowed fields, redaction rules, appeal path, retention/legal-hold behavior, ttl, revocation behavior, and audit refs.
  - Output: `POST /overvault/escrow`, escrow record, review state machine, evidence checklist, and `overvault.escrow_requested` event.
  - Validation: Escrow tests reject missing release policy, missing evidence, missing appeal path, missing redaction profile, incompatible approver role, and regulated workload admission without escrow support.

- **8.2 Implement controlled escrow release.**
  - Design: Release escrowed material only after required evidence and approver policy succeed, with field-level redaction, ttl, grant refs, post-release review, dispute refs, and correction/retraction support.
  - Output: Escrow release contract, `overvault.escrow_released` event, release result record, post-action task, and audit refs.
  - Validation: Release tests prove central AI may analyze scoped evidence packages or recommend a release path, but final release belongs to Overvault plus the owning compliance, dispute, stewardship, or recovery workflow.

- **8.3 Implement emergency access and break-glass.**
  - Design: Require signed break-glass command through Overgate, Overguard emergency policy decision, protected operator/steward credential class, explicit target refs, purpose, data class, ttl, allowed fields, redaction profile, incident/evidence refs, and high-severity Overwatch evidence.
  - Output: Emergency access record, high-severity audit mapping, grant expiry, post-action review task, and correction/retraction refs.
  - Validation: Emergency tests reject self-authorized central AI access, missing protected credential, missing incident refs, missing purpose, overbroad fields, indefinite ttl, and absent post-action review.

- **8.4 Implement regulated workload admission gates.**
  - Design: Block regulated workloads until escrow, emergency, evidence hold, dispute preservation, user/organization recovery, compliance release, appeal, redaction, and retention paths exist for the required data class.
  - Output: Admission gate contract, Overguard policy input, deny/block reason codes, regulated fixture set, and operator remediation notes.
  - Validation: Admission tests prove missing escrow policy, missing appeal path, missing redaction profile, or missing evidence hold is a deny/block condition.

- **8.5 Emit escrow and emergency evidence.**
  - Design: Record escrow creation, review, release, denial, emergency request, emergency grant, emergency expiry, post-action review, correction, retraction, appeal, and compliance hold evidence.
  - Output: Overmeter usage mapping, Overwatch high-severity evidence, compliance export refs, dispute refs, and redacted replay bundles.
  - Validation: Evidence tests prove emergency and escrow actions are visible to owning services and dispute/compliance workflows without exposing raw private material beyond the approved release path.

## Phase 9: Observability, Metering, Operations, And Service Integration

### Work Items

- **9.1 Integrate Overwatch audit and evidence views.**
  - Design: Publish access request, decision, grant, denial, mount lease, rotation, revocation, quarantine, escrow, emergency, redaction, and cleanup evidence refs to Overwatch without raw secret values.
  - Output: Overwatch event mapping, severity matrix, replay refs, incident linkage, and redacted operator views.
  - Validation: Audit tests prove Overwatch can explain who requested access, why, which policy allowed or denied it, when grants expired or revoked, and which cleanup evidence exists.

- **9.2 Integrate Overmeter raw vault usage.**
  - Design: Emit usage facts for secret writes, version writes, encrypted record writes, metadata reads, access requests, decisions, grants, mount leases, rotations, revocations, escrow workflows, emergency workflows, denied requests, redaction checks, and cleanup work.
  - Output: Overmeter usage contract, dimensions for tenant/app/actor/service/data class/target kind/policy refs/operation state, and retention notes.
  - Validation: Usage tests prove denied and emergency operations remain visible for audit/risk review and Overvault never mutates ORU balances, Seal Ledger entries, provider payouts, invoices, or payment systems.

- **9.3 Integrate dependency health checks.**
  - Design: Report health for encryption/decryption path, envelope refs, key policy refs, Overkey, Overguard, Overwatch, Overmeter, Overbase, Overstore, Overrun mount delivery, Overmesh where required, storage backend, and cleanup workers.
  - Output: Health endpoint/schema, metrics/events, dashboard fields, dependency outage behavior, and safe diagnostics.
  - Validation: Health tests prove dependency outages block or degrade access safely and diagnostics include evidence refs and reason codes but not private payloads, raw secrets, or key material.

- **9.4 Integrate SDK, CLI, admin UI, and native app clients.**
  - Design: Provide generated contracts for secret metadata management, access requests, grants, lease status, rotation, revocation, escrow, emergency review, private records, and redacted audit views.
  - Output: Client contract matrix, command/API examples, UI-safe field matrix, and role-based capability notes.
  - Validation: Client tests prove UI/CLI/SDK surfaces call Overgate/Overvault APIs and never become privileged backdoors or raw secret exfiltration paths.

- **9.5 Prepare backup, restore, and migration handoffs.**
  - Design: Document how Backup and Restore Service, Migration Tooling, Incident Response Service, Compliance Boundary Service, Overbase, Overstore, and native apps preserve encrypted payload refs, envelope refs, key policy refs, grants, tombstones, revocations, and evidence without broadening access.
  - Output: Handoff matrix, dependency owners, ref lifecycles, restore notes, migration notes, and phased readiness notes.
  - Validation: Handoff review confirms downstream consumers can use Overvault refs without weakening Overkey, Overbase, Overstore, Overguard, Overwatch, Overmeter, Overrun, or governance boundaries.

## Phase 10: Validation, Documentation, Queue State, And Handoff

### Work Items

- **10.1 Validate contract and schema coverage.**
  - Design: Run focused checks for secret records, secret versions, encrypted records, access policies, access requests, decisions, grants, mount leases, rotations, revocations, escrow records, emergency records, redaction policies, tombstones, APIs, events, and reason codes.
  - Output: Schema-test report, state-machine test report, fixture coverage matrix, failure notes, and remediation list.
  - Validation: Tests pass before implementation advances beyond each documented gate; any blocker is recorded in build-plan progress.

- **10.2 Validate Phase 8 vault behavior end to end.**
  - Design: Prove one signed tenant/app flow creates a secret, adds an encrypted version, performs metadata read, requests access, receives an Overguard decision, issues a grant, creates a mount lease, revokes the lease, rotates the secret, and emits usage/audit evidence.
  - Output: End-to-end vault fixture, source-ref bundle, operation records, usage ref, audit trail, and replay report.
  - Validation: Replay confirms successful, denied, blocked, expired, revoked, quarantined, escrowed, emergency, cleanup-failed, and purge-blocked paths produce distinct auditable states.

- **10.3 Validate security, privacy, and tech-stack alignment.**
  - Design: Scan implementation and docs for raw secret leakage, decrypted payload leakage, unauthorized private search, direct storage reads, external Vault/KMS product-boundary drift, structured-state drift, object-storage drift, policy-authority drift, billing/ledger mutation, blockchain/NFT mechanics, pricing/revenue/customer-count assumptions, and TypeScript core-runtime drift.
  - Output: Security/privacy checklist, tech-stack alignment report, negative-control scan results, and remediation notes.
  - Validation: Review confirms Overvault remains Rust-first/native-Overrid infrastructure and uses canonical JSON/JSON Schema, optional Protobuf, Ed25519 signatures, BLAKE3/content fingerprints, signed envelopes, policy refs, envelope encryption, redaction checks, and native service boundaries.

- **10.4 Validate master-plan and downstream handoff alignment.**
  - Design: Confirm SDS #29, the Overvault service plan, master build plan, build-plan crosswalk, Phase 8 plan, Phase 11 plan, Phase 12 plan, Phase 13 plan, queue state, and progress docs link to this plan and preserve the Phase 8 first full build point.
  - Output: Updated source-document links, sub-build-plan index entries, progress evidence, queue status, and downstream handoff matrix.
  - Validation: Markdown link checks pass and review confirms no master Phase 0 through Phase 13 ordering change was required.

- **10.5 Validate implementation handoff readiness.**
  - Design: Prepare the handoff for builders by listing required crates/modules, schemas, reason-code catalogs, envelope model, redaction harness, service clients, fixture groups, lease scenarios, escrow scenarios, acceptance tests, and phase gates.
  - Output: Implementation handoff checklist, validation command list, known blockers, dependency owners, and first vault-backed app fixture target.
  - Validation: Handoff review confirms a builder can start Overvault Phase 8 implementation without reading informal agent notes or weakening SDS boundaries.

## Alignment Review

- The sub-build plan keeps Overvault first full build work in master Phase 8, matching SDS #29, the service catalog entry, Phase 8 plan, master build plan, and build-plan crosswalk.
- The plan treats Phases 0, 1, 3, 4, 5, and 7 as prerequisites for shared schemas, signed identity/tenant/request/audit rails, minimal founder-local secret refs, policy controls, raw usage emission, and protected system-service readiness rather than as full Overvault implementation phases.
- The plan keeps Overkey, Overbase, Overstore, Overguard, Overwatch, Overmeter, Overrun, Overcell, Overmesh, Overclaim, Oververify, Challenge Task Service, native apps, personal AI, compliance services, and governance authority outside Overvault while defining the refs and evidence Overvault consumes or emits.
- The plan preserves SDS #29 resolved decisions: Overvault-owned envelope encryption, Overkey metadata separation, explicit founder-local stub constraints, central AI emergency-access limits, access-scoped private projections, and escrow requirements before regulated workload admission.
- The plan gates public-provider execution through Phase 11 deny-by-default secret-bearing workload rules and rejects silent fallback to public placement for private, regulated, third-party-secret, or Overvault-required assignments.
- The plan keeps long-running rotation, revocation, cleanup, projection rebuild, escrow review, emergency post-action review, migration, backup/restore, and purge work in queued or resumable jobs instead of ordinary request budgets.
- The plan preserves the master Phase 0 through Phase 13 order and uses later phases only for deployment secret provisioning, public-provider constraints, native-app consumption, and governance/compliance hardening.
- The plan aligns with `docs/overrid_tech_stack_choice.md`: Rust-first Overvault core, native Overrid boundaries, Tokio, Axum/Tower/Hyper-style service boundaries, canonical JSON/JSON Schema, optional Protobuf, Ed25519 signatures, BLAKE3/content fingerprints, signed envelopes, envelope encryption, redaction checks, and no conventional database, object-store, vault/KMS, queue, blockchain, NFT, pricing, revenue, customer-count, or external-payment assumptions.
