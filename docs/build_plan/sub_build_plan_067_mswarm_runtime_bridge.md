# SUB BUILD PLAN #67 - mSwarm Runtime Bridge

Attached SDS: [SDS #67 - mSwarm Runtime Bridge](../sds/adapters/mswarm_runtime_bridge.md)

## Purpose

This sub-build plan turns SDS #67 into an implementation sequence for mSwarm Runtime Bridge. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

mSwarm Runtime Bridge is the Phase 6 product/runtime adapter that maps mSwarm local-first sessions, sync intents, discovery announcements, collaboration sessions, and cloud coordination hooks into ordinary Overrid identity, tenant, route, policy, storage, audit, usage, and replay contracts. It owns bridge sessions, runtime capability snapshots, sync manifests, sync cursors, discovery announcements, collaboration refs, cloud hook refs, failure refs, usage refs, audit refs, and replay bundles. It does not own Overrid identity truth, tenant membership, policy decisions, namespace truth, vault secrets, canonical native-app content, app-specific conflict resolution, accounting truth, public discovery policy, or final dispute outcomes.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #67: mSwarm Runtime Bridge](../sds/adapters/mswarm_runtime_bridge.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [mSwarm Runtime Bridge service plan](../service_catalog/adapters/mswarm_runtime_bridge.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, signed envelopes, trace ids, idempotency keys, stable errors, local stack, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant scopes, Overkey signing refs, Overgate ingress, Overregistry service/capability records, Overwatch audit refs, and Overqueue command/event primitives. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Workload Classifier facts, Overguard policy checks, Policy Dry-Run previews, Overmesh trusted routing, data-class limits, deny-by-default behavior, and replayable policy reasons. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies Overmeter usage refs, ORU/Seal Ledger/Overbill handoffs, wallet receipt visibility, dispute visibility, and the rule that the bridge emits usage but never maintains balances or prices. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Controls the first build point for bridge sessions, runtime capability snapshots, private product/runtime discovery, sync manifests, collaboration refs, cloud hooks, usage, audit, and product proof. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase state refs, Overstore object/artifact refs, Overvault grant/secret refs, Universal Namespace route refs, Overmesh route resolution, retention classes, and replay substrates. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Later consumes the bridge for app-owned local-first sync, opt-in discovery, collaboration, mobile/offline behavior, and cloud coordination without moving the first bridge build out of Phase 6. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies hardening for runtime identity spoofing, stale/degraded capability snapshots, offline reconciliation abuse, discovery leakage, collaboration participant injection, cloud-hook replay, raw payload leakage, usage gaps, incident response, security review, and compliance controls. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #67 first build work aligned to master Phase 6 with earlier contract/identity/policy/accounting/storage prerequisites, Phase 12 native-app expansion, and Phase 13 governance/security/compliance hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, speculative-market, pricing, revenue, customer-count, broad public-discovery, raw private payload, permission-widening offline sync, or direct app-data ownership drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 6, 8, 12, and 13 | Attach SDS #67, preserve Phase 6 as the first build point, record prerequisites, correct the Phase 12 expansion wording, and freeze the bridge authority boundary. |
| 2 | Master Phases 0, 1, 4, 5, 6, and 8 | Define Rust contracts, canonical schemas, state machines, stable errors, signed refs, hashes, events, and fixtures. |
| 3 | Master Phases 1, 4, 6, and 8 | Implement bridge session registration, Overrid credential binding, capability snapshots, session lifecycle, and degraded/offline behavior. |
| 4 | Master Phases 4, 5, 6, and 8 | Implement sync manifest validation, sync creation, cursors, checkpointing, conflict summaries, policy denials, and offline reconciliation limits. |
| 5 | Master Phases 4, 5, 6, 8, and 12 | Implement bounded discovery announcements, collaboration refs, cloud hook registration/delivery, expiry/revocation, redacted payload refs, and app-owned handoffs. |
| 6 | Master Phase 6, with prerequisites from Phases 0, 1, 4, 5, and 8 | Prove a bounded private product/runtime bridge path for sessions, capability snapshots, sync/discovery/collaboration/cloud-hook records, usage, audit, and replay. |
| 7 | Master Phases 1, 4, 5, 6, 8, and 13 | Implement role-scoped diagnostics, replay, usage reconciliation, SDK/CLI/admin flows, failure visibility, and downstream handoff projections. |
| 8 | Master Phase 12, with prerequisites from Phases 0, 1, 4, 5, 6, and 8 | Prepare native-app and mobile local-first expansion while keeping app-owned data, conflict semantics, permissions, and public discovery outside bridge authority. |
| 9 | Master Phase 13, with prerequisites from Phases 0 through 8 | Harden compatibility, feature degradation, offline abuse, discovery visibility, collaboration membership, hook replay, redaction, incident, retention, security, and compliance behavior. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- mSwarm Runtime Bridge core is a Rust service/module using shared contract crates, Tokio for bounded session/sync/discovery/collaboration/hook/replay workers, and Axum/Tower/Hyper-style HTTP only where a service boundary is needed.
- Bridge sessions, runtime capability snapshots, sync manifests, sync cursors, discovery announcements, collaboration refs, cloud hook refs, failure refs, usage refs, audit records, events, fixtures, replay bundles, and diagnostics use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be generated later only through the shared contract layer.
- Mutating APIs require signed actor/service envelopes, tenant/app/device scope, trace id, idempotency key, capability refs, route refs, policy refs, data-class refs, storage/vault refs where applicable, schema version, stable reason codes, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for session records, capability snapshots, sync manifests, cursor/checkpoint refs, discovery announcements, collaboration refs, hook payload refs or redacted summaries, failure refs, usage refs, replay bundles, audit exports, and deterministic fixtures.
- Storage, queueing where needed, object refs, private records, secret refs, namespace routes, policy, identity, key status, usage, audit, accounting handoffs, and diagnostics must use native Overrid service boundaries such as Overbase, Overstore, Overvault, Universal Namespace Service, Overmesh, Overpass, Overtenant, Overkey, Overgate, Overwatch, Overmeter, Overguard, Workload Classifier, Policy Dry-Run API, SDK, CLI, Admin and Developer UI, Mobile SDK, Mobile Backend Gateway, and native app services.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, speculative assets, pricing, revenue projections, customer-count assumptions, raw private synced content, raw vault secrets, broad public discovery, app-owned canonical content, app-specific merge truth, permission-widening offline reconciliation, or direct mSwarm trust authority the bridge boundary.

## Phase 1: SDS Attachment, Phase 6 Scope, And Bridge Boundary

### Work Items

- **1.1 Attach the build plan to SDS #67.**
  - Design: Link this document from the mSwarm Runtime Bridge SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/adapters/mswarm_runtime_bridge.md`, `docs/service_catalog/adapters/mswarm_runtime_bridge.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #67 returns both the mSwarm Runtime Bridge SDS and this sub-build plan.

- **1.2 Preserve master Phase 6 as the first build point.**
  - Design: Correct the ambiguous "or earlier native runtime" wording into Phase 6 first-build work with later Phase 12 native-app expansion because Phase 12 consumes app-owned runtime behavior after product/runtime bridge contracts exist.
  - Output: Phase-gate note that Phases 0, 1, 4, 5, and 8 supply prerequisites; Phase 6 builds the private product/runtime bridge proof; Phase 12 expands app-owned local-first use; Phase 13 hardens it.
  - Validation: Review proves the plan does not move mSwarm Runtime Bridge into Phase 0, Phase 1, Phase 8 storage, or Phase 12 native apps as its first implementation point, and does not reorder master Phase 0 through Phase 13.

- **1.3 Freeze the bridge ownership boundary.**
  - Design: Record that the bridge owns bridge sessions, capability snapshots, sync manifests, sync cursors, discovery records, collaboration refs, cloud hook refs, failure refs, usage refs, audit refs, and replay bundles.
  - Output: Ownership checklist for architecture, API, implementation, operations, review, and handoff gates.
  - Validation: Review confirms the bridge does not own identity truth, tenant membership, app permissions, route truth, vault secrets, canonical documents/messages/files/maps/listings/wallet state, app-specific merge semantics, ledger truth, or dispute finality.

- **1.4 Carry forward resolved SDS #67 decisions.**
  - Design: Preserve the resolved decisions for advisory-only pre-binding facts, generic sync coordination policy, Phase 6 private discovery, Phase 12 opt-in app discovery, classed offline windows, and per-feature capability degradation.
  - Output: Resolved-decision checklist covering transport-local staging hints, sync policy boundaries, private Phase 6 discovery, app-owned Phase 12 discovery, risk-classed offline windows, compatible-subset continuation, and no permission widening.
  - Validation: Review rejects unbound identity authority, bridge-owned semantic merges, public discovery firehoses, offline wallet/accounting mutation, stale-snapshot feature widening, and convenience-based permission escalation.

- **1.5 Define upstream and downstream boundaries.**
  - Design: Record how mSwarm runtime instances, native apps, Personal AI Assistant, AI Gateway Router, Overpass, Overtenant, Overkey, Overgate, Universal Namespace Service, Overmesh, Overbase, Overstore, Overvault, Overguard, Overwatch, Overmeter, ORU Account Service, Seal Ledger, Wallet/Usage Center, Mobile SDK, and Mobile Backend Gateway interact through refs.
  - Output: Consumer-boundary matrix naming allowed inputs, owned outputs, denied direct authority, usage refs, audit refs, replay requirements, and app-owned handoff points.
  - Validation: Review confirms downstream services keep final identity, tenant, route, policy, storage, vault, metering, accounting, native-app data, public discovery, and conflict-resolution authority.

## Phase 2: Contracts, Schemas, State Machines, And Fixtures

### Work Items

- **2.1 Create the mSwarm Runtime Bridge Rust contract module.**
  - Design: Add contract types for bridge sessions, runtime capability snapshots, sync manifests, sync cursors, discovery announcements, collaboration refs, cloud hooks, usage refs, failure refs, events, stable errors, and replay bundles.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, feature-state enums, sync-direction enums, discovery-visibility enums, hook-state enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from mSwarm runtime internals, app-owned data stores, policy finality, storage internals, vault secret material, and accounting internals.

- **2.2 Define session and capability schemas.**
  - Design: Model mSwarm session id, Overrid actor/tenant/device/client/app refs, credential refs, runtime endpoint refs, connection state, capability snapshot refs, feature flags, supported sync modes, encryption modes, storage adapters, route classes, offline support, freshness, and compatibility refs.
  - Output: JSON Schema files, valid examples, invalid examples, generated validators, capability-freshness fixtures, degraded-feature fixtures, stale-snapshot fixtures, and unsupported-feature fixtures.
  - Validation: Schema tests reject side-effectful sessions without bound Overrid identity, missing tenant/app/device refs, stale capabilities for new work, unsupported feature widening, missing audit refs, and untrusted mSwarm facts used as authority.

- **2.3 Define sync, discovery, collaboration, and hook schemas.**
  - Design: Model sync manifests, cursors, checkpoints, conflict summaries, discovery announcements, collaboration session refs, cloud coordination hooks, redacted payload refs, retry policies, revocation, expiry, and app-owned handoff refs.
  - Output: JSON Schema set, lifecycle state machines, deterministic examples, BLAKE3 hash examples, stable error catalog, redacted payload fixtures, conflict fixtures, offline-window fixtures, and revoke/expiry fixtures.
  - Validation: Tests reject missing owner/data-class/route/policy/conflict refs, broad discovery without scope/expiry, collaboration refs without participant permission refs, hooks without idempotency/retry bounds, and raw private payloads in events.

- **2.4 Define events, stable errors, and replay bundles.**
  - Design: Model session registration, capability snapshot, sync validation/start/checkpoint/conflict/denial, discovery publish/revoke, collaboration attach, hook delivery/failure, usage emission, and replay assembly.
  - Output: Event schemas, stable error enums, role-scoped replay schema, deterministic event-order rules, redacted projection examples, and compatibility replay fixtures.
  - Validation: Tests prove events carry refs and reason codes, omit raw private synced content, raw vault secrets, raw messages, raw documents, exact private location payloads, and unredacted collaboration payloads, and reconstruct decisions from versioned refs.

- **2.5 Create deterministic fixtures and golden traces.**
  - Design: Build fixtures for valid session, identity mismatch, stale capability, sync scope too broad, vault grant missing, route unavailable, conflict detected, offline reconciliation denied, discovery visibility denied, hook delivery failed, usage pending, degraded feature, and replay.
  - Output: Fixture directory, canonical inputs, expected states, expected events, stable errors, hashes, usage refs, audit refs, redacted projections, and replay bundles.
  - Validation: Fixture tests produce stable ids, hashes, lifecycle states, reason codes, usage refs, audit refs, redacted outputs, and replay output across repeated runs.

## Phase 3: Session Registration, Capability Snapshots, And Identity Binding

### Work Items

- **3.1 Implement bridge session registration.**
  - Design: Add `POST /adapters/mswarm/sessions` with signed actor/service envelope, tenant/app/device/client refs, mSwarm session id, runtime endpoint refs, trace id, idempotency key, credential refs, and audit refs.
  - Output: Session registration handler, idempotent refresh behavior, initial lifecycle state, capability-required state, identity mismatch records, and `mswarm_bridge.session_registered` events.
  - Validation: API tests cover valid registration, duplicate idempotency, wrong actor/tenant/device/app binding, missing credential refs, stale sessions, replayed requests, and role-safe error projections.

- **3.2 Bind runtime facts through Overrid authority.**
  - Design: Treat mSwarm pre-binding facts as challenge and capability-refresh hints only, then validate actor, tenant, device, client, app, credential, service-account, and delegation refs through Overpass, Overtenant, Overkey, and Overgate.
  - Output: Identity-binding adapter, challenge path, credential freshness checks, denied-state records, degraded-state records, and audit refs.
  - Validation: Tests prove mSwarm session id, runtime endpoint id, runtime version, network route, device claim, or capability claim cannot authorize sync, discovery, collaboration, hook delivery, vault access, wallet access, namespace authority, or app permissions by itself.

- **3.3 Implement runtime capability snapshots.**
  - Design: Add `POST /adapters/mswarm/capabilities` for session or runtime endpoint capability facts with feature-level states, schema versions, supported sync modes, encryption modes, storage adapters, route classes, offline support, freshness, health refs, and compatibility refs.
  - Output: Capability snapshot handler, freshness model, feature-state projection, compatible-subset rules, stale/unsupported reason codes, and `mswarm_bridge.capability_snapshot_recorded` events.
  - Validation: Tests prove new manifests, discovery announcements, collaboration sessions, and hooks require fresh compatible snapshots, while existing manifests continue only on already validated compatible subsets.

- **3.4 Implement session read and lifecycle behavior.**
  - Design: Add `GET /adapters/mswarm/sessions/{session_id}` with active manifests, capability refs, recent failure refs, health/degraded/offline state, revoked/expired state, and role-scoped diagnostics.
  - Output: Session read handler, lifecycle state machine, degraded/offline/expired/revoked transitions, terminal reason codes, and role-scoped projections.
  - Validation: Tests prove unauthorized callers cannot inspect other tenants, private payloads are absent, revoked sessions block new side effects, expired sessions require refresh, and degraded feature states do not disable unrelated safe features.

- **3.5 Implement early failure and audit foundations.**
  - Design: Record failure refs for identity mismatch, stale capability, dependency failure, replay, route unavailable, policy denial, and usage pending before broader sync/discovery/collaboration work starts.
  - Output: Failure-ref writer, audit event mapping, Overwatch refs, failure read projection, and replay hooks.
  - Validation: Tests prove every registration/capability mutation has an audit ref, failures preserve correction paths, replay reconstructs session decisions, and broad logs/events omit raw private content.

## Phase 4: Sync Manifests, Cursors, Checkpoints, And Offline Reconciliation

### Work Items

- **4.1 Implement side-effect-free sync manifest validation.**
  - Design: Add `POST /adapters/mswarm/sync-manifests/validate` for app ref, owner refs, collection/object refs, data classes, sync direction, route refs, conflict policy, encryption profile, retention class, offline allowance, policy refs, and vault/storage refs without starting sync.
  - Output: Validation handler, validation report refs, missing-prerequisite refs, `sync_scope_too_broad`, `data_class_denied`, `vault_grant_missing`, `conflict_policy_missing`, and `route_unavailable` behavior.
  - Validation: Tests prove validation cannot start sync, write cursors, publish discovery, attach collaboration, deliver hooks, or widen data access.

- **4.2 Implement bounded sync manifest creation.**
  - Design: Add `POST /adapters/mswarm/sync-manifests` to create sync manifests only after identity, tenant, app, data-class, route, storage, vault, retention, conflict, offline, and policy checks pass.
  - Output: Sync manifest writer, lifecycle state, Overbase refs, Overstore/Overvault refs where applicable, policy refs, audit refs, and `mswarm_bridge.sync_manifest_validated` events.
  - Validation: API tests cover valid creation, missing owner refs, broad data classes, missing conflict policy, missing vault grant, revoked app permission, stale capability snapshot, and idempotent duplicate submission.

- **4.3 Implement start, pause, resume, and cursor behavior.**
  - Design: Add start/resume/pause APIs and cursor records that preserve local cursor, remote cursor, checkpoint refs, pending change counts, retry state, stale markers, and current feature capability state.
  - Output: Start/pause handlers, sync cursor writer, checkpoint writer, retry state, stale-cursor detection, duplicate delivery suppression, and `mswarm_bridge.sync_started` / `mswarm_bridge.sync_checkpointed` events.
  - Validation: Tests prove paused sync preserves cursors and audit refs, stale cursors require reconciliation, duplicate checkpoints are suppressed idempotently, and unsupported/degraded features pause only affected manifests.

- **4.4 Implement reconciliation and conflict summaries.**
  - Design: Add `POST /adapters/mswarm/sync-manifests/{manifest_id}/reconcile` for checkpoint refs, conflict summaries, app-resolution-required states, denied checkpoint refs, and app-owned semantic merge handoff.
  - Output: Reconciliation handler, conflict summary records, `waiting_for_app_resolution` state, policy-denied checkpoint records, and `mswarm_bridge.sync_conflict_detected` / `mswarm_bridge.sync_denied` events.
  - Validation: Tests prove the bridge never invents app-specific merge semantics for messages, documents, listings, maps, social posts, wallet/accounting projections, moderation, disputes, or other domain data.

- **4.5 Enforce classed offline reconciliation windows.**
  - Design: Implement wallet/accounting read-only cache rules, messaging 24-hour encrypted outbound envelope rules, workspace/directory/social 7-day draft/proposal rules, maps source/consent/30-day area-manifest rules, exact/current location expiry, and current-policy checks on reconnect.
  - Output: Offline-window policy module, fixtures for each app class, expired/revoked/widened-scope denial records, user-visible correction paths, and audit refs.
  - Validation: Tests prove offline reconciliation cannot mutate wallet balances, holds, grants, payments, or receipts; cannot publish/share/export without current checks; and denies or holds expired, revoked, or widened scopes.

## Phase 5: Discovery, Collaboration, Cloud Hooks, And Redacted Payload Refs

### Work Items

- **5.1 Implement private Phase 6 discovery announcements.**
  - Design: Add `POST /adapters/mswarm/discovery` for tenant/app/device-scoped endpoint refs, capability refs, private route refs, session health, retryable cloud-hook targets, visibility scope, data class, expiry, trust refs, and revoke state.
  - Output: Discovery publish handler, private discovery schema, namespace/route refs, visibility checks, expiry records, and `mswarm_bridge.discovery_published` events.
  - Validation: Tests prove Phase 6 discovery has no public broadcast, people search, exact location exposure, contact exposure, behavioral discovery, or bridge-owned public discovery firehose.

- **5.2 Implement discovery revoke and expiry.**
  - Design: Add `POST /adapters/mswarm/discovery/{announcement_id}/revoke` plus expiry workers that tombstone active announcements, preserve audit, and stop stale route/capability use.
  - Output: Revoke handler, expiry worker, tombstone records, route/namespace cleanup refs, failure refs, and `mswarm_bridge.discovery_revoked` events.
  - Validation: Tests prove revoked or expired announcements cannot be resolved, replay keeps prior visibility decisions, and revocation does not delete audit evidence.

- **5.3 Implement collaboration session refs.**
  - Design: Add `POST /adapters/mswarm/collaboration-sessions` for app ref, room/session id, participant refs, permission manifest refs, route refs, moderation refs, conflict refs, state, and audit refs.
  - Output: Collaboration attach handler, participant/permission validation, moderation handoff refs, conflict handoff refs, and `mswarm_bridge.collaboration_attached` events.
  - Validation: Tests prove collaboration sessions require app-owned permission manifests, cannot silently add participants, cannot widen data access, and cannot store raw collaboration payloads in bridge-owned records.

- **5.4 Implement cloud coordination hooks.**
  - Design: Add `POST /adapters/mswarm/cloud-hooks` for hook type, target service refs, route refs, trigger refs, redacted payload refs, retry limits, delivery attempts, idempotency keys, and failure refs.
  - Output: Hook registration handler, delivery worker, retry state, terminal failure behavior, route/fallback handling, and `mswarm_bridge.cloud_hook_delivered` / `mswarm_bridge.cloud_hook_failed` events.
  - Validation: Tests prove hooks cannot deliver without route refs, policy refs, idempotency keys, retry limits, and redacted payload refs, and cannot call privileged internal APIs directly.

- **5.5 Implement redacted event and payload handling.**
  - Design: Ensure discovery, collaboration, and hook events include refs, reasons, and redacted summaries rather than raw private synced content, vault secrets, raw messages, private documents, exact private locations, or collaboration payloads.
  - Output: Redaction profiles, event projection matrix, payload-ref validators, log quarantine rules, and replay-safe summaries.
  - Validation: Tests prove broad logs/events/metrics/exports omit raw private payloads and only role-authorized replay projections can see permitted refs.

## Phase 6: Bounded Phase 6 Product Runtime Bridge Proof

### Work Items

- **6.1 Implement the first bounded bridge proof template.**
  - Design: Define a Phase 6 proof template using a signed bridge session, fresh capability snapshot, private route refs, one bounded sync manifest, one private discovery announcement, optional collaboration ref, optional cloud hook, usage refs, audit refs, and replay refs.
  - Output: `mswarm_runtime_bridge_v0` proof template, manifest fixture, capability fixture, policy fixture, route fixture, usage fixture, and replay fixture.
  - Validation: End-to-end test proves a private product/runtime integration can use the bridge without direct internal API calls, unbound identity facts, public discovery, raw payload logging, or permission-widening offline behavior.

- **6.2 Prove identity and capability freshness.**
  - Design: Exercise valid binding, identity mismatch, stale capability, degraded compatible feature, read-only feature, sync paused feature, refresh-required feature, and unsupported feature cases.
  - Output: Proof records, feature-state refs, failure refs, correction paths, user/operator diagnostics, and replay bundle.
  - Validation: Tests prove affected features degrade independently, new work requires fresh compatible snapshots, and existing manifests continue only on the already validated compatible subset.

- **6.3 Prove sync and offline reconciliation boundaries.**
  - Design: Run proof cases for valid sync, missing refs, broad scope, conflict detected, app resolution required, offline messaging queue, workspace draft proposal, wallet read-only cache, and denied widened scope.
  - Output: Sync proof records, cursor/checkpoint refs, conflict refs, offline-window decisions, usage refs, audit refs, and replay bundle.
  - Validation: Tests prove sync/discovery/collaboration events are auditable and replayable without raw private content and offline reconciliation cannot widen permissions or hide prior denials.

- **6.4 Prove discovery, collaboration, and hook behavior.**
  - Design: Run proof cases for private discovery publish/revoke, collaboration attachment, denied participant add, hook delivery, hook retry, hook terminal failure, and route unavailable behavior.
  - Output: Discovery/collaboration/hook proof records, revoke/expiry refs, delivery attempt refs, failure refs, usage refs, audit refs, and replay bundle.
  - Validation: Tests prove Phase 6 discovery remains tenant/app/device scoped, collaboration stays app-permission bound, hooks are idempotent/retry-bounded, and cloud coordination remains auditable.

- **6.5 Prove SDK, CLI, and admin experience.**
  - Design: Exercise SDK/CLI/admin flows for register session, record capabilities, validate/create/start/pause/reconcile sync, publish/revoke discovery, attach collaboration, register cloud hook, inspect status, show usage, and render replay summaries.
  - Output: Product proof checklist, Rust SDK examples, TypeScript/web bindings where required for clients, CLI JSON examples, admin projection examples, degraded-state examples, and support-safe diagnostics.
  - Validation: Review confirms product users can inspect bridge state, debug failures, retrieve refs, see usage, and replay decisions without manually calling internal APIs or violating stack guardrails.

## Phase 7: Usage, Audit, Replay, Diagnostics, And Handoffs

### Work Items

- **7.1 Emit bridge usage refs.**
  - Design: Emit usage refs for sync item counts, object bytes, metadata operations, discovery announcements, collaboration coordination, hook delivery, retries, bandwidth, storage, compute, denials, cancellations, and replay reads where material.
  - Output: Overmeter integration, `bridge_usage_ref` writer, `mswarm_bridge.usage_emitted` events, usage reconciliation records, and downstream Wallet/Usage Center, ORU Account Service, Seal Ledger, and Overbill handoff refs.
  - Validation: Tests prove usage links bridge session, app, tenant, actor, device, sync manifest, route, storage/object refs, collaboration session, hook id, retry/failure refs, and wallet receipt refs without maintaining balances in the bridge.

- **7.2 Emit Overwatch audit evidence.**
  - Design: Record signed audit refs for session registration, capability snapshots, sync validation/creation/start/pause/reconcile, discovery publish/revoke, collaboration attach, hook delivery/failure, usage emission, failure states, and replay reads.
  - Output: Audit event mapping, Overwatch refs, redacted projections, immutable decision evidence, and audit completeness checks.
  - Validation: Tests prove every mutating API and terminal state has an audit ref and audit events omit raw private content, vault secrets, raw messages, private documents, exact private locations, and unredacted collaboration payloads.

- **7.3 Implement replay API.**
  - Design: Add `GET /adapters/mswarm/replay/{record_id}` to reconstruct bridge session, capability, sync, discovery, collaboration, hook, usage, failure, compatibility, offline, and audit decisions from versioned refs.
  - Output: Replay assembler, replay bundle schema, role-scoped projections, pagination for linked records, deterministic replay fixtures, and redaction-aware output.
  - Validation: Tests prove replay is deterministic, authorization-aware, complete enough for support/dispute/security review, and does not expose private content outside allowed scopes.

- **7.4 Implement role-scoped diagnostics.**
  - Design: Provide user, app developer, operator, support, security, compliance, and dispute-review projections with appropriate reason codes, redactions, hashes, usage refs, audit refs, and replay links.
  - Output: Projection matrix, admin/support diagnostics, capability mismatch summaries, sync backlog summaries, hook retry summaries, conflict summaries, and alert inputs.
  - Validation: Tests prove no audience gets raw private payloads, vault secrets, other-tenant data, exact private location details, or unredacted collaboration content through the bridge.

- **7.5 Implement downstream handoff projections.**
  - Design: Provide safe refs for native apps, Mobile Backend Gateway, Mobile SDK, Personal AI Assistant, AI Gateway Router, Wallet/Usage Center, Overclaim disputes, Overwatch, Overmeter, and central AI stewardship review where policy allows.
  - Output: Handoff contract, role-scoped projections, denied/degraded refs, usage refs, audit refs, replay refs, and app-owned correction paths.
  - Validation: Tests prove downstream consumers cannot bypass final identity, app permission, policy, context, storage/vault, usage/accounting, replay, native-app data, public discovery, or conflict-resolution boundaries.

## Phase 8: Native App And Mobile Local-First Expansion Readiness

### Work Items

- **8.1 Define app-owned sync profiles for Phase 12.**
  - Design: Define how wallet, messaging, workspace, directory, maps, social, search, personal AI, and mobile surfaces use bridge sync profiles while keeping domain data, domain merges, permissions, side effects, and final publication authority in owning services.
  - Output: App sync profile matrix, data-class refs, allowed offline windows, owner-service checks, conflict handoff refs, and denied-side-effect classes.
  - Validation: Review proves the bridge remains a generic coordination adapter and does not become a canonical store for messages, workspace docs, directory listings, map places/routes, social posts, wallet state, AI context, or mobile app state.

- **8.2 Prepare opt-in native-app discovery.**
  - Design: Define Phase 12 discovery as app-owned records with signed owner refs, coarse locality/category/capability summaries, explicit visibility scope, expiry, rate limits, revocation, abuse reporting, redaction, and Search/Directory/Maps authority where applicable.
  - Output: Native discovery expansion contract, app-owned authority matrix, abuse/reporting refs, redaction profile, and broad-public-discovery denial rules.
  - Validation: Tests prove public people search, exact location discovery, contact exposure, broad behavioral discovery, and native-app ranking/discovery side effects never become bridge-owned defaults.

- **8.3 Prepare mobile offline and push handoffs.**
  - Design: Align bridge sessions and cursors with Mobile Backend Gateway and Mobile SDK session, sync, offline replay, push notification refs, media upload coordination, wallet/usage readers, AI/RAG handoffs, abuse refs, and rate-limit evidence.
  - Output: Mobile handoff contract, offline intent envelope refs, push payload redaction rules, media/upload session refs, rate-limit refs, and replay evidence.
  - Validation: Tests prove mobile clients use normal Overrid identity, credential, policy, storage, usage, and audit rails and cannot use bridge offline queues to bypass current owner-service checks.

- **8.4 Prepare Personal AI and AI Gateway runtime-state handoffs.**
  - Design: Define how runtime state affects Personal AI Assistant tool routing, device-local execution availability, context grants, route decisions, model/resource candidates, and degraded-device behavior without exposing raw runtime payloads.
  - Output: AI runtime-state handoff refs, route hint constraints, capability freshness rules, leakage profile refs, degraded-mode UX examples, and replay refs.
  - Validation: Tests prove runtime facts remain advisory until bound through Overrid refs, AI routing remains in AI Gateway Router, and context authorization remains in owning context/RAG services.

- **8.5 Prepare app-owned conflict and moderation handoffs.**
  - Design: Define collaboration conflict summaries, moderation refs, participant refs, app permission refs, abuse refs, and app-owned resolution endpoints for workspace, messaging, directory, maps, and social flows.
  - Output: Conflict/moderation handoff matrix, app-owned resolution refs, audit refs, usage refs, and user-visible correction paths.
  - Validation: Tests prove the bridge records conflict evidence and prevents permission widening but does not decide semantic merges, moderation outcomes, listing/place correctness, social rights, disputes, or wallet/accounting truth.

## Phase 9: Compatibility, Abuse, Security, Incident, And Compliance Hardening

### Work Items

- **9.1 Harden runtime compatibility and feature degradation.**
  - Design: Threat-model runtime version drift, schema mismatch, feature flag disagreement, encryption-mode incompatibility, storage adapter mismatch, route-class mismatch, stale freshness, incompatible offline support, and cross-device divergence.
  - Output: Compatibility matrix, feature-degradation rules, unsupported-feature behavior, refresh-required behavior, monitoring hooks, and accepted-risk records.
  - Validation: Tests prove the bridge prefers the lowest safe common capability, disables unsupported advanced features, emits compatibility/failure refs, and never widens permissions or data access to preserve convenience.

- **9.2 Harden identity, sync, and offline abuse controls.**
  - Design: Threat-model runtime identity spoofing, session replay, token/key drift, broad sync-scope requests, stale cursor abuse, duplicate delivery abuse, tombstone bypass, offline queue abuse, permission widening, and policy-denial hiding.
  - Output: Threat model records, mitigation checklist, identity replay checks, offline abuse fixtures, denial preservation rules, alert inputs, and incident hooks.
  - Validation: Security tests prove stale or missing identity/capability facts fail closed, offline sync cannot mutate unauthorized data, prior denials cannot be erased, and widened scopes require fresh owner-service authority.

- **9.3 Harden discovery, collaboration, and hook abuse controls.**
  - Design: Threat-model discovery scraping, visibility leaks, exact location exposure, contact exposure, participant injection, collaboration room replay, cloud-hook replay, hook payload leakage, retry amplification, and route fallback abuse.
  - Output: Discovery/collaboration/hook hardening checklist, rate-limit fixtures, revocation tests, participant validation tests, hook replay tests, payload redaction tests, alert inputs, and incident hooks.
  - Validation: Security tests prove broad discovery, unauthorized participant addition, unbounded hook retries, stale route fallback, and raw payload exposure are denied or quarantined with audit evidence.

- **9.4 Harden privacy, logs, retention, and replay.**
  - Design: Enforce redaction, role-scoped diagnostics, retention classes, tombstones where applicable, log quarantine, usage reconciliation, audit completeness, and replay availability without reopening private content.
  - Output: Redaction profiles, retention rules, log quarantine flows, support-safe exports, compliance projections, and replay gap alerts.
  - Validation: Tests prove raw synced content, vault secrets, raw messages, private documents, exact private locations, unredacted collaboration payloads, other-tenant data, and app-owned private state are absent from broad logs/events/metrics/exports.

- **9.5 Harden incident and compliance response.**
  - Design: Add incident playbooks for identity spoofing, capability drift, offline abuse, discovery leakage, collaboration injection, cloud-hook replay, hook payload exposure, usage emission failure, replay gaps, and policy bypass.
  - Output: Incident playbooks, alert thresholds, freeze/quarantine behavior, affected-tenant/user notification refs, correction/retraction workflows, compliance export projections, and remediation tracking refs.
  - Validation: Drills prove incidents preserve evidence, stop unsafe sessions/manifests/hooks, quarantine logs/payload refs, reconcile usage, notify owning services/users where policy requires, and produce compliance-safe reports.

## Phase 10: Validation, Documentation Alignment, Queue State, And Handoff

### Work Items

- **10.1 Validate SDS #67 build-breakdown coverage.**
  - Design: Map each SDS build-breakdown item to sub-build phases covering schemas, session registration, capability snapshots, sync validation, replay, identity/policy checks, sync manifests, cursors, discovery, collaboration, cloud hooks, usage, diagnostics, compatibility checks, and native-app tests.
  - Output: Coverage checklist in review notes and implementation handoff records.
  - Validation: Review proves no SDS #67 build-breakdown item is missing and the plan preserves mSwarm Runtime Bridge as a Phase 6 product/runtime adapter.

- **10.2 Validate structure and work-item quality.**
  - Design: Check title prefix, attached SDS link, phase headings 1 through 10, five work items per phase, Design/Output/Validation bullets, final newline, and no tab/format drift.
  - Output: Focused validation script evidence for this file.
  - Validation: Script passes for `SUB BUILD PLAN #67`, attached SDS link, phase headings 1 through 10, 50 work items, and complete work-item structure.

- **10.3 Validate links and source alignment.**
  - Design: Check local Markdown links across this plan, the SDS, service catalog entry, master plan, crosswalk, Phase 6, Phase 12, Phase 13, progress docs, and queue docs.
  - Output: Link-check evidence and corrected backlinks where needed.
  - Validation: Link checker reports no missing local targets and Docdex search returns aligned SDS/service/sub-build-plan/crosswalk results.

- **10.4 Validate tech-stack guardrails.**
  - Design: Scan the changed docs for accepted Rust-first, canonical JSON/JSON Schema, signed envelope, Ed25519, BLAKE3, native Overrid boundary, no conventional database/queue/object-store/vault/KMS, no Kubernetes-first, no blockchain/NFT, no pricing/revenue/customer-count, no broad public discovery, no raw private payloads, and no permission-widening offline sync.
  - Output: Guardrail scan evidence and any required wording fixes.
  - Validation: Scan output contains only accepted tech-stack language, native Overrid service names, or explicit non-goal/authority-boundary statements.

- **10.5 Update queue, progress, index, and handoff evidence.**
  - Design: Mark `067-build-plan` complete, update progress docs, run targeted Docdex index refresh, run retrieval checks, record the `docdexd run-tests` blocker if still present, and save repo memory.
  - Output: Updated `.codex55_sds_queue/state.json`, `.codex55_sds_queue/progress.md`, `docs/build_plan/progress.md`, Docdex index/search evidence, and implementation handoff note.
  - Validation: Queue JSON validates, next incomplete build-plan task is `068-build-plan`, Docdex search finds the new sub-build plan with SDS/service backlinks, and repo-wide test execution status is recorded.

## Alignment Review

- SDS #67 correctly defines mSwarm Runtime Bridge as an adapter, not a replacement control plane, and separates bridge-owned coordination refs from Overrid identity, policy, route, storage, vault, app-data, usage/accounting, and dispute authority.
- The SDS and service catalog needed a small wording correction: first build is master Phase 6, with later Phase 12 native-app runtime expansion where app-owned local-first flows require it. The earlier "or earlier native runtime" phrasing was ambiguous against the master Phase 0 through Phase 13 order.
- Phase 6 wording needed expansion so product integration names mSwarm Runtime Bridge sessions, capability snapshots, sync manifests, discovery, collaboration, cloud hooks, failure refs, usage, audit, replay, and private product/runtime proof.
- Phase 12 wording needed expansion so native apps and mobile clients consume the bridge for local-first sync, opt-in discovery, collaboration, cloud coordination, and offline handoffs without bypassing normal Overrid rails.
- Phase 13 wording needed expansion so mSwarm Runtime Bridge receives explicit threat-model and security-review coverage for identity spoofing, stale/degraded capabilities, offline reconciliation abuse, discovery leaks, collaboration injection, cloud-hook replay, raw payload leakage, usage gaps, and replay/audit gaps.
- The master Phase 0 through Phase 13 order remains unchanged. Required alignment updates are backlinks/index rows for SDS #67, the Phase 6/12/13 wording expansions, service catalog sub-build-plan linkage, queue/progress evidence, and Docdex index/search refresh.
