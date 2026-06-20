SDS #82

# Mobile Backend Gateway SDS

## Purpose

Expose stable mobile-friendly APIs over Overrid core services.

Mobile Backend Gateway is the server-side adaptation layer for mobile clients. It translates mobile app constraints such as intermittent connectivity, device-bound sessions, push notification delivery, offline command replay, sync cursors, media upload sessions, and compact response shapes into normal Overrid service calls. It does not replace Overgate, identity, wallet/accounting, native app backends, storage, or AI routing. It front-loads mobile safety and ergonomics while keeping all authority in the existing Overrid rails.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [mobile_backend_gateway.md](../../service_catalog/mobile/mobile_backend_gateway.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md) |
| Sub-build plan | [SUB BUILD PLAN #82 - Mobile Backend Gateway](../../build_plan/sub_build_plan_082_mobile_backend_gateway.md) |

## Service Family

- Family: Mobile service layer
- Owning layer: Mobile API adaptation, device/session coordination, mobile sync, push routing, offline command intake, and mobile-safe response shaping
- Primary data scope: device registrations, mobile session refs, app install refs, capability snapshots, sync cursors, offline command envelopes, push token refs, notification delivery refs, media upload sessions, mobile usage refs, and device-risk refs
- First build phase from service plan: [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md)

## Problem Statement

Mobile apps should be able to use Overrid as a backend/resource plane for identity, wallet, sync, storage, messaging, media, AI, encrypted Docdex RAG, and native service workflows. Directly exposing every low-level service to phones would create fragile clients, duplicated security logic, poor offline behavior, inconsistent metering, and privacy leaks through notifications or logs.

The gateway must give mobile clients a stable, compact surface while preserving the normal Overrid model: Overgate admission, Overpass identity, Overtenant boundaries, Overkey credentials, Overguard policy, Overwatch audit, Overmeter usage, ORU/Seal Ledger accounting, and service-owned domain state.

## Goals

- Provide mobile-shaped APIs for authentication/session bootstrap, app capability discovery, sync, offline command submission, media upload coordination, push notification registration, wallet/usage reads, and AI/native-app handoffs.
- Keep mutating commands signed, tenant-scoped, idempotent, traceable, policy-checked, and routed through Overgate or service-approved ingress.
- Coordinate device registration and revocation without making device state the sole source of identity truth.
- Support intermittent connectivity through explicit sync cursors, bounded offline queues, retry rules, and conflict responses.
- Prevent private data leakage through push payloads, logs, telemetry, cached responses, and mobile diagnostics.
- Emit mobile-specific usage, risk, health, and audit refs for Overwatch, Overmeter, Wallet/Usage Center, and fraud controls.
- Preserve native-service economics as structural near-cost operation without private extraction or ad-trap behavior.

## Non-Goals

- Do not replace Overgate, Overpass, Overtenant, Overkey, Overguard, Overmeter, ORU Account Service, Seal Ledger, Overwatch, or the native app services.
- Do not become a separate app-specific backend that owns messaging, media, workspace, maps, search, wallet, or AI domain records.
- Do not store raw private content, secrets, seed phrases, decrypted Docdex context, long-lived bearer tokens, or full push message bodies.
- Do not trust device attestation alone as identity, authorization, fraud, or abuse proof.
- Do not bypass service-level APIs with direct database/object-store writes.
- Do not add ad-tech tracking, device fingerprinting beyond security necessity, pricing forecasts, or speculative platform economics.

## Primary Actors And Clients

- Mobile SDK and mobile apps for native utilities and approved third-party apps.
- Wallet and Usage Center reading mobile-safe usage, receipt, permission, grant, and dispute refs.
- Personal AI Assistant, AI Gateway Router, and Encrypted Docdex RAG Adapter for mobile AI flows.
- Messaging Center, Social Photo/Video App, Workspace and Office Suite, Search Engine, Maps and Navigation, and Directory Listings for app-specific mobile actions.
- Overgate, Overpass, Overtenant, Overkey, Overguard, Overwatch, Overmeter, ORU Account Service, and Seal Ledger as platform rails.
- Overstore, Overbase, Overvault, Overqueue, Overregistry, Fraud Control Service, and Reputation and Anti-Sybil Service as backend dependencies.
- Operators and support tooling reading redacted mobile diagnostics and replay bundles.

## Dependencies

- [Overgate](../control_plane/overgate.md) for command admission, signatures, idempotency, trace propagation, quota prechecks, and forwarding.
- [Overpass](../control_plane/overpass.md), [Overtenant](../control_plane/overtenant.md), and [Overkey](../control_plane/overkey.md) for identity, tenant membership, credentials, device-bound key refs, rotation, and revocation.
- [Overguard](../trust_policy_verification/overguard.md) and [Workload Classifier](../trust_policy_verification/workload_classifier.md) for mobile data-class, workload, role, and abuse policy decisions.
- [Overwatch](../control_plane/overwatch.md) and [Overmeter](../execution_scheduling/overmeter.md) for audit, traces, health, mobile usage events, and rollups.
- [Overbase](../data_storage_namespace/overbase.md), [Overstore](../data_storage_namespace/overstore.md), and [Overvault](../data_storage_namespace/overvault.md) for state refs, object refs, encrypted refs, secret refs, and media handoff.
- [Overqueue](../control_plane/overqueue.md) for asynchronous command delivery, retry, and offline reconciliation work.
- [AI Gateway Router](../ai_rag_model_routing/ai_gateway_router.md), [Personal AI Assistant](../ai_rag_model_routing/personal_ai_assistant.md), and [Encrypted Docdex RAG Adapter](../ai_rag_model_routing/encrypted_docdex_rag_adapter.md) for mobile AI requests.
- Native app SDS files for wallet, messaging, social media, workspace, directory, search, maps, and stewardship flows.

## Owned Responsibilities

Mobile Backend Gateway owns:

- Mobile API contracts and compatibility profiles for supported app versions.
- Device registration records, device capability summaries, app install refs, push-token refs, and revocation state.
- Mobile session refs and session refresh orchestration through Overkey/Overpass, without becoming identity truth.
- Mobile sync cursors, compact delta responses, conflict hints, and cursor replay evidence.
- Offline command intake, validation, bounded queueing, idempotency correlation, and downstream delivery refs.
- Push notification routing metadata, delivery refs, notification redaction class, and user preference refs.
- Media upload session coordination for mobile clients, including resumable upload refs and Overstore handoff.
- Mobile capability discovery and app configuration refs.
- Mobile-specific usage, telemetry, risk, rate-limit, and replay records.

Domain services remain responsible for their own records and decisions. The gateway adapts and coordinates; it does not own messaging threads, wallet balances, media posts, workspace documents, AI outputs, map data, or directory listings.

## Data Model

- `device_registration`: device id, actor ref, tenant/app refs, platform, app version, SDK version, key refs, push capability, attestation refs, revoked state, last seen, and audit refs.
- `app_install_ref`: app id, install id, app version, capability profile, permission grants, notification preferences, data-class permissions, and revocation refs.
- `mobile_session_ref`: session id, actor/tenant refs, credential refs, device refs, expiration, refresh state, risk class, capability refs, and Overkey/Overpass refs.
- `mobile_capability_profile`: app version, SDK version, feature flags, service capabilities, minimum schema versions, degraded-mode rules, and upgrade guidance.
- `sync_cursor`: actor/app/device refs, service scope, cursor token hash, last acknowledged event or version, data-class scope, conflict marker, and expiration.
- `mobile_delta_bundle`: sync cursor ref, changed record refs, tombstone refs, media refs, conflict hints, retry hints, redaction class, and usage refs.
- `offline_command_envelope`: command id, actor/tenant/app/device refs, command type, target service, idempotency key, request hash, queued time, expiry, payload ref/hash, and delivery state.
- `push_token_ref`: token ref, provider class, platform, app/device refs, permission state, redaction profile, rotation/revocation state, and last delivery state.
- `notification_delivery_ref`: notification id, target refs, category, payload class, queued/delivered/failed state, provider response refs, and audit refs.
- `media_upload_session`: upload id, actor/app/device refs, object intent, chunk/session refs, Overstore target refs, integrity checks, resumable state, and final object refs.
- `mobile_usage_ref`: request class, app/device/session refs, service target, bandwidth/storage/compute/model dimensions, trace id, Overmeter refs, and wallet-visible summary refs.
- `mobile_replay_bundle`: device/session/sync/offline/push/media records, Overgate refs, downstream service refs, audit refs, and redacted diagnostic facts.

Common envelope fields: `id`, `tenant_id`, `actor_id`, `app_id`, `device_id`, `trace_id`, `idempotency_key`, `schema_version`, `state`, `created_at`, `updated_at`, `policy_refs`, `usage_refs`, and `audit_refs`.

## API Surface

- `POST /mobile/devices`: register or refresh a mobile device with app, SDK, platform, key, and capability refs.
- `DELETE /mobile/devices/{device_id}`: revoke a device registration and linked push token refs.
- `POST /mobile/sessions`: create or refresh a mobile session through approved identity and credential refs.
- `POST /mobile/sessions/{session_id}/revoke`: revoke a mobile session and downstream cached capability refs.
- `GET /mobile/capabilities`: return app/service capability profile, required SDK/schema versions, degraded-mode rules, and upgrade guidance.
- `POST /mobile/sync/pull`: read compact deltas for authorized services using a cursor.
- `POST /mobile/sync/push`: submit acknowledged cursors, local mutations, and conflict hints for downstream service processing.
- `POST /mobile/offline-commands`: submit queued commands with idempotency keys, expiry, and payload refs.
- `GET /mobile/offline-commands/{command_id}`: read accepted, queued, delivered, denied, expired, or failed state.
- `POST /mobile/push-tokens`: register, rotate, or refresh a push token ref.
- `DELETE /mobile/push-tokens/{token_ref}`: revoke a push token ref.
- `POST /mobile/media-upload-sessions`: create resumable upload sessions and Overstore handoff refs.
- `POST /mobile/ai-routes`: submit mobile AI requests to AI Gateway Router with privacy, context, budget, and app refs.
- `GET /mobile/usage`: read wallet-visible mobile usage summaries and receipt refs.
- `GET /mobile/replay/{trace_id}`: return redacted mobile replay evidence for authorized support or operator workflows.

Mutating APIs require signed identity, device/session refs, tenant/app context, trace id, idempotency key, app capability version, and policy refs where required. Stable errors include `device_revoked`, `session_expired`, `capability_unsupported`, `cursor_expired`, `offline_command_expired`, `payload_ref_required`, `push_payload_too_sensitive`, `media_upload_conflict`, `mobile_rate_limited`, `attestation_not_sufficient`, and `policy_denied`.

## Event Surface

- `mobile_gateway.device_registered`: device registration created or refreshed.
- `mobile_gateway.device_revoked`: device registration revoked.
- `mobile_gateway.session_created`: mobile session created.
- `mobile_gateway.session_refreshed`: mobile session refreshed.
- `mobile_gateway.session_revoked`: mobile session revoked.
- `mobile_gateway.capability_profile_served`: capability profile served to a mobile client.
- `mobile_gateway.sync_cursor_advanced`: sync cursor advanced or corrected.
- `mobile_gateway.sync_conflict_recorded`: mobile sync conflict surfaced.
- `mobile_gateway.offline_command_accepted`: offline command accepted for downstream processing.
- `mobile_gateway.offline_command_delivered`: downstream service accepted an offline command.
- `mobile_gateway.offline_command_denied`: offline command denied, expired, or conflicted.
- `mobile_gateway.push_token_registered`: push token ref registered or rotated.
- `mobile_gateway.notification_enqueued`: notification ref queued with redaction class.
- `mobile_gateway.notification_delivered`: notification delivery succeeded or failed.
- `mobile_gateway.media_upload_session_created`: resumable media upload session created.
- `mobile_gateway.mobile_usage_emitted`: mobile usage refs emitted to Overmeter.

Events include mobile refs, service target refs, trace ids, policy refs, usage refs, and audit refs. Events must not include raw private messages, media, workspace content, vault material, decrypted RAG context, push provider secrets, or sensitive notification bodies.

## Core Workflow

1. Mobile SDK registers a device and retrieves capability profile for the app, SDK version, and tenant context.
2. The gateway creates or refreshes a mobile session using Overgate, Overpass, Overtenant, and Overkey refs.
3. The client pulls service deltas with a sync cursor and receives compact responses filtered by actor, tenant, app, permissions, and data class.
4. Offline commands are submitted with idempotency keys, expiry, payload refs/hashes, and target service types.
5. The gateway validates and forwards accepted commands through Overgate, Overqueue, or approved service APIs, then records downstream refs.
6. Media upload sessions are coordinated with Overstore refs and integrity checks.
7. Push notifications are routed with redacted payload classes and user preference checks.
8. AI and encrypted Docdex RAG requests are handed to AI Gateway Router and related adapters with mobile privacy and usage refs.
9. Usage, audit, risk, denial, and replay refs are emitted for Wallet/Usage Center, Overwatch, Overmeter, fraud controls, and support diagnostics.

## State Machine

Device lifecycle:

1. `registered`
2. `active`
3. `refresh_required`
4. `restricted`
5. `revoked`
6. `expired`

Mobile session lifecycle:

1. `requested`
2. `identity_checked`
3. `credential_checked`
4. `policy_checked`
5. `active`
6. `refresh_pending`
7. `restricted`
8. `revoked`
9. `expired`

Sync cursor lifecycle:

1. `issued`
2. `pull_requested`
3. `delta_prepared`
4. `acknowledged`
5. `conflict_detected`
6. `advanced`
7. `expired`
8. `reset_required`

Offline command lifecycle:

1. `received`
2. `validated`
3. `accepted`
4. `queued`
5. `delivered`
6. `duplicate_resolved`
7. `denied`
8. `expired`
9. `failed`

Push token lifecycle:

1. `registered`
2. `active`
3. `rotation_required`
4. `revoked`
5. `provider_failed`

## Policy And Security

- Mobile sessions must be short-lived and refreshable; the gateway must not issue unbounded bearer tokens.
- Device registration improves risk decisions but cannot replace actor, tenant, credential, and policy checks.
- Mutating offline commands must carry idempotency keys, request hashes, expiry, target service, and trace ids.
- Push payloads should contain notification refs and minimum visible text; sensitive content remains behind authenticated fetch.
- Cached mobile responses must include data-class and retention hints so SDKs can clear or protect local state.
- App attestation can contribute evidence but cannot become the only trust boundary.
- Telemetry and diagnostics must be redacted by default and avoid raw private content, secrets, and persistent tracking not needed for security.
- Background sync must respect platform permissions, user privacy settings, app capability profile, and tenant policy.
- Operator replay views require signed role authority and must be audience-redacted.

## Metering And Accounting

- Emit usage refs for gateway requests, sync deltas, offline command intake, media upload coordination, push routing, AI handoff, RAG handoff, and diagnostic replay.
- Link usage to actor, tenant, app, device class, request class, target service, trace id, and result state.
- Surface wallet-visible usage summaries and receipt refs without mutating ORU balances or Seal Ledger entries.
- Treat gateway overhead as infrastructure usage and keep mobile native utilities structurally near-cost.
- Avoid direct payment-provider calls, hardcoded charges, or economic projections inside mobile gateway logic.

## Observability And Operations

- Expose health and readiness for Overgate, identity/tenant/key dependencies, policy checks, queue delivery, storage handoff, AI routing, notification providers, and local gateway storage.
- Track device registrations, active sessions, sync cursor age, delta sizes, offline queue depth, command expiry, push delivery outcomes, media session failures, and mobile usage emission latency.
- Alert on high device revocation rates, push payload redaction failures, cursor replay loops, offline command conflicts, mobile rate-limit spikes, unsupported SDK versions, and downstream denial surges.
- Provide redacted trace/replay views joining mobile requests to Overgate, Overqueue, native services, AI Gateway Router, Overwatch, and Overmeter refs.
- Support capability-profile deprecation and staged SDK minimum-version changes without breaking existing sessions unexpectedly.

## Failure Modes And Recovery

- Device lost or revoked: revoke device registration, sessions, push token refs, and cached capability refs.
- Session expired during offline replay: reject or require refresh before downstream submission.
- Sync cursor expired or corrupted: return reset-required state with safe rebootstrap path.
- Offline command duplicate: return prior compatible result when request hash matches; deny conflict when it differs.
- Push provider failure: record provider response, retry within policy, and expose notification state without leaking content.
- Media upload interrupted: resume through upload session and integrity refs or expire with clear retry state.
- AI routing unavailable: return retryable/degraded state and preserve context/privacy refs.
- Overwatch or usage emission degraded: spool bounded mobile audit/usage refs and block terminal success for high-risk mutations until reconciled.
- Downstream native service denies command: preserve denial reason, trace id, audit refs, and correction fields for SDK display.

## Validation Plan

The service implementation plan lists these specific validation requirements:

- Mobile clients can perform common flows without direct low-level service calls.
- Mobile usage is metered and rate limited.
- Abuse signals are visible to Overwatch and fraud controls.

Additional SDS-level validation:

- Contract tests cover device registration, session creation/refresh/revocation, capability discovery, sync pull/push, offline command intake/status, push token registration, media upload sessions, AI route handoff, usage reads, and replay.
- Security tests prove revoked devices, expired sessions, wrong tenants, unsupported apps, invalid signatures, and denied data classes fail before side effects.
- Offline tests prove idempotency replay, conflict denial, expiry, retry, and downstream delivery refs behave deterministically.
- Push tests prove sensitive content is not sent in payloads and notification preference revocation is honored.
- Sync tests cover cursor advancement, cursor expiry, conflict hints, tombstones, and permission-filtered deltas.
- Usage tests prove mobile operations emit Overmeter-compatible usage refs and wallet-visible summaries.
- Replay tests reconstruct mobile request flow without exposing private payloads.

## Build Breakdown

1. Define mobile device, app install, session, capability profile, sync cursor, delta bundle, offline command, push token, notification, media upload, usage, and replay schemas.
2. Implement device registration, session creation/refresh/revocation, and capability discovery APIs.
3. Integrate Overgate, Overpass, Overtenant, Overkey, Overguard, Overwatch, and Overmeter refs.
4. Add sync pull/push with cursor lifecycle, permission filtering, conflict hints, and compact response shape.
5. Add offline command intake, idempotency correlation, expiry, and Overqueue/downstream delivery refs.
6. Add push token registration, notification redaction classes, delivery refs, and preference handling.
7. Add media upload session coordination with Overstore handoff and integrity checks.
8. Add AI Gateway Router, Personal AI Assistant, Encrypted Docdex RAG, wallet/usage, and native app handoffs.
9. Add observability, redacted replay, compatibility profile management, and abuse/fraud signal integration.

The first build should support the smallest native mobile app path: login, capability discovery, usage view, sync, offline replay, push registration, media upload session, and AI route handoff.

## Handoff And Downstream Use

Mobile Backend Gateway hands stable mobile API contracts, device/session refs, sync cursors, offline command refs, push refs, upload refs, usage refs, and replay bundles to Mobile SDK, native apps, Overgate, Overqueue, native service APIs, AI Gateway Router, Overmeter, Wallet/Usage Center, Overwatch, and fraud controls.

Downstream services should treat the gateway as a mobile adapter. They must still enforce their own authority, policy, state transitions, and audit rules.

## Open Design Questions

Resolved decisions:

- The first Phase 12 mobile device pilot should use a generated Kotlin/Android Mobile SDK binding after the shared schema package, foundation SDK, golden request-envelope fixtures, generated error objects, and cross-language contract tests are stable. The gateway itself remains a Rust/native-Overrid service with platform-neutral JSON/JSON Schema contracts and generated fixtures; it must not hand-maintain Android-only public objects or make gateway semantics depend on a specific app platform. Swift/iOS follows from the same contracts before broad public mobile access.
- Offline command support before general mobile expansion is limited to replay-safe, bounded, owner-service-revalidated flows: sync cursor acknowledgements and reset requests, encrypted Messaging Center outbound envelopes plus delivery/read/notification retries, resumable media upload session work, workspace draft/version proposals, social/directory/maps drafts that do not publish or widen visibility, and permission revocation or narrowing requests that become pending until live owner-service confirmation. Wallet/accounting remains read-only cached projection offline; spending, grants, refunds, statement issuance, public publishing, moderation, AI/RAG execution, share links, search visibility, and any permission expansion require live Mobile Backend Gateway, Overgate, Overguard, source-service, and accounting checks before acceptance.
- The first compatibility policy is current stable mobile app/SDK major version plus one previous stable major version for non-security-breaking behavior. Capability profiles must return minimum, recommended, deprecated, and unsupported app/SDK/schema versions plus feature-level states such as `degraded_compatible`, `read_only`, `sync_paused`, `refresh_required`, and `unsupported`. Additive schema changes can stay within a major version; breaking command-envelope, signing, session, tenant, privacy, policy, accounting, push-redaction, attestation, or data-class changes require a new major version, migration guidance, compatibility tests, and stable `unsupported_sdk_version`, `schema_version_unsupported`, or `capability_unsupported` responses. Security-critical breaks may shorten the window, but the gateway must fail closed or degrade explicitly rather than silently accepting unsafe payloads.
- Early push delivery may use APNs for iOS, FCM for Android, Web Push where supported, and a local development provider stub, but only as untrusted delivery adapters behind `push_token_ref` and `notification_delivery_ref` records. The minimum redaction profile is reference-only: notification id/ref, category, urgency, coarse badge/count, delivery route, preference refs, and generic user-visible text such as a new activity indicator. Push payloads must not include raw messages, media captions, wallet balances, usage details, location trails, workspace content, decrypted RAG or AI text, vault refs, secrets, signed URLs, grants, private identifiers beyond target refs, or policy/fraud internals; sensitive content is fetched only after authenticated app open.
- Mobile attestation is represented as advisory device-risk evidence, not as identity, tenant, credential, permission, or policy authority. Device registration may store `attestation_ref` records with provider class, platform claim, nonce/challenge result, freshness, verifier version, confidence, failure reason, privacy class, and audit refs. Overguard, Overwatch, Fraud Control, and Reputation/Anti-Sybil may use those refs to require step-up checks, reduce rate limits, disable high-risk offline features, or mark `attestation_not_sufficient`, but low-risk clients without platform attestation can still use flows allowed by identity, tenant, credential, policy, and data-class checks. The gateway must avoid persistent device fingerprinting beyond security necessity and must make attestation decisions replayable from evidence refs.
