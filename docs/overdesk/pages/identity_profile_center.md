# Identity And Profile Center

## Slug

`identity-profile-center`

## Title

Identity And Profile Center

## Navigation Group

Identity, Privacy, And Data

## Description

Identity And Profile Center is the Overdesk page for personal, organization, institution, app-owner, device, session, credential, handle, visibility, recovery, and delegated-scope projections. It gives users one place to understand who they are acting as inside Overrid while keeping durable identity, tenant membership, and credential authority with Overpass, Overtenant, Overkey, and related owner services.

## Primary Users

- Regular users
- Organization admins
- Institution admins
- App owners
- Resource providers
- Delegated account managers
- Support operators with authorized identity views

## Primary User Goals

- See the active identity and account/scope being used by Overdesk.
- Switch between personal, organization, institution, app-owner, and delegated scopes.
- Manage user-visible profile fields and visibility settings.
- Review handles, verification markers, and namespace-linked identity refs.
- Inspect sessions, devices, credentials, recovery state, and delegated access.
- Revoke sessions or devices and start recovery flows safely.
- Understand which profile facts are public, tenant-local, private, or restricted.

## Entry Points

- Identity, Privacy, And Data navigation.
- Account/scope switcher in the Overdesk shell.
- Wallet account selector.
- Messaging Center profile or recipient view.
- Namespace Manager owner refs.
- Privacy And Permissions Center actor selector.
- Settings And Security account/session area.
- Address bar command: `/profile`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active identity.
- Active account/scope.
- Identity lifecycle state.
- Session freshness state.
- Verification marker summary.
- Primary action: Edit Profile.
- Secondary actions: Switch Scope, Revoke Session, Recovery.

### Identity Summary

Content:

- Stable identity ref.
- Identity type.
- Display handle.
- Lifecycle state.
- Created/updated markers.
- Tombstone/merge marker where applicable.
- Verification markers.
- Public/private visibility marker.
- Owner-service refs.

Links and handoffs:

- Activity And Receipts Timeline.
- Namespace Manager.

### Profile Fields

Content:

- Display name.
- Avatar/media refs.
- Bio/description.
- Public contact prefs.
- Language/locale.
- Accessibility prefs.
- Profile completeness marker.
- Field-level visibility.
- Edit draft state.

Links and handoffs:

- Privacy And Permissions Center.
- Overvault Secure Storage Center for sensitive refs.

### Accounts, Tenants, And Organizations

Content:

- Personal account refs.
- Organization memberships.
- Institution memberships.
- App-owner scopes.
- Provider scopes.
- Delegated scopes.
- Role refs.
- Suspension/offboarding markers.
- Default scope setting.

Links and handoffs:

- Wallet.
- Owned Apps.
- Node Fleet Manager.
- Provider Earnings And Payouts.

### Handles And Verification

Content:

- Usernames/handles.
- Namespace-linked routes.
- Verification marker type.
- Issuer refs.
- Expiry.
- Revocation state.
- Dispute marker.
- Claim or correction handoff.

Links and handoffs:

- Namespace Manager.
- Disputes And Appeals.
- Activity And Receipts Timeline.

### Sessions And Devices

Content:

- Current desktop session.
- Other sessions.
- Device refs.
- Last active timestamps.
- Credential class marker.
- Location/coarse client marker where allowed.
- Revoke session action.
- Revoke device action.
- Fresh verification requirement.

Links and handoffs:

- Settings And Security.
- Activity And Receipts Timeline.

### Credentials And Recovery

Content:

- Credential refs.
- Key-provider status.
- Rotation/revocation markers.
- Recovery options.
- Lost-device flow.
- Malicious-rotation warning.
- Emergency revocation handoff.
- Audit refs.

Links and handoffs:

- Overvault Secure Storage Center.
- Settings And Security.

### Delegated Access

Content:

- Delegations granted by the user.
- Delegations granted to the user.
- Scope and role.
- Allowed operations.
- Expiry.
- Revocation state.
- Review reminder.
- Add/revoke delegation actions.

Links and handoffs:

- Privacy And Permissions Center.
- Activity And Receipts Timeline.

### Visibility And Privacy

Content:

- Public profile fields.
- Tenant-local fields.
- Private fields.
- Search visibility.
- Messaging visibility.
- Directory visibility.
- Social visibility.
- AI context eligibility marker.
- Review privacy action.

Links and handoffs:

- Privacy And Permissions Center.
- Global Search.
- Directory Listings.

## Primary Actions

- Edit profile.
- Switch active scope.
- Revoke session.
- Revoke device.
- Start recovery.
- Manage handles.
- Manage delegated access.

## Secondary Actions

- Copy identity ref.
- Open namespace.
- Open activity refs.
- Export profile summary.
- Review visibility.
- Ask AI to explain active scope.
- Open settings.

## States

- Loading.
- Live.
- Profile draft.
- Profile save pending.
- Profile save denied.
- Session stale.
- Fresh verification required.
- Recovery pending.
- Delegation expiring.
- Verification expired.
- Dispute open.
- Identity suspended.
- Identity tombstoned.
- Permission denied.
- Partial owner-service outage.
- Offline cached view.
- Error with retry.

## Permissions And Privacy Behavior

- Identity And Profile Center displays identity and profile projections and drafts actions; Overpass owns identity refs and lifecycle, Overtenant owns membership and role scope, and Overkey/Overvault own credential and secret refs.
- Raw credentials, private keys, recovery secrets, vault secrets, and unrelated identity internals must never appear.
- Session, device, credential, recovery, and delegation changes require fresh verification where policy requires it.
- Public profile visibility must be field-level and reversible where owner services allow it.
- Offline mode may display cached identity state, but scope switching, session revocation, recovery, credential, and profile changes must be revalidated online.

## Design Notes

- Use a profile overview with clear scope-switching, then tabs for Profile, Scopes, Sessions, Credentials, Delegations, and Visibility.
- Keep active scope visible in the header because many Overdesk pages inherit it.
- Put sensitive credential and recovery actions behind confirmation and contextual drawers.
- Use plain role labels and safe reason codes instead of leaking tenant-policy internals.
