# Local Cache And Offline Sync

## Slug

`local-cache-offline-sync`

## Title

Local Cache And Offline Sync

## Navigation Group

System And Help

## Description

Local Cache And Offline Sync is the Overdesk page for encrypted local store status, cache classes, offline queue state, sync cursors, retention limits, retry/backoff behavior, conflict handling, safe clearing, and repair. It helps users understand what is stored locally and what can be discarded while preserving owner-service authority for data truth, permissions, wallet state, messages, media, AI/RAG context, app state, and signed mutations.

## Primary Users

- Regular users
- Developers
- App owners
- Organization admins
- Institution admins
- Provider operators
- Support operators
- Security reviewers

## Primary User Goals

- See what classes of data are cached locally and why.
- Confirm encrypted local storage state and retention rules.
- Review offline queues, pending sync, failed sync, and stale data.
- Clear discardable caches without corrupting owner-service state.
- Flush eligible offline work when connectivity returns.
- Inspect conflicts and owner-service revalidation results.
- Generate redacted cache/offline diagnostics for support.

## Entry Points

- System And Help navigation.
- Local Device Settings.
- Settings And Security.
- Diagnostics And Support Bundles.
- Messaging Center offline state.
- Workspace offline state.
- Personal AI Assistant context panel.
- Social Photo/Video upload queue.
- Address bar command: `/cache`.

## Sections To Have

### Page Header

Content:

- Page title.
- Encrypted local store state.
- Total local cache size.
- Offline queue count.
- Failed sync count.
- Stale data count.
- Last successful sync.
- Primary action: Sync Now.
- Secondary actions: Clear Cache, Repair, Export Diagnostics, Retention Settings.

Links and handoffs:

- Settings And Security.
- Local Device Settings.
- Diagnostics And Support Bundles.

### Cache Inventory

Content:

- Cache class.
- Owner service.
- Data class.
- Size.
- Retention limit.
- Encryption state.
- Discardable marker.
- Last read/write time.
- Staleness marker.
- Clear action.

Links and handoffs:

- Activity And Receipts Timeline.
- Privacy And Permissions Center.
- Settings And Security.

### Encrypted Local Store

Content:

- Store availability.
- Platform encryption state.
- Key-provider ref marker.
- Overvault/Overkey dependency marker.
- Locked/unlocked state.
- Repair warnings.
- Secret exclusion marker.
- Last verification time.
- Store reset action.

Links and handoffs:

- Overvault Secure Storage Center.
- Settings And Security.
- Local Device Settings.

### Offline Queue

Content:

- Queue entries by service.
- Command class.
- Payload ref/hash.
- Idempotency key marker.
- Expiry.
- Retry count.
- Last error.
- Eligibility to flush.
- User action needed marker.
- Cancel action where allowed.

Links and handoffs:

- Messaging Center.
- Workspace.
- Social Photo/Video.
- Wallet.

### Sync Cursors And Freshness

Content:

- Service scope.
- Cursor ref.
- Last pull.
- Last push.
- Delta availability.
- Stale state.
- Conflict marker.
- Permission snapshot age.
- Revalidation state.
- Force refresh action where allowed.

Links and handoffs:

- Activity And Receipts Timeline.
- Privacy And Permissions Center.
- Global Search.

### Conflicts And Revalidation

Content:

- Conflict id.
- Owner service.
- Local action summary.
- Remote authoritative state.
- Required user choice.
- Policy denial reason.
- Revalidation receipt.
- Retry/resolve/discard controls.
- Support link.

Links and handoffs:

- Disputes And Appeals.
- App Incidents And Support.
- Diagnostics And Support Bundles.

### Retention And Clearing

Content:

- Address-bar history retention.
- Search history retention.
- AI session retention.
- RAG retrieval metadata retention.
- Messaging preview retention.
- Media upload queue retention.
- Map/location retention.
- Analytics retention.
- Clear all discardable cache action.
- Retention reset action.

Links and handoffs:

- Settings And Security.
- Personal AI Assistant.
- Docdex And RAG Index Manager.
- Maps And Navigation.

### Service-Specific Cache Views

Content:

- Messaging cache.
- Workspace cache.
- Search cache.
- Browser/address cache.
- AI/RAG cache.
- Map/location cache.
- Social/media cache.
- Wallet read-model cache.
- Owned-app analytics cache.

Links and handoffs:

- Messaging Center.
- Workspace.
- Global Search.
- Wallet.
- Owned Apps.

### Repair And Diagnostics

Content:

- Store integrity check.
- Cursor repair status.
- Queue replay dry-run.
- Orphaned cache refs.
- Permission mismatch warnings.
- Rebuild index/cache action.
- Redacted diagnostics summary.
- Support bundle handoff.

Links and handoffs:

- Diagnostics And Support Bundles.
- Local Device Settings.
- Settings And Security.

## Primary Actions

- Sync now.
- Clear discardable cache.
- Clear selected cache class.
- Flush eligible offline queue.
- Resolve conflict.
- Repair local store.
- Reset retention preferences.
- Export redacted cache diagnostics.

## Secondary Actions

- Filter by service.
- Filter by cache class.
- Copy safe cache ref.
- Open owner-service receipt.
- Open related app page.
- Ask AI to explain stale/offline state.

## States

- Loading.
- Live.
- Offline.
- Syncing.
- Synced.
- Stale data.
- Queue pending.
- Queue failed.
- Conflict detected.
- Revalidation required.
- Encrypted store locked.
- Encrypted store unavailable.
- Clearing.
- Cleared.
- Repair required.
- Action denied.

## Permissions And Privacy Behavior

- Local cache entries must be encrypted where platform support exists, bounded by retention policy, and user-clearable when discardable.
- Owner services remain authoritative for messages, wallet, documents, media, app state, permissions, AI/RAG context, and signed mutations.
- Offline mutating actions must be idempotent, expiry-bound, signed where required, and revalidated by owner services before final application.
- Clearing local cache must not delete canonical owner-service data unless the user explicitly triggers a separate owner-service delete flow.
- Raw secrets, private keys, payment details, vault contents, sensitive evidence, and decrypted RAG/source contents must not appear in cache inventory or diagnostics.
- Offline and sync counts must not leak hidden private records across scopes.

## Design Notes

- Use a service-by-service table with clear icons for encrypted, stale, queued, failed, and discardable states.
- Put "Clear" actions close to their cache class, with a confirmation for broad clears.
- Show safe explanations for stale data without exposing hidden private content.
- Make owner-service authority visible on every cache row.
- Keep conflict resolution compact and action-oriented.
- Avoid implying offline work is committed until owner-service revalidation succeeds.
