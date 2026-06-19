SDS #67

# mSwarm Runtime Bridge SDS

## Purpose

Connect Overrid resource control to mSwarm local-first runtime concerns such as identity sessions, sync, discovery, collaboration, and cloud coordination hooks.

mSwarm Runtime Bridge is an adapter, not a replacement control plane. It maps mSwarm runtime sessions, sync intents, discovery announcements, collaboration rooms, and cloud coordination hooks into ordinary Overrid identity, tenant, route, storage, policy, audit, and metering contracts. It lets local-first apps use mSwarm runtime capabilities without bypassing Overpass, Overtenant, Overkey, Overgate, Overguard, Overwatch, Overmeter, Overbase, Overstore, or Overvault.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [mswarm_runtime_bridge.md](../../service_catalog/adapters/mswarm_runtime_bridge.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md) |

## Service Family

- Family: Ecosystem adapters
- Owning layer: Product/runtime integration layer for local-first app runtime coordination
- Primary data scope: bridge sessions, identity/tenant mappings, sync manifests, sync cursors, discovery records, collaboration room refs, cloud coordination hooks, runtime capability snapshots, handoff records, usage refs, and audit refs
- First build phase from service plan: [Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md), or earlier only when a native runtime path requires it

## Problem Statement

Overrid needs native and product integrations to work in a local-first runtime world. mSwarm can provide identity-session, sync, discovery, collaboration, and cloud coordination behavior, but those runtime flows become unsafe if they bypass Overrid's identity, tenant, policy, privacy, audit, storage, and accounting rails.

The bridge must make the relationship explicit. mSwarm runtime facts can help apps coordinate across devices or collaborators, but Overrid remains the authority for actor identity, tenant membership, app permissions, data classes, namespace routes, vault access, usage visibility, and audit evidence. The design issue to fix is uncontrolled peer/runtime sync masquerading as trusted platform work.

## Goals

- Map mSwarm runtime actors and sessions to Overrid actor, tenant, device, client, and service-account refs.
- Translate sync intents into bounded Overrid sync manifests with data class, owner, route, retention, conflict, and encryption metadata.
- Publish discovery records and collaboration hooks through normal namespace, route, policy, and audit channels.
- Capture runtime capability snapshots so native apps know which runtime features are available without depending on hidden global state.
- Route cloud coordination hooks through Overgate, Overguard, Overwatch, Overmeter, and storage services where applicable.
- Preserve offline/local-first behavior while making reconciliation, conflict, policy denial, and replay auditable.
- Keep Phase 6 narrow: establish identity/session mapping, sync/discovery bridges, collaboration hooks, capability declarations, and failure/audit integration.

## Non-Goals

- Do not replace Overpass identity, Overtenant membership, Overkey credentials, Universal Namespace Service routes, or Overgate ingress.
- Do not create an uncontrolled peer-to-peer data plane that bypasses policy, storage ownership, vault rules, or audit.
- Do not let mSwarm sessions grant app permissions, vault secrets, workspace access, wallet access, or native-app side effects by themselves.
- Do not persist canonical user documents, messages, files, maps, listings, wallet state, or AI context as bridge-owned data.
- Do not own external payments, ORU balances, Seal Ledger entries, grants, disputes, or provider payouts.
- Do not treat local/offline sync as permission to widen data access during reconciliation.
- Do not add pricing, customer-count, revenue, blockchain, NFT, or per-transaction fee assumptions.

## Primary Actors And Clients

- mSwarm runtime instances running on user devices, local nodes, or app runtime environments.
- Native apps using local-first sync, discovery, collaboration, or cloud coordination hooks.
- Personal AI Assistant and AI Gateway Router when runtime state affects tool routing or device-local execution.
- Overpass, Overtenant, Overkey, Universal Namespace Service, and Overgate resolving identities, tenants, credentials, routes, and ingress.
- Overguard, Workload Classifier, Public Sandbox Profile, and Compliance Boundary Service enforcing policy and data-class limits.
- Overbase, Overstore, Overvault, and Overmesh storing app state/object refs, secret refs, and private connectivity facts.
- Overwatch, Overmeter, ORU Account Service, Seal Ledger, Wallet/Usage Center, and Overclaim receiving audit, usage, receipt, and dispute refs.

## Dependencies

- mSwarm runtime APIs for local session, sync, discovery, collaboration, and cloud coordination primitives.
- [Overpass](../control_plane/overpass.md), [Overtenant](../control_plane/overtenant.md), and [Overkey](../control_plane/overkey.md) for actor, tenant, device, credential, service-account, and delegation refs.
- [Overgate](../control_plane/overgate.md) for API ingress and signed command handling.
- [Universal Namespace Service](../data_storage_namespace/universal_namespace_service.md) and [Overmesh](../execution_scheduling/overmesh.md) for route bindings, private connectivity, and service discovery.
- [Overbase](../data_storage_namespace/overbase.md), [Overstore](../data_storage_namespace/overstore.md), and [Overvault](../data_storage_namespace/overvault.md) for state refs, object refs, secret refs, encrypted records, and vault grants.
- [Overguard](../trust_policy_verification/overguard.md), [Overwatch](../control_plane/overwatch.md), [Overmeter](../execution_scheduling/overmeter.md), and [ORU Account Service](../accounting/oru_account_service.md) for policy, audit, usage, and account visibility.
- Native app SDS files for app-owned data, sync scopes, and side-effect boundaries.

## Owned Responsibilities

mSwarm Runtime Bridge owns:

- Bridge session records connecting mSwarm runtime sessions to Overrid actor, tenant, device, and client refs.
- Runtime capability snapshots for the mSwarm instance, device, or cloud coordination endpoint.
- Sync manifest translation, validation, and handoff records.
- Sync cursors, reconciliation checkpoints, conflict summaries, and policy denial refs for bridge-owned coordination.
- Discovery announcement records with namespace, tenant, data-class, and expiry boundaries.
- Collaboration room/session refs that map runtime collaboration to Overrid app, tenant, permission, and audit context.
- Cloud coordination hook records and delivery attempts.
- Failure records for offline, stale, denied, conflict, dependency, and replay states.
- Usage and audit handoff refs for sync, discovery, collaboration, and cloud coordination work.

The bridge does not own canonical app content, final conflict resolution for app-specific data, policy decisions, identity truth, route truth, vault secrets, accounting truth, or final dispute outcomes.

## Data Model

- `bridge_session`: mSwarm session id, Overrid actor ref, tenant ref, device/client refs, credential refs, app refs, runtime endpoint refs, connection state, capability snapshot refs, and audit refs.
- `runtime_capability_snapshot`: runtime version, supported sync modes, discovery support, collaboration support, cloud coordination hooks, storage adapters, encryption modes, offline support, health refs, and freshness timestamp.
- `sync_manifest`: app ref, data owner refs, collection/object refs, data classes, sync direction, conflict policy, encryption profile, retention class, offline allowance, route refs, policy refs, and state.
- `sync_cursor`: manifest ref, local cursor, remote cursor, checkpoint refs, last successful reconciliation, pending changes count, conflict refs, retry state, and stale markers.
- `discovery_announcement`: announcement id, actor/tenant/app refs, namespace route refs, capability refs, data class, visibility scope, expiry, trust refs, and revoke state.
- `collaboration_session_ref`: app ref, room/session id, participant refs, permission manifest refs, route refs, moderation refs, conflict refs, state, and audit refs.
- `cloud_coordination_hook`: hook type, target service refs, route refs, trigger refs, payload ref or redacted summary, retry policy, delivery attempts, and failure refs.
- `bridge_usage_ref`: sync bytes/items, discovery events, collaboration events, hook deliveries, storage/bandwidth/compute usage, Overmeter refs, and wallet receipt refs.
- `bridge_failure_ref`: failure type, affected manifest/session/hook, dependency refs, retryability, user-visible correction path, and policy/audit refs.

Common envelope fields: `id`, `tenant_id`, `actor_id`, `device_id`, `app_id`, `trace_id`, `idempotency_key`, `schema_version`, `state`, `created_at`, `updated_at`, `policy_refs`, and `audit_refs`.

## API Surface

- `POST /adapters/mswarm/sessions`: registers or refreshes an mSwarm runtime session under an Overrid actor, tenant, device, and app context.
- `GET /adapters/mswarm/sessions/{session_id}`: returns bridge session state, capability snapshot refs, active sync manifests, and recent failure refs.
- `POST /adapters/mswarm/capabilities`: records a runtime capability snapshot for a session or runtime endpoint.
- `POST /adapters/mswarm/sync-manifests/validate`: validates app sync scope, data classes, routes, conflict policy, and permission refs without starting sync.
- `POST /adapters/mswarm/sync-manifests`: creates a bounded sync manifest and returns policy, storage, route, and audit refs.
- `POST /adapters/mswarm/sync-manifests/{manifest_id}/start`: starts or resumes bridge-coordinated sync.
- `POST /adapters/mswarm/sync-manifests/{manifest_id}/pause`: pauses sync without deleting cursors or audit refs.
- `POST /adapters/mswarm/sync-manifests/{manifest_id}/reconcile`: submits reconciliation checkpoint refs and conflict summaries.
- `POST /adapters/mswarm/discovery`: publishes a bounded discovery announcement.
- `POST /adapters/mswarm/discovery/{announcement_id}/revoke`: revokes an active announcement.
- `POST /adapters/mswarm/collaboration-sessions`: creates or attaches a runtime collaboration session to an app/tenant permission context.
- `POST /adapters/mswarm/cloud-hooks`: registers a cloud coordination hook for app-owned or platform-owned callbacks.
- `GET /adapters/mswarm/replay/{record_id}`: reconstructs bridge session, sync, discovery, collaboration, or hook decisions.

Mutating APIs require signed actor or service-account identity, tenant/app context, trace id, idempotency key, policy refs, data-class refs, and route/storage/vault refs where applicable. Stable errors include `session_identity_mismatch`, `tenant_not_authorized`, `capability_snapshot_stale`, `sync_scope_too_broad`, `data_class_denied`, `route_unavailable`, `vault_grant_missing`, `conflict_policy_missing`, `offline_reconcile_denied`, `announcement_visibility_denied`, and `hook_delivery_failed`.

## Event Surface

- `mswarm_bridge.session_registered`: bridge session created or refreshed.
- `mswarm_bridge.capability_snapshot_recorded`: runtime capability facts accepted.
- `mswarm_bridge.sync_manifest_validated`: sync manifest validated without starting side effects.
- `mswarm_bridge.sync_started`: sync manifest started or resumed.
- `mswarm_bridge.sync_checkpointed`: sync cursor/checkpoint recorded.
- `mswarm_bridge.sync_conflict_detected`: conflict summary created.
- `mswarm_bridge.sync_denied`: sync denied by policy, scope, data class, vault, or route checks.
- `mswarm_bridge.discovery_published`: discovery announcement published.
- `mswarm_bridge.discovery_revoked`: discovery announcement revoked or expired.
- `mswarm_bridge.collaboration_attached`: collaboration session attached to app/tenant context.
- `mswarm_bridge.cloud_hook_delivered`: cloud coordination hook delivered.
- `mswarm_bridge.cloud_hook_failed`: hook delivery failed with retry refs.
- `mswarm_bridge.usage_emitted`: usage refs emitted for metering and wallet visibility.

Events include session refs, manifest refs, route refs, app refs, tenant refs, data classes, state, reason codes, policy refs, checkpoint refs, and redacted summaries. Events must not include raw private synced content, vault secrets, raw messages, private documents, or unredacted collaboration payloads.

## Core Workflow

1. Runtime instance registers a bridge session with Overrid identity, tenant, device, client, and app context.
2. Bridge validates credentials through Overpass/Overtenant/Overkey and records capability snapshot refs.
3. App submits a sync, discovery, collaboration, or cloud hook request through the bridge.
4. Bridge validates data scope, route scope, vault/storage refs, conflict policy, retention, and policy requirements.
5. For sync, bridge creates a sync manifest and cursor, then coordinates local/remote checkpoints through app-owned storage APIs.
6. For discovery, bridge publishes bounded announcements through namespace/route and policy controls.
7. For collaboration, bridge maps runtime room/session refs to app permission manifests and Overwatch evidence.
8. For cloud hooks, bridge delivers callbacks through Overgate/Overmesh routes with retry and audit.
9. Bridge emits usage, audit, failure, conflict, and replay refs.

## State Machine

Bridge session lifecycle:

1. `registered`
2. `authenticated`
3. `capability_recorded`
4. `active`
5. `degraded`
6. `offline`
7. `expired`
8. `revoked`

Sync manifest lifecycle:

1. `draft`
2. `validated`
3. `policy_checked`
4. `ready`
5. `syncing`
6. `checkpointed`
7. `conflict_detected`
8. `waiting_for_app_resolution`
9. `paused`
10. `completed`
11. `denied`
12. `failed`
13. `expired`

Discovery/collaboration/hook lifecycle:

1. `submitted`
2. `validated`
3. `published_or_attached`
4. `active`
5. `delivery_pending`
6. `delivered`
7. `revoked`
8. `expired`
9. `failed`

State transitions are append-only or correction-based. Offline runtime state may be reconciled later, but reconciliation cannot erase prior policy denials or widen earlier permissions.

## Policy And Security

- Runtime sessions must map to explicit Overrid actor, tenant, device, and app refs before side effects.
- Sync scopes are deny-by-default when owner refs, data classes, conflict policy, retention class, route refs, or vault grants are missing.
- mSwarm runtime identity facts are advisory until bound through Overpass, Overtenant, and Overkey.
- Discovery announcements require explicit visibility scope and expiry; broad public discovery is not the default.
- Collaboration sessions require app-owned permission manifests and cannot silently add participants or widen data access.
- Offline reconciliation cannot submit changes for data classes or actors that were not authorized when the manifest was created.
- Cloud coordination hooks must carry idempotency keys, route refs, retry limits, and redacted payload refs.
- Secrets remain in Overvault or app-owned encrypted records; the bridge stores only refs and policy decisions.

## Metering And Accounting

- Emit usage refs for sync item counts, object bytes, metadata operations, discovery announcements, collaboration coordination, hook delivery, retries, bandwidth, storage, and compute.
- Link usage to bridge session, app, tenant, actor, device, sync manifest, route, storage/object refs, collaboration session, hook id, and wallet receipt refs.
- The bridge does not maintain balances, billing docs, grants, ledger entries, provider payouts, or external payment refs.
- Native-app economics remain structural and near-cost; the bridge only emits usage dimensions and receipt refs.
- Do not encode hardcoded prices, revenue forecasts, payout rules, or per-transaction fee logic.

## Observability And Operations

- Expose active sessions, stale sessions, capability freshness, sync backlog, conflict counts, denied syncs, discovery announcement counts, collaboration attachment failures, hook delivery latency, retry queues, and usage by app/resource class.
- Alert on repeated identity mismatches, broad sync-scope requests, vault-grant denials, route failures, offline reconciliation spikes, conflict storms, discovery abuse, and hooks missing idempotency.
- Provide redacted replay for session, sync, discovery, collaboration, and hook decisions.
- Support compatibility checks across mSwarm runtime versions and adapter schema versions.
- Support backfills for capability snapshots and sync cursors without fabricating missing policy decisions.

## Failure Modes And Recovery

- Runtime identity mismatch: deny session or mark it `degraded` until credentials are corrected.
- Capability snapshot stale: require refresh before accepting new sync/discovery/collaboration work.
- Sync scope too broad: return field-level denials and allow narrower resubmission.
- Route unavailable: hold sync/hook in retry state or fail with route reason codes.
- Vault grant missing: deny secret-bearing sync and require explicit grant from Overvault.
- Conflict detected: preserve checkpoint refs and require app-owned resolution; bridge does not invent domain-specific merge semantics.
- Offline reconciliation denied: record denied checkpoint refs and keep prior state immutable.
- Hook delivery failure: retry within declared limits, then emit terminal failure with correction path.
- Usage emission failure: keep operation pending for reconciliation or mark usage pending before completion visibility.

## Validation Plan

- mSwarm runtime sessions map cleanly to Overrid actors, tenants, devices, credentials, and apps.
- Sync manifests cannot be created without data-class, owner, route, conflict, and policy refs.
- Sync/discovery/collaboration events are auditable and replayable without raw private content.
- Native apps can use the bridge without bypassing Overguard, Overbase, Overstore, Overvault, Overwatch, or metering.
- Discovery announcements enforce visibility and expiry.
- Offline reconciliation cannot widen permissions or hide policy denials.
- Collaboration sessions require app permission refs and participant checks.
- Cloud hooks are idempotent, retry-bounded, and audit-linked.
- Usage refs flow to Overmeter and wallet/accounting visibility.

## Build Breakdown

1. Define bridge session, runtime capability, sync manifest, sync cursor, discovery, collaboration, and cloud hook schemas.
2. Implement session registration, capability snapshot, sync-manifest validation, and replay endpoints.
3. Add Overpass, Overtenant, Overkey, Overgate, and Overguard checks for all mutating flows.
4. Implement bounded sync manifest creation and cursor/checkpoint recording against fixtures.
5. Add discovery announcements with namespace/route refs, visibility scope, expiry, and revoke behavior.
6. Add collaboration session attachment using app permission manifests and audit refs.
7. Add cloud coordination hook registration, delivery attempts, retry, and failure records.
8. Add usage emission, redacted diagnostics, compatibility checks, and native-app integration tests.

## Handoff And Downstream Use

mSwarm Runtime Bridge supports local-first app runtime and native application development while preserving Overrid governance. It should become the reusable runtime bridge pattern for native apps that need offline sync, discovery, collaboration, or cloud coordination.

Downstream apps must own their domain data and call the bridge through documented APIs/events. If a native app needs a new sync mode or collaboration shape, update that app SDS, the bridge SDS, and the service catalog together.

## Open Design Questions

Resolved decisions:

- Before Overrid credential binding, the bridge may treat only transport-local facts as staging hints: mSwarm session id, runtime endpoint id, advertised runtime version, observed network route, presented device/runtime capability claim, and freshness/challenge metadata. These facts can select a challenge path or capability refresh path, but they cannot authorize side effects. Actor identity, tenant membership, organization role, device ownership, app permission, service-account authority, vault grant, sync scope, discovery visibility, collaboration membership, wallet authority, and namespace/route authority are untrusted until bound through Overpass, Overtenant, Overkey, Overgate, and policy-visible refs.
- The bridge owns generic sync coordination policies only: manifest scope validation, idempotency, cursor/checkpoint comparison, stale-cursor detection, duplicate delivery suppression, revocation and tombstone fences, source-service-authoritative denial, conflict summary emission, and `app_resolution_required` handoff. App-owned logic handles semantic merges and final outcomes for messages, workspace documents, directory listings, map/place corrections, social posts, wallet/accounting projections, moderation, disputes, and any domain-specific edit conflict. The bridge records conflict evidence and prevents permission widening; it does not invent merge semantics for app data.
- Phase 6 discovery is private and bounded: registered product/runtime sessions may discover only tenant/app/device-scoped endpoints, capability snapshots, private route refs, session health, and retryable cloud-hook targets needed by Docdex, Mcoda, Codali, SDK/CLI, or admin visibility. There is no public broadcast, people search, exact location, contact exposure, or behavioral discovery in Phase 6. Phase 12 can add opt-in native-app discovery through app-owned records with signed owner refs, coarse locality/category/capability summaries, explicit visibility scope, expiry, rate limits, revocation, abuse reporting, redaction, and Search/Directory/Maps authority where applicable. Broad public discovery flows through native app, Search Engine, Directory Listings, and Maps policies, not through a bridge-owned public firehose.
- Offline reconciliation windows are classed by source-service risk and may be tightened by the owning service: wallet/accounting is read-only cached projection with stale markers and no offline balance, hold, grant, payment, or receipt mutation; messaging can queue encrypted outbound envelopes, delivery/read receipts, and notification retries for up to 24 hours if grants and first-contact policy still validate on reconnect; workspace can keep drafts, editor sessions, and version proposals for up to 7 days under unchanged share/vault grants, while share, public-link, search, AI, and export side effects wait for online owner-service checks; directory can keep drafts/edit proposals for up to 7 days, while publish, renew, contact handoff, moderation, and dispute effects require current policy checks; maps can keep offline area manifests until the shorter of consent expiry, source freshness, or 30 days, with exact/current route/location refs expiring at 24 hours or sooner; social can keep drafts/uploads for up to 7 days and feed/view caches for up to 24 hours, while publish, comment, reaction, follow, group, moderation, and rights side effects reconcile through current app policy. Expired, revoked, or widened scopes are denied or held for app-owned resolution.
- Capability snapshots degrade per feature, not as a single global on/off flag. Each snapshot carries runtime version, schema version, feature flags, supported sync modes, encryption modes, storage adapters, route classes, offline support, freshness, and compatibility refs. Divergence moves affected features through `fresh`, `degraded_compatible`, `read_only`, `sync_paused`, `refresh_required`, or `unsupported` states. Existing manifests may continue only on the already validated compatible subset; new manifests, discovery announcements, collaboration sessions, and hooks require a fresh compatible snapshot. The bridge must prefer the lowest safe common capability, disable unsupported advanced features, emit compatibility/failure refs, and never widen permissions or data access to preserve cross-device convenience.
