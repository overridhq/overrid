# Directory Listings

## Slug

`directory-listings`

## Title

Directory Listings

## Navigation Group

Daily Apps

## Description

Directory Listings is the Overdesk surface for local services, jobs, housing, events, community posts, organization pages, public-interest pools, resource offers, app pages, and marketplace-style discovery. It should be useful like a classifieds directory without ad-trap ranking, surveillance targeting, or manipulative paid placement.

## Primary Users

- Regular users
- Local service providers
- Organizations
- Institutions
- App owners
- Community moderators
- Public-interest project owners

## Primary User Goals

- Browse or search useful listings.
- Create and manage listings.
- Contact a listing owner safely.
- Understand locality, category, trust, and freshness.
- Report, dispute, or appeal listing decisions.
- Move from a listing to maps, messaging, wallet, namespace, or app pages.

## Entry Points

- Daily Apps navigation.
- Home Dashboard fast app shortcut.
- Global Search result.
- Overrid Browser not-found fallback.
- Maps And Navigation.
- Social Photo/Video.
- Native App Catalog.
- Address bar command: `/directory`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active location/locality scope.
- Active account/scope.
- Primary action: Create Listing.
- Secondary actions: Search, Saved Listings, My Listings.

### Search And Discovery

Content:

- Search box.
- Category selector.
- Locality selector.
- Freshness filter.
- Trust marker filter.
- Public-interest filter.
- Sort mode.

### Category And Filter Rail

Content:

- Services.
- Jobs.
- Housing.
- Events.
- Community.
- Organizations.
- Resource offers.
- Public-interest projects.
- Apps.
- Institution listings.
- Saved filters.

### Listing Results

Content:

- Listing title.
- Category.
- Locality.
- Owner marker.
- Trust/verification marker.
- Freshness.
- Price/credit/resource indicator where applicable.
- Contact action.
- Save action.
- Report action.

Links and handoffs:

- Messaging Center.
- Maps And Navigation.
- Wallet.
- Overrid Browser.

### Listing Detail

Content:

- Listing title.
- Description.
- Media refs.
- Owner profile.
- Location/locality.
- Terms or eligibility.
- Usage/cost refs where relevant.
- Freshness and expiry.
- Related namespace/app/resource refs.
- Report/dispute state.

Links and handoffs:

- Identity And Profile Center.
- Messaging Center.
- Maps And Navigation.
- Namespace Manager.
- Disputes And Appeals.

### Create Or Edit Listing Panel

Content:

- Listing type.
- Title.
- Description.
- Category.
- Tags and purpose tags.
- Locality.
- Media.
- Contact route.
- Visibility.
- Expiry.
- Moderation preview.
- Usage estimate.

### My Listings

Content:

- Draft listings.
- Active listings.
- Expired listings.
- Flagged listings.
- Disputed listings.
- Performance summary.
- Renewal actions.

### Safety And Moderation Panel

Content:

- Report listing.
- Block owner.
- Appeal moderation.
- View reason code.
- View public/private redaction state.
- Open dispute.

## Primary Actions

- Search.
- Open Listing.
- Create Listing.
- Contact Owner.
- Save Listing.
- Edit Listing.

## Secondary Actions

- Share route.
- Open map.
- Report.
- Open dispute.
- Renew listing.
- Archive listing.
- Request namespace binding.

## States

- Empty category.
- Loading.
- Live.
- Search no results.
- Locality unavailable.
- Permission restricted.
- Listing expired.
- Listing under review.
- Listing suspended.
- Offline cached browse.
- Partial moderation outage.

## Permissions And Privacy Behavior

- Directory ranking must not use hidden surveillance targeting.
- Contact actions must protect the user's selected identity and scope.
- Listing owner private contact details must not be exposed unless explicitly published.
- Moderation and fraud internals must be redacted by viewer role.
- Location should default to locality/coarse area unless exact location is intentionally published.

## Design Notes

- Use searchable lists with optional map/locality companion view.
- Avoid infinite-scroll addiction patterns; provide clear pagination or user-controlled loading.
- Make category, locality, and freshness visible in every listing card.
- Create/edit should be a side panel or wizard that preserves browse context.
