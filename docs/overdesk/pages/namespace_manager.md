# Namespace Manager

## Slug

`namespace-manager`

## Title

Namespace Manager

## Navigation Group

Identity, Privacy, And Data

## Description

Namespace Manager is the Overdesk page for readable Overrid names, namespace records, route bindings, target refs, verification markers, delegations, transfers, disputes, reservations, tombstones, and resolution previews. It helps users and app owners manage names and routes without turning names into speculative assets or confusing names with identity, assets, ledger state, or private connectivity.

## Primary Users

- Regular users
- Organization admins
- Institution admins
- App owners
- Developers
- Community managers
- Delegated namespace managers
- Support operators with authorized namespace views

## Primary User Goals

- See names and routes owned or delegated to the active scope.
- Claim or request a new name where policy allows.
- Bind names to apps, profiles, services, native app pages, directory listings, or routes.
- Review target refs, route health, route disclosure, and trust markers.
- Delegate namespace management safely.
- Start or review transfer, verification, dispute, release, and tombstone flows.
- Preview how a namespace resolves for different viewer scopes without exposing private targets.

## Entry Points

- Identity, Privacy, And Data navigation.
- Overrid Browser namespace info drawer.
- Identity And Profile Center handle/verification section.
- Owned Apps app route section.
- App Detail identity/routes section.
- Deploy New App namespace step.
- Native App Catalog app page.
- Disputes And Appeals namespace case.
- Address bar command: `/namespaces`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active owner scope.
- Namespace count.
- Delegated namespace count.
- Disputed namespace count.
- Route-health summary.
- Primary action: Claim Name.
- Secondary actions: Bind Route, Delegations, Transfers, Disputes.

### Namespace Inventory

Content:

- Raw name.
- Normalized name.
- Namespace type.
- Scope.
- Owner refs.
- Target refs.
- Visibility.
- State.
- Verification marker.
- Route binding marker.
- Dispute/tombstone marker.
- Last update timestamp.

Links and handoffs:

- Overrid Browser.
- Identity And Profile Center.
- Owned Apps.
- App Detail.

### Claim Or Reserve Name

Content:

- Name input.
- Namespace type selector.
- Scope selector.
- Availability check.
- Reserved-name warning.
- Anti-impersonation policy result.
- Required evidence refs.
- Expiry/reservation state.
- Submit claim action.

Links and handoffs:

- Disputes And Appeals.
- Security And Compliance Reviews.

### Route Bindings

Content:

- App route bindings.
- Service route bindings.
- Native app page refs.
- Directory/listing target refs.
- Profile target refs.
- Overmesh route refs.
- TTL and cache marker.
- Health refs.
- Visibility/disclosure policy.
- Route update draft.

Links and handoffs:

- Overrid Browser.
- Release And Rollback Manager.
- Developer Console.

### Target And Resolution Preview

Content:

- Selected namespace.
- Viewer scope selector.
- Public resolution result.
- Tenant-local resolution result.
- Private target redaction marker.
- Route trust marker.
- Policy denied reason codes.
- Cache state.
- Invalidated route marker.

Links and handoffs:

- Overrid Browser.
- Activity And Receipts Timeline.

### Verification Markers

Content:

- Marker kind.
- Issuer ref.
- Evidence refs.
- Expiry.
- Revocation state.
- Display policy.
- Add marker request.
- Revoke marker request.
- Dispute marker.

Links and handoffs:

- Identity And Profile Center.
- Governance Center.

### Delegations

Content:

- Delegated managers.
- Delegation scope.
- Allowed operations.
- TTL/expiry.
- Revocation state.
- Last use marker where allowed.
- Add delegation draft.
- Revoke delegation draft.

Links and handoffs:

- Privacy And Permissions Center.
- Activity And Receipts Timeline.

### Transfers And Releases

Content:

- Transfer eligibility.
- Required signatures.
- Rights refs.
- Transfer state.
- Target owner refs.
- Release policy.
- Tombstone/reuse policy.
- Rollback or correction marker.

Links and handoffs:

- Overasset Assets.
- Seal Ledger refs through Wallet where visible.
- Disputes And Appeals.

### Disputes And Tombstones

Content:

- Impersonation disputes.
- Squatting disputes.
- Route hijack disputes.
- Misleading-name disputes.
- Abandoned-name cases.
- Temporary restrictions.
- Tombstone refs.
- Appeal/correction refs.

Links and handoffs:

- Disputes And Appeals.
- Activity And Receipts Timeline.

## Primary Actions

- Claim name.
- Bind route.
- Update target refs.
- Add verification marker.
- Delegate management.
- Start transfer.
- Release name.
- Open dispute.

## Secondary Actions

- Preview resolution.
- Copy namespace ref.
- Open route in browser.
- Open app detail.
- View activity refs.
- Export namespace record.
- Ask AI to explain route denial.

## States

- Empty inventory.
- Loading.
- Live.
- Claim draft.
- Availability checking.
- Name unavailable.
- Claim pending.
- Claim denied.
- Active.
- Suspended.
- Disputed.
- Transfer pending.
- Delegated.
- Verification expired.
- Route unhealthy.
- Tombstoned.
- Permission denied.
- Partial owner-service outage.
- Offline cached view.
- Error with retry.

## Permissions And Privacy Behavior

- Namespace Manager displays projections and drafts actions; Universal Namespace Service owns namespace records, claims, route bindings, verification markers, delegations, transfers, disputes, tombstones, and resolution cache.
- Overpass owns identity refs, Overasset/Seal Ledger own rights evidence, Overmesh owns connectivity refs, Overguard owns policy decisions, and Overclaim owns dispute workflows.
- Resolution previews must apply visibility, data-class, tenant, role, and route-disclosure policy before showing targets.
- Private route targets, private identity details, protected app refs, and dispute evidence must be redacted by viewer role.
- Offline mode may show cached namespace records, but claims, binds, transfers, delegation, verification, release, and disputes require online signed validation.

## Design Notes

- Use an inventory table plus a route/resolution detail drawer.
- Make namespace state, route health, verification, and dispute markers scannable in the table.
- Keep claim/bind/transfer flows wizard-like because they affect public resolution.
- Avoid market-style or asset-trading language; names are operational references, not speculative assets.
