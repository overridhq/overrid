# Maps And Navigation

## Slug

`maps-navigation`

## Title

Maps And Navigation

## Navigation Group

Daily Apps

## Description

Maps And Navigation is the Overdesk surface for places, local discovery, route planning, map layers, location permissions, and offline map packs. It should make maps useful without turning location into a tracking product, ad-ranking surface, or hidden marketplace authority.

## Primary Users

- Regular users
- Local service providers
- Organizations
- Institutions
- Community map maintainers
- Public-interest project users
- App owners linking places or routes

## Primary User Goals

- Find places, listings, organizations, and public-interest map layers.
- Plan routes with clear location and privacy controls.
- Choose coarse, exact, or denied location per purpose.
- Open listings, messages, search results, and social posts from map context.
- Save places or offline packs where policy allows.
- Report map, place, route, or listing issues.

## Entry Points

- Daily Apps navigation.
- Home Dashboard native app shortcut.
- Global Search result.
- Directory Listings location handoff.
- Social Photo/Video place handoff.
- Messaging Center shared place or route ref.
- Personal AI Assistant map tool result.
- Address bar command: `/maps`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active account/scope.
- Active locality or map region.
- Location permission state.
- Map freshness state.
- Primary action: Search Place Or Route.
- Secondary actions: Offline Packs, Location Settings, Saved Places.

### Search And Route Bar

Content:

- Place or address search input.
- From and To route inputs.
- Current-location selector.
- Route mode selector.
- Accessibility preferences.
- Avoid tolls, private roads, or restricted routes where supported.
- Search filters for listings, organizations, map layers, and public-interest projects.

Links and handoffs:

- Global Search.
- Directory Listings.
- Personal AI Assistant.

### Map Canvas

Content:

- Base map layer.
- Place pins.
- Directory listing pins.
- Organization pins.
- Community and public-interest layer overlays.
- Route path and route alternatives.
- Current location marker where allowed.
- Selected item highlight.
- Cached/offline/stale markers.

### Results And Layers Panel

Content:

- Search results.
- Nearby listings.
- Layer selector.
- Result source refs.
- Distance or locality.
- Trust/verification marker.
- Freshness marker.
- Visibility or permission marker.
- Sort and filter controls.

Links and handoffs:

- Directory Listings.
- Native App Catalog.
- Central AI Stewardship for public-interest project layers.

### Place Detail Panel

Content:

- Place name.
- Category.
- Locality and address display according to precision rules.
- Source refs and confidence.
- Directory/listing refs.
- Organization or app refs.
- Hours or contact refs where allowed.
- Saved state.
- Correction/report state.

Links and handoffs:

- Directory Listings.
- Messaging Center.
- Overrid Browser.
- Disputes And Appeals.

### Route Planner Panel

Content:

- Route summary.
- Steps list.
- Estimated duration and distance.
- Route mode.
- Accessibility notes.
- Route freshness.
- Privacy and retention note.
- Share route action.
- Export/open handoff where allowed.

### Location Permission Panel

Content:

- Current location mode: denied, coarse, exact, background, or offline.
- Purpose.
- Expiry.
- Device/app requesting access.
- Precision class.
- Audit refs.
- Revoke, narrow, and expire controls.

Links and handoffs:

- Privacy And Permissions Center.
- Activity And Receipts Timeline.

### Offline Packs Panel

Content:

- Saved offline areas.
- Route-corridor packs.
- Locality packs.
- Public-interest packs.
- Storage usage.
- Freshness and invalidation state.
- Download, update, pause, delete, and revoke controls.

### Corrections And Reports Panel

Content:

- Report place issue.
- Report route issue.
- Report unsafe or stale layer.
- Attach evidence refs.
- Correction status.
- Appeal or dispute refs where available.

## Primary Actions

- Search place.
- Plan route.
- Open place.
- Open listing.
- Grant or revoke location.
- Save place.
- Download offline pack.
- Report correction.

## Secondary Actions

- Share route.
- Copy place ref.
- Open in Directory.
- Ask AI about route.
- Filter map layer.
- Clear local map cache.
- Open usage receipt.

## States

- Empty map.
- Loading.
- Live.
- Search no results.
- Location denied.
- Coarse location only.
- Exact location pending.
- Route unavailable.
- Offline cached map.
- Offline pack stale.
- Layer restricted.
- Place disputed.
- Partial service outage.
- Permission denied.
- Error with retry.

## Permissions And Privacy Behavior

- Coarse location is the default when location is needed.
- Exact location requires explicit purpose, account/device/app scope, expiry, and revocation path.
- Overdesk must not display precise private location trails or continuous mobility history.
- Map handoffs must preserve source refs and redaction classes instead of copying private map data across apps.
- Maps And Navigation owns place, route, map-layer, location permission, and offline-area truth; Overdesk owns map view state and local display preferences only.

## Design Notes

- Use a split map/list layout so users can browse without losing context.
- Keep location controls visible whenever the page uses current location.
- Do not use infinite-scroll map discovery patterns; provide user-controlled loading and filtering.
- Show stale, cached, restricted, and denied states directly on the affected pin, layer, or route.
- Keep correction/report actions close to the selected place, route, or layer.
