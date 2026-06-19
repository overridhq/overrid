# SUB BUILD PLAN #9 - Overkey

Attached SDS: [docs/sds/control_plane/overkey.md](../sds/control_plane/overkey.md)

## Purpose

This sub-build plan turns SDS #9 into an implementation sequence for Overkey. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Overkey is the credential and public verification metadata authority for Overrid. It enrolls credential metadata, API key hashes, signing public keys, service-account keys, delegation records, rotation records, revocation records, last-used metadata, and Overvault secret refs without storing raw private key or secret material. Overkey supplies verification facts to Overgate and trusted internal services while leaving request admission, tenant authorization, policy finality, secret custody, and business authorization with their owning services.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #9: Overkey](../sds/control_plane/overkey.md) | Controls Overkey purpose, data model, API surface, lifecycle states, verification flow, policy/security rules, failure behavior, validation, and resolved open-question decisions. |
| [Overkey service plan](../service_catalog/control_plane/overkey.md) | Controls service-catalog objective, first build phase, dependencies, development order, contracts, validation, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical build order from master Phase 0 through master Phase 13. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Controls the first build point for Overkey-lite as the credential and signing-record primitive in the signed control-plane path. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies later Overguard policy constraints for credential class, delegation, workload sensitivity, and deny-by-default behavior. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies later usage attribution and control-plane metering refs without moving accounting or ledger mutation into Overkey. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Supplies SDK, CLI, admin UI, Docdex, Mcoda, Codali, adapter, and product-client credential use cases. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies protected system-service workload, backup, restore, failover, rolling update, and break-glass operation requirements. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overvault secret refs, broader key services, delegated access, namespace-aware credential bindings, and private-storage integration. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #9 first build work aligned to master Phase 1, with broader expansion through Phase 8 and later hardening phases. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first control-plane services, Axum/Tower/Hyper-style HTTP, signed command envelopes, Ed25519, BLAKE3 refs, native Overwatch/Overvault boundaries, and no conventional cloud product-boundary drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phase 1 | Attach SDS #9 to the build-plan layer, freeze Overkey as the credential metadata authority, and preserve resolved key-management decisions. |
| 2 | Master Phases 0 and 1 | Build the Rust service skeleton, canonical schemas, local Overrid-shaped storage, and deterministic test fixtures. |
| 3 | Master Phase 1 | Implement credential enrollment, public key records, API key hashes, service-account records, lifecycle states, and metadata reads. |
| 4 | Master Phase 1 | Implement restricted verification helpers for Overgate using explicit key ids, Ed25519, BLAKE3 hashes, stable reason codes, and tenant-scoped checks. |
| 5 | Master Phases 1 and 7 | Implement rotation, revocation, revocation epochs, cache invalidation, propagation status, and signed break-glass revocation behavior. |
| 6 | Master Phases 1, 4, and 5 | Add delegated access, service-account scoping, policy handoff, last-used reporting, and usage-relevant events without policy or accounting ownership. |
| 7 | Master Phase 8 | Bind Overkey metadata to Overvault secret refs, protection classes, high-risk credential controls, and namespace-aware key services. |
| 8 | Master Phases 1 and 7 | Emit Overwatch-compatible audit evidence, expose observability, implement readiness behavior, and prepare grid-resident operations. |
| 9 | Master Phases 6, 8, 12, and 13 | Harden SDK, CLI, admin UI, native/mobile/app credentials, operator views, migration tooling, and downstream handoff contracts. |
| 10 | Master Phase 1 through Phase 13 | Validate structure, tech-stack alignment, documentation links, queue state, security contracts, and phase-order consistency. |

## Tech Stack Guardrails

- Overkey core is a Rust service using Tokio and Axum/Tower/Hyper-style HTTP, with rustls/mTLS where early seed control-plane transport requires it.
- Credential records, API errors, lifecycle events, audit refs, verification results, rotation records, revocation records, secret refs, and fixtures use canonical JSON plus JSON Schema from the shared schema package.
- Ed25519 is the initial command-signature algorithm for signed command envelopes, service-account signatures, node enrollment signatures, and operator/admin commands. Future algorithms require schema-versioned `algorithm`, `canonicalization_version`, fixtures, compatibility tests, and explicit migration records before production verification.
- BLAKE3 is used for canonical body hashes, key fingerprints, evidence refs, and hash-linked audit evidence. Transport security with rustls/mTLS does not replace command signatures.
- Overkey stores public verification metadata, API key hashes, non-secret prefixes, key ids, key fingerprints, status, rotation metadata, revocation metadata, and secret refs. It must never store raw private keys, seed phrases, passwords, bearer tokens, raw API keys, or unencrypted secret values.
- Overvault owns protected secret material. Overkey may bind to Overvault refs and protection-class metadata, but must not replace Overvault with Vault, cloud KMS, local plaintext files, or another product boundary.
- Overkey persists state through Overrid-owned abstractions or Overrid-shaped local stubs during early phases. It must not make PostgreSQL, Redis, Kafka, NATS, S3, MinIO, Vault, or similar products the platform boundary.
- Overkey calls Overpass, Overtenant, Overgate, Overwatch, Overguard, Overvault, Overmeter, SDK/CLI, and downstream services through explicit contracts. It must not write their private records directly.
- Overkey must keep ORU, Seal Ledger, Overgrant, and Overasset references structural. It must not implement blockchain, NFT mechanics, speculative token behavior, pricing tables, revenue projections, or customer-count assumptions.

## Phase 1: SDS Attachment, Authority, And Boundary Rules

### Work Items

- **1.1 Attach the build plan to SDS #9.**
  - Design: Link this document from the numbered Overkey SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/control_plane/overkey.md`, `docs/service_catalog/control_plane/overkey.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #9 returns both the Overkey SDS and this sub-build plan.

- **1.2 Freeze Overkey as the credential metadata authority.**
  - Design: Record that Overkey owns credential records, API key hashes, public signing key metadata, service-account key records, delegated access metadata, rotation links, revocation records, last-used metadata, verification results, and secret-ref contracts.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms Overkey owns public verification metadata but not raw secret custody, business authorization, tenant membership, policy finality, request admission, or accounting settlement.

- **1.3 Preserve master Phase 1 as the first build point.**
  - Design: Keep first implementation in master Phase 1 because Overkey-lite is required before Overgate can verify signed tenant-scoped commands against key records instead of hardcoded development secrets.
  - Output: Phase-gate note that SDS #9 starts in Phase 1 and expands later through policy, metering, product, grid-resident, and Phase 8 private-storage gates.
  - Validation: Review proves this plan does not move Overkey into Phase 0 or change the master Phase 0 through Phase 13 order.

- **1.4 Carry forward resolved SDS decisions.**
  - Design: Preserve the SDS decisions for Ed25519, BLAKE3 hashes, explicit non-secret credential/key ids, production protection classes, short verification caches, cache invalidation on revocation epoch changes, and signed break-glass revocation through Overgate.
  - Output: Resolved-decision checklist tied to SDS #9 open-question answers.
  - Validation: Review proves the plan does not reopen settled decisions or replace them with JWT-only, external KMS-specific, RSA/ECDSA-required, raw-key, or long-lived positive-cache behavior.

- **1.5 Define runtime authority boundaries.**
  - Design: Require Overkey lifecycle APIs to flow through Overgate for user/operator-facing commands while internal verification helpers remain restricted to signed service accounts.
  - Output: Boundary matrix for external lifecycle commands, internal verification helpers, Overvault secret refs, Overpass identity refs, Overtenant scope refs, Overguard policy refs, and Overwatch audit refs.
  - Validation: Design review rejects direct credential mutation by downstream services and rejects credential use paths that bypass Overgate admission or approved internal service-account authentication.

## Phase 2: Rust Service Skeleton, Schemas, And Record Model

### Work Items

- **2.1 Create the Overkey Rust service crate.**
  - Design: Add an Overkey service crate under the control-plane workspace using Tokio, Axum/Tower/Hyper-style HTTP, shared config loading, tracing setup, and dependency injection for storage, schema validation, Overpass, Overtenant, Overwatch, Overvault, and Overgate callbacks.
  - Output: Service crate, module layout, local-stack service entrypoint, and testable handler boundaries.
  - Validation: Compile and service-start checks pass once implementation exists; crate layout review confirms Overkey stays separate from Overvault, Overgate, Overtenant, and Overguard internals.

- **2.2 Define canonical credential schemas.**
  - Design: Add shared schemas for `credential_record`, `api_key_record`, `public_key_record`, `service_account_key`, `delegation_record`, `rotation_record`, `revocation_record`, `verification_result`, `secret_ref`, API errors, and Overwatch event payloads.
  - Output: JSON Schema files, Rust generated or hand-written types, compatibility fixtures, and stable reason-code enums.
  - Validation: Schema tests reject missing tenant, subject, status, credential class, allowed use, expiry, algorithm, canonicalization version, audit refs, and protection-class fields where required.

- **2.3 Implement Overrid-owned storage boundaries.**
  - Design: Define an append-friendly credential metadata repository with status history, rotation links, revocation records, verification logs, audit refs, and local Overrid-shaped storage stubs for early phases.
  - Output: Repository traits, local storage adapter, migration hooks, and append-only history model.
  - Validation: Tests prove lifecycle updates append records or state transitions rather than overwriting identity and never persist raw private material or raw API keys.

- **2.4 Add API route skeletons.**
  - Design: Add route handlers for `POST /v1/credentials/api-keys`, `POST /v1/credentials/signing-keys`, `POST /v1/credentials/service-accounts`, `POST /v1/credentials/{credential_id}/rotate`, `POST /v1/credentials/{credential_id}/revoke`, `GET /v1/credentials/{credential_id}`, `POST /v1/verify/signature`, `POST /v1/verify/api-key`, `POST /v1/usage/last-used`, `GET /v1/healthz`, and `GET /v1/readyz`.
  - Output: Typed route map with placeholder handlers returning stable schema-shaped responses.
  - Validation: Route tests confirm method/path registration, content type behavior, tenant context handling, trace propagation, and internal verification route restrictions.

- **2.5 Connect local development and integration harness fixtures.**
  - Design: Wire Overkey into the loopback-only local stack and integration harness with deterministic tenants, actors, service accounts, Ed25519 public key fixtures, BLAKE3 body hashes, API key hash fixtures, revocation fixtures, and Overvault secret-ref stubs.
  - Output: Local service config, fixture references, and harness scenario names.
  - Validation: Local smoke tests can create a test credential and verify a signed fixture through Overgate without production credentials, raw secrets, or non-Overrid product dependencies.

## Phase 3: Credential Enrollment, Lifecycle APIs, And Metadata Reads

### Work Items

- **3.1 Implement API key hash enrollment.**
  - Design: Enroll API key metadata using a non-secret prefix for lookup and a server-side strong lookup hash for verification while discarding raw key material immediately.
  - Output: API key enrollment handler, hash metadata, prefix hint rules, and raw-key redaction path.
  - Validation: API key tests prove raw key values never appear in stored records, logs, traces, errors, audit exports, fixtures, or diagnostics.

- **3.2 Implement signing public key enrollment.**
  - Design: Enroll Ed25519 public keys with tenant, subject, key id, credential id, key version, canonicalization version, allowed signature use, not-before, not-after, protection-class metadata, and audit refs.
  - Output: Public key enrollment handler and key-fingerprint generation.
  - Validation: Tests reject weak algorithms, unknown canonicalization versions, missing expiry metadata, duplicate active key ids, wrong tenant scope, and production keys without required protection-class metadata.

- **3.3 Implement service-account credential records.**
  - Design: Create or attach service-account credentials for Overgate, node agents, system services, workers, Overvault resolvers, and later grid-resident services with narrow allowed services and command classes.
  - Output: Service-account credential handler, allowed-service matrix, and command-class restrictions.
  - Validation: Tests reject broad service-account permissions, hardcoded development secrets, missing tenant scope, unsupported command classes, and unsigned service-account calls.

- **3.4 Implement caller-visible metadata reads.**
  - Design: Return credential metadata to authorized callers with tenant/role filtering, redaction, lifecycle status, expiry, last-used summaries, rotation refs, revocation refs, and safe protection-class labels.
  - Output: `GET /v1/credentials/{credential_id}` implementation and metadata response schema.
  - Validation: Tenant isolation tests prove one tenant cannot inspect another tenant's credentials, raw secrets, raw keys, private key material, hash internals, or sensitive resolver refs.

- **3.5 Implement lifecycle state transitions.**
  - Design: Support `pending`, `active`, `rotating`, `suspended`, `revoked`, `expired`, and `tombstoned` as explicit status values with append-only history and reason codes.
  - Output: Lifecycle state machine and transition validation.
  - Validation: State tests reject invalid transitions, resurrection from revoked/tombstoned without a new credential record, silent overwrites, and missing audit refs.

## Phase 4: Verification Helpers, Canonicalization, And Overgate Integration

### Work Items

- **4.1 Implement internal signature verification.**
  - Design: Restrict `POST /v1/verify/signature` to Overgate and approved internal service accounts, then verify credential id, key id, key version, algorithm, canonicalization version, timestamp, replay-window fields, body hash, tenant scope, allowed use, status, expiry, rotation window, and revocation epoch.
  - Output: Signature verification helper and typed verification-result record.
  - Validation: Signature tests cover valid, malformed, expired, replayed, revoked, rotated, suspended, wrong-tenant, wrong-key-version, wrong-command-class, and unknown-credential requests.

- **4.2 Implement internal API key verification.**
  - Design: Restrict `POST /v1/verify/api-key` to approved service accounts and verify prefix, lookup hash, tenant scope, subject, credential class, status, expiry, allowed use, and revocation state without returning raw key material.
  - Output: API key verification helper and reason-code mapping.
  - Validation: Tests prove wrong keys cannot verify, unknown prefixes are safe, denial responses are stable, and verification logs never expose reusable secret values.

- **4.3 Implement canonicalization and BLAKE3 evidence refs.**
  - Design: Define canonical request components and BLAKE3 hashes for body hash checks, key fingerprints, verification evidence, cache keys, audit refs, and denial evidence.
  - Output: Canonicalization module with version metadata and golden fixtures.
  - Validation: Golden tests prove canonicalization is deterministic and sensitive to method, path, tenant, credential id, key id, key version, body hash, timestamp, allowed use, and command class.

- **4.4 Integrate Overpass and Overtenant checks.**
  - Design: Check subject identity state and tenant scope before returning positive verification so disabled identities, suspended tenants, missing memberships, wrong service accounts, and deleted-marker subjects cannot verify as active.
  - Output: Overpass/Overtenant client adapters and dependency reason codes.
  - Validation: Dependency tests prove missing, suspended, disabled, deleted-marker, wrong-tenant, and wrong-role subjects deny or block verification before Overgate admission proceeds.

- **4.5 Define the Overgate verification contract.**
  - Design: Return verification results with credential id, key version, subject refs, tenant refs, allowed use, command-class refs, revocation epoch, cache guidance, audit refs, retryability, and stable denial reason codes.
  - Output: Overgate client contract, fixtures, and denial mapping.
  - Validation: Overgate integration tests prove invalid credentials are denied before side effects and clients never parse free-form text to determine verification state.

## Phase 5: Rotation, Revocation, Cache Invalidation, And Break-Glass

### Work Items

- **5.1 Implement rotation plans.**
  - Design: Model predecessor/successor credential links, grace windows, rotation state, initiated-by refs, reason codes, activation timing, and audit refs without overwriting old credential history.
  - Output: Rotation handler and `rotation_record` state machine.
  - Validation: Rotation tests prove both old and new credential refs remain auditable, grace windows are explicit, invalid successors do not silently disable working credentials, and completed rotations preserve evidence.

- **5.2 Implement revocation records.**
  - Design: Create append-only `revocation_record` entries with revoked-by, revoked time, effective time, reason code, affected command classes, incident/evidence refs, expected current state, and audit refs.
  - Output: Revocation handler and revocation state transition logic.
  - Validation: Revocation tests prove revoked credentials fail verification immediately after the effective time and cannot be restored by mutable record edits.

- **5.3 Implement verification cache guidance and revocation epochs.**
  - Design: Treat verification caches as optimizations keyed by tenant, credential id, key version, allowed use, command class, canonicalization version, and revocation epoch, with max 30-second positive cache for ordinary Phase 1 credentials and max 5-second or bypass for high-risk credentials.
  - Output: Cache guidance in verification results, revocation epoch model, and invalidation events.
  - Validation: Cache tests prove rotation, suspension, revocation, expiry, tenant suspension, and algorithm deprecation bump epochs and invalidate Overgate caches immediately.

- **5.4 Implement signed break-glass revocation.**
  - Design: Require emergency revocation to enter through a signed, idempotent Overgate command using an operator/admin credential with the required protection class and role.
  - Output: Break-glass command schema, handler, idempotency handling, and strict audit evidence.
  - Validation: Break-glass tests prove unsigned, wrong-role, wrong-protection-class, cross-tenant, non-idempotent, or missing-evidence revocation commands fail closed.

- **5.5 Expose propagation status and affected inventory.**
  - Design: Track propagation of revocation and cache invalidation to Overgate, Overvault, Overqueue, Oversched, Overcell, system services, and relevant product clients while exposing affected tenant, subject, credential, and command-class refs to operators.
  - Output: Propagation-status model and operator report shape.
  - Validation: Incident tests prove protected commands using affected credentials fail closed until propagation is confirmed and operators receive follow-up rotation or re-enrollment tasks.

## Phase 6: Delegated Access, Service Accounts, Policy Handoff, And Usage Hooks

### Work Items

- **6.1 Implement delegated access metadata.**
  - Design: Model delegator, delegate, tenant id, allowed scopes, allowed command classes, expiry, revocation state, evidence refs, and audit refs without moving authorization finality out of Overtenant or Overguard.
  - Output: `delegation_record` schema and lifecycle handler.
  - Validation: Delegation tests reject missing delegator evidence, stale delegates, overbroad scopes, expired delegation, cross-tenant delegation, and delegated access without revocation evidence.

- **6.2 Harden service-account scopes.**
  - Design: Scope service-account credentials for Overgate verification, Overvault resolver calls, node enrollment, system-service operations, queue execution callbacks, and worker/runtime callbacks.
  - Output: Service-account scope matrix and command-class validation.
  - Validation: Tests prove service accounts cannot affect queue execution, vault access, accounting, rights, payout, namespace, or policy state unless their credential class explicitly allows it.

- **6.3 Add Overguard policy handoff.**
  - Design: Route credential-class, delegation, protection-class, command-class, and high-risk operation policy questions to Overguard once available, while keeping local Phase 1 deny-by-default checks for missing policy prerequisites.
  - Output: Policy-check adapter and policy-decision refs in lifecycle/verification results.
  - Validation: Policy tests prove Overkey honors deny decisions but does not become the policy engine or store policy truth.

- **6.4 Implement last-used and usage-relevant event reporting.**
  - Design: Accept last-used updates from Overgate after successful admission and emit usage-relevant events for verification volume, lifecycle operations, operator actions, service-account use, and credential class.
  - Output: `POST /v1/usage/last-used` implementation, retry-safe update queue, and Overmeter-facing event refs.
  - Validation: Tests prove last-used update failure does not fail admission after verification succeeds, retries are audit-safe, and Overkey does not mutate ORU balances or Seal Ledger entries.

- **6.5 Add operator lifecycle controls.**
  - Design: Require signed operator/admin credentials and stricter protection classes for high-risk lifecycle actions, emergency revocation, service-account changes, vault access refs, namespace credentials, policy override refs, and accounting-adjacent credential classes.
  - Output: Operator guard, protection-class checks, and admin lifecycle command shapes.
  - Validation: Admin tests prove high-risk actions emit audit evidence, deny weak protection classes, fail closed when Overwatch evidence is unavailable, and preserve tenant isolation.

## Phase 7: Secret References, Protection Classes, And Phase 8 Expansion

### Work Items

- **7.1 Implement Overvault secret-ref contracts.**
  - Design: Bind credentials or service accounts to Overvault refs with secret class, resolver service, rotation policy, allowed resolver services, access audit refs, and blocked/pending behavior when Overvault is unavailable.
  - Output: `secret_ref` schema, binding handler, and Overvault resolver contract.
  - Validation: Secret-ref tests prove Overkey stores refs and metadata only, never raw secret values, bearer values, private keys, or unencrypted secret material.

- **7.2 Enforce production protection classes.**
  - Design: Require recorded protection classes for production operator/admin credentials, emergency revocation credentials, system-service credentials, node enrollment keys, service-account keys that affect execution, vault access, accounting, rights, payout, namespace, or policy state, and later native/mobile recovery credentials.
  - Output: Protection-class validator for host keychain, hardware-backed key, TPM/secure-enclave-style signer, or another Overkey/Overvault-approved non-exporting signer.
  - Validation: Tests reject production high-risk credentials without protection-class evidence and reject production use of loopback file-backed test keys.

- **7.3 Isolate local and test credentials.**
  - Design: Permit local file-backed test keys only for loopback development and seed smoke tests, with environment scope, visible test-only marking, blocked production endpoint use, and deterministic reset behavior.
  - Output: Test credential policy and fixture enforcement.
  - Validation: Tests prove development/test credentials cannot verify against production endpoints, production tenants, production Overvault refs, or production grid-resident system-service operations.

- **7.4 Expand namespace-aware and native-app credential bindings.**
  - Design: Add Phase 8 credential bindings for app/service names, routes, native app pages, namespace ownership, transfer/delegation, storage entitlements, and Overasset utility refs without speculative asset behavior.
  - Output: Namespace-aware credential metadata and binding rules.
  - Validation: Namespace tests prove route, app, service, and storage credential bindings require explicit ownership, delegation, policy, and audit evidence.

- **7.5 Define blocked-state recovery for protected dependencies.**
  - Design: Keep credentials pending or blocked when Overvault, Overpass, Overtenant, Overwatch, or policy dependencies cannot prove activation or verification safely.
  - Output: Blocked-state reason codes, retry rules, and operator-facing recovery hints.
  - Validation: Dependency-failure tests prove high-risk verification fails closed and ordinary verification falls back to fresh Overkey lookup when invalidation or evidence is unavailable.

## Phase 8: Audit, Observability, Operations, And Grid-Resident Readiness

### Work Items

- **8.1 Emit Overwatch-compatible lifecycle events.**
  - Design: Emit `overkey.credential_requested`, `overkey.credential_created`, `overkey.credential_activated`, `overkey.credential_rotation_started`, `overkey.credential_rotated`, `overkey.credential_suspended`, `overkey.credential_revoked`, `overkey.credential_expired`, `overkey.verification_denied`, and `overkey.secret_ref_bound`.
  - Output: Event builder, Overwatch client, and event-to-state transition map.
  - Validation: Audit tests prove lifecycle and verification-denial events include refs/hashes/reason codes and never include raw secret values or private keys.

- **8.2 Implement audit-safe metrics and traces.**
  - Design: Record credential counts by status/type/tenant/subject type, verification success and denial counts by reason code, rotation latency, revocation latency, cache invalidation latency, expiring credentials, last-used summaries, and dependency failures.
  - Output: Rust tracing and OpenTelemetry-compatible metric hooks with Overwatch as authoritative audit evidence.
  - Validation: Metrics tests prove labels avoid private data, tenant leakage, reusable secrets, raw payloads, raw keys, and high-cardinality unbounded values.

- **8.3 Implement readiness and degraded-state behavior.**
  - Design: Separate liveness from readiness for storage, shared schemas, Overpass, Overtenant, Overwatch, Overvault, Overgate service accounts, and policy dependencies.
  - Output: `GET /v1/healthz`, `GET /v1/readyz`, dependency matrix, and degraded-state reason codes.
  - Validation: Readiness tests fail when required verification dependencies are unavailable and allow only explicitly safe degraded operations with audit evidence.

- **8.4 Add migration tooling for algorithms and schemas.**
  - Design: Provide migration flows for algorithm upgrades, canonicalization-version changes, credential class changes, revocation epoch changes, schema upgrades, and fixture compatibility.
  - Output: Migration command plan, compatibility matrix, and rollback rules.
  - Validation: Migration tests prove old credentials continue or stop according to explicit compatibility rules and every change preserves audit history.

- **8.5 Prepare grid-resident operations behavior.**
  - Design: Define system-service workload needs for Overkey, including protected placement, backup, restore, failover, rolling update, rollback, maintenance mode, break-glass controls, and incident runbooks.
  - Output: Phase 7 operations checklist for Overkey.
  - Validation: Grid-readiness review confirms founder seed hardware can later be removed from the normal path without changing Overkey's public or internal verification contract.

## Phase 9: Clients, Admin Views, Migration Paths, And Downstream Handoff

### Work Items

- **9.1 Harden SDK and CLI credential providers.**
  - Design: Provide generated contract bindings and Rust-first SDK/CLI credential flows for enrollment, signing, verification diagnostics, rotation, revocation, last-used views, and safe local test credentials.
  - Output: SDK/CLI contract examples, credential provider interface, and stable JSON output shapes.
  - Validation: SDK/CLI tests prove clients use generated contracts, never hardcode signing assumptions, never print raw secrets, and handle stable reason codes.

- **9.2 Implement admin and operator views.**
  - Design: Expose tenant-isolated views for expiring credentials, stale credentials, rotation plans, revocation propagation, service-account scope, protection-class coverage, verification-denial distribution, and secret-ref health.
  - Output: Admin read-model requirements and operator diagnostic endpoints or UI contract.
  - Validation: Admin tests prove authorized operators can diagnose credential issues while tenant users cannot see cross-tenant private metadata or secret refs.

- **9.3 Harden native app, mobile, and product credential flows.**
  - Design: Define credential use patterns for native apps, mobile clients, adapters, AI assistant requests, Docdex, Mcoda, Codali, node agents, workers, and later central AI stewardship interfaces.
  - Output: Product credential checklist and integration fixtures for Phase 6, Phase 8, and Phase 12 consumers.
  - Validation: Product integration tests fail when clients bypass Overgate, omit credential ids, use raw bearer values, skip trace ids, skip idempotency, or use unapproved protection classes.

- **9.4 Define Overrun, Overvault, and system-service handoff.**
  - Design: Document how Overrun secret mounts, Overvault access checks, Overgate verification, Overqueue callbacks, Oversched system-service operations, Overcell node enrollment, and grid-resident system services consume Overkey refs.
  - Output: Downstream handoff checklist and approved internal service-account flows.
  - Validation: Handoff tests prove services request verification through Overgate or approved internal service accounts and do not read or mutate Overkey private records directly.

- **9.5 Define incident and recovery runbooks.**
  - Design: Document how operators investigate verification denials, revoked credentials, compromised credentials, rotation failures, cache invalidation failures, Overvault blocked refs, weak protection classes, and break-glass revocation propagation.
  - Output: Operator runbook checklist tied to Overwatch events, Overkey admin views, and affected-subject inventory.
  - Validation: Incident drills prove operators can trace each failure from request to credential record, revocation epoch, audit ref, dependency state, and recovery task.

## Phase 10: Validation, Documentation Alignment, And Final Gates

### Work Items

- **10.1 Validate sub-build-plan structure.**
  - Design: Check title prefix, attached SDS link, phase headings 1 through 10, work-item structure, Design/Output/Validation fields, alignment review, and exit gate.
  - Output: Focused validation result for `SUB BUILD PLAN #9`.
  - Validation: Scripted text checks pass for required headings, numbered phases, local links, and work-item sections.

- **10.2 Validate tech-stack alignment.**
  - Design: Audit this plan and linked docs for Rust-first control-plane implementation, Axum/Tower/Hyper-style HTTP, Ed25519, BLAKE3, signed command envelopes, canonical JSON plus JSON Schema, Overvault secret custody, native Overwatch evidence, and no conventional cloud product-boundary drift.
  - Output: Tech-stack alignment checklist for Overkey.
  - Validation: Scans find no PostgreSQL, Redis, S3, MinIO, NATS, Kafka, Vault, blockchain, NFT, pricing, revenue, or customer-count assumptions except lines that explicitly reject those assumptions.

- **10.3 Validate master-plan alignment.**
  - Design: Confirm the master Phase 0 through Phase 13 order remains unchanged and SDS #9 is represented as a Phase 1 control-plane service with broader Phase 8 expansion and later hardening through policy, metering, product, grid-resident, native app, and governance phases.
  - Output: Updated master-plan and crosswalk rows for SDS #9.
  - Validation: Review confirms only per-SDS sub-build indexing changed; no master phase-order dependency was altered.

- **10.4 Validate service-catalog and SDS alignment.**
  - Design: Ensure SDS #9 and the Overkey service plan link back to this sub-build plan and preserve Overkey's credential-metadata authority boundary.
  - Output: Updated source-document and sub-build-plan references.
  - Validation: Local Markdown link validation across changed docs returns no missing local targets.

- **10.5 Prepare security, contract, and downstream handoff gates.**
  - Design: Require tests for enrollment, verification, rotation, revocation, cache invalidation, break-glass revocation, delegated access, service-account scopes, Overvault secret refs, protection classes, tenant isolation, redaction, audit events, readiness, and downstream consumers.
  - Output: Final validation checklist for Overkey implementation.
  - Validation: Handoff review confirms Overgate, SDK, CLI, Overvault, Overwatch, Overpass, Overtenant, Overguard, Overmeter, Overrun, Overqueue, Oversched, Overcell, adapters, native apps, mobile services, and grid-resident system services can depend on Overkey verification metadata without moving their runtime authority into Overkey.

## Alignment Review

- The sub-build plan keeps Overkey's first build point in master Phase 1, matching SDS #9, the service catalog entry, Phase 1 plan, master build plan, and build-plan crosswalk.
- The plan treats master Phase 0 as prerequisite work for schemas, local stack, fixtures, test harness, and shared API/event discipline, not as the Overkey implementation phase.
- The plan treats later phases as hardening or expansion gates: trust/policy in Phase 4, metering/accounting refs in Phase 5, product clients in Phase 6, grid-resident operations in Phase 7, Overvault/private-storage/namespace credential expansion in Phase 8, deployment/system-service flows in Phase 9, federation/public-provider controls in Phases 10 and 11, native/mobile credentials in Phase 12, and governance/compliance hardening in Phase 13.
- The plan carries forward SDS #9 resolved decisions for Ed25519, BLAKE3, explicit credential/key ids, production protection classes, verification cache bounds, cache invalidation on revocation epoch changes, and signed break-glass revocation through Overgate.
- The plan keeps Overkey narrow: no raw secret custody, no request admission ownership, no tenant authorization ownership, no policy finality, no accounting or Seal Ledger mutation, no direct downstream private-record writes, no raw secret logging, and no conventional cloud product boundary assumptions.
- The plan does not require any change to the master Phase 0 through Phase 13 dependency order.

## Exit Gate

SUB BUILD PLAN #9 is complete when a builder can implement Overkey as the Phase 1 Rust control-plane credential metadata authority with canonical credential schemas, API key hash enrollment, Ed25519 public key enrollment, service-account records, explicit lifecycle states, restricted verification helpers for Overgate, BLAKE3-backed canonical evidence, Overpass and Overtenant dependency checks, stable denial reason codes, append-only rotation and revocation records, short verification-cache guidance with revocation epochs, signed break-glass revocation, delegated access metadata, service-account scope controls, Overguard policy handoff, last-used and usage-relevant events, Overvault secret refs with production protection-class enforcement, Overwatch-compatible lifecycle events, audit-safe observability, readiness/degraded-state behavior, SDK/CLI/admin/native/mobile handoff rules, and documentation links that preserve the master Phase 0 through Phase 13 order.
