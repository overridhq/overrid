# SUB BUILD PLAN #83 - Mobile SDK

Attached SDS: [SDS #83 - Mobile SDK](../sds/mobile/mobile_sdk.md)

## Purpose

This sub-build plan turns SDS #83 into an implementation sequence for the Mobile SDK. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Mobile SDK is a versioned mobile client package, not a backend service or authority. It gives approved native and third-party mobile apps safe client primitives for configuration, device and session bootstrap, signed requests, idempotency, trace propagation, secure local storage adapters, offline queueing, sync, push registration, media uploads, wallet and usage reads, AI/RAG handoffs, permission prompts, telemetry redaction, compatibility handling, and deterministic tests. It must keep mobile apps on normal Overrid rails instead of encouraging direct service shortcuts.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #83: Mobile SDK](../sds/mobile/mobile_sdk.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, package API surface, local diagnostics, workflows, state machines, policy/security, metering, operations, failure modes, validation, build breakdown, downstream handoff, and resolved open-question decisions. |
| [Mobile SDK plan](../service_catalog/mobile/mobile_sdk.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order and keeps the Mobile SDK as a Phase 12 mobile service-layer client package with Phase 13 hardening. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies repository, shared schema, canonical JSON plus JSON Schema, stable error, fixture, generated-contract, local-stack, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overgate admission, Overpass identity refs, Overtenant scope, Overkey credential refs, Overwatch audit refs, Overqueue primitives, signing, idempotency, and command-envelope discipline. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy checks, data-class decisions, policy dry-run previews, device/app risk signals, challenge evidence, and replayable denial reasons. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies usage refs, wallet-visible summaries, quota/precheck refs, receipt refs, accounting boundaries, and non-mutating wallet/usage handoffs. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Supplies foundation SDK and generated client discipline, AI Gateway Router, Encrypted Docdex RAG Adapter, adapter handoffs, and model/resource route refs used by mobile AI flows. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase records, Overstore object/media refs, Overvault private refs, namespace refs, route facts, storage grants, and private-data retention/redaction boundaries. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public-provider, fraud, and low-sensitivity constraints that third-party mobile traffic must not bypass. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Controls the first full mobile client package build for approved native and third-party mobile apps. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies formal threat/security review, incident/compliance/reporting, privacy, retention, reliability, scale, and public-participation hardening. |
| [SUB BUILD PLAN #6 - SDK](sub_build_plan_006_sdk.md) | Supplies foundation SDK boundaries: generated Rust first, TypeScript/web second, mobile/offline extension points only, and Swift/Kotlin gated by stable shared schemas, golden fixtures, generated errors, and cross-language contract tests. |
| [SUB BUILD PLAN #82 - Mobile Backend Gateway](sub_build_plan_082_mobile_backend_gateway.md) | Supplies the server-side mobile adaptation layer that the Mobile SDK must call for device/session refs, compatibility profiles, sync, offline replay, push refs, media upload sessions, usage reads, AI/RAG handoffs, and redacted replay evidence. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #83 aligned to Phase 12 with Phase 13 hardening and earlier phase prerequisites. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services and generated Rust SDK contracts first, TypeScript/web bindings second for web/UI surfaces, mobile bindings only after contract gates, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database/object-store/vault/queue/mobile-backend SaaS product boundary, blockchain, NFT, hardcoded pricing, revenue, or customer-count assumptions. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 6, 8, 12, and 13 | Attach SDS #83, freeze client-package ownership, preserve Phase 12 as first full build, and map hardening to Phase 13. |
| 2 | Master Phases 0, 1, 6, and 12 | Define shared schemas, generated models, API module contracts, stable errors, and golden fixtures before mobile binding work. |
| 3 | Master Phases 0, 1, 4, 8, and 12 | Implement configuration, credential providers, secure local stores, signing, idempotency, trace propagation, and diagnostics. |
| 4 | Master Phases 1, 4, 5, 8, and 12 | Implement device registration, auth/session bootstrap, capability profiles, and compatibility states over platform refs. |
| 5 | Master Phases 1, 3, 4, 5, 8, and 12 | Implement bounded offline queueing, crash-safe persistence, expiry, safe replay, duplicate resolution, and reconnect behavior. |
| 6 | Master Phases 1, 4, 5, 8, and 12 | Implement sync cursors, delta handling, conflict callbacks, retention hints, local invalidation, and reset flows. |
| 7 | Master Phases 5, 8, and 12 | Implement push registration, media upload helpers, wallet/usage readers, and refs over owner-service APIs. |
| 8 | Master Phases 4, 5, 6, 8, 11, and 12 | Implement native-app helper modules, AI Gateway Router and Personal AI handoffs, encrypted Docdex RAG requests, and permission controls. |
| 9 | Master Phases 0, 6, 12, and 13 | Generate the first Kotlin/Android binding only after foundation gates, prepare Swift parity gates, examples, packaging, release, and compatibility operations. |
| 10 | Master Phase 12 with Phase 13 hardening | Validate mobile flows, privacy/security boundaries, usage/accounting handoffs, operational drills, docs links, queue state, and implementation readiness. |

## Tech Stack Guardrails

- Mobile SDK is a client package built from shared Overrid contracts. It must not become a backend, identity authority, policy engine, ledger authority, storage system, AI router, push provider, analytics product, or native app domain service.
- Foundation SDK and shared schema work remain Rust-first. Generated Rust SDK contracts and fixture behavior must stabilize first; TypeScript/web bindings remain for web/UI/adapter surfaces; Kotlin/Android is the first Phase 12 mobile binding only after schema compatibility checks, golden request-envelope fixtures, generated error objects, and cross-language contract tests are stable. Swift/iOS follows from the same contracts.
- Package APIs, model payloads, stable errors, compatibility profiles, local diagnostics, replay bundles, and docs-facing examples use canonical JSON plus JSON Schema where applicable. Compact Protobuf may be generated later only through the shared contract layer.
- Mutating helpers require signed request envelopes, tenant/app/device/session refs, trace id, idempotency key, schema version, capability profile version, policy refs where required, usage refs, stable errors, and Overwatch audit refs returned by runtime services.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for request hashes, payload refs, media chunk checks, fixture inputs, replay evidence, and deterministic comparison artifacts. BLAKE3 must not be described as encryption.
- Production mobile storage must use approved platform secure storage provider interfaces for credential refs, session refs, offline queue entries, cached deltas, media upload state, push refs, permission snapshots, usage views, and diagnostics. Memory-only and fake providers are test fixtures only.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Firebase, Supabase, Parse, conventional mobile-backend SaaS, notification SaaS, analytics SaaS, ad-tech tracking, device fingerprinting beyond security necessity, blockchain, NFTs, hardcoded pricing, revenue forecasts, customer-count assumptions, direct accounting mutation, direct storage writes, direct policy finality, or platform-specific handwritten public object drift the SDK boundary.

## Phase 1: SDS Attachment, Mobile SDK Charter, And Phase Boundary

### Work Items

- **1.1 Attach the build plan to SDS #83.**
  - Design: Link this plan from the Mobile SDK SDS, service plan, master build plan, Phase 12 plan, Phase 13 hardening plan, and build-plan crosswalk so builders can navigate from client-package scope to implementation order.
  - Output: Stable links between this file, `docs/sds/mobile/mobile_sdk.md`, `docs/service_catalog/mobile/mobile_sdk.md`, `docs/build_plan/master_plan.md`, `docs/build_plan/phase_12_native_application_layer.md`, `docs/build_plan/phase_13_governance_compliance_scale_hardening.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #83 returns both the Mobile SDK SDS and this sub-build plan.

- **1.2 Preserve Phase 12 as the first full build point.**
  - Design: Keep the Mobile SDK in master Phase 12 because it depends on identity, tenant scope, signing, policy, metering, protected storage refs, native app APIs, Mobile Backend Gateway, AI/RAG routing, and wallet/usage projections.
  - Output: Phase-gate note that earlier phases provide prerequisites while the full mobile client package, generated mobile binding, offline/sync helpers, push/media helpers, and AI/native app helpers start in Phase 12.
  - Validation: Review confirms the plan does not move full mobile client-package delivery before Phase 12 or formal security/compliance hardening before Phase 13.

- **1.3 Freeze SDK ownership boundaries.**
  - Design: Record that the SDK owns client-side configuration, credential-provider interfaces, local ref stores, signed request building, idempotency, offline queue entries, sync cursor cache, media upload state, push registration refs, usage view cache, permission snapshots, and redacted diagnostics.
  - Output: Ownership checklist for package modules, local models, provider interfaces, diagnostics, package APIs, and handoffs.
  - Validation: Review confirms the SDK does not own identity truth, policy finality, wallet balances, ledger state, storage truth, message delivery truth, media publication truth, native app records, AI route decisions, RAG context truth, or backend state transitions.

- **1.4 Carry forward resolved SDS #83 decisions.**
  - Design: Preserve decisions for generated Kotlin/Android first binding after shared-schema/foundation-SDK gates, Swift/iOS parity later, four secure-storage provider interfaces, classed retention, bounded offline support, first helper scope, and compatibility support for the current stable mobile major plus one previous stable major.
  - Output: Resolved-decision checklist covering binding order, provider interfaces, retention classes, first validation path, compatibility states, and fail-closed security breaks.
  - Validation: Review rejects Android-only semantic forks, handwritten public objects, unsafe offline side effects, raw secret storage, silent downgrade behavior, and any platform-specific authority model.

- **1.5 Map prerequisites and consumers.**
  - Design: Identify upstream owner services and downstream app consumers before contracts so refs, APIs, stable errors, and authority boundaries stay consistent.
  - Output: Dependency matrix covering Foundation SDK, Shared Schema Package, Mobile Backend Gateway, Overgate, Overpass, Overtenant, Overkey, Overguard, Overwatch, Overmeter, Overbase, Overstore, Overvault, Wallet/Usage Center, native app services, AI Gateway Router, Personal AI Assistant, Encrypted Docdex RAG Adapter, mSwarm Runtime Bridge, and developer test harnesses.
  - Validation: Matrix review confirms every dependency is used through APIs/events/refs and no plan bypasses owner-service APIs with direct database, object-store, vault, queue, privileged worker, or admin calls.

## Phase 2: Contracts, Generated Models, Stable Errors, And Fixtures

### Work Items

- **2.1 Align shared schema and foundation SDK gates.**
  - Design: Treat the foundation SDK and shared schema package as the contract authority for request envelopes, generated models, error objects, signing inputs, idempotency behavior, compatibility checks, and cross-language fixtures.
  - Output: Binding-readiness gate requiring stable Rust SDK fixtures, TypeScript/web parity where needed, golden request-envelope fixtures, generated error objects, schema compatibility checks, and cross-language tests before mobile binding generation.
  - Validation: Release checks reject mobile binding work when shared schema coverage, generated Rust SDK behavior, error generation, or golden fixtures are incomplete.

- **2.2 Define Mobile SDK local data models.**
  - Design: Model `mobile_sdk_config`, `credential_provider_ref`, `device_context`, `mobile_session`, `request_context`, `signed_mobile_request`, `offline_queue_entry`, `sync_cursor_cache`, `media_upload_state`, `push_registration_state`, `permission_snapshot`, `usage_view_cache`, and `sdk_diagnostic_event`.
  - Output: Canonical schema references, generated mobile models, docs-facing field descriptions, positive fixtures, and negative fixtures.
  - Validation: Model tests reject raw private keys, seed phrases, long-lived bearer tokens, decrypted RAG context, private messages, raw media bytes, unscoped session refs, and diagnostics without redaction class.

- **2.3 Define package API module contracts.**
  - Design: Split the public package API into auth, device, requests, offline, sync, push, media, wallet, native-app helpers, AI, RAG, permissions, and diagnostics modules while keeping runtime authority in backend services.
  - Output: API contract map for `OverridClient.configure`, auth/session, device registration, request builder/sender, offline queue, sync pull/apply, push registration, media upload, wallet/usage reads, native helper modules, AI/RAG requests, permissions, and diagnostic export.
  - Validation: Contract review confirms every mutating helper requires trace/idempotency handling and either signs through the credential provider or fails before network submission.

- **2.4 Define stable result and error contracts.**
  - Design: Align client result states and stable errors for accepted, denied, duplicate, retryable, terminal, expired, conflicted, unsupported schema, revoked credential, unsupported SDK version, privacy-denied AI/RAG context, and secure-storage failure cases.
  - Output: Generated error/result objects, stable reason-code table, redaction rules, and fixtures for gateway/service response decoding.
  - Validation: Tests prove errors are stable, audience-safe, traceable, and do not leak secrets, private payloads, raw RAG context, wallet internals, push provider secrets, or policy/fraud internals.

- **2.5 Build deterministic contract fixtures.**
  - Design: Create fixture sets for configuration, device registration, session restore/refresh/logout, signed requests, offline queue, sync, push, media upload, wallet/usage, AI/RAG, permissions, diagnostics, compatibility profiles, and negative cases.
  - Output: Golden request/response/event fixtures, BLAKE3 fixture hashes, fake gateway responses, fake signer outputs, fake secure-storage states, and compatibility matrix examples.
  - Validation: Fixture tests verify stable ordering, stable hashes, stable reason codes, no missing final newlines, no handwritten public object drift, and no conventional cloud/mobile-backend product assumptions.

## Phase 3: Configuration, Credential Providers, Secure Storage, And Request Pipeline

### Work Items

- **3.1 Implement SDK configuration and environment handling.**
  - Design: Configure gateway URL, app id, tenant defaults, environment, schema versions, SDK version, timeout budgets, retry policy, feature flags, redaction mode, credential provider, storage provider, and compatibility profile handling.
  - Output: `OverridClient.configure(config)` implementation, validation errors, environment guardrails, feature-state parsing, and deterministic configuration fixtures.
  - Validation: Tests prove live endpoints require explicit production configuration, wrong or missing environment data fails before network calls, and unsupported schema/profile states return stable local errors.

- **3.2 Implement credential provider interfaces.**
  - Design: Provide a `credential_provider` abstraction that signs through Android Keystore or StrongBox-backed non-exportable keys where available, later Keychain/Secure Enclave-style providers on iOS, external approved providers where allowed, and fake providers only in tests.
  - Output: Provider trait/interface, key-ref metadata handling, rotation/revocation hints, signing error mapping, and fake signer fixtures.
  - Validation: Secure-storage tests prove the SDK never stores raw private key material, seed phrases, vault secrets, or long-lived bearer tokens in default stores, logs, diagnostics, crash reports, or replay bundles.

- **3.3 Implement secure reference stores.**
  - Design: Provide `secure_ref_store`, `offline_queue_store`, and bounded local cache interfaces for device, session, push, permission, usage, capability, offline queue, sync cursor, and media upload refs with classed retention and clear-all behavior.
  - Output: Provider interfaces, retention policy mapping, logout/device-revoke clearing behavior, fixture stores, and fake secure-storage adapters for tests.
  - Validation: Tests prove local storage is bounded, encrypted where platform support exists, user-clearable, and safe to discard without corrupting server truth.

- **3.4 Implement signed request and idempotency helpers.**
  - Design: Build canonical request contexts with trace id, idempotency key, actor/tenant/app/device refs, command type, schema version, timestamp, data-class hints, body hash, credential id, replay window, request class, and redaction class.
  - Output: Request builder, canonical signing input, idempotency key manager, request-hash helper, send/decode pipeline, and golden signing fixtures.
  - Validation: Golden tests prove canonical signing input, request hash, idempotency key behavior, trace propagation, and error decoding remain stable across releases.

- **3.5 Implement redacted diagnostics baseline.**
  - Design: Record local diagnostic events for configuration, device/session, request build/sign/send, retry, offline queue, sync, push, media, AI/RAG, permissions, and diagnostic export without hidden telemetry.
  - Output: Diagnostic event model, redaction profiles, local metrics hooks, trace filtering, export bundle builder, and diagnostic redactor interface.
  - Validation: Redaction tests prove diagnostics may include ids, reason codes, timings, SDK/app versions, request classes, and scrubbed metadata but not secrets, private payloads, raw messages, raw media, workspace content, location trails, or decrypted RAG text by default.

## Phase 4: Device Registration, Auth, Sessions, And Capability Profiles

### Work Items

- **4.1 Implement device registration helpers.**
  - Design: Register or refresh device refs and capability profiles through Mobile Backend Gateway with app install ref, platform, app version, SDK version, push capability, privacy settings, and audit refs returned by the gateway.
  - Output: `client.device.register()`, `client.device.revoke(deviceRef)`, device context cache, revoked-device cleanup, and registration fixtures.
  - Validation: Integration tests prove registration is signed, tenant/app scoped, idempotent on refresh, clears local state on revoke, and does not rely on broad device fingerprinting beyond security necessity.

- **4.2 Implement auth and session bootstrap helpers.**
  - Design: Start login, restore session refs, refresh sessions, and logout through Mobile Backend Gateway, Overgate, Overpass, Overtenant, and Overkey refs without making the SDK identity truth.
  - Output: `client.auth.startLogin`, `restoreSession`, `refreshSession`, `logout`, local session cache, credential correction paths, and auth fixtures.
  - Validation: Tests prove missing credential provider, expired session, revoked device, wrong tenant, unsupported app, invalid schema, and refresh denial fail before unsafe side effects.

- **4.3 Implement session state machine handling.**
  - Design: Encode session states from `unconfigured` through `configured`, `device_registered`, `login_pending`, `active`, `refresh_required`, `refreshing`, `restricted`, `logged_out`, `revoked`, and `expired`.
  - Output: Session state reducer, state transition fixtures, local state persistence hooks, and app callback hooks for restricted/refresh-required states.
  - Validation: State tests prove the SDK never treats a local session as active, accepted, or refreshed without a compatible runtime response.

- **4.4 Implement capability and compatibility profile handling.**
  - Design: Parse minimum, recommended, deprecated, unsupported, degraded-compatible, read-only, sync-paused, refresh-required, and unsupported feature states for app, SDK, schema, and helper versions.
  - Output: Compatibility evaluator, feature flag resolver, upgrade guidance surfaces, degraded-state behavior, and migration fixture matrix.
  - Validation: Compatibility tests prove current stable mobile app/SDK major plus one previous stable major works for non-security-breaking behavior, while mutating, offline, sensitive-content, and security-critical breaks pause or fail closed.

- **4.5 Attach usage and audit refs to client lifecycle calls.**
  - Design: Preserve actor, tenant, app, device, trace, request class, service target, SDK version, and runtime usage/audit refs returned by gateway and owner services.
  - Output: Lifecycle usage-ref capture, audit-ref propagation, wallet-visible usage cache hooks, and diagnostic correlation.
  - Validation: Tests prove lifecycle helpers surface runtime refs without mutating ORU balances, Seal Ledger entries, invoices, grants, statements, or policy decisions.

## Phase 5: Offline Queue, Crash-Safe Replay, And Background Flush

### Work Items

- **5.1 Implement offline queue entry storage.**
  - Design: Store local entry id, command type, target service, payload ref/hash, idempotency key, trace id, expiry, retry count, current state, last error, and gateway command ref using the `offline_queue_store` provider.
  - Output: Offline queue model, encrypted store adapter interface, crash-safe write plan, state machine, and fixture entries.
  - Validation: Offline storage tests reject entries without idempotency key, request hash, expiry, command type, target service, trace id, or redaction class.

- **5.2 Restrict offline command classes.**
  - Design: Allow only replay-safe, bounded, owner-service-revalidated flows such as sync acknowledgements/resets, encrypted messaging envelopes, resumable media work, low-risk drafts/proposals, safe local cache updates, and permission revocation or narrowing requests.
  - Output: Offline command allowlist, denied reason mapping, command classification helper, and fixtures for denied high-risk cases.
  - Validation: Negative tests prove spend, grants, refunds, statement issuance, public publication, moderation finality, AI/RAG execution, share-link creation, search visibility expansion, and permission expansion cannot be reported as terminal offline success.

- **5.3 Implement enqueue, retry, and background flush behavior.**
  - Design: Enqueue commands with request hash, expiry, retry policy, connectivity/session gates, battery/network/user-permission hints, and crash-safe flush cursors.
  - Output: `client.offline.enqueue(command)`, `client.offline.flush(policy)`, retry scheduler, session-refresh-before-flush hook, and background policy fixtures.
  - Validation: Offline tests cover network loss, app restart, duplicate replay, retryable errors, terminal failures, session refresh before flush, and bounded background execution.

- **5.4 Implement duplicate, conflict, expiry, and discard handling.**
  - Design: Resolve duplicates by idempotency key and request hash, keep local evidence for conflicts, expire commands without submission after expiry, and support user discard with clear local state.
  - Output: Duplicate resolver, conflict states, expiry sweeper, discard flow, and stable errors for duplicate mismatch, expired command, denied replay, and conflict pending.
  - Validation: Replay tests prove reconnect storms do not duplicate non-idempotent commands and mismatched request hashes produce stable conflict errors rather than silent acceptance.

- **5.5 Emit offline diagnostics and usage refs.**
  - Design: Track queue depth, oldest entry age, retry counts, denied reasons, session-refresh blockers, gateway command refs, downstream refs, usage refs, and redacted replay evidence.
  - Output: Offline diagnostics, local metrics hooks, replay bundle snippets, and wallet/usage ref capture.
  - Validation: Tests reconstruct offline command flow through refs without raw private payloads and prove high-risk mutations cannot report terminal success when audit/usage refs cannot be reconciled.

## Phase 6: Sync Cursors, Delta Handling, Conflict Callbacks, And Retention

### Work Items

- **6.1 Implement sync pull and cursor cache helpers.**
  - Design: Pull authorized deltas by service scope, cursor token hash, last acknowledged version, pending local changes, data-class scope, and permission snapshot.
  - Output: `client.sync.pull(scope, cursor)`, cursor cache model, gateway response decoding, and pull fixtures.
  - Validation: Sync tests cover cursor issuance, pull, expiry, tenant scoping, data-class filtering, permission-denied scopes, and reset-required state.

- **6.2 Implement delta application helpers.**
  - Design: Apply deltas, tombstones, changed record refs, media refs, conflict hints, retry hints, redaction class, and reset instructions through app-provided handlers without copying owner-service private records into SDK authority.
  - Output: `client.sync.apply(delta, handlers)`, handler contract, tombstone handling, app acknowledgement hooks, and compact delta fixtures.
  - Validation: Tests prove private data remains behind owner-service refs and unauthorized or revoked permissions omit records before previews, snippets, cached content, or local hints.

- **6.3 Implement acknowledgement, conflict, and reset flows.**
  - Design: Advance cursors only after app acknowledgement, surface conflicts through callbacks, and clear affected cursor scope through gateway-provided reset flows.
  - Output: Ack helper, conflict callback contract, reset handler, cursor state reducer, and conflict fixture set.
  - Validation: Tests prove ack replay is idempotent, conflicting base versions are surfaced safely, and owner services retain final mutation authority.

- **6.4 Implement local retention and invalidation rules.**
  - Design: Apply the SDS retention decisions by data class: short caps for replay-safe offline commands and sensitive cursors, bounded media/draft persistence, stale usage view timers, redacted diagnostic rings, support bundle retention, and reset on permission/tenant/schema/compatibility changes.
  - Output: Retention policy evaluator, invalidation hooks, logout/device-revoke clearing, privacy-request clear-all behavior, and retention fixtures.
  - Validation: Retention tests prove private data is not retained beyond policy, sensitive cursor caps are enforced, user clear works, and cleanup does not erase required audit/evidence refs returned by services.

- **6.5 Add sync diagnostics, fixtures, and metrics hooks.**
  - Design: Track cursor age, denied counts, delta sizes, tombstone counts, conflict counts, reset counts, local apply failures, and mobile usage refs without exposing private content.
  - Output: Sync diagnostic events, local metrics hooks, golden sync fixtures, and replay snippets.
  - Validation: Observability tests prove stale cursor loops, conflict spikes, permission-denial spikes, reset storms, and app-handler failures are visible without raw source records.

## Phase 7: Push, Media Uploads, Wallet Readers, And Usage Views

### Work Items

- **7.1 Implement push registration helpers.**
  - Design: Register, rotate, and revoke push token refs through gateway-approved contracts with provider class, permission state, notification preference refs, rotation state, and last gateway state.
  - Output: `client.push.register(tokenRef)`, token-revoke helper, push registration state, preference hooks, and token-ref fixtures.
  - Validation: Tests prove raw provider tokens stay out of broad records, revoked sessions stop refresh, push permission revocation clears local state, and sensitive content requires authenticated retrieval.

- **7.2 Enforce notification redaction and preference behavior.**
  - Design: Keep notification display separate from message/content fetch; apply notification preferences, quiet hours, data-class permissions, redaction class, and app capability profile before local notification helpers expose payloads.
  - Output: Notification redaction evaluator, preference state helpers, stable denied reasons, and reference-only push payload fixtures.
  - Validation: Push tests prove local display helpers do not include raw messages, wallet balances, usage details, location trails, workspace content, decrypted RAG/AI text, vault refs, secrets, or signed URLs by default.

- **7.3 Implement media upload session helpers.**
  - Design: Coordinate media upload session creation, chunk manifest refs, BLAKE3/content hash checks, retry state, local file refs, Overstore target refs, and completion refs without owning media publication truth.
  - Output: `client.media.createUploadSession(input)`, `client.media.uploadChunk(session, chunk)`, upload state reducer, checksum helpers, and resumable upload fixtures.
  - Validation: Media tests prove interrupted uploads resume or expire deterministically, conflicting finalization returns a stable error, raw media bytes stay out of diagnostics, and final visibility remains owner-service authority.

- **7.4 Implement wallet and usage readers.**
  - Design: Read wallet-visible usage summaries, quota/precheck refs, receipt refs, statement refs, grant/hold refs, stale state, refresh time, and permission inventory without mutating accounting truth.
  - Output: `client.wallet.getUsage(query)`, usage view cache, stale/fresh/pending/unavailable states, receipt/statement display helpers, and wallet fixtures.
  - Validation: Tests prove wallet/accounting remains read-only through the SDK and spend, grants, refunds, statements, disputes, and accounting-changing actions require live owner-service checks.

- **7.5 Add cross-flow diagnostics for push, media, and wallet.**
  - Design: Correlate push registration, media upload, wallet usage reads, request ids, trace ids, usage refs, audit refs, retryability, redaction class, and SDK/app version in local diagnostics.
  - Output: Cross-flow diagnostic events, support bundle filters, local metrics hooks, and replay fixture snippets.
  - Validation: Diagnostic tests reconstruct push/media/wallet flows through refs without raw provider tokens, raw media bytes, private usage internals, or sensitive notification content.

## Phase 8: Native App Helpers, AI/RAG Handoffs, Permissions, And Privacy Controls

### Work Items

- **8.1 Implement generated native-app helper modules.**
  - Design: Provide typed helpers for Messaging Center, Workspace, Social Photo/Video, Search, Maps, Directory, Wallet/Usage Center, Central AI Stewardship, and related native app flows over approved APIs while preserving owner-service state authority.
  - Output: `client.messaging`, `client.workspace`, `client.mediaApp`, `client.search`, `client.maps`, `client.directory`, and narrow helper modules with feature flags and capability checks.
  - Validation: Tests prove helper modules adapt mobile calls without owning message threads, posts, documents, search indexes, map truth, directory listings, wallet state, or stewardship decisions.

- **8.2 Implement Personal AI and AI Gateway helpers.**
  - Design: Submit mobile AI requests through Personal AI Assistant or AI Gateway Router with privacy mode, context refs, route classification metadata, tool proposal refs, usage refs, leakage-profile visibility, and route-bound expiry.
  - Output: `client.ai.ask(input)`, AI request/result models, route-ref decoding, degraded/fallback states, and AI fixture bundles.
  - Validation: Tests prove AI execution is never accepted offline, route decisions remain AI Gateway Router/owner-service authority, and decrypted context is not stored in SDK caches or diagnostics.

- **8.3 Implement encrypted Docdex RAG request helpers.**
  - Design: Request authorized encrypted Docdex RAG context through approved adapters with context grants, leakage profiles, encrypted context bundle refs, route refs, usage refs, and revocation handling.
  - Output: `client.rag.query(input)`, RAG grant/ref models, context-source preview helpers, revocation error handling, and RAG fixture bundles.
  - Validation: Tests prove privacy mode, context refs, encrypted Docdex grants, model-route refs, and usage refs survive request/response decoding without persisting decrypted RAG text.

- **8.4 Implement permission prompts and revocation helpers.**
  - Design: Read and revoke app permissions, data-class grants, notification permissions, background sync settings, AI/RAG context grants, and permission snapshots with explicit user-visible state.
  - Output: `client.permissions.list()`, `client.permissions.revoke(ref)`, permission prompt model, revocation state reducer, and permission fixtures.
  - Validation: Tests prove permissions are visible and revocable, high-risk revocations fail closed where required, and permission expansion cannot occur offline or without a live owner-service response.

- **8.5 Implement privacy and data-class guardrails.**
  - Design: Apply privacy mode, data-class hints, local cache rules, background sync bounds, AI/RAG context restrictions, push redaction, location/contact/message/workspace/media sensitivity, and diagnostics redaction consistently across helper modules.
  - Output: Privacy guardrail helper, data-class policy mapping, local denial reasons, redaction-class propagation, and cross-module privacy fixtures.
  - Validation: Privacy tests prove denied data classes, revoked grants, exact/current/background location denial, private content restrictions, and AI/RAG context denials fail closed before local cache hints, previews, snippets, notifications, or diagnostics expose protected data.

## Phase 9: Mobile Binding Generation, Examples, Release, And Compatibility Operations

### Work Items

- **9.1 Generate the first Kotlin/Android Mobile SDK binding.**
  - Design: Generate the Phase 12 pilot binding from shared schema package outputs, foundation SDK request-envelope fixtures, generated error objects, signing/idempotency fixtures, and cross-language contract tests.
  - Output: Kotlin/Android package modules, generated models, provider interfaces, Gradle/package metadata where applicable, fixture conformance tests, and no handwritten public object drift.
  - Validation: Binding tests prove Android semantics are not a fork: Mobile Backend Gateway remains platform-neutral and Kotlin public models match shared contracts.

- **9.2 Prepare Swift/iOS and shared-core parity gates.**
  - Design: Define the gate for Swift/iOS and any shared core/platform-adapter packaging to follow from the same contracts after Kotlin/Android pilot evidence is stable.
  - Output: Swift/iOS readiness checklist, parity fixture matrix, provider-interface mapping to Keychain/Secure Enclave-style storage, and blocked-until-stable release state.
  - Validation: Review confirms no iOS or shared-core package defines separate authority semantics, separate request objects, or weaker secure-storage expectations.

- **9.3 Build fixture gateways, fake providers, and sample app flows.**
  - Design: Provide fake gateway, fake credential providers, fake secure storage, deterministic clocks, offline network simulators, and sample flows for authentication, sync, media, messaging, AI, permission revocation, and offline replay.
  - Output: Test harness package, sample app integration flows, local fixtures, and deterministic scenario scripts.
  - Validation: Sample tests prove a mobile app can authenticate, sync, store media, send messages, submit AI requests, revoke permissions, and replay offline-safe commands through normal Overrid rails.

- **9.4 Implement release, version, and compatibility operations.**
  - Design: Manage SDK version metadata, app compatibility profile parsing, deprecated/unsupported feature states, migration guidance, release notes, fixture hashes, and support for current stable plus one previous stable major where safe.
  - Output: Release checklist, compatibility profile tests, migration docs, semantic version gate, fixture hash manifest, and upgrade guidance templates.
  - Validation: Compatibility checks reject breaking command-envelope, signing, idempotency, session, tenant, privacy, data-class, accounting, push-redaction, attestation, AI/RAG context, or policy changes without new major guidance and fail-closed behavior.

- **9.5 Complete developer documentation and support handoff.**
  - Design: Document package configuration, provider interfaces, local storage behavior, request signing, offline queueing, sync, push, media, wallet/usage, AI/RAG, permissions, diagnostics, compatibility profiles, and safe failure handling.
  - Output: Developer docs, API reference, troubleshooting guide, support bundle guide, security notes, and integration checklist for approved native and third-party apps.
  - Validation: Documentation review confirms no docs recommend direct service database access, private worker endpoints, privileged admin APIs, raw secret storage, mobile-backend SaaS replacement, hidden telemetry, or platform-specific shortcuts.

## Phase 10: End-To-End Validation, Phase 13 Hardening, And Documentation Handoff

### Work Items

- **10.1 Validate common mobile flows end to end.**
  - Design: Run end-to-end tests for configuration, device registration, login, session restore/refresh/logout, signed request sending, usage view, sync pull/apply/ack, offline replay, push registration, media upload, messaging, AI/RAG handoff, permission revocation, and diagnostics export.
  - Output: Integration suite, golden traces, fixture snapshots, sample app evidence, and acceptance records.
  - Validation: Tests prove approved native and third-party mobile apps can use Overrid as a backend/resource plane without direct low-level service calls.

- **10.2 Validate security and privacy boundaries.**
  - Design: Cover missing configuration, missing credential provider, invalid schema, signing failure, network timeout, expired session, revoked device, unsupported SDK version, denied data class, sensitive push payload, raw private replay attempt, secure-storage failure, AI/RAG privacy denial, and owner-service denial.
  - Output: Security/privacy test suite, negative fixtures, redaction reports, stable reason-code evidence, and fail-closed checklist.
  - Validation: Tests prove failures occur before unsafe side effects and private data does not leak through push payloads, logs, hidden telemetry, local cache hints, replay bundles, diagnostics, support exports, or crash reports.

- **10.3 Validate metering and accounting handoffs.**
  - Design: Prove request helpers, lifecycle calls, sync, offline queue, push registration, media upload, wallet/usage reads, AI/RAG calls, permission changes, diagnostics, and replay preserve runtime usage/accounting refs without mutating accounting truth.
  - Output: Usage test suite, wallet-visible summary fixtures, receipt/statement display fixtures, and accounting boundary checklist.
  - Validation: Tests prove usage is metered and rate-limited while the SDK does not maintain balances, mutate ledger entries, call payment providers, calculate payouts, issue statements, issue grants, or decide disputes.

- **10.4 Validate operations, scale, and Phase 13 hooks.**
  - Design: Run drills for app restart after queued commands, reconnect storms, queue backlog, sync loop, upload backlog, push permission churn, notification provider outage, AI route outage, compatibility rollout, redaction failure, storage provider failure, support bundle export, unsupported major version, and mobile abuse spike.
  - Output: Drill records, alert evidence, recovery notes, threat/security review refs, compliance/incident/reporting hooks, public-safe summary fields, and open risk list.
  - Validation: Drills prove SDK behavior is replayable, bounded, fail-closed where required, privacy-safe, compatible with Phase 13 review, and ready for broader mobile participation hardening.

- **10.5 Complete documentation and handoff readiness.**
  - Design: Update SDS, service catalog, master plan, build-plan crosswalk, Phase 12, Phase 13, tech-stack notes, progress, queue state, and Docdex index/search evidence so implementation teams have one aligned source of truth.
  - Output: Linked docs, queue/progress metadata, validation evidence, implementation handoff notes, and Docdex retrieval evidence.
  - Validation: Structure checks pass for title prefix, attached SDS, phases 1 through 10, 50 work items, Design/Output/Validation fields, local links, stack guardrails, queue state, Docdex search, pre-commit hooks, and documented test-runner blockers if repo tests remain unavailable.
