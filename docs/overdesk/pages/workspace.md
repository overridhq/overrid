# Workspace

## Slug

`workspace`

## Title

Workspace

## Navigation Group

Daily Apps

## Description

Workspace is the Overdesk productivity surface for documents, tables, pages, files, comments, approvals, versions, sharing, imports, exports, search, and AI assist. It should feel like a serious work tool: dense, stable, permission-aware, and fast for repeated office work.

## Primary Users

- Regular users
- Organization teams
- Institution users
- Researchers
- Builders
- App owners

## Primary User Goals

- Open, create, edit, and organize workspace objects.
- Share documents or folders with precise permissions.
- Review comments, approvals, versions, and import/export jobs.
- Search workspace content.
- Use AI assist with explicit document context.
- Work safely with offline drafts and conflict states.

## Entry Points

- Daily Apps navigation.
- Home Dashboard fast app shortcut.
- Global Search result.
- Personal AI Assistant context handoff.
- Directory or governance report attachment link.
- Address bar route to workspace object.

## Sections To Have

### Page Header

Content:

- Page title.
- Active workspace/account/scope.
- Sync state.
- Primary action: New.
- Secondary actions: Upload, Import, Search, Share.

### Workspace Navigator

Content:

- Personal workspaces.
- Organization workspaces.
- Institution workspaces.
- Shared with me.
- Recent.
- Favorites.
- Offline marked.
- Trash/archive where allowed.

Links and handoffs:

- Identity And Profile Center.
- Privacy And Permissions Center.
- Settings And Security.

### Object List

Content:

- Folders.
- Documents.
- Tables.
- Pages/presentations.
- Files.
- Media refs.
- Owner.
- Last edited.
- Share state.
- Offline state.
- Version/conflict marker.

### Editor Or Preview Area

Content:

- Selected document editor or preview.
- Table/grid editor or preview.
- Page/presentation editor or preview.
- File preview.
- Read-only state.
- Unsaved draft state.
- Conflict state.

Links and handoffs:

- Personal AI Assistant.
- Global Search.
- Activity And Receipts Timeline.

### Share And Permissions Panel

Content:

- Current collaborators.
- Roles.
- Public/private link status.
- Expiry.
- Revocation controls.
- AI context eligibility.
- Search index eligibility.
- Vault grant state where relevant.

Links and handoffs:

- Privacy And Permissions Center.
- Overvault Secure Storage Center.

### Comments And Approvals Panel

Content:

- Comments.
- Mentions.
- Assigned reviewers.
- Approval state.
- Change requests.
- Internal notes where allowed.
- Resolution history.

Links and handoffs:

- Messaging Center.
- Notifications Center.

### Versions And History Panel

Content:

- Version list.
- Current version.
- Restore action.
- Compare action.
- Import version.
- Conflict branch.
- Audit/replay refs.

Links and handoffs:

- Activity And Receipts Timeline.

### Import And Export Panel

Content:

- Upload/import queue.
- Export type.
- Included objects.
- Redaction level.
- Conversion warnings.
- Export readiness.
- Failure state.

### AI Assist Panel

Content:

- Summarize.
- Draft.
- Rewrite.
- Table cleanup.
- Import review.
- Ask about document.
- Context scope.
- Usage estimate.
- AI receipt refs.

Links and handoffs:

- Personal AI Assistant.
- Docdex And RAG Index Manager.

## Primary Actions

- New Document.
- New Table.
- New Page.
- Upload.
- Share.
- Search Workspace.
- Comment.

## Secondary Actions

- Favorite.
- Mark Offline.
- Export.
- Restore Version.
- Compare Versions.
- Ask AI.
- Revoke Access.

## States

- Empty workspace.
- Loading.
- Live.
- Syncing.
- Offline draft.
- Read-only.
- Permission denied.
- Conflict detected.
- Import running.
- Export running.
- Search indexing pending.
- Partial owner-service outage.

## Permissions And Privacy Behavior

- Workspace object truth belongs to the Workspace service, not Overdesk.
- Sharing and revocation must show affected users, roles, expiry, search/AI implications, and audit refs.
- AI assist must require explicit context scope.
- Offline drafts must be clearly marked and revalidated before authority-changing sync.
- Private document contents must not leak into notifications, search snippets, AI context, or support bundles without permission.

## Design Notes

- Use a file-manager plus editor split layout for desktop.
- Keep navigation, object list, and editor dimensions stable.
- Make share state visible without forcing users into settings.
- Conflict and read-only states must be obvious before the user edits.
