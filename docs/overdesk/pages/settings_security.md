# Settings And Security

## Slug

`settings-security`

## Title

Settings And Security

## Navigation Group

Identity, Privacy, And Data

## Description

Settings And Security is the Overdesk page for account scope preferences, local device security, session and credential refs, encrypted local cache behavior, notification rules, retention choices, accessibility, display density, language, developer mode, and redacted diagnostics exports. It centralizes desktop shell settings without storing secret material or overriding the identity, vault, key, permission, or owner-service records that hold authority.

## Primary Users

- Regular users
- Developers
- App owners
- Provider operators
- Organization admins
- Institution admins
- Support operators with delegated scope
- Security reviewers

## Primary User Goals

- Review the active account, organization, institution, tenant, app-owner, and delegated scopes.
- Manage desktop session visibility, device refs, credential status, and logout/revoke actions.
- Confirm key-provider and vault connectivity without exposing raw secrets.
- Set local cache, offline retention, address-bar history, search history, AI session retention, map/location retention, analytics retention, and media upload retention.
- Configure notifications, quiet hours, theme, display density, accessibility, language, and keyboard shortcuts.
- Enable developer mode for signed test environments without weakening production safety.
- Generate a redacted support bundle that the user can review before export.
- Understand which high-risk actions require fresh credential verification.

## Entry Points

- Identity, Privacy, And Data navigation.
- Shell account menu.
- Identity And Profile Center.
- Privacy And Permissions Center.
- Overvault Secure Storage Center.
- Docdex And RAG Index Manager.
- Developer Console.
- Diagnostics And Support Bundles.
- Updates And Release Notes.
- Address bar command: `/settings`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active user/account ref.
- Active organization, institution, tenant, and app-owner scope chips.
- Device trust state.
- Session state.
- Key-provider state.
- Local encrypted-store state.
- Support bundle readiness.
- Primary action: Review Security.
- Secondary actions: Export Diagnostics, Clear Cache, Revoke Device, Open Privacy Center.

Links and handoffs:

- Identity And Profile Center.
- Privacy And Permissions Center.
- Overvault Secure Storage Center.
- Activity And Receipts Timeline.

### Account And Scope Settings

Content:

- Active account ref.
- Username/namespace summary.
- Personal profile ref.
- Organization scope.
- Institution scope.
- Tenant scope.
- App-owner scope.
- Delegated administration scope.
- Current desktop role summary.
- Scope-switcher policy result.
- Last scope change receipt.

Links and handoffs:

- Identity And Profile Center.
- Namespace Manager.
- Privacy And Permissions Center.
- Activity And Receipts Timeline.

### Sessions Devices And Credentials

Content:

- Current desktop session.
- Other active sessions.
- Device refs.
- Credential class marker.
- Authentication freshness state.
- Last active timestamps.
- Revoke session control.
- Revoke device control.
- Logout all devices control.
- Fresh verification requirement.
- Failed action reason codes.

Links and handoffs:

- Identity And Profile Center.
- Activity And Receipts Timeline.
- Diagnostics And Support Bundles.

### Keys Vaults And Secret Boundaries

Content:

- Overkey-controlled ref status.
- Overvault connectivity state.
- Platform credential-provider state.
- Recovery method refs.
- Vault grant summary.
- Unavailable key reason codes.
- Rotating credential action where allowed.
- Secret exposure warning when a user attempts unsafe export.

Links and handoffs:

- Overvault Secure Storage Center.
- Privacy And Permissions Center.
- Security And Compliance Reviews.

### Local Cache And Encrypted Store

Content:

- Local cache status.
- Encrypted local store status.
- Offline sync status.
- Cache size.
- Cache classes by data type.
- Clear cache control.
- Clear offline queue control where allowed.
- Discardable/non-discardable marker.
- Retention policy summary.
- Cache repair and rebuild actions.

Links and handoffs:

- Local Cache And Offline Sync.
- Diagnostics And Support Bundles.
- Activity And Receipts Timeline.

### Notification Preferences

Content:

- Global notification toggle.
- Quiet hours.
- Security alert priority.
- Wallet alert priority.
- Message alert priority.
- App incident alert priority.
- Governance update alert priority.
- Device/resource alert priority.
- Per-scope overrides.
- Last notification delivery receipt.

Links and handoffs:

- Notifications Center.
- Messaging Center.
- Wallet.
- Governance Center.

### Retention Preferences

Content:

- Address-bar history retention.
- Search history retention.
- AI session retention.
- RAG retrieval receipt retention view.
- App analytics retention.
- Map/location retention.
- Social/media upload retention.
- Messaging local preview retention.
- Support bundle retention.
- Reset-to-default control.

Links and handoffs:

- Global Search.
- Personal AI Assistant.
- Docdex And RAG Index Manager.
- Maps And Navigation.
- Social Photo/Video.

### Accessibility Language And Display

Content:

- Theme.
- Contrast preference.
- Motion preference.
- Font scale.
- Display density.
- Language.
- Region.
- Time format.
- Keyboard shortcuts.
- Screen-reader labels status.
- Focus outline setting.

Links and handoffs:

- Help.
- Local Device Settings.

### Developer Mode

Content:

- Developer mode enablement state.
- Signed test environment status.
- Local Overrid stack target.
- Manifest preview setting.
- Debug log level.
- Test account visibility.
- Local RAG/context source visibility.
- Developer warning state.
- Production-scope protection marker.
- Disable developer mode action.

Links and handoffs:

- Developer Console.
- Deploy New App.
- Release And Rollback Manager.
- Security And Compliance Reviews.

### Diagnostics And Support Bundle

Content:

- Bundle type selector.
- Included ref classes.
- Redaction summary.
- Excluded secret classes.
- User review checklist.
- Export destination.
- Copy bundle id action.
- Delete generated bundle action.
- Support handoff ref.
- Recent diagnostics failures.

Links and handoffs:

- Diagnostics And Support Bundles.
- App Incidents And Support.
- Activity And Receipts Timeline.

## Primary Actions

- Review security state.
- Revoke session.
- Revoke device.
- Logout from all devices.
- Clear local cache.
- Update retention preferences.
- Configure notifications.
- Enable or disable developer mode.
- Generate redacted support bundle.
- Open privacy grants.

## Secondary Actions

- Switch account/scope.
- Copy safe diagnostic refs.
- Open key-provider status.
- Open vault grants.
- Reset display preferences.
- Reset retention preferences.
- View activity receipts.
- Ask AI to explain a setting.

## States

- Loading.
- Live.
- Scope switched.
- Fresh verification required.
- Device not trusted.
- Key provider unavailable.
- Vault unavailable.
- Local encrypted store unavailable.
- Cache clearing.
- Cache cleared.
- Cache repair required.
- Developer mode active.
- Support bundle pending review.
- Support bundle exported.
- Action denied.

## Permissions And Privacy Behavior

- Overdesk may store shell preferences, cache preferences, non-secret device refs, and display settings locally.
- Raw secrets, private keys, recovery secrets, and credential material remain in approved platform credential providers or Overkey/Overvault-controlled refs.
- Identity, session, permission, vault, key, and grant truth comes from the owning services; Overdesk only presents state and submits signed user actions.
- High-risk actions require fresh session or credential verification before execution.
- Local caches must be encrypted where the platform supports it, bounded by retention policy, user-clearable, and safe to discard.
- Support bundles are redacted by default and must show a human-review step before export.
- Developer mode cannot disable production credential, wallet, vault, deployment, compliance, or governance checks.

## Design Notes

- Use a calm settings layout with grouped panels and clear status chips.
- Keep dangerous actions visually separated from ordinary preferences.
- Use icon buttons for revoke, clear, export, and reset actions, with tooltips.
- Show owner-service authority labels on security-critical rows so users understand what Overdesk can and cannot change.
- Make retention controls compact but explicit; each data class should show current value, owner service, and last change receipt.
- Do not expose raw tokens, keys, secret strings, sensitive diagnostics, or full private evidence in the UI.
