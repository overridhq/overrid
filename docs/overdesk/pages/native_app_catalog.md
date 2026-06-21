# Native App Catalog

## Slug

`native-app-catalog`

## Title

Native App Catalog

## Navigation Group

Daily Apps

## Description

Native App Catalog is the Overdesk surface for discovering, opening, pinning, installing, updating, removing, and inspecting Overrid-native apps and app-like service surfaces. It should feel like a utility catalog, not an extractive app store or paid-ranking marketplace.

## Primary Users

- Regular users
- Builders
- App owners
- Organization admins
- Institution users
- Stewards reviewing public utility apps

## Primary User Goals

- Find trusted native apps and utility surfaces.
- Understand what an app does before opening it.
- Review app permissions, data classes, resource usage, and privacy summary.
- Open, pin, install, update, or remove an app.
- Inspect incidents, support status, version state, and owner refs.
- Move from catalog discovery to app detail, deployment, wallet, or support flows.

## Entry Points

- Daily Apps navigation.
- Home Dashboard native app shortcut.
- Overrid Browser not-found or app route fallback.
- Global Search app result.
- Owned Apps.
- Deploy New App.
- Address bar command: `/apps`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active account/scope.
- Catalog state.
- Search input.
- Primary action: Open Selected App.
- Secondary actions: Updates, Pinned Apps, Installed Apps.

### Search And Category Rail

Content:

- App search input.
- Category selector.
- Native public utilities.
- Workspace tools.
- Developer tools.
- Institution apps.
- Organization apps.
- User-owned apps.
- Experimental/private-beta filter.
- Restricted/deprecated/disputed filter.

### App Results

Content:

- App name.
- Icon.
- Short description.
- Owner marker.
- Namespace route.
- Installed/open/pinned state.
- Permission summary.
- Resource usage marker.
- Safety label.
- Update or incident marker.

Links and handoffs:

- Overrid Browser.
- Wallet.
- Privacy And Permissions Center.
- App Incidents And Support.

### App Detail Panel

Content:

- App title and namespace.
- Owner refs.
- Version refs.
- Package/source refs where public.
- Description.
- Supported accounts/scopes.
- Required permissions.
- Data classes.
- Resource usage model.
- Support links.
- Reviews or trust summaries where allowed.
- Incident and deprecation state.

Links and handoffs:

- Owned Apps.
- Developer Console.
- Release And Rollback Manager.
- Disputes And Appeals.

### Permissions And Data Use Panel

Content:

- Requested permission classes.
- Current granted permissions.
- Data classes touched.
- AI/RAG/location/vault/message/workspace access markers.
- Scope and expiry.
- Permission history.
- Revoke, narrow, or review controls.

Links and handoffs:

- Privacy And Permissions Center.
- Wallet.
- Activity And Receipts Timeline.

### Usage And Cost Panel

Content:

- Expected ORU dimensions.
- Recent usage refs.
- Receipt refs.
- App owner/service refs.
- Grant or sponsored-credit eligibility marker.
- Usage precheck state.
- No hidden fees marker.

Links and handoffs:

- Wallet.
- Buy Credits.
- Grants And Public-Interest Projects.

### Install, Open, And Pin Panel

Content:

- Open action.
- Install action where local install is needed.
- Pin/unpin action.
- Default app selector.
- Update action.
- Remove action.
- Open-session destination.
- Confirmation state for risky permissions.

### Updates And Incidents Panel

Content:

- Available updates.
- Release notes refs.
- Version pin state.
- Incident state.
- Restricted or suspended state.
- Deprecated marker.
- Rollback/previous-version option where allowed.

Links and handoffs:

- Updates And Release Notes.
- App Incidents And Support.
- Release And Rollback Manager.

## Primary Actions

- Search apps.
- Open app.
- Install app.
- Pin app.
- Review permissions.
- Update app.

## Secondary Actions

- Remove app.
- Set default app.
- Open namespace.
- Open usage receipts.
- Report app.
- Contact support.
- Open package/source refs where public.

## States

- Empty catalog.
- Loading.
- Live.
- Search no results.
- Installed.
- Not installed.
- Update available.
- Permission review required.
- App restricted.
- App deprecated.
- App suspended.
- Incident active.
- Private beta denied.
- Offline cached catalog.
- Partial owner-service outage.

## Permissions And Privacy Behavior

- Catalog ranking must not use paid placement, hidden ad targeting, or surveillance profiles.
- Private-beta, institution-only, suspended, disputed, or restricted apps must not leak protected details to unauthorized users.
- Permission summaries must be shown before install/open when the app asks for sensitive access.
- Overdesk owns catalog display, pins, and local open sessions; app owner services, Deployment Planner, Package Validator, Release Strategy Service, Universal Namespace Service, Overguard, and Overwatch own app truth, releases, policy, and incidents.

## Design Notes

- Use a dense catalog grid with a detail side panel for fast comparison.
- Keep install/open/pin actions visually distinct from permission review.
- Safety labels should be compact and consistent across all app cards.
- Do not imply that catalog order is popularity or paid rank unless the sort mode explicitly says so.
- Show owner-service refs in detail, not as clutter on every card.
