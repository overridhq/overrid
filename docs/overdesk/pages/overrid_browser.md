# Overrid Browser

## Slug

`overrid-browser`

## Title

Overrid Browser

## Navigation Group

Home And Fast Access

## Description

The Overrid Browser is the address-based navigation surface for the Overrid net. It lets users enter names, routes, app paths, documents, message refs, search queries, and commands from a single address bar. It must feel familiar like a browser, but route through Overrid identity, namespace, policy, and app-session rules.

## Primary Users

- Regular users
- Builders
- App owners
- Institution users
- Support users with explicit permission

## Primary User Goals

- Visit an Overrid address such as `/hugo`.
- Open apps, profiles, documents, routes, listings, and namespace-bound pages.
- Understand why a route opened, failed, or was denied.
- Move between tabs without losing context.
- Use search or directory fallback when a namespace does not resolve.

## Entry Points

- Top address bar.
- Home Dashboard primary action.
- Browse item in primary navigation.
- Command palette: `browse`.
- Links from app cards, directory listings, namespace records, search results, messages, and governance reports.

## Sections To Have

### Browser Header

Content:

- Address bar.
- Back/forward controls.
- Refresh/retry.
- Current account/scope.
- Route status.
- Trust marker.
- New tab control.
- Bookmark/pin control.

### Tab Strip

Content:

- Open Overrid pages.
- App icons or route markers.
- Loading, denied, stale, and offline indicators.
- Close, reorder, duplicate, and pin behavior.

### Main Page Frame

Content:

- Embedded app view.
- Namespace route view.
- Document/listing/profile view.
- Denied page.
- Not-found page.
- External-link warning page.

### Namespace Info Drawer

Content:

- Raw input.
- Normalized address.
- Namespace owner ref.
- Target ref.
- Route ref.
- Verification marker.
- Visibility.
- Dispute state.
- Tombstone state.
- Source service.

Links and handoffs:

- Namespace Manager.
- Disputes And Appeals.
- Activity And Receipts Timeline.

### Privacy And Permission Drawer

Content:

- Active identity and scope.
- Permissions used by the current route.
- Data classes requested.
- Location, vault, AI/RAG, workspace, message, or wallet grants requested by the page.
- Revoke or inspect links.

Links and handoffs:

- Privacy And Permissions Center.
- Overvault Secure Storage Center.
- Docdex And RAG Index Manager.

### Fallback Panel

Content:

- Search fallback.
- Directory fallback.
- Suggested similar names.
- Recently visited matching routes.
- Create/request namespace route where allowed.

Links and handoffs:

- Global Search.
- Directory Listings.
- Namespace Manager.

## Primary Actions

- Resolve address.
- Open route.
- Open new tab.
- Search fallback.
- Directory fallback.
- Bookmark route.

## Secondary Actions

- Copy route ref.
- Inspect namespace.
- Report route.
- Open dispute.
- Clear browsing data.
- Open privacy drawer.

## States

- Empty new tab.
- Resolving.
- Route found.
- Route denied.
- Namespace not found.
- App session denied.
- External link warning.
- Offline cached page.
- Stale cached page.
- Partial route-service outage.

## Permissions And Privacy Behavior

- The browser must not bypass Universal Namespace Service, Overmesh, Overgate, or Overguard.
- The browser must not silently grant permissions to embedded apps.
- External links must be marked clearly.
- Private browsing state must be scoped by account and retention settings.
- Route-denied pages must explain the safe reason category without leaking protected data.

## Design Notes

- The address bar is the most important control and must stay visually dominant.
- Denied and not-found states should be useful, with safe fallbacks.
- Browser chrome should be compact so app content has room.
- Tabs must use stable sizes and avoid layout shifts during loading.
