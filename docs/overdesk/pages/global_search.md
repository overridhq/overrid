# Global Search

## Slug

`global-search`

## Title

Global Search

## Navigation Group

Home And Fast Access

## Description

Global Search is the permission-aware discovery page for Overdesk. It searches across routes, apps, workspace objects, directory listings, messages, assets, wallet receipts, governance reports, maps, social posts, developer records, and local activity where the active account has permission.

## Primary Users

- Regular users
- App owners
- Builders
- Institution users
- Stewards
- Support users with explicit permission

## Primary User Goals

- Find anything the active scope is allowed to see.
- Filter search by content type, source, account, time, and permission class.
- Understand why results appear or are omitted.
- Jump directly to the owner page for a result.
- Search private sources only with explicit permission.

## Entry Points

- Top address bar search fallback.
- Search item in primary navigation.
- Home Dashboard.
- Command palette: `search`.
- Not-found route fallback.
- Directory, workspace, wallet, activity, governance, and developer pages.

## Sections To Have

### Search Header

Content:

- Search input.
- Active account/scope.
- Search mode selector.
- Source filters.
- Privacy scope indicator.
- Saved search control.

### Source Filter Rail

Content:

- All.
- Routes.
- Apps.
- Workspace.
- Directory.
- Messages.
- Assets.
- Wallet and receipts.
- Maps.
- Social.
- Governance.
- Developer.
- Local activity.

### Results List

Content:

- Result title.
- Source service.
- Type marker.
- Permission-safe snippet.
- Last updated time.
- Owner/account marker where allowed.
- Ranking or match reason.
- Omitted/denied count summary.
- Open action.

### Result Detail Panel

Content:

- Expanded snippet.
- Source refs.
- Permission refs.
- Usage refs where search has a cost.
- Related routes.
- Owner page link.
- Report/correction/dispute actions where allowed.

### Private Source Permission Panel

Content:

- Private sources available but not searched.
- What permission is required.
- Expiry option.
- Data classes involved.
- Search receipt behavior.

Links and handoffs:

- Privacy And Permissions Center.
- Docdex And RAG Index Manager.
- Overvault Secure Storage Center.

### Saved Searches

Content:

- Saved query title.
- Scope.
- Sources.
- Last run.
- Alert state.
- Edit/delete controls.

## Primary Actions

- Run search.
- Open result.
- Filter results.
- Save search.
- Grant private source search for this query.

## Secondary Actions

- Copy result link.
- Open source app.
- Report result.
- Request correction.
- Open dispute.
- Clear search history.

## States

- Empty search.
- Searching.
- No results.
- Results found.
- Partial results.
- Permission-limited results.
- Private sources available.
- Search service unavailable.
- Offline local-history search.

## Permissions And Privacy Behavior

- Search must use permission-safe snippets only.
- Private sources must not be searched until the user grants scope.
- Result ranking explanations must not leak private source signals.
- Search history must follow retention settings.
- Denied result counts may be shown only as safe aggregates.

## Design Notes

- The page should feel fast and direct.
- Filters should be visible without overwhelming the first search.
- Result rows should be compact and comparable.
- The detail panel should prevent unnecessary navigation for quick inspection.
