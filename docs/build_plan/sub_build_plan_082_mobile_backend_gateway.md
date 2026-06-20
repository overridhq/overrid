# SUB BUILD PLAN #82 - Mobile Backend Gateway

Attached SDS: [SDS #82 - Mobile Backend Gateway](../sds/mobile/mobile_backend_gateway.md)

## Purpose

This sub-build plan turns SDS #82 into an implementation sequence for the Mobile Backend Gateway. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Mobile Backend Gateway is the server-side adaptation layer for mobile clients. It exposes stable mobile-shaped APIs for device registration, mobile sessions, capability discovery, sync cursors, offline command replay, push token refs, media upload sessions, wallet/usage reads, AI handoffs, and redacted replay evidence. It does not replace Overgate, Overpass, Overtenant, Overkey, Overguard, Overwatch, Overmeter, ORU/Seal Ledger, storage services, AI routing, Mobile SDK, or native app domain services.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #82: Mobile Backend Gateway](../sds/mobile/mobile_backend_gateway.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflow, state machines, policy/security, metering, operations, failure modes, validation, build breakdown, downstream handoff, and resolved open-question decisions. |
| [Mobile Backend Gateway plan](../service_catalog/mobile/mobile_backend_gateway.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order and keeps the gateway as a Phase 12 mobile service-layer build with Phase 13 hardening. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies Rust workspace rules, shared contracts, canonical JSON plus JSON Schema, stable errors, signed envelopes, trace ids, idempotency keys, deterministic fixtures, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overgate admission, Overpass identity refs, Overtenant scope, Overkey credential refs, Overwatch audit refs, Overqueue primitives, service accounts, and command-envelope discipline. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy checks, Workload Classifier data classes, policy dry-runs, provider/device risk signals, abuse controls, challenge evidence, and replayable denial reasons. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies usage refs, wallet-visible summaries, ORU/Seal Ledger boundaries, billing/receipt projections, grant refs, dispute refs, and non-mutating accounting handoffs. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Supplies AI Gateway Router, Encrypted Docdex RAG Adapter, SDK/CLI/admin client discipline, product adapter handoffs, and model/resource route refs used by mobile AI flows. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase records, Overstore object/media refs, Overvault private refs, Universal Namespace refs, Overmesh route facts, storage grants, and private-data retention/redaction boundaries. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public-provider and fraud constraints that mobile public/third-party traffic must not bypass. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Controls the first full mobile service-layer build for approved native and third-party mobile apps. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies formal threat/security review, incident/compliance/reporting, privacy, retention, reliability, scale, and public-participation hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #82 aligned to Phase 12 with Phase 13 hardening and earlier phase prerequisites. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first services, Tokio where bounded workers are needed, Axum/Tower/Hyper-style HTTP where service boundaries exist, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, TypeScript only for client surfaces, native Overrid service boundaries, and no conventional database/object-store/vault/queue/mobile-backend SaaS product boundary, blockchain, NFT, hardcoded pricing, revenue, or customer-count assumptions. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 6, 8, 11, 12, and 13 | Attach SDS #82, freeze gateway ownership, preserve Phase 12 as first full build, and map hardening to Phase 13. |
| 2 | Master Phases 0, 1, 4, 5, 8, and 12 | Define Rust contracts, canonical schemas, envelopes, stable errors, events, and deterministic fixtures before handlers. |
| 3 | Master Phases 0, 1, 4, 5, 8, and 12 | Implement device registration, session orchestration, and capability profiles on top of identity, credential, policy, usage, and storage refs. |
| 4 | Master Phases 1, 4, 5, 8, and 12 | Implement sync cursors, compact deltas, permission filtering, conflict hints, and cursor replay evidence. |
| 5 | Master Phases 1, 3, 4, 5, 8, and 12 | Implement bounded offline command intake, idempotent replay, expiry, Overqueue/downstream delivery refs, and owner-service revalidation. |
| 6 | Master Phases 1, 4, 5, 8, 11, and 12 | Implement push token refs, redacted notification classes, delivery refs, preference refs, and abuse controls. |
| 7 | Master Phases 5, 8, 9, and 12 | Implement media upload sessions, resumable refs, integrity checks, Overstore handoff, usage refs, and replay evidence. |
| 8 | Master Phases 5, 6, 8, 11, and 12 | Implement wallet/usage, AI/RAG, native app, Mobile SDK, and mSwarm Runtime Bridge handoffs without owning domain state. |
| 9 | Master Phases 1, 4, 5, 7, 8, 12, and 13 | Implement observability, compatibility management, redacted replay, retention, abuse/fraud signals, and operational recovery. |
| 10 | Master Phase 12 with Phase 13 hardening | Validate mobile flows, cross-service handoffs, privacy/security boundaries, scale behavior, docs links, queue state, and implementation readiness. |

## Tech Stack Guardrails

- Mobile Backend Gateway is a Rust-first Overrid service with typed Rust contracts, Tokio-compatible async work where needed, and Axum/Tower/Hyper-style HTTP boundaries for mobile APIs. TypeScript is allowed only for generated client bindings, SDK surfaces, admin/operator UI, or mobile tooling that calls authorized Overrid APIs.
- Contracts, mobile record payloads, events, stable errors, compatibility profiles, push redaction classes, replay bundles, usage refs, and deterministic fixtures use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf may be generated later only through the shared contract layer.
- Mutating APIs require signed actor or service envelopes, tenant/app/device/session refs, trace id, idempotency key, schema version, capability profile version, policy refs, usage refs, stable errors, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for request hashes, payload refs, replay bundles, media chunks, upload manifests, fixture inputs, and deterministic comparison artifacts. BLAKE3 must not be described as encryption.
- The gateway stores mobile refs, hashes, state, and redacted diagnostics. Raw private messages, media, workspace content, vault material, decrypted RAG context, push provider secrets, long-lived bearer tokens, signed URLs, seed phrases, and broad device fingerprints stay out of gateway logs, push payloads, replay views, and durable records.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Firebase, Supabase, Parse, conventional mobile-backend SaaS, notification SaaS, analytics SaaS, ad-tech tracking, device fingerprinting beyond security necessity, blockchain, NFTs, hardcoded pricing, revenue forecasts, customer-count assumptions, direct accounting mutation, direct storage writes, direct policy finality, or native-app domain ownership the gateway boundary.

## Phase 1: SDS Attachment, Mobile Gateway Charter, And Phase Boundary

### Work Items

- **1.1 Attach the build plan to SDS #82.**
  - Design: Link this plan from the Mobile Backend Gateway SDS, service plan, master build plan, Phase 12 plan, and build-plan crosswalk so builders can navigate from mobile API scope to implementation order.
  - Output: Stable links between this file, `docs/sds/mobile/mobile_backend_gateway.md`, `docs/service_catalog/mobile/mobile_backend_gateway.md`, `docs/build_plan/master_plan.md`, `docs/build_plan/phase_12_native_application_layer.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #82 returns both the Mobile Backend Gateway SDS and this sub-build plan.

- **1.2 Preserve Phase 12 as the first full build point.**
  - Design: Keep the gateway in master Phase 12 because it depends on identity, tenant scope, signing, policy, metering, storage refs, AI routing, native apps, wallet/usage views, and Mobile SDK readiness.
  - Output: Phase-gate note that earlier phases provide prerequisites while full mobile gateway APIs, sync, offline replay, push, media, wallet/usage, and AI handoffs start in Phase 12.
  - Validation: Review confirms the plan does not move full mobile service-layer authority before Phase 12 or formal security/compliance hardening before Phase 13.

- **1.3 Freeze gateway ownership boundaries.**
  - Design: Record that the gateway owns mobile API contracts, compatibility profiles, device registration refs, mobile session refs, sync cursors, offline command envelopes, push token refs, delivery refs, upload sessions, usage refs, and replay bundles.
  - Output: Ownership checklist for records, APIs, events, diagnostics, and handoffs.
  - Validation: Review confirms the gateway does not own identity truth, wallet/accounting state, native app records, message threads, media posts, workspace documents, AI outputs, map data, directory listings, storage truth, or policy finality.

- **1.4 Carry forward resolved SDS #82 decisions.**
  - Design: Preserve resolved decisions for generated Kotlin/Android first binding after contract stability, bounded offline support, compatibility policy, APNs/FCM/Web Push/provider stub redaction boundaries, and advisory attestation evidence.
  - Output: Resolved-decision checklist covering SDK major-version support, replay-safe offline command classes, reference-only push payloads, attestation ref fields, and fail-closed compatibility behavior.
  - Validation: Review rejects Android-only gateway semantics, unsafe offline publication/spend/AI execution, raw push payload content, silent SDK acceptance, and attestation-as-identity assumptions.

- **1.5 Map prerequisites and consumers.**
  - Design: Identify upstream owner services and downstream consumers before contracts so refs, event names, and authority boundaries are consistent.
  - Output: Dependency matrix covering Overgate, Overpass, Overtenant, Overkey, Overguard, Overwatch, Overmeter, Overqueue, Overbase, Overstore, Overvault, native apps, Wallet/Usage Center, AI Gateway Router, Personal AI Assistant, Encrypted Docdex RAG Adapter, Mobile SDK, mSwarm Runtime Bridge, Fraud Control, and support/operator views.
  - Validation: Matrix review confirms every dependency is used through APIs/events/refs and no plan bypasses owner-service APIs with direct database, object-store, vault, or queue writes.

## Phase 2: Mobile Contracts, Schemas, Events, And Fixtures

### Work Items

- **2.1 Define shared mobile envelopes.**
  - Design: Create canonical envelope fields for tenant id, actor id, app id, device id, session ref, trace id, idempotency key, schema version, capability profile version, policy refs, usage refs, audit refs, and created/updated times.
  - Output: Rust contract modules, JSON Schema files, docs-facing examples, and fixture snapshots for mobile request, command, event, and replay envelopes.
  - Validation: Contract tests reject missing scope, missing trace id, missing idempotency key on mutations, incompatible schema versions, and unsupported capability profiles.

- **2.2 Model mobile gateway records.**
  - Design: Define schemas for `device_registration`, `app_install_ref`, `mobile_session_ref`, `mobile_capability_profile`, `sync_cursor`, `mobile_delta_bundle`, `offline_command_envelope`, `push_token_ref`, `notification_delivery_ref`, `media_upload_session`, `mobile_usage_ref`, and `mobile_replay_bundle`.
  - Output: Canonical JSON Schema files, Rust typed contracts, stable field docs, and positive/negative fixtures.
  - Validation: Schema fixtures round-trip through Rust serializers and reject raw push tokens, raw private content, ownerless device refs, unscoped sessions, and replay bundles without redaction class.

- **2.3 Define stable API errors.**
  - Design: Align SDS errors with stable error contracts for revoked devices, expired sessions, unsupported capabilities, expired cursors, expired offline commands, missing payload refs, sensitive push payloads, upload conflicts, mobile rate limits, insufficient attestation, and policy denials.
  - Output: Error schema, Rust enum, docs table, client fixture set, and compatibility tests.
  - Validation: Tests prove errors are stable, audience-safe, traceable, and do not leak private payloads, policy internals, push provider secrets, or fraud heuristics.

- **2.4 Define gateway event surface.**
  - Design: Encode SDS events for device/session/capability/sync/offline/push/media/usage state transitions with trace, policy, usage, and audit refs.
  - Output: Event schemas and fixtures for `mobile_gateway.device_registered`, `device_revoked`, `session_created`, `session_refreshed`, `session_revoked`, `capability_profile_served`, `sync_cursor_advanced`, `sync_conflict_recorded`, `offline_command_accepted`, `offline_command_delivered`, `offline_command_denied`, `push_token_registered`, `notification_enqueued`, `notification_delivered`, `media_upload_session_created`, and `mobile_usage_emitted`.
  - Validation: Event tests confirm no event includes raw messages, media, workspace content, vault material, decrypted RAG context, signed URLs, push provider secrets, or sensitive notification bodies.

- **2.5 Build deterministic contract fixtures.**
  - Design: Create fixture sets for common mobile flows, negative cases, compatibility profiles, offline replay, push redaction, media upload, usage refs, and replay views before handler implementation.
  - Output: Golden request/response/event fixtures, BLAKE3 fixture hashes, local test vectors, and compatibility matrix examples.
  - Validation: Fixture tests verify stable ordering, stable reason codes, stable hashes, no missing final newlines, and no conventional cloud/mobile-backend product assumptions.

## Phase 3: Device Registration, Sessions, And Capability Profiles

### Work Items

- **3.1 Implement device registration APIs.**
  - Design: Build `POST /mobile/devices` and `DELETE /mobile/devices/{device_id}` with actor, tenant, app, platform, SDK, key refs, push capability, attestation refs, revocation state, and audit refs.
  - Output: Rust handlers, contract fixtures, policy checks, lifecycle events, and read projections for registered, active, refresh-required, restricted, revoked, and expired device states.
  - Validation: Integration tests prove idempotent refresh, signed registration, tenant scoping, revocation propagation, audit emission, and rejection of untrusted raw identifiers.

- **3.2 Implement mobile session orchestration.**
  - Design: Build `POST /mobile/sessions` and `POST /mobile/sessions/{session_id}/revoke` through Overgate, Overpass, Overtenant, and Overkey refs without making the gateway identity truth.
  - Output: Session handlers, refresh logic, expiration rules, risk class propagation, and lifecycle events for requested, identity-checked, credential-checked, policy-checked, active, refresh-pending, restricted, revoked, and expired states.
  - Validation: Tests prove expired, revoked, wrong-tenant, wrong-device, missing-credential, and unsupported-app sessions fail before side effects.

- **3.3 Implement capability discovery.**
  - Design: Build `GET /mobile/capabilities` for app version, SDK version, feature flags, service capabilities, minimum/recommended/deprecated/unsupported versions, degraded-mode rules, and upgrade guidance.
  - Output: Capability profile records, compatibility evaluator, feature-state responses, and profile-served events.
  - Validation: Compatibility tests prove current stable major plus one previous stable major works for non-security-breaking behavior, while unsafe breaks return explicit unsupported or degraded states.

- **3.4 Integrate advisory attestation evidence.**
  - Design: Store attestation refs with provider class, platform claim, challenge result, freshness, verifier version, confidence, failure reason, privacy class, and audit refs as risk evidence only.
  - Output: Attestation-ref schema, risk fact handoff to Overguard/Overwatch/Fraud/Reputation, and step-up or restriction hooks.
  - Validation: Tests prove attestation cannot replace actor, tenant, credential, permission, or policy checks and low-risk non-attested clients can still use allowed flows.

- **3.5 Emit session and device usage refs.**
  - Design: Record gateway request class, app/device/session refs, service target, trace id, result state, and Overmeter-compatible usage refs for registration, capability, session, and revocation operations.
  - Output: Mobile usage events and wallet-visible summary refs.
  - Validation: Usage tests prove device/session flows emit usage refs without mutating ORU balances, Seal Ledger entries, or billing records directly.

## Phase 4: Sync Cursors, Delta Bundles, And Conflict Hints

### Work Items

- **4.1 Implement sync cursor issuance and pull.**
  - Design: Build `POST /mobile/sync/pull` with actor/app/device refs, service scope, cursor token hash, last acknowledged version, data-class scope, conflict marker, and expiration.
  - Output: Cursor lifecycle handlers for issued, pull-requested, delta-prepared, acknowledged, conflict-detected, advanced, expired, and reset-required states.
  - Validation: Sync tests cover cursor issuance, pull, expiry, reset-required state, tenant scoping, and permission-filtered deltas.

- **4.2 Implement compact delta bundles.**
  - Design: Produce changed record refs, tombstone refs, media refs, conflict hints, retry hints, redaction class, and usage refs without copying owner-service private records into the gateway.
  - Output: Delta bundle builders, service-scope adapters, compact response fixtures, and tombstone handling.
  - Validation: Tests prove private data remains behind owner-service refs and unauthorized or revoked permissions omit records before snippets, previews, or local cache hints.

- **4.3 Implement sync push acknowledgements.**
  - Design: Build `POST /mobile/sync/push` for acknowledged cursors, reset requests, local mutation proposals, and conflict hints that owner services revalidate.
  - Output: Sync push handler, cursor advancement logic, conflict events, and downstream ref handoffs.
  - Validation: Tests prove ack replay is idempotent, conflicting base versions are surfaced safely, and source services retain final mutation authority.

- **4.4 Add local cache and retention hints.**
  - Design: Include data-class, retention, encryption, invalidation, and local-clearing hints so Mobile SDKs can protect cached state.
  - Output: Cache-hint schema, response headers/fields, SDK fixture examples, and redaction policy mapping.
  - Validation: Tests prove private/sensitive classes get stricter hints and revoked permissions invalidate or fail closed on reconnect.

- **4.5 Emit sync observability and usage.**
  - Design: Track cursor age, delta size, denied counts, conflict counts, reset counts, replay loops, downstream denial refs, and mobile usage refs.
  - Output: Metrics, audit events, usage events, and alert rules for sync behavior.
  - Validation: Observability tests prove stale cursor loops, conflict spikes, and permission-denial spikes are visible without exposing private content.

## Phase 5: Offline Command Intake, Idempotent Replay, And Downstream Delivery

### Work Items

- **5.1 Implement offline command submission.**
  - Design: Build `POST /mobile/offline-commands` with command id, actor/tenant/app/device refs, command type, target service, idempotency key, request hash, expiry, payload ref/hash, and delivery state.
  - Output: Offline command handler, schema validation, accepted/denied events, and durable state transitions.
  - Validation: Tests reject missing idempotency, missing request hash, missing expiry, unsupported command type, missing payload ref, wrong tenant, revoked device, and expired session cases.

- **5.2 Restrict offline command classes.**
  - Design: Limit early offline support to replay-safe, bounded, owner-service-revalidated flows from the SDS: sync acknowledgements/resets, encrypted Messaging envelopes, resumable media work, workspace draft/version proposals, safe social/directory/maps drafts, and permission revocation or narrowing requests.
  - Output: Offline command allowlist, policy fixtures, and denied reason mappings for spend, grants, refunds, statement issuance, public publishing, moderation, AI/RAG execution, share links, search visibility, and permission expansion.
  - Validation: Negative tests prove high-risk or authority-expanding offline commands remain pending/denied until live owner-service checks complete.

- **5.3 Implement downstream delivery and status.**
  - Design: Route accepted commands through Overgate, Overqueue, or approved service APIs and expose `GET /mobile/offline-commands/{command_id}` for accepted, queued, delivered, denied, expired, duplicate-resolved, or failed state.
  - Output: Delivery workers, downstream refs, status handler, retry/expiry rules, and delivery events.
  - Validation: Integration tests prove deterministic delivery refs, bounded retries, expiry behavior, downstream denial preservation, and no direct owner-service state writes.

- **5.4 Implement duplicate and conflict handling.**
  - Design: Resolve duplicate command ids by idempotency key and request hash; return prior compatible result when hashes match and conflict denial when they differ.
  - Output: Duplicate resolver, conflict events, replay evidence, and stable `offline_command_expired` or conflict errors.
  - Validation: Replay tests prove duplicate-safe behavior across reconnect storms and reject replay attempts with mismatched request hashes.

- **5.5 Emit offline replay audit and usage refs.**
  - Design: Record mobile usage, Overwatch refs, downstream refs, denied reasons, queue depth, expiry age, and redacted replay facts for offline commands.
  - Output: Offline command audit events, usage refs, replay bundle links, and alertable metrics.
  - Validation: Tests reconstruct offline command flow without raw private payloads and prove high-risk mutations cannot report terminal success when audit/usage refs cannot be spooled or reconciled.

## Phase 6: Push Tokens, Notification Redaction, And Preference Routing

### Work Items

- **6.1 Implement push token APIs.**
  - Design: Build `POST /mobile/push-tokens` and `DELETE /mobile/push-tokens/{token_ref}` with token refs, provider class, platform, app/device refs, permission state, redaction profile, rotation/revocation state, and last delivery state.
  - Output: Push token handlers, lifecycle events, rotation logic, and provider-adapter boundary.
  - Validation: Tests prove raw provider tokens stay out of broad records, revoked device sessions revoke linked token refs, and provider errors are audience-redacted.

- **6.2 Implement notification delivery refs.**
  - Design: Create notification id, target refs, category, payload class, queued/delivered/failed state, provider response refs, and audit refs without storing raw message bodies.
  - Output: Notification delivery schema, enqueue/delivery events, retry policy, and provider response refs.
  - Validation: Tests prove delivery refs do not include raw messages, wallet balances, usage details, location trails, workspace content, decrypted RAG/AI text, vault refs, secrets, signed URLs, grants, private identifiers beyond target refs, or policy/fraud internals.

- **6.3 Enforce reference-only push payloads.**
  - Design: Restrict push payloads to notification id/ref, category, urgency, coarse badge/count, delivery route, preference refs, and generic user-visible text.
  - Output: Redaction-profile evaluator, push payload builder, stable `push_payload_too_sensitive` error, and fixture set.
  - Validation: Push tests prove sensitive content is fetched only after authenticated app open and policy-preference revocation is honored before enqueue.

- **6.4 Integrate notification preferences and abuse controls.**
  - Design: Route through app install refs, user notification preferences, quiet hours, data-class permissions, rate limits, device risk, fraud signals, and tenant policy.
  - Output: Preference resolver, rate-limit hooks, abuse events, and Overguard/Overwatch handoffs.
  - Validation: Tests prove notification flooding, unsupported categories, revoked preferences, high-risk device states, and denied data classes fail closed.

- **6.5 Emit push operations metrics.**
  - Design: Track token registrations, rotations, revocations, delivery attempts, provider failures, redaction denials, preference denials, retry counts, and mobile usage refs.
  - Output: Metrics, health checks, audit refs, usage refs, and replay snippets.
  - Validation: Observability tests prove push provider failures and redaction failures are alertable without exposing provider secrets or private content.

## Phase 7: Media Upload Sessions And Overstore Handoff

### Work Items

- **7.1 Implement media upload session creation.**
  - Design: Build `POST /mobile/media-upload-sessions` with actor/app/device refs, object intent, chunk/session refs, Overstore target refs, integrity checks, resumable state, and final object refs.
  - Output: Upload-session handler, schema fixtures, lifecycle events, and upload-state projections.
  - Validation: Tests reject missing object intent, wrong tenant, revoked devices, unsupported media classes, missing integrity refs, and direct object-store writes.

- **7.2 Implement resumable upload coordination.**
  - Design: Coordinate chunk refs, session expiry, retry hints, partial uploads, finalization, and cancellation without owning media posts or object truth.
  - Output: Resumable session state machine, chunk manifest refs, BLAKE3/content hash checks, retry/cancel handlers, and stable conflict errors.
  - Validation: Media tests prove interrupted uploads resume or expire deterministically and conflicting finalization returns `media_upload_conflict`.

- **7.3 Integrate Overstore and native app ownership.**
  - Design: Hand final object refs to Overstore and owning native services such as Social, Messaging, Workspace, Directory, Maps, or Wallet/Usage views through approved APIs/events.
  - Output: Handoff contracts, owner-service refs, upload completion events, and usage refs.
  - Validation: Integration tests prove final media visibility, publishing, moderation, rights, and retention are owner-service decisions, not gateway decisions.

- **7.4 Add upload policy and privacy controls.**
  - Design: Apply data-class permissions, user privacy settings, app capability profiles, size/type limits, safety scan refs where applicable, and vault/private refs for protected media.
  - Output: Policy evaluator, stable denial reasons, redaction class mapping, and safety-scan handoff refs.
  - Validation: Tests prove sensitive/private uploads cannot widen visibility, bypass moderation, or place raw private material in gateway diagnostics.

- **7.5 Emit upload observability and usage refs.**
  - Design: Track upload session counts, chunk failures, expiry, integrity failures, finalization latency, Overstore handoff refs, usage dimensions, and replay evidence.
  - Output: Metrics, usage refs, audit events, replay bundle refs, and health checks.
  - Validation: Tests prove upload diagnostics reconstruct request flow through refs and never include raw media bytes in replay or logs.

## Phase 8: Wallet, AI, Native App, Mobile SDK, And Runtime Bridge Handoffs

### Work Items

- **8.1 Implement wallet and usage reads.**
  - Design: Build `GET /mobile/usage` as a mobile-safe read projection over Wallet/Usage Center, Overmeter, ORU Account Service, Seal Ledger, Overbill, and Overgrant refs.
  - Output: Usage summary handler, receipt refs, permission inventory refs, read-only offline projection fixtures, and usage events.
  - Validation: Tests prove wallet/accounting remains read-only through the gateway and spend, grants, refunds, statements, disputes, and accounting-changing actions require live owner-service checks.

- **8.2 Implement mobile AI route handoff.**
  - Design: Build `POST /mobile/ai-routes` to submit mobile AI requests to AI Gateway Router with privacy, context, budget, app refs, leakage-profile visibility, route-bound bundle expiry, and usage refs.
  - Output: AI route handler, context/ref redaction checks, fallback/degraded states, and replay events.
  - Validation: Tests prove AI/RAG execution is never accepted offline, decrypted context is not stored, and route decisions remain AI Gateway Router/owner-service authority.

- **8.3 Implement native app action handoffs.**
  - Design: Define gateway handoffs for Messaging, Social, Workspace, Search, Maps, Directory, Wallet, and Central AI surfaces using app-owned refs and owner-service side-effect confirmation.
  - Output: Handoff contract set, service target registry, app capability mapping, and downstream denial propagation.
  - Validation: Tests prove the gateway adapts mobile requests without owning message threads, posts, documents, search indexes, map truth, directory listings, wallet state, or stewardship decisions.

- **8.4 Align with Mobile SDK contract generation.**
  - Design: Provide stable JSON/JSON Schema contracts, generated fixture bundles, compatibility profiles, and stable errors for Mobile SDK bindings, with Kotlin/Android first after shared contract stability and Swift/iOS following from the same contracts.
  - Output: SDK contract export, generated fixture packs, compatibility matrix, and SDK conformance tests.
  - Validation: Tests prove gateway semantics are platform-neutral and do not hand-maintain Android-only public objects or make authority depend on app platform.

- **8.5 Integrate mSwarm Runtime Bridge where allowed.**
  - Design: Support local-first sync, opt-in discovery, collaboration refs, cloud coordination hooks, and offline replay only where app-owned policies permit and live owner-service revalidation controls side effects.
  - Output: Runtime bridge handoff contracts, feature degradation states, sync-scope refs, collaboration refs, and replay hooks.
  - Validation: Tests prove runtime bridge handoffs do not widen discovery, permissions, collaboration participants, cloud hooks, or offline effects beyond gateway and owner-service policy.

## Phase 9: Observability, Redacted Replay, Compatibility Operations, And Abuse Signals

### Work Items

- **9.1 Implement health and readiness checks.**
  - Design: Expose readiness for Overgate, identity/tenant/key dependencies, policy checks, queue delivery, storage handoff, AI routing, notification providers, local gateway storage, and usage/audit emission.
  - Output: Health endpoints, dependency probes, degraded states, and operator view projections.
  - Validation: Tests prove dependency outages return explicit degraded states and high-risk mutations fail closed when critical audit/usage dependencies cannot reconcile.

- **9.2 Implement redacted replay bundles.**
  - Design: Build `GET /mobile/replay/{trace_id}` for authorized support/operator workflows with audience-redacted refs spanning mobile, Overgate, Overqueue, native services, AI Gateway Router, Overwatch, Overmeter, and fraud controls.
  - Output: Replay bundle schema, authorized query handler, redaction profiles, and reportable evidence refs.
  - Validation: Replay tests reconstruct device/session/sync/offline/push/media flows without raw private payloads, vault material, decrypted context, push provider secrets, or sensitive notification bodies.

- **9.3 Implement compatibility-profile lifecycle.**
  - Design: Manage minimum, recommended, deprecated, unsupported, degraded-compatible, read-only, sync-paused, refresh-required, and unsupported feature states for app/SDK/schema versions.
  - Output: Compatibility profile management APIs/events, staged deprecation records, upgrade guidance, and migration fixtures.
  - Validation: Compatibility tests prove additive changes stay within a major version, breaking safety changes require new major guidance, and security-critical breaks fail closed.

- **9.4 Emit abuse, fraud, and risk signals.**
  - Design: Forward mobile rate-limit spikes, unsupported SDK versions, high revocation rates, sync loops, offline conflicts, redaction failures, push abuse, upload integrity failures, and attestation-not-sufficient refs to Overwatch, Fraud Control, Reputation/Anti-Sybil, and Overguard.
  - Output: Risk signal events, alert rules, dashboard metrics, and policy input facts.
  - Validation: Tests prove signals are evidence refs and recommendations, not direct fraud finality, policy finality, or public-report publication.

- **9.5 Implement retention and diagnostic cleanup.**
  - Design: Apply data-class retention, cache-clearing hints, replay retention, push provider response retention, device-risk evidence retention, and cleanup jobs.
  - Output: Retention policy configs, cleanup jobs, tombstone refs, metrics, and operator evidence.
  - Validation: Retention tests prove private data is not retained beyond policy, public/support views remain redacted, and cleanup does not erase required audit/evidence refs.

## Phase 10: End-To-End Validation, Phase 13 Hardening, And Documentation Handoff

### Work Items

- **10.1 Validate common mobile flows.**
  - Design: Run end-to-end tests for login, device registration, capability discovery, usage view, sync pull/push, offline replay, push registration, media upload session, AI route handoff, and replay.
  - Output: Integration suite, golden traces, fixture snapshots, and acceptance evidence.
  - Validation: Tests prove common mobile clients can use Overrid without direct low-level service calls.

- **10.2 Validate security and privacy boundaries.**
  - Design: Cover revoked devices, expired sessions, wrong tenants, unsupported apps, invalid signatures, denied data classes, sensitive push payloads, raw private replay attempts, attestation limits, and owner-service denial.
  - Output: Security test suite, negative fixtures, stable reason-code evidence, and redaction reports.
  - Validation: Tests prove failures occur before side effects and private data does not leak through push payloads, logs, telemetry, cached responses, replay bundles, or diagnostics.

- **10.3 Validate metering and accounting handoffs.**
  - Design: Prove gateway requests, sync deltas, offline command intake, media coordination, push routing, AI/RAG handoff, usage reads, and replay emit Overmeter-compatible usage refs.
  - Output: Usage test suite, wallet-visible summary fixtures, and accounting boundary checklist.
  - Validation: Tests prove usage is metered and rate-limited while the gateway does not mutate ORU balances, Seal Ledger entries, invoices, payouts, grants, or statements directly.

- **10.4 Validate operations, scale, and Phase 13 hooks.**
  - Design: Run drills for reconnect storms, replay bursts, notification provider outage, queue backlog, sync loop, upload backlog, AI route outage, usage/audit spool degradation, redaction failure, unsupported SDK rollout, and mobile abuse spike.
  - Output: Drill records, alert evidence, recovery notes, Phase 13 threat/security review refs, incident/compliance/reporting hooks, and open risk list.
  - Validation: Drills prove gateway behavior is replayable, bounded, fail-closed where required, and ready for formal Phase 13 governance/security/compliance hardening.

- **10.5 Complete documentation and handoff readiness.**
  - Design: Update SDS, service catalog, master plan, crosswalk, Phase 12, progress, queue state, and Docdex index/search evidence so implementation teams have one aligned source of truth.
  - Output: Linked docs, queue/progress metadata, validation evidence, implementation handoff notes, and Docdex retrieval evidence.
  - Validation: Structure checks pass for title prefix, attached SDS, phases 1 through 10, 50 work items, Design/Output/Validation fields, local links, stack guardrails, queue state, Docdex search, pre-commit hooks, and documented test-runner blockers if repo tests remain unavailable.
