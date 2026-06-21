# Privacy And Permissions Center

## Slug

`privacy-permissions-center`

## Title

Privacy And Permissions Center

## Navigation Group

Identity, Privacy, And Data

## Description

Privacy And Permissions Center is the Overdesk dashboard for app permissions, service grants, retention settings, location access, AI context access, workspace/message access, vault grants, RAG grants, notification redaction, denied attempts, review reminders, and permission audit refs. It gives users and admins one place to inspect, narrow, expire, renew, revoke, and export permission state without exposing raw private content.

## Primary Users

- Regular users
- Organization admins
- Institution admins
- App owners
- Delegated privacy managers
- Support operators with authorized privacy views
- Stewards reviewing public-interest permission evidence

## Primary User Goals

- See which apps and services can access which data classes.
- Revoke, narrow, expire, renew, or review permissions.
- Inspect AI, RAG, vault, location, workspace, messaging, social, maps, wallet, and app analytics grants.
- Understand denied access attempts with safe reason codes.
- Review retention and local cache choices.
- Export permission summaries and privacy audit refs.
- Confirm that permission changes route through owner services.

## Entry Points

- Identity, Privacy, And Data navigation.
- Wallet permissions and privacy audit section.
- Personal AI Assistant context source selector.
- Messaging Center AI triage permission panel.
- Workspace sharing controls.
- App Detail permissions/data section.
- Overvault Secure Storage Center grant detail.
- Docdex And RAG Index Manager grant detail.
- Settings And Security privacy section.
- Address bar command: `/permissions`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active account/scope.
- Permission freshness state.
- Grant count.
- Expiring grant count.
- Denied attempt count.
- Primary action: Review Permissions.
- Secondary actions: Revoke, Export, Retention.

### Scope And Actor Selector

Content:

- Personal scope.
- Organization scope.
- Institution scope.
- App-owner scope.
- Delegated scope.
- Role visibility marker.
- Current actor refs.
- Redaction state.

Links and handoffs:

- Identity And Profile Center.
- Wallet.

### Permission Inventory

Content:

- App/service name.
- Permission class.
- Data class.
- Scope.
- Allowed operation.
- Expiry.
- Last used marker where allowed.
- Source service refs.
- Risk/sensitivity marker.
- Review required marker.

Links and handoffs:

- App Detail.
- Native App Catalog.
- Activity And Receipts Timeline.

### AI And RAG Grants

Content:

- Personal AI context grants.
- Encrypted Docdex/RAG index grants.
- Workspace/document context grants.
- Messaging/search context grants.
- Allowed model/client refs.
- Purpose refs.
- Context expiry.
- Retrieval receipt refs.
- Revoke/narrow actions.

Links and handoffs:

- Personal AI Assistant.
- Docdex And RAG Index Manager.
- Overvault Secure Storage Center.

### Vault And Secret Grants

Content:

- Vault grant refs.
- Sealed reference refs.
- Secret/encrypted-record class.
- Grantee app/service.
- TTL.
- Rotation/revocation marker.
- Access request state.
- Denied reason codes.

Links and handoffs:

- Overvault Secure Storage Center.
- Activity And Receipts Timeline.

### Location, Maps, And Device Permissions

Content:

- Exact location grants.
- Coarse location grants.
- Offline map pack grants.
- Device capability grants.
- Node/provider local signal grants.
- Expiry.
- Last used marker where allowed.
- Revoke and review actions.

Links and handoffs:

- Maps And Navigation.
- Resource Sharing Rules.
- Local Device Settings.

### Workspace, Messaging, And Social Permissions

Content:

- Workspace sharing grants.
- Document/table/page access.
- Messaging access and AI triage grants.
- Social/media upload permissions.
- Directory listing media permissions.
- Public/private link state.
- Expiry/revocation controls.
- Owner-service refs.

Links and handoffs:

- Workspace.
- Messaging Center.
- Social Photo/Video.
- Directory Listings.

### Retention And Local Cache

Content:

- Address-bar history retention.
- Search history retention.
- AI session retention.
- App analytics retention.
- Map/location retention.
- Social/media upload retention.
- Local cache policy.
- Clear local cache action.
- Offline sync marker.

Links and handoffs:

- Settings And Security.
- Local Cache And Offline Sync.

### Denied Attempts And Audit

Content:

- Denied permission attempts.
- Safe reason codes.
- App/service refs.
- Time range.
- Policy refs.
- Related grants.
- Open dispute/report action.
- Export audit action.

Links and handoffs:

- Activity And Receipts Timeline.
- Disputes And Appeals.

## Primary Actions

- Review permissions.
- Revoke permission.
- Narrow permission.
- Set expiry.
- Renew grant.
- Export permission summary.
- Clear local cache.

## Secondary Actions

- Filter by app/service.
- Filter by data class.
- Open source refs.
- Open denied attempt.
- Copy grant ref.
- Ask AI to explain a permission.
- Open dispute.

## States

- Empty permission list.
- Loading.
- Live.
- Review required.
- Grant expiring.
- Grant revoked.
- Revocation pending.
- Narrowing pending.
- Denied attempt present.
- Retention change pending.
- Permission denied.
- Partial owner-service outage.
- Offline cached view.
- Error with retry.

## Permissions And Privacy Behavior

- Privacy And Permissions Center displays permission projections and drafts actions. Owner services own their own grants, while Overguard, Overpass, Overtenant, Overkey, Overvault, Encrypted Docdex RAG Adapter, and native apps own authorization facts behind the projections.
- Raw private content, raw messages, raw workspace documents, raw RAG snippets, vault secrets, precise location trails, and fraud/security internals must not appear.
- Revoke, narrow, renew, expire, retention, and export actions must show affected account/scope, app/service, data class, expiry, owner-service refs, and audit refs before submit.
- Denied attempts must use safe reason codes and must not leak protected targets.
- Offline mode may allow draft preferences, but permission-changing actions must be revalidated online.

## Design Notes

- Use a dense grant table with data-class filters and a details drawer.
- Make revoke, narrow, and expire actions easier to find than broad renewal.
- Keep AI/RAG, vault, location, and workspace/message grants visually distinct because their risks differ.
- Put retention settings in a separate tab so they do not hide active permission grants.
