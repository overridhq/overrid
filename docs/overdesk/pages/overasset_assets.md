# Overasset Assets

## Slug

`overasset-assets`

## Title

Overasset Assets

## Navigation Group

Wallet, Credits, And Ownership

## Description

Overasset Assets is the Overdesk surface for scoped utility rights, operational ownership refs, resource rights, storage entitlements, capacity claims, grant rights, namespace-linked rights, app/service ownership refs, delegations, transfers, revocations, expiries, and disputes. It must make rights usable for operations without importing NFT-style speculation or market behavior.

## Primary Users

- Regular users
- App owners
- Organization admins
- Institution admins
- Resource providers
- Grantees
- Delegated asset managers
- Stewards and support operators with authorized views

## Primary User Goals

- See owned and delegated operational rights.
- Understand what a right allows, where it came from, and when it expires.
- Bind rights to apps, namespaces, storage, grants, or resources where allowed.
- Request delegation, revoke delegation, or request transfer.
- See disputes, holds, restrictions, revocations, and corrections.
- Export or replay asset evidence.
- Open related wallet, namespace, app, dispute, or grant pages.

## Entry Points

- Wallet, Credits, And Ownership navigation.
- Home Dashboard asset summary.
- Owned Apps.
- App Detail.
- Namespace Manager.
- Grants And Public-Interest Projects.
- Disputes And Appeals.
- Activity And Receipts Timeline.
- Address bar command: `/assets`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active account/scope.
- Asset visibility state.
- Asset count.
- Disputed/expiring warning.
- Primary action: Open Asset.
- Secondary actions: Delegations, Transfers, Export.

### Asset Inventory

Content:

- Asset/right name.
- Right type.
- Owner refs.
- Delegation marker.
- App/service/namespace/storage binding marker.
- Grant or capacity marker.
- Expiry.
- Transferability marker.
- Dispute/restriction marker.

Links and handoffs:

- Wallet.
- Namespace Manager.
- Owned Apps.
- Grants And Public-Interest Projects.

### Filters And Grouping

Content:

- Account/scope filter.
- Right type filter.
- App/service filter.
- Namespace filter.
- Grant filter.
- Delegated-to-me filter.
- Expiring soon filter.
- Disputed/restricted filter.
- Transferable filter.

### Asset Detail Panel

Content:

- Asset/right title.
- Right id.
- Right type.
- Owner refs.
- Source evidence refs.
- Scope and allowed use.
- Resource dimensions.
- Validity window.
- Delegation policy.
- Transfer policy.
- Current state.
- Related app, namespace, storage, grant, or resource refs.

Links and handoffs:

- Activity And Receipts Timeline.
- Disputes And Appeals.
- Overrid Browser.

### Evidence And Replay Panel

Content:

- Ownership evidence refs.
- Source service refs.
- Policy refs.
- Ledger or grant refs where applicable.
- State transition refs.
- Replay summary.
- Export asset record action.

### Delegation Panel

Content:

- Active delegations.
- Delegatee refs.
- Scope.
- Allowed operations.
- Expiry.
- Revocation conditions.
- Create delegation draft.
- Revoke delegation draft.
- Owner-service response state.

Links and handoffs:

- Identity And Profile Center.
- Privacy And Permissions Center.

### Transfer And Binding Panel

Content:

- Transferability state.
- Transfer request state.
- Blocked transfer reasons.
- App binding state.
- Namespace binding state.
- Storage/resource binding state.
- Policy preview.
- Final confirmation controls where allowed.

Links and handoffs:

- Namespace Manager.
- App Detail.
- Overvault Secure Storage Center.

### Disputes, Holds, And Restrictions

Content:

- Active dispute refs.
- Hold effect.
- Restriction reason.
- Revocation refs.
- Correction refs.
- Appeal path.
- Tombstone marker where applicable.

Links and handoffs:

- Disputes And Appeals.
- Security And Compliance Reviews.

## Primary Actions

- Open asset.
- Filter assets.
- Request delegation.
- Revoke delegation.
- Request transfer.
- Bind to app or namespace.
- Open dispute.

## Secondary Actions

- Export asset record.
- View replay.
- Copy asset ref.
- Open source refs.
- View related wallet usage.
- Ask AI to explain asset scope.

## States

- Empty inventory.
- Loading.
- Live.
- Active.
- Delegated.
- Transfer requested.
- Transfer blocked.
- Transferred.
- Disputed.
- Restricted.
- Held.
- Revoked.
- Expired.
- Corrected.
- Tombstoned.
- Permission denied.
- Partial owner-service outage.
- Offline cached view.
- Error with retry.

## Permissions And Privacy Behavior

- Overasset rights are scoped operational facts, not universal ownership claims.
- Transferability defaults to false unless evidence, policy, and jurisdiction allow it.
- Overdesk must not expose raw private storage content, vault secrets, provider-private evidence, or unrelated owner details.
- Disputed, held, restricted, revoked, expired, or tombstoned rights must not appear usable without clear state markers.
- Overasset owns right records, delegation, transfer, revocation, expiry, correction, and dispute state; storage, namespace, app, grant, accounting, and policy owner services own their own final decisions.

## Design Notes

- Use a dense inventory table with a detail side panel for evidence and actions.
- Treat asset actions as high-risk when they affect ownership, delegation, transfer, or access.
- Show allowed use and expiry near the title, not buried in metadata.
- Avoid collectible, rarity, market, or speculative language.
- Keep evidence refs readable enough for trust without cluttering the inventory list.
