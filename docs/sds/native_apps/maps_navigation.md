SDS #70

# Maps and Navigation SDS

## Purpose

Build native maps, local discovery, routes, places, business listings, community map layers, and privacy-preserving location controls.

Maps and Navigation is the native map and route utility for Overrid. It owns place records, map-layer refs, route requests/results, location permission records, offline area manifests, correction reports, directory/search/place handoffs, and usage refs. It is not a surveillance system, advertising map, directory authority, payment authority, social feed, or raw mobility-history store. Location access must be explicit, purpose-bound, revocable, and visible to users.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [maps_navigation.md](../../service_catalog/native_apps/maps_navigation.md) |
| Sub-build plan | [SUB BUILD PLAN #70 - Maps and Navigation](../../build_plan/sub_build_plan_070_maps_navigation.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md) |

## Service Family

- Family: Native applications
- Owning layer: Native public utility application layer for location, place, route, and map-layer experiences
- Primary data scope: place records, route requests/results, map layer refs, location consent records, locality/place corrections, directory/search handoff refs, offline area manifests, cache freshness refs, usage refs, and audit refs
- First build phase from service plan: [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md), after privacy and abuse controls mature

## Problem Statement

Maps are essential public utility infrastructure, but dominant map products often become surveillance and advertising surfaces. Overrid needs maps for local discovery, routes, business listings, community map layers, mobile apps, and native-app handoffs without turning user location into a behavioral profile.

Maps and Navigation must support useful places, routes, local discovery, offline areas, corrections, and integrations while keeping location consent explicit and scoped. The design issue to avoid is allowing maps to become a hidden ranking, tracking, or marketplace authority over the native ecosystem.

## Goals

- Define place, route, map layer, location permission, offline area, correction, directory handoff, search handoff, and usage records.
- Provide privacy-preserving current-location, saved-place, route, and local-discovery APIs.
- Integrate Directory Listings, Search Engine, Messaging Center, Personal AI Assistant, and native mobile clients through refs and explicit permissions.
- Support coarse, exact, ephemeral, and denied location modes with user-visible audit and revocation.
- Support offline or cached map areas where freshness, policy, storage, and privacy constraints are satisfied.
- Keep places and listings interoperable while preserving clear authority boundaries.
- Avoid ad-ranking, surveillance, and private location resale mechanics.

## Non-Goals

- Do not collect continuous location history by default or store raw mobility traces as map-owned canonical data.
- Do not become an advertising marketplace, paid-ranking surface, business-listing authority, directory authority, payment authority, or social feed.
- Do not bypass Overpass, Overtenant, Overkey, Overguard, Overvault, Overwatch, Overmeter, Wallet/Usage Center, Directory Listings, or Search Engine.
- Do not expose exact private location, saved places, route history, or offline areas without explicit permission and audience filtering.
- Do not replace Directory Listings for listings/pages, Search Engine for broad ranking, Messaging Center for contact, or Overclaim for disputes.
- Do not add pricing, customer-count, revenue, blockchain, NFT, or per-transaction fee assumptions.

## Primary Actors And Clients

- End users searching maps, saving places, requesting routes, enabling location, downloading offline areas, and reporting corrections.
- Organizations and local businesses linking verified pages/listings to map places.
- Community groups publishing approved community map layers.
- Personal AI Assistant using user-approved map/location tools.
- Directory Listings and Search Engine sending place/local discovery refs and receiving map-display refs.
- Messaging Center linking place or route refs in conversations without exposing private location beyond consent.
- Mobile SDK and Mobile Backend Gateway providing mobile location/session flows.
- Overpass, Universal Namespace Service, Overbase, Overstore, Overvault, Overguard, Overwatch, Overmeter, Wallet/Usage Center, Overclaim, Fraud Control, and Central AI Stewardship Interface.

## Dependencies

- [Directory Listings](directory_listings.md) for listing/page refs, category/locality refs, organization/business pages, and listing-specific disputes.
- [Search Engine](search_engine.md) for broad discovery and permission-aware result ranking.
- [Messaging Center](messaging_center.md) for contact/share handoffs.
- [Personal AI Assistant](../ai_rag_model_routing/personal_ai_assistant.md) for user-approved map and route tool use.
- [Overpass](../control_plane/overpass.md), [Overtenant](../control_plane/overtenant.md), [Overkey](../control_plane/overkey.md), and [Universal Namespace Service](../data_storage_namespace/universal_namespace_service.md) for identity, tenant, device, namespace, and place/page route refs.
- [Overbase](../data_storage_namespace/overbase.md), [Overstore](../data_storage_namespace/overstore.md), and [Overvault](../data_storage_namespace/overvault.md) for place state, map tiles/object refs, offline/cache refs, and private location grants.
- [Overguard](../trust_policy_verification/overguard.md), [Overwatch](../control_plane/overwatch.md), [Overclaim](../trust_policy_verification/overclaim.md), [Fraud Control Service](../federation_public/fraud_control_service.md), and [Compliance Boundary Service](../governance_ops/compliance_boundary_service.md) for policy, audit, corrections, disputes, fraud, and compliance boundaries.
- [Overmeter](../execution_scheduling/overmeter.md), [ORU Account Service](../accounting/oru_account_service.md), [Seal Ledger](../accounting/seal_ledger.md), and [Wallet and Usage Center](wallet_usage_center.md) for usage visibility.

## Owned Responsibilities

Maps and Navigation owns:

- Place records, place aliases, locality hierarchy refs, and place lifecycle state.
- Route request/result records, route preference refs, route freshness, and route explanation refs.
- Map layer records for base layers, community layers, public-interest layers, directory overlays, and user-authorized private overlays.
- Location consent records for current location, exact/coarse mode, background/offline use, sharing, retention, revocation, and audit.
- Offline area manifests, cache freshness records, and cache invalidation refs.
- Place correction reports, map issue reports, disputed place refs, and handoff records to Overclaim or owning services.
- Directory/search/messaging/assistant handoff records for map display and route sharing.
- Usage refs for map rendering, tile/object access, search/local discovery, route computation, offline cache, corrections, and exports.

Maps and Navigation does not own listing truth, broad search ranking truth, direct messages, payments, identity truth, vault secret material, final disputes, or final accounting truth.

## Data Model

- `place_record`: place id, names/aliases, namespace refs, category refs, geometry refs, locality refs, directory/listing refs, verification refs, source refs, visibility, state, and audit refs.
- `place_geometry_ref`: geometry type, coordinate precision class, source refs, freshness, confidence, privacy class, and correction refs.
- `route_request`: requester refs, origin/destination refs or private location refs, mode, constraints, accessibility needs, privacy mode, route preference refs, and policy refs.
- `route_result`: route request ref, route geometry refs, step summary refs, estimated duration/distance, source/freshness refs, privacy redaction profile, and expiry.
- `map_layer`: layer id, owner refs, layer type, tile/object refs, data classes, visibility scope, freshness, attribution/source refs, policy refs, and state.
- `location_permission`: actor/device/app refs, permission mode, precision class, purpose, retention window, background/offline allowance, sharing allowance, expiry, revocation state, and audit refs.
- `offline_area_manifest`: area refs, layer refs, tile/object refs, freshness window, storage refs, encryption profile, consent refs, invalidation refs, and usage refs.
- `local_discovery_query`: query refs, locality/place refs, directory/search refs, category filters, ranking constraints, privacy mode, and result refs.
- `place_correction_report`: reporter refs, place/layer/route refs, correction type, evidence refs, source confidence, claim refs, moderation refs, and state.
- `map_handoff_ref`: source app/service, target place/route/layer refs, redaction class, consent refs, result refs, and state.
- `map_usage_ref`: rendering, tile/object fetch, geocoding/search, route, offline cache, correction, export, bandwidth, storage, compute, Overmeter refs, and wallet receipt refs.

Common envelope fields: `id`, `tenant_id`, `actor_id`, `device_id`, `app_id`, `trace_id`, `idempotency_key`, `schema_version`, `state`, `created_at`, `updated_at`, `policy_refs`, and `audit_refs`.

## API Surface

- `GET /maps/places/{place_id}`: returns place details according to visibility, role, and precision rules.
- `POST /maps/places`: creates a place candidate or organization-linked place ref.
- `PATCH /maps/places/{place_id}`: submits an allowed place update or correction proposal.
- `GET /maps/places`: lists places by locality, category, visibility, and source refs.
- `POST /maps/routes`: computes or requests a route using current permission, origin/destination refs, and route preferences.
- `GET /maps/routes/{route_id}`: returns route result refs and redacted geometry according to caller permission.
- `POST /maps/location-permissions`: creates or updates location consent for an app/device/purpose.
- `POST /maps/location-permissions/{permission_id}/revoke`: revokes future location access.
- `POST /maps/local-discovery`: runs a local discovery query with directory/search refs and privacy constraints.
- `POST /maps/layers`: creates a map layer manifest.
- `PATCH /maps/layers/{layer_id}`: updates layer visibility, refs, or freshness metadata.
- `POST /maps/offline-areas`: creates an offline area manifest under consent and storage constraints.
- `POST /maps/corrections`: submits a place, route, layer, or locality correction report.
- `POST /maps/handoffs`: records a handoff from Directory, Search, Messaging, Assistant, or another native app.
- `GET /maps/replay/{record_id}`: reconstructs permission, route, layer, place, correction, handoff, and usage decisions.

Mutating APIs require signed actor/service identity, tenant/app/device context, trace id, idempotency key, policy refs, and consent refs where location is involved. Stable errors include `location_permission_required`, `precision_denied`, `background_location_denied`, `place_source_untrusted`, `geometry_too_precise`, `route_unavailable`, `offline_cache_not_allowed`, `layer_visibility_denied`, `directory_ref_invalid`, `search_handoff_denied`, `correction_requires_evidence`, and `location_grant_revoked`.

## Event Surface

- `maps_navigation.place_created`: place candidate or verified place ref created.
- `maps_navigation.place_updated`: place update or correction applied.
- `maps_navigation.place_correction_reported`: correction report recorded.
- `maps_navigation.route_requested`: route request accepted.
- `maps_navigation.route_completed`: route result refs created.
- `maps_navigation.route_denied`: route denied by missing permission or policy.
- `maps_navigation.location_permission_granted`: location permission created or expanded.
- `maps_navigation.location_permission_revoked`: location permission revoked.
- `maps_navigation.local_discovery_run`: local discovery query completed.
- `maps_navigation.layer_created`: map layer created.
- `maps_navigation.layer_updated`: map layer updated.
- `maps_navigation.offline_area_created`: offline area manifest created.
- `maps_navigation.offline_area_invalidated`: offline area stale or revoked.
- `maps_navigation.handoff_recorded`: directory/search/messaging/assistant handoff recorded.
- `maps_navigation.usage_emitted`: usage refs emitted.

Events include place/route/layer/offline refs, actor/device/app refs, permission refs, precision class, policy refs, source refs, handoff refs, correction refs, reason codes, audit refs, and usage refs. Events must not include raw continuous location traces, private saved places, exact private locations, private route geometry, or private contact data outside the allowed audience.

## Core Workflow

1. User opens map or another app requests map display; Maps resolves actor, tenant, device, app, and current location permission.
2. User searches or browses local discovery; Maps requests directory/search refs and applies privacy, ranking, and visibility constraints.
3. User requests route; Maps validates origin/destination precision, location permission, route preferences, and policy.
4. Maps creates route result refs with expiry and redaction profile, then records usage refs.
5. User saves places, downloads offline areas, or enables location modes with explicit consent and revocation paths.
6. Directory Listings, Search Engine, Messaging Center, and Personal AI Assistant create handoff refs instead of copying private map data.
7. Corrections, disputed places, unsafe data, or abuse reports create evidence refs for moderation, Overclaim, Fraud Control, or owning services.
8. Usage and audit records flow to Wallet/Usage Center, Overmeter, Overwatch, and accounting refs.

## State Machine

Place lifecycle:

1. `candidate`
2. `source_checked`
3. `active`
4. `verification_pending`
5. `correction_pending`
6. `disputed`
7. `merged`
8. `deprecated`
9. `removed`

Route lifecycle:

1. `requested`
2. `permission_checked`
3. `computing`
4. `ready`
5. `expired`
6. `denied`
7. `failed`
8. `cancelled`

Location permission lifecycle:

1. `requested`
2. `granted`
3. `active`
4. `narrowed`
5. `revoked`
6. `expired`

Offline area lifecycle:

1. `requested`
2. `policy_checked`
3. `building`
4. `ready`
5. `stale`
6. `invalidated`
7. `deleted`

State transitions are append-only. Location permission expansions create new records or explicit revisions; revocation blocks future access and triggers cache invalidation where required.

## Policy And Security

- Location access is permission-first and purpose-bound. No current, background, exact, saved-place, route, or offline location access is allowed without explicit consent refs.
- Coarse precision is the default. Exact precision requires explicit scope, purpose, and expiry.
- Raw continuous location history is not stored by default. Route and location refs expire or are redacted according to retention policy.
- Directory and search handoffs must carry visibility and redaction refs; maps cannot reveal hidden listing or search fields.
- Offline areas require consent, storage, freshness, encryption, and invalidation refs.
- Community layers require owner identity, source refs, visibility scope, moderation path, and dispute/correction route.
- Place corrections require evidence refs and cannot silently erase prior place state.
- Ranking and discovery must avoid paid ad traps, surveillance targeting, and hidden business suppression.

## Metering And Accounting

- Emit usage refs for map tile/object access, place reads/writes, local discovery, route computation, offline cache build/refresh, layer updates, corrections, exports, bandwidth, storage, and compute.
- Link usage to actor, tenant, device, app, place/route/layer/offline refs, directory/search/messaging handoff refs, permission refs, and wallet receipt refs.
- Maps and Navigation does not maintain balances, payment records, ad bids, listing promotion, invoices, provider payouts, or ledger truth.
- Native-service economics remain structural and near-cost; surplus routing and grants are outside map ranking.
- Do not encode hardcoded prices, revenue forecasts, paid-placement rules, or per-transaction fees.

## Observability And Operations

- Expose place volume, place correction backlog, route request latency, route denial rate, permission grant/revocation rate, precision-denial rate, local discovery latency, offline cache build failures, layer freshness, handoff failures, and usage by resource class.
- Alert on unexpected background location attempts, exact-location access without consent, offline cache stale bursts, route failures, search/directory handoff denials, public layer abuse, correction spam, and missing usage refs.
- Provide user-visible location audit: which apps used location, precision, purpose, time, route/place refs, and how to revoke.
- Provide operator-safe diagnostics without private route geometry or raw location traces.
- Provide replay for route, place, layer, offline area, handoff, permission, correction, and usage decisions.

## Failure Modes And Recovery

- Location permission missing: request scoped consent or continue with manual/place refs if useful.
- Precision denied: degrade to coarse location or deny route/display where exact precision is required.
- Route engine unavailable: return retryable failure, fallback route provider refs if allowed, or manual direction alternatives without expanding privacy.
- Directory/search refs unavailable: run map-only place lookup or show partial results with handoff pending.
- Offline area stale: invalidate or refresh under current consent; do not serve stale sensitive data.
- Community layer abuse: hide or quarantine layer while preserving evidence refs.
- Place correction conflict: open correction/dispute workflow and preserve previous state.
- Usage emission failure: mark operation usage pending and reconcile before final receipt display.

## Validation Plan

- User location preferences and revocations are enforced for current, exact, background, route, saved-place, and offline use.
- Directory and Search results can appear on maps through handoff refs without making maps the listing/search authority.
- Route requests require location or place refs and produce expiring/redacted route result refs.
- Offline/cached data respects consent, freshness, storage, encryption, and invalidation policy.
- Exact private location, saved places, and route geometry are not exposed outside authorized audience and retention rules.
- Place corrections and disputed map facts preserve evidence and correction history.
- Ranking/local discovery tests prove the app does not depend on paid placement or surveillance targeting.
- Usage refs flow to Overmeter and Wallet/Usage Center.
- Replay reconstructs permissions, route/place/layer decisions, handoffs, corrections, and usage refs.

## Build Breakdown

1. Define place, geometry, route, layer, location permission, offline area, local discovery, correction, handoff, and usage schemas.
2. Implement place read/create/update, route request/read, permission grant/revoke, and replay APIs with fixtures.
3. Add Overpass/device/app identity, Overguard policy, Overvault private location grant refs, and Overwatch audit events.
4. Add Directory Listings and Search Engine handoffs for local discovery and map display.
5. Add privacy-preserving route computation and expiring route result refs.
6. Add offline area manifests with Overstore/Overbase refs, freshness, encryption, and invalidation.
7. Add community map layer and correction/dispute flows.
8. Add Wallet/Usage Center visibility, user location audit, mobile SDK integration, and native app UI.

## Handoff And Downstream Use

Maps and Navigation integrates directory, search, messaging, personal AI, local discovery, and mobile clients while keeping location permission and map authority explicit.

Downstream services should use place, route, layer, permission, and handoff APIs rather than copying map-owned private data. Directory Listings remains the listing/page authority; Search Engine remains the broad search authority; Messaging Center remains the communication authority.

## Open Design Questions

Resolved decisions:

- The first build should accept only source-attributed, policy-compatible map inputs: open/public-interest base-map extracts, public civic datasets, verified organization/place submissions, Directory Listings map handoffs, Search source refs, and user or community corrections that preserve provenance. Scraped private mobility traces, ad-tech location feeds, opaque commercial ranking datasets, and proprietary APIs that become the product boundary are not acceptable. Every `place_record`, `place_geometry_ref`, `map_layer`, tile/object ref, and offline manifest must carry source refs, license or attribution refs, import job refs, freshness, confidence, precision class, redaction policy, and Overwatch audit refs; attribution must remain visible in map views, exports, and offline packs.
- Phase 12 required route support is tiered. The v0 required modes are walking and driving, with accessibility constraints supported as route preferences and explanations for pedestrian routes. Cycling can enter after source coverage and safety metadata are good enough for the locality. Transit is initially a source-backed display and handoff mode, not a guaranteed routing mode, unless authoritative schedule/stop/route feeds exist for the region. Delivery and emergency routing are disabled or review-gated in Phase 12 because they create higher safety, dispatch, labor, regulatory, and liability requirements; they require later explicit policy, partner/source authority, replay evidence, and abuse controls before becoming active modes.
- Location precision defaults are coarse or denied unless a narrower explicit grant exists. Directory Listings defaults to public place refs for intentionally published venues and approximate locality for individual or service listings; exact pickup, route, or address refs release only through authorized Messaging or Maps handoffs. Social posts default to no location, with optional public place or coarse locality tags; exact geotags, private-home locations, vulnerable-person contexts, and sensitive-event locations require stricter audience, retention, and moderation controls. Messaging shares default to ephemeral route/place refs or coarse live-location summaries, while exact live location requires recipient-scoped consent, expiry, revocation, and no broad thread indexing. Personal AI tool calls receive no location by default, coarse per-turn locality only when useful, and exact/current/background/offline location only through just-in-time purpose-bound grants that expire and are visible in the user's location audit.
- Offline areas start as bounded public utility caches, not raw mobility-history stores. Phase 12 v0 supports route-corridor packs, neighborhood/small-town packs, and selected public-interest layer packs within the Mobile Backend Gateway and Mobile SDK capability budgets; broad city, regional, country, continuous-background, and sensitive private-layer packs remain disabled until storage, freshness, consent, and abuse evidence prove safe. Public base tiles and public layers may keep short refresh windows, while route-specific, exact-location, private overlay, or search-derived offline refs must expire quickly, refresh under current consent, and be deleted on permission revocation, logout, device revocation, stale freshness, or app uninstall. Seed hardware may retain signed public tile/layer shards longer for repair and replication, but user-specific offline manifests and private location refs remain consent-scoped and revocable.
- Community map layers use risk-tiered moderation instead of blanket pre-approval. Low-risk public-interest layers such as amenities, community resources, event maps, trails, accessibility notes, and neighborhood corrections can publish with source refs, owner identity, rate limits, report links, confidence labels, and reversible state. Higher-risk layers involving private homes, vulnerable groups, safety hazards, emergency/disaster claims, regulated services, disputed boundaries, identity claims, exact sensitive locations, or harassment risk require review, limited visibility, or Overclaim/Fraud Control evidence before broad exposure. Moderation preserves prior state, correction history, reporter and reviewer refs, source confidence, dispute refs, and user-visible reason codes; Maps treats community layers as overlays, not canonical place/listing/search truth.
