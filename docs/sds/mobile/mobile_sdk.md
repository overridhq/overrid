SDS #83

# Mobile SDK SDS

## Purpose

Let mobile apps use Overrid as a backend/resource plane for identity, wallet, sync, storage, messaging, media, AI, offline queueing, and permissions.

Mobile SDK is a versioned iOS/Android client package, not a backend service. It gives mobile apps safe client primitives for Overrid: device/session bootstrap, request signing, idempotency, trace propagation, secure local storage adapters, offline queueing, sync, push notification registration, media upload helpers, wallet/usage readers, AI gateway calls, encrypted Docdex RAG context requests, permission prompts, telemetry redaction, and stable error handling. The SDK must keep mobile apps on the normal Overrid rails instead of encouraging private shortcuts.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [mobile_sdk.md](../../service_catalog/mobile/mobile_sdk.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md) |
| Sub-build plan | [SUB BUILD PLAN #83 - Mobile SDK](../../build_plan/sub_build_plan_083_mobile_sdk.md) |

## Service Family

- Family: Mobile service layer
- Owning layer: Mobile client package, secure local adapters, request pipeline, offline queue, sync client, push registration client, media helpers, AI/native-app helpers, and mobile diagnostics
- Runtime shape: versioned Swift/Kotlin or cross-platform package modules, generated models, fixtures, test harnesses, and app integration examples
- Primary data scope: local config, device refs, session refs, credential refs, request envelopes, local offline queue entries, sync cursors, push token refs, media upload state, permission snapshots, usage views, and local diagnostics
- First build phase from service plan: [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md)

## Problem Statement

Mobile app developers should not hand-build Overrid security, identity, offline, sync, wallet, AI, or media flows. Without an SDK, each app would duplicate signing, store secrets differently, retry unsafe requests, leak private payloads in logs or push notifications, drift from shared schemas, and bypass usage visibility.

The SDK must turn Overrid into a practical mobile backend/resource plane while staying honest about authority. Runtime services decide identity, policy, storage, accounting, and domain state. The SDK only prepares, protects, submits, retries, and decodes mobile workflows.

## Goals

- Provide a canonical mobile client pipeline for configuration, trace id creation, request signing, idempotency, schema validation, retry classification, and error decoding.
- Support mobile auth/session bootstrap through Mobile Backend Gateway, Overgate, Overpass, Overtenant, and Overkey refs.
- Provide secure local adapters for credential refs, session refs, offline queue entries, cached deltas, media upload state, and permission snapshots.
- Implement bounded offline queueing and sync helpers that preserve idempotency, expiry, trace ids, conflict state, and replay evidence.
- Provide typed helpers for wallet/usage views, messaging, media uploads, workspace/search/maps/directory flows, Personal AI Assistant, AI Gateway Router, and encrypted Docdex RAG requests.
- Keep app permissions, notification preferences, background sync, local cache retention, and privacy audit visible to users.
- Keep diagnostics redacted and useful for support without exposing private content, secrets, or raw RAG context.

## Non-Goals

- Do not make the SDK a backend, identity authority, policy engine, storage system, ledger authority, wallet balance authority, or native app domain service.
- Do not store raw private keys, seed phrases, vault secrets, long-lived bearer tokens, decrypted Docdex context, private messages, or raw media by default.
- Do not call internal service databases, private worker endpoints, or privileged admin APIs.
- Do not silently downgrade policy, privacy, data-class, or authorization failures into local fallbacks.
- Do not include ad trackers, hidden telemetry, behavioral manipulation, speculative monetization logic, or marketplace ad ranking helpers.
- Do not hardcode prices, usage charges, model-provider choices, or economic projections.

## Primary Actors And Clients

- Native Overrid mobile apps for wallet, messaging, social photo/video, workspace, search, maps, directory listings, central AI stewardship, and personal AI.
- Third-party mobile app developers using Overrid as a backend/resource plane through approved app manifests.
- Mobile Backend Gateway as the primary mobile API adaptation layer.
- Overgate, Overpass, Overtenant, Overkey, Overguard, Overwatch, Overmeter, ORU Account Service, and Seal Ledger as platform rails.
- AI Gateway Router, Personal AI Assistant, Encrypted Docdex RAG Adapter, Lightweight Classifier, and ADES Enrichment Adapter for mobile AI flows.
- Overstore, Overbase, Overvault, Messaging Center, Social Photo/Video App, Workspace and Office Suite, Search Engine, Maps and Navigation, Directory Listings, and Wallet/Usage Center through approved APIs.
- Mobile operators and support tools consuming redacted SDK diagnostics and replay refs.

## Dependencies

- [SDK](../foundation/sdk.md) for shared client conventions, request envelopes, schema-generated models, error shapes, signing, idempotency, and test fixtures.
- [Mobile Backend Gateway](mobile_backend_gateway.md) for mobile-specific API adaptation, device/session refs, sync, offline command intake, push, media upload sessions, and mobile usage summaries.
- [Overgate](../control_plane/overgate.md), [Overpass](../control_plane/overpass.md), [Overtenant](../control_plane/overtenant.md), and [Overkey](../control_plane/overkey.md) for admission, identity, tenant, and credential refs.
- [Overguard](../trust_policy_verification/overguard.md), [Overwatch](../control_plane/overwatch.md), and [Overmeter](../execution_scheduling/overmeter.md) for policy decisions, audit refs, traces, and usage refs.
- [Overbase](../data_storage_namespace/overbase.md), [Overstore](../data_storage_namespace/overstore.md), and [Overvault](../data_storage_namespace/overvault.md) for authorized state/object/secret refs surfaced through APIs.
- [AI Gateway Router](../ai_rag_model_routing/ai_gateway_router.md), [Personal AI Assistant](../ai_rag_model_routing/personal_ai_assistant.md), and [Encrypted Docdex RAG Adapter](../ai_rag_model_routing/encrypted_docdex_rag_adapter.md) for AI and RAG request helpers.
- Native app service SDS files for wallet, messaging, social media, workspace, directory, search, maps, and stewardship flows.

## Owned Responsibilities

Mobile SDK owns client-side correctness:

- Mobile client configuration, environment selection, compatibility profile handling, and SDK version reporting.
- Credential provider interfaces that reference platform keychain/keystore/secure enclave or approved external key providers without storing raw private material.
- Device registration, session restore, session refresh, logout, device revoke, and push token registration helpers.
- Signed request builders, idempotency key management, trace propagation, stable error decoding, and retry classification.
- Offline queue storage, command expiry, request-hash checks, replay handling, and safe background flush behavior.
- Sync cursor management, local delta application helpers, tombstone handling, conflict callbacks, and reset flows.
- Media upload session helpers for chunking, resumable upload, checksum, retry, and Overstore handoff refs.
- Wallet/usage readers and receipt/statement/ref display helpers that do not mutate accounting truth.
- AI request helpers for route classification metadata, privacy modes, context refs, encrypted Docdex RAG authorization refs, and tool-call proposals.
- Redacted local diagnostics, local metrics, fixture clients, and contract-test harnesses.

Runtime services own authoritative state. The SDK must not infer acceptance, completion, balance changes, message delivery, media publication, or AI result finality without service responses.

## Data Model

- `mobile_sdk_config`: environment, gateway base URL, app id, tenant defaults, schema versions, SDK version, timeout budgets, retry policy, feature flags, and log redaction mode.
- `credential_provider_ref`: platform keychain/keystore/secure enclave slot, service credential ref, API key ref, delegated credential ref, rotation state, and revocation hints; no raw private key material.
- `device_context`: device ref, app install ref, platform, app version, SDK version, capability profile, push capability, privacy settings, and audit refs returned by gateway.
- `mobile_session`: session ref, actor/tenant refs, expiry, refresh state, credential refs, device refs, risk class, and last verified state.
- `request_context`: trace id, idempotency key, actor id, tenant id, app id, device id, command type, schema version, timestamp, and data-class hints.
- `signed_mobile_request`: canonicalized method/path, body hash, credential id, signature metadata, replay window, request class, and redaction class.
- `offline_queue_entry`: local entry id, command type, target service, payload ref/hash, idempotency key, trace id, expiry, retry count, current state, last error, and gateway command ref.
- `sync_cursor_cache`: service scope, cursor token hash, last acknowledged version, pending local changes, tombstone refs, conflict markers, and reset state.
- `media_upload_state`: upload session ref, chunk manifest, checksum state, retry state, local file ref, Overstore target ref, and completion ref.
- `push_registration_state`: push token ref, provider class, permission state, rotation state, last gateway state, and notification preference refs.
- `permission_snapshot`: app permission refs, data-class permissions, notification permissions, background sync settings, AI/RAG context grants, and revocation timestamps.
- `usage_view_cache`: wallet-visible usage summary refs, receipt refs, statement refs, quota/precheck refs, refresh time, and stale state.
- `sdk_diagnostic_event`: local event category, trace id, request class, reason code, timing, retryability, SDK/app version, redaction class, and scrubbed metadata.

Local storage must be bounded, encrypted where platform support exists, user-clearable, and safe to discard without corrupting server truth.

## API Surface

The SDK exposes package APIs rather than network endpoints:

- `OverridClient.configure(config)`: create a mobile client with gateway URL, app id, environment, schema versions, credential provider, storage provider, and redaction policy.
- `client.auth.startLogin(input)`: start mobile login or identity bootstrap through approved Overrid flows.
- `client.auth.restoreSession()`: restore a local session ref and verify freshness.
- `client.auth.refreshSession()`: refresh a session through Mobile Backend Gateway.
- `client.auth.logout(options)`: clear local session state and optionally revoke device/session refs.
- `client.device.register()`: register or refresh device refs and capability profile.
- `client.device.revoke(deviceRef)`: revoke device refs and clear local state.
- `client.requests.buildSigned(input)`: build, validate, sign, and redact a mobile request envelope.
- `client.requests.sendSigned(request)`: send a signed request and decode accepted, denied, duplicate, retryable, or terminal responses.
- `client.offline.enqueue(command)`: enqueue a bounded offline command with idempotency key, expiry, trace id, and payload ref/hash.
- `client.offline.flush(policy)`: flush eligible queued commands and update local entry state from gateway responses.
- `client.sync.pull(scope, cursor)`: fetch authorized deltas for a service scope.
- `client.sync.apply(delta, handlers)`: apply deltas, tombstones, conflicts, and reset instructions through app-provided handlers.
- `client.push.register(tokenRef)`: register, rotate, or revoke push token refs.
- `client.media.createUploadSession(input)`: create resumable media upload state and gateway upload refs.
- `client.media.uploadChunk(session, chunk)`: upload chunks with checksum and retry handling.
- `client.wallet.getUsage(query)`: read wallet-visible usage summaries, receipts, statements, and quota/precheck refs.
- `client.messaging`, `client.workspace`, `client.mediaApp`, `client.search`, `client.maps`, and `client.directory`: typed native-app helper modules over approved APIs.
- `client.ai.ask(input)`: submit a mobile AI request through Personal AI Assistant or AI Gateway Router with privacy, usage, and context refs.
- `client.rag.query(input)`: request authorized encrypted Docdex RAG context through approved adapters.
- `client.permissions.list()` and `client.permissions.revoke(ref)`: read and revoke app/data/notification/AI context permissions.
- `client.diagnostics.export(traceId, redactionProfile)`: export redacted local diagnostics for support or replay.

All mutating helpers require trace and idempotency handling. Test fixtures may provide fake signers and local storage, but production configuration must require a real credential provider.

## Event Surface

The SDK does not emit platform-authoritative events. It emits local diagnostics and forwards runtime event refs unchanged.

- Local diagnostics: `sdk.configured`, `sdk.device_registered`, `sdk.session_restored`, `sdk.session_refreshed`, `sdk.request_built`, `sdk.request_signed`, `sdk.request_sent`, `sdk.response_received`, `sdk.retry_scheduled`, `sdk.offline_queued`, `sdk.offline_flushed`, `sdk.sync_pulled`, `sdk.sync_conflict`, `sdk.push_registered`, `sdk.media_upload_progress`, `sdk.ai_request_sent`, `sdk.rag_request_sent`, `sdk.permission_revoked`, and `sdk.diagnostic_exported`.
- Runtime refs surfaced unchanged: Overgate admission refs, Mobile Backend Gateway refs, native service refs, AI route refs, RAG retrieval refs, Overwatch audit refs, Overmeter usage refs, wallet receipt refs, and denial reason codes.
- Privacy rule: local diagnostics may include ids, reason codes, timings, SDK/app versions, request classes, and redacted metadata, but must not include raw secrets, private payloads, messages, media bytes, workspace content, location trails, or decrypted RAG text by default.

## Core Workflow

1. App configures the SDK with environment, app id, gateway URL, schema versions, credential provider, storage provider, and redaction policy.
2. SDK registers or refreshes device refs and retrieves a compatibility profile.
3. User starts or restores an identity/session flow; SDK stores only approved refs in secure local storage.
4. App submits requests through SDK helpers that attach trace ids, idempotency keys, actor/tenant/app/device refs, schema versions, signatures, and redaction classes.
5. SDK decodes gateway/service responses into typed success, denial, duplicate, conflict, retryable, expired, or terminal failure results.
6. Offline commands are queued locally with expiry and request hash, then flushed when connectivity and policy allow.
7. Sync helpers pull deltas, apply tombstones, surface conflicts, and advance cursors only after app acknowledgement.
8. Media, push, wallet, native-app, AI, and encrypted RAG helpers use gateway-approved contracts and surface runtime refs.
9. SDK records redacted diagnostics and exposes replay bundles to authorized support flows when the user or operator permits.

## State Machine

Session lifecycle:

1. `unconfigured`
2. `configured`
3. `device_registered`
4. `login_pending`
5. `active`
6. `refresh_required`
7. `refreshing`
8. `restricted`
9. `logged_out`
10. `revoked`
11. `expired`

Request lifecycle:

1. `prepared`
2. `validated`
3. `signed`
4. `queued_for_send`
5. `sent`
6. `accepted`
7. `duplicate_resolved`
8. `retry_wait`
9. `denied`
10. `failed`
11. `completed`

Offline queue entry lifecycle:

1. `queued`
2. `waiting_for_connectivity`
3. `waiting_for_session_refresh`
4. `ready_to_flush`
5. `flushing`
6. `accepted_by_gateway`
7. `delivered`
8. `duplicate_resolved`
9. `conflicted`
10. `expired`
11. `failed`
12. `discarded_by_user`

Sync lifecycle:

1. `idle`
2. `pulling`
3. `applying_delta`
4. `conflict_pending`
5. `acknowledging`
6. `advanced`
7. `reset_required`
8. `failed`

The SDK must never move a local request to `accepted`, `delivered`, or `completed` without a compatible runtime response.

## Policy And Security

- Require explicit production configuration before live Overrid endpoints can be used.
- Store credential material only through approved platform keychain/keystore/secure enclave or external credential provider interfaces.
- Do not persist raw secrets, private keys, seed phrases, vault values, decrypted RAG context, private messages, media bytes, or private documents unless the app explicitly owns and protects that local content.
- Encrypt local SDK storage when platform support exists and provide clear-all behavior for logout, device revoke, and user privacy requests.
- Deny unsigned mutating calls in production.
- Treat revoked credentials, expired sessions, unsupported schemas, wrong tenants, denied data classes, and privacy-denied AI/RAG context as terminal until corrected by an approved flow.
- Keep push registration and notification display separate from message/content fetch; sensitive content requires authenticated retrieval.
- Keep background sync bounded by user permission, OS constraints, tenant policy, battery/network settings, and app capability profile.
- Redact logs and diagnostics by default, with explicit opt-in for broader support bundles.

## Metering And Accounting

- Attach actor, tenant, app, device, trace, request class, service target, and SDK version refs so runtime services can meter accurately.
- Surface Overmeter usage summaries, ORU refs, Seal Ledger refs, receipts, statements, grants, holds, and dispute refs returned by runtime services.
- Provide wallet/usage readers that clearly separate fresh, stale, pending, and unavailable usage views.
- Do not maintain balances, mutate ledger entries, call payment providers, or calculate provider payouts.
- Keep mobile helper behavior structural and testable without embedding charge tables, forecasts, or market assumptions.

## Observability And Operations

- Provide redacted structured diagnostics for request lifecycle, session refresh, offline queue depth, sync cursor state, media upload progress, push token state, AI/RAG handoff, denial reason codes, retries, and SDK version.
- Expose local metrics hooks for app developers without sending hidden telemetry.
- Provide fixture gateways, fake credential providers, fake secure storage, deterministic clocks, and offline network simulators for contract tests.
- Support compatibility warnings and minimum-version guidance returned by Mobile Backend Gateway.
- Provide local diagnostic export with user-visible redaction profile and trace id filtering.
- Include crash-safe local queue recovery so app restarts do not duplicate non-idempotent commands.

## Failure Modes And Recovery

- Missing configuration: fail before network calls with local configuration reason.
- Missing credential provider: fail before signing and do not retry.
- Invalid schema: fail before signing.
- Signing failure: fail before network submission and surface credential correction path.
- Network timeout before response: retry only if idempotency key and request hash are available.
- Session expired: refresh session before replaying eligible requests; fail terminally when refresh is denied.
- Device revoked: clear local session, push refs, cached capability profile, and offline commands that cannot be safely resubmitted.
- Offline command expired: mark expired locally and do not submit.
- Gateway duplicate conflict: surface terminal conflict and keep local evidence for user/app resolution.
- Sync reset required: clear affected cursor scope and rebootstrap through gateway-provided reset flow.
- Push permission revoked: stop registration refresh and clear local push state.
- Unsupported SDK version: surface upgrade guidance and avoid silent downgrade unless a signed compatibility profile allows it.

## Validation Plan

The service implementation plan lists these specific validation requirements:

- Mobile app can authenticate, sync, store media, send messages, and submit AI requests.
- Offline operations replay idempotently.
- Permissions are visible and revocable.

Additional SDS-level validation:

- Contract tests cover configuration, device registration, session restore/refresh/logout, signed requests, offline queue, sync, push, media upload, wallet/usage, AI, RAG, permissions, and diagnostics.
- Golden tests prove canonical signing input, request hashes, idempotency keys, trace propagation, and error decoding remain stable across releases.
- Secure-storage tests prove secrets are not written to default logs, plain local storage, diagnostics, or crash reports.
- Offline tests cover network loss, app restart, duplicate replay, conflict, expiry, and session refresh before flush.
- Sync tests cover delta application, tombstones, conflict callbacks, cursor reset, permission changes, and data-class filtering.
- Push tests prove sensitive content is fetched only after authenticated app open.
- AI/RAG tests prove privacy mode, context refs, encrypted Docdex grants, model-route refs, and usage refs survive request/response decoding.
- Compatibility tests prove old SDK versions receive clear upgrade/degraded responses.

## Build Breakdown

1. Define mobile SDK config, credential provider, device context, session, request context, signed request, error, offline queue, sync cursor, media upload, push, permission, usage, and diagnostic models.
2. Generate mobile models from shared schemas and align error/response decoding with the foundation SDK.
3. Implement configuration, secure storage adapters, credential provider interfaces, trace/idempotency helpers, signing, and redacted diagnostics.
4. Implement device registration, session restore/refresh/logout, and capability profile handling through Mobile Backend Gateway.
5. Implement offline queue, queue persistence, expiry, request-hash checks, safe retries, and flush behavior.
6. Implement sync pull/apply/ack/reset helpers with conflict callbacks and tombstone handling.
7. Implement push token registration, media upload sessions/chunk helpers, wallet/usage readers, and native-app helper modules.
8. Implement Personal AI Assistant, AI Gateway Router, and encrypted Docdex RAG helpers with privacy/context/usage refs.
9. Add fixtures, fake gateway, fake signers, secure-storage test adapters, compatibility tests, and sample native app flows.

The first build may ship one platform/language binding, but all later bindings must pass the same contract and fixture suite.

## Handoff And Downstream Use

Mobile SDK hands typed mobile client primitives to native Overrid apps, approved third-party mobile apps, Mobile Backend Gateway, Wallet/Usage Center, Personal AI Assistant, native app services, AI Gateway Router, Encrypted Docdex RAG Adapter, Overwatch diagnostics, and developer test harnesses.

Apps should use SDK helpers instead of building private request signing, offline, sync, wallet, media, AI, or RAG paths. New mobile helpers require matching updates to the Mobile Backend Gateway SDS, service catalog plan, shared schemas, and contract tests.

## Open Design Questions

Resolved decisions:

- The first shipping mobile binding is a generated Kotlin/Android Mobile SDK for the Phase 12 pilot, built from the shared schema package, foundation SDK request-envelope fixtures, generated error objects, signing/idempotency fixtures, and cross-language contract tests. This is not an Android-only semantic fork: Mobile Backend Gateway remains a Rust/native-Overrid, platform-neutral service boundary, and the Kotlin binding must not define gateway behavior, core runtime behavior, or private service shortcuts. Swift/iOS and any shared core/platform-adapter packaging follow from the same contracts before broad public mobile access.
- The first production release requires four secure-storage provider interfaces: a `credential_provider` that signs with Android Keystore or StrongBox-backed non-exportable keys where available and carries Overkey credential refs; a `secure_ref_store` for device, session, push, permission, usage, and capability refs with OS-backed encryption and clear-all behavior; an `offline_queue_store` with per-entry encryption, request-hash/idempotency evidence, crash-safe writes, expiry, and replay state; and a `diagnostic_redactor` that proves exports cannot include secrets, private payloads, media bytes, decrypted RAG text, or raw messages. Memory-only and fake providers are allowed only in fixtures/local tests. Later Swift/iOS providers must implement the same interfaces through Keychain/Secure Enclave-style storage rather than creating a separate authority model.
- Local retention is classed and bounded by the shortest applicable user clear, logout, device revoke, tenant policy, gateway capability profile, data-class rule, or owner-service expiry. Replay-safe offline commands default to a 24-hour maximum, while resumable media sessions and draft-only workspace, social, directory, or maps proposals may persist up to 7 days if they cannot publish, widen visibility, spend, grant, share, execute AI/RAG, or expand permissions offline. Sync cursor hashes retain until the shorter of gateway expiry or 7 days, with exact location/current-route/search/feed-sensitive cursors capped at 24 hours and all cursors reset on permission, tenant, schema, or compatibility changes. Usage projections become stale after 30 seconds for active wallet/accounting views, offline personal usage snapshots may remain visible for up to 24 hours, and immutable receipt or statement refs follow their export/cache expiry. Redacted local diagnostic rings retain for 7 days by default; user-approved support bundles retain for up to 30 days; crash/security replay refs may retain refs and hashes for up to 90 days without raw private content.
- The first Phase 12 mobile path requires stable SDK helpers for configuration, auth/device/session bootstrap, signed requests, wallet/usage reads, sync cursors, bounded offline replay, permissions, Messaging Center send/read/notification flows, push registration, media upload sessions, Personal AI or AI Gateway Router handoff, encrypted Docdex RAG authorization refs, and redacted diagnostics. Full Workspace, Search, Maps, Directory, Social feed, and Central AI Stewardship convenience modules may start as typed generated service clients or narrow draft/upload helpers; they become first-class mobile helper modules only after their owner-service contracts, Mobile Backend Gateway routes, permissions, and fixtures are stable. The first validation path therefore covers authenticate, sync, store media, send messages, submit AI requests, revoke permissions, and replay offline-safe commands without private low-level service calls.
- Compatibility profiles support the current stable mobile app/SDK major plus one previous stable major for non-security-breaking behavior. Capability profiles must return minimum, recommended, deprecated, and unsupported app/SDK/schema versions plus feature states such as `degraded_compatible`, `read_only`, `sync_paused`, `refresh_required`, and `unsupported`. Additive schema and helper changes can remain within a major version, but breaking command-envelope, signing, idempotency, session, tenant, privacy, data-class, accounting, push-redaction, attestation, AI/RAG context, or policy changes require a new major version, migration guidance, golden fixtures, stable reason codes, and explicit fail-closed behavior. Older clients may keep safe read-only or cached views when policy allows, but mutating, offline, sensitive-content, or security-critical features must pause or fail closed instead of silently downgrading.
