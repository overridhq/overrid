# Overvault Secure Storage Center

## Slug

`overvault-secure-storage-center`

## Title

Overvault Secure Storage Center

## Navigation Group

Identity, Privacy, And Data

## Description

Overvault Secure Storage Center is the Overdesk page for vault-safe projections of sensitive material: vault item refs, encrypted private record refs, secret refs, access requests, access grants, sealed references, mount leases, rotation state, revocation state, recovery/escrow refs, backup/export eligibility, and redacted audit trails. It must make protected material manageable without showing raw secrets or turning Overdesk into a vault authority.

## Primary Users

- Regular users
- Organization admins
- Institution admins
- App owners
- Developers
- Resource providers
- Delegated vault managers
- Support operators with authorized vault metadata views

## Primary User Goals

- See vault item metadata without exposing secret values.
- Understand which apps, services, workloads, AI flows, or indexes have vault grants.
- Request, approve where authorized, revoke, rotate, quarantine, or recover sensitive refs.
- Review sealed references and encrypted private records.
- Inspect access decisions, denied attempts, leases, expiry, and audit refs.
- Export vault metadata safely where allowed.
- Confirm that support bundles and diagnostics do not include raw secret material.

## Entry Points

- Identity, Privacy, And Data navigation.
- Privacy And Permissions Center vault grant section.
- Developer Console secrets refs.
- Deploy New App manifest validation.
- App Detail permissions/data section.
- Docdex And RAG Index Manager key/grant detail.
- Settings And Security credential/key-provider section.
- Diagnostics And Support Bundles redaction review.
- Address bar command: `/vault`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active account/scope.
- Vault visibility state.
- Vault item count.
- Active grant count.
- Rotation/revocation alert count.
- Primary action: Add Secure Ref.
- Secondary actions: Grants, Rotate, Recovery, Export Metadata.

### Vault Inventory

Content:

- Vault item ref.
- Secret/encrypted-record class.
- Owner scope.
- App/service scope.
- Data class.
- Current version marker.
- Retention policy.
- Rotation policy.
- State.
- Last access marker where allowed.
- Grant count.

Links and handoffs:

- Privacy And Permissions Center.
- Activity And Receipts Timeline.

### Vault Item Detail

Content:

- Item ref.
- Secret kind.
- Encrypted payload ref marker.
- Key policy ref.
- Current version.
- Checksum/fingerprint marker.
- Data class.
- Allowed subjects.
- Retention state.
- Searchability flag for encrypted records.
- Redacted display state.

Links and handoffs:

- Settings And Security.
- Security And Compliance Reviews.

### Access Requests And Grants

Content:

- Access request list.
- Requesting app/service/workload.
- Purpose.
- Requested TTL.
- Decision state.
- Policy refs.
- Access grant refs.
- Grant expiry.
- Revocation state.
- Denied reason codes.

Links and handoffs:

- Privacy And Permissions Center.
- App Detail.
- Personal AI Assistant.

### Sealed Refs And Mount Leases

Content:

- Sealed reference refs.
- Workload/run refs.
- Node refs where visible.
- Mount lease refs.
- Lease TTL.
- Renewal policy.
- Cleanup evidence.
- Revocation hook state.
- Delivery channel marker.

Links and handoffs:

- Node Detail.
- App Incidents And Support.

### Rotation And Revocation

Content:

- Rotation jobs.
- Source version.
- Target version.
- Dependent refs.
- Rollout state.
- Rollback eligibility.
- Revocation records.
- Quarantine state.
- Tombstone/deletion marker.

Links and handoffs:

- Activity And Receipts Timeline.
- Security And Compliance Reviews.

### Recovery And Escrow

Content:

- Recovery options.
- Escrowed record refs.
- Required evidence.
- Reviewer refs.
- Release policy.
- Current review state.
- Release result marker.
- Appeal/correction refs.

Links and handoffs:

- Identity And Profile Center.
- Disputes And Appeals.

### Backup, Export, And Diagnostics

Content:

- Backup eligibility refs.
- Export eligibility refs.
- Metadata export controls.
- Redaction profile.
- Support bundle inclusion marker.
- Diagnostics redaction marker.
- Backup/restore refs where visible.
- Export job state.

Links and handoffs:

- Diagnostics And Support Bundles.
- Local Cache And Offline Sync.

## Primary Actions

- Add secure ref.
- Request access.
- Approve or deny request where authorized.
- Revoke grant.
- Rotate secret.
- Quarantine item.
- Start recovery.
- Export metadata.

## Secondary Actions

- Filter vault items.
- Copy vault ref.
- Open grant detail.
- Open access audit.
- Open affected app.
- View diagnostics redaction.
- Ask AI to explain metadata only.

## States

- Empty inventory.
- Loading.
- Live.
- Metadata only.
- Access requested.
- Access denied.
- Grant active.
- Grant expiring.
- Grant revoked.
- Rotation pending.
- Rotation failed.
- Quarantined.
- Recovery pending.
- Escrow review.
- Backup/export unavailable.
- Permission denied.
- Partial owner-service outage.
- Offline cached metadata.
- Error with retry.

## Permissions And Privacy Behavior

- Overvault Secure Storage Center displays vault metadata and drafts handoffs; Overvault owns secret records, encrypted private records, access decisions, access grants, mount leases, rotation, revocation, escrow, and redaction rules.
- Overdesk must never show raw secret values, private keys, decrypted private records, raw access payloads, payment secrets, or vault delivery material.
- Reveal, grant, rotate, revoke, quarantine, recovery, escrow, backup, and export flows require signed identity, scope, purpose, TTL where relevant, policy refs, and audit refs.
- Support bundles and diagnostics must include only redacted metadata unless a separately authorized private export path exists.
- Offline mode may show cached metadata only and cannot reveal, grant, rotate, revoke, recover, or export vault material.

## Design Notes

- Use an inventory table with a metadata-only item detail drawer.
- Keep "Reveal" flows rare, explicit, and separate from normal metadata browsing.
- Show grant and rotation alerts in the header because stale grants are a primary risk.
- Use redacted placeholders consistently so designers never imply a secret value is visible in the page.
