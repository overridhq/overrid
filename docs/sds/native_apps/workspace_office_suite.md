SDS #75

# Workspace and Office Suite SDS

## Purpose

Build native productivity tools for documents, structured tables, pages/presentations, team folders, permissions, version history, search, and AI-assisted editing.

Workspace and Office Suite is the native productivity utility for Overrid. It owns workspaces, folders, documents, structured tables, pages/presentations, collaboration state, share records, version history, comments, import/export jobs, search handoff refs, AI assist request refs, and usage refs. It is not raw object storage, vault secret storage, a model provider, a search engine, a messaging replacement, or a proprietary lock-in surface. Private workspace data must stay permission scoped and portable.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [workspace_office_suite.md](../../service_catalog/native_apps/workspace_office_suite.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md) |

## Service Family

- Family: Native applications
- Owning layer: Native public utility application layer for collaborative documents, structured work, sharing, versioning, and permissioned AI assistance
- Primary data scope: workspace refs, folder refs, document/table/page/presentation records, editor state, share/permission records, version refs, comment refs, import/export jobs, search index refs, AI assist refs, usage refs, and audit refs
- First build phase from service plan: [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md)

## Problem Statement

Office suites and workspace tools are major lock-in surfaces. They can trap documents, monetize collaboration, train hidden models on private work, and charge organizations for basic productivity infrastructure. Overrid needs a native workspace that uses the same public utility rails as the rest of the system: identity, storage, vault grants, search, usage metering, AI routing, permissions, audit, and export.

The design issue is to keep collaboration state separate from storage, vault secrets, search indexes, AI model execution, and messaging. Workspace should own workspace objects and collaborative workflows while Overstore stores large objects, Overvault protects sensitive/private grants, Search indexes only authorized fields, Personal AI/AI Gateway handle assistant work, and Messaging Center handles notifications/private conversations.

## Goals

- Define workspace, folder, document, structured table, page/presentation, editor session, share permission, version, comment, task/approval, import/export, search handoff, AI assist, and usage records.
- Support create, edit, share, lock, comment, version, restore, archive, delete, import, export, and search flows.
- Support personal, team, organization, and public workspace scopes with explicit permission boundaries.
- Support AI-assisted writing, summarization, table work, document search, and editing through Personal AI Assistant, AI Gateway Router, and Encrypted Docdex RAG Adapter.
- Support encrypted/private workspace data through Overvault grants and source-controlled RAG context.
- Provide open export/import paths to avoid proprietary lock-in.
- Emit usage and receipt refs to Wallet/Usage Center without pricing or revenue assumptions.

## Non-Goals

- Do not become Overstore, Overvault, Search Engine, Messaging Center, Personal AI Assistant, AI Gateway Router, Encrypted Docdex RAG Adapter, ORU Account Service, Seal Ledger, Overbill, or Overclaim.
- Do not store raw secrets, raw large binary objects, final search indexes, model weights, hidden training datasets, card/payment data, or ledger truth.
- Do not bypass workspace permissions, vault grants, source-owned search policies, AI context permissions, tenant/org roles, or audit requirements.
- Do not use private workspace content for AI training, recommendations, analytics, or central AI governance without explicit source and user/org permission.
- Do not implement vendor lock-in by blocking export, hiding version history, or making collaboration depend on proprietary-only formats.
- Do not replace private messaging; use Messaging Center for private conversations and notifications.
- Do not add pricing, customer-count, revenue, blockchain, NFT, or per-transaction fee assumptions.

## Primary Actors And Clients

- Users creating, editing, sharing, commenting, searching, exporting, and restoring personal documents.
- Teams and organizations managing shared workspaces, folders, permissions, approvals, and version history.
- Personal AI Assistant assisting writing, summarization, search, table edits, import/export, and review with permission.
- Search Engine indexing public/authorized workspace records and returning result refs.
- Messaging Center delivering share invitations, comments, mentions, approval notifications, and private communication handoffs.
- Wallet and Usage Center showing storage, editing, AI assist, search, export, and collaboration usage.
- Mobile SDK and native clients supporting offline drafts, sync, notifications, and editing.
- Overpass, Overtenant, Overkey, Universal Namespace Service, Overbase, Overstore, Overvault, Overguard, Overwatch, Overmeter, ORU Account Service, Seal Ledger, Overclaim, and AI/RAG/model-routing services.

## Dependencies

- [Overbase](../data_storage_namespace/overbase.md) for structured workspace object state, versions, permissions, comments, and job refs.
- [Overstore](../data_storage_namespace/overstore.md) for large document assets, embedded objects, attachments, import/export bundles, and generated artifacts.
- [Overvault](../data_storage_namespace/overvault.md) for private workspace grants, sensitive document material, shared secrets, and encrypted RAG context refs.
- [Overpass](../control_plane/overpass.md), [Overtenant](../control_plane/overtenant.md), [Overkey](../control_plane/overkey.md), and [Universal Namespace Service](../data_storage_namespace/universal_namespace_service.md) for identity, tenant/org roles, workspace routes, service accounts, and namespace refs.
- [Overguard](../trust_policy_verification/overguard.md), [Overwatch](../control_plane/overwatch.md), [Overclaim](../trust_policy_verification/overclaim.md), and [Compliance Boundary Service](../governance_ops/compliance_boundary_service.md) for policy, audit, disputes, retention, and compliance boundaries.
- [Search Engine](search_engine.md) for permission-aware workspace indexing and discovery.
- [Messaging Center](messaging_center.md) for share invites, mentions, comments, approval notifications, and private handoffs.
- [Personal AI Assistant](../ai_rag_model_routing/personal_ai_assistant.md), [AI Gateway Router](../ai_rag_model_routing/ai_gateway_router.md), and [Encrypted Docdex RAG Adapter](../ai_rag_model_routing/encrypted_docdex_rag_adapter.md) for permissioned AI assistance and context retrieval.
- [Overmeter](../execution_scheduling/overmeter.md), [ORU Account Service](../accounting/oru_account_service.md), [Seal Ledger](../accounting/seal_ledger.md), and [Wallet and Usage Center](wallet_usage_center.md) for usage visibility and receipts.

## Owned Responsibilities

Workspace and Office Suite owns:

- Workspace, folder, document, table, page/presentation, and app-owned collaboration records.
- Document editor state, structured table schemas, page/presentation layout refs, and embedded object refs.
- Share records, workspace roles, object permissions, invitation refs, public-link refs, and revocation refs.
- Version history, snapshots, diffs, restore refs, lock refs, retention refs, and deletion/tombstone refs.
- Comments, mentions, review/approval refs, task refs, and Messaging Center notification refs.
- Import/export job records, format mappings, export manifests, and portability refs.
- Search indexing handoff refs and source permission contracts for workspace objects.
- AI assist request refs, context plan refs, proposal/apply/reject refs, and assistant audit refs.
- Usage refs for editing, storage, search, AI assist, comments, exports, imports, sync, replay, and retention.

Workspace does not own raw object storage, vault secret material, global search ranking, model execution, assistant sessions, private direct messages, payment/accounting truth, or final dispute decisions.

## Data Model

- `workspace_record`: workspace id, owner actor/org refs, namespace route refs, workspace type, default permissions, retention policy, search policy, AI policy, state, and audit refs.
- `workspace_folder`: workspace refs, parent refs, name refs, ordering, inherited permission refs, retention refs, and state.
- `workspace_object`: object id, workspace/folder refs, object type, title refs, content refs, schema/layout refs, embedded refs, permission refs, current version refs, state, and audit refs.
- `document_content_ref`: object refs, content chunk refs, Overstore refs, encryption/vault refs, editor format, redaction profile, and freshness refs.
- `structured_table_ref`: object refs, table schema, row/column refs, formula refs, validation refs, view refs, import/export refs, and state.
- `page_presentation_ref`: object refs, layout refs, slide/page refs, embedded media refs, theme refs, export refs, and state.
- `editor_session`: object refs, actor/client refs, lock/coedit refs, cursor/presence refs, unsaved change refs, conflict refs, and state.
- `share_permission`: target actor/org/group/app refs, object/workspace refs, role, scope, expiry, public-link refs, revocation state, inherited refs, and audit refs.
- `workspace_version`: object refs, version number, snapshot/diff refs, editor refs, restore refs, policy refs, search update refs, and audit refs.
- `workspace_comment`: object/version refs, author refs, anchor refs, text refs, mention refs, state, notification refs, and audit refs.
- `review_approval_ref`: object/version refs, reviewer refs, requested action, approval state, due refs, messaging refs, and audit refs.
- `workspace_search_handoff`: object refs, source policy, indexable fields, redaction profile, permission snapshot refs, search update refs, and state.
- `ai_assist_request`: object/session refs, requester refs, assistant refs, context-source refs, tool proposal refs, route refs, proposed edits refs, apply/reject refs, and usage refs.
- `import_export_job`: source/destination refs, format, object refs, redaction/export profile, Overstore bundle refs, failure refs, delivery refs, and state.
- `workspace_usage_ref`: edit/storage/search/comment/version/AI/import/export/sync/replay/retention usage, Overmeter refs, and wallet receipt refs.

Common envelope fields: `id`, `tenant_id`, `actor_id`, `organization_id`, `app_id`, `trace_id`, `idempotency_key`, `schema_version`, `state`, `created_at`, `updated_at`, `policy_refs`, and `audit_refs`.

## API Surface

- `POST /workspace/workspaces`: creates a personal, team, organization, or public workspace.
- `GET /workspace/workspaces/{workspace_id}`: returns workspace metadata according to role and visibility.
- `POST /workspace/folders`: creates a folder with inherited permissions.
- `PATCH /workspace/folders/{folder_id}`: renames, moves, archives, or updates folder policy refs.
- `POST /workspace/objects`: creates a document, table, page, presentation, or supported object type.
- `GET /workspace/objects/{object_id}`: reads object metadata and authorized content refs.
- `PATCH /workspace/objects/{object_id}`: applies edits, metadata changes, lock changes, or state changes.
- `POST /workspace/objects/{object_id}/sessions`: starts or resumes an editor session.
- `POST /workspace/objects/{object_id}/versions`: creates a snapshot/version.
- `POST /workspace/versions/{version_id}/restore`: restores an allowed object version.
- `POST /workspace/shares`: creates or updates share permissions, invites, or public-link refs.
- `POST /workspace/shares/{share_id}/revoke`: revokes future access and emits search/vault/cache invalidation refs.
- `POST /workspace/comments`: creates a comment, mention, review note, or approval request.
- `POST /workspace/import-jobs`: starts an import job with format and ownership refs.
- `POST /workspace/export-jobs`: starts an export job with redaction and format refs.
- `POST /workspace/search-handoffs`: creates or updates a Search Engine source/index update handoff.
- `POST /workspace/ai-assist`: requests a permissioned AI assist proposal through Personal AI/AI Gateway/RAG refs.
- `POST /workspace/ai-assist/{request_id}/apply`: applies an accepted AI proposal as a versioned edit.
- `POST /workspace/ai-assist/{request_id}/reject`: rejects an AI proposal and preserves audit refs.
- `GET /workspace/replay/{record_id}`: reconstructs workspace, object, edit, share, version, search, AI, import/export, and usage decisions.

Mutating APIs require signed actor/service identity, tenant/org role refs, workspace/object authority, trace id, idempotency key, policy refs, permission refs, vault/storage refs where applicable, and AI context permissions where applicable. Stable errors include `workspace_not_visible`, `object_not_visible`, `edit_permission_required`, `share_permission_denied`, `public_link_denied`, `vault_grant_required`, `version_conflict`, `editor_lock_conflict`, `export_policy_denied`, `search_index_denied`, `ai_context_permission_required`, `ai_proposal_not_applicable`, and `workspace_state_conflict`.

## Event Surface

- `workspace_office.workspace_created`: workspace created.
- `workspace_office.folder_updated`: folder created, moved, archived, or updated.
- `workspace_office.object_created`: document/table/page/presentation object created.
- `workspace_office.object_updated`: object metadata or content refs changed.
- `workspace_office.editor_session_started`: editor session started.
- `workspace_office.version_created`: object version/snapshot created.
- `workspace_office.version_restored`: prior version restored.
- `workspace_office.share_updated`: share/invite/public-link permissions changed.
- `workspace_office.share_revoked`: share revoked and invalidation refs emitted.
- `workspace_office.comment_created`: comment, mention, review note, or approval request created.
- `workspace_office.import_job_created`: import job created.
- `workspace_office.export_job_created`: export job created.
- `workspace_office.search_handoff_updated`: search source/index refs updated.
- `workspace_office.ai_assist_requested`: AI assist requested with context refs.
- `workspace_office.ai_assist_applied`: AI proposal applied as versioned edit.
- `workspace_office.ai_assist_rejected`: AI proposal rejected.
- `workspace_office.usage_emitted`: usage refs emitted.

Events include workspace/object/version refs, actor/org refs, permission refs, vault/storage refs, search refs, AI route/context refs, message/notification refs, reason codes, audit refs, and usage refs. Events must not include unauthorized document bodies, private table data, vault secrets, private AI context, direct messages, payment data, or hidden model-training payloads.

## Core Workflow

1. User or organization creates a workspace with default permissions, retention, search, and AI policies.
2. User creates folders and objects. Workspace records structured state while content chunks/assets live through Overstore and private grants through Overvault.
3. Editor sessions validate object permission, role, lock/coedit state, and policy before applying edits.
4. Version snapshots, diffs, comments, mentions, and approval refs are recorded append-only.
5. Shares, invites, public links, and revocations update permission records and trigger vault/search/cache invalidation where required.
6. Search handoffs index only allowed fields under source policy and permission snapshots.
7. AI assist requests go through Personal AI Assistant, Encrypted Docdex RAG Adapter, and AI Gateway Router with explicit context permissions and proposal/apply/reject flow.
8. Import/export jobs create portable bundles, redaction manifests, and delivery refs through Overstore.
9. Usage and audit records flow to Overmeter, Wallet/Usage Center, Overwatch, and accounting views.

## State Machine

Workspace lifecycle:

1. `created`
2. `active`
3. `restricted`
4. `archived`
5. `deleting`
6. `deleted`
7. `restored`

Object lifecycle:

1. `draft`
2. `active`
3. `shared`
4. `locked`
5. `under_review`
6. `archived`
7. `deleted`
8. `restored`
9. `tombstoned`

Editor session lifecycle:

1. `requested`
2. `permission_checked`
3. `active`
4. `syncing`
5. `conflict`
6. `saved`
7. `closed`
8. `failed`

Share lifecycle:

1. `requested`
2. `policy_checked`
3. `active`
4. `narrowed`
5. `revoked`
6. `expired`
7. `denied`

AI assist lifecycle:

1. `requested`
2. `context_planned`
3. `permission_checked`
4. `route_selected`
5. `proposal_ready`
6. `applied`
7. `rejected`
8. `cancelled`
9. `failed`

Import/export lifecycle:

1. `requested`
2. `scope_checked`
3. `building`
4. `ready`
5. `delivered`
6. `expired`
7. `failed`

State transitions are append-only. Deletes create tombstone/invalidation refs where search, share, vault, export, or audit policy requires them.

## Policy And Security

- Workspace/object access is deny-by-default unless owner, role, share, public link, app permission, or service account refs allow it.
- Private content refs require Overvault grants and cannot be indexed, exported, or used by AI without explicit permission.
- Share revocation must invalidate future search visibility, AI context access, public links, cached refs, and vault grants where applicable.
- AI assist must produce proposals and audit refs; side-effecting edits apply only after user/role/policy approval.
- Private workspace content cannot be used for hidden training, profiling, recommendations, or central governance without explicit source permissions.
- Public links require scope, expiry, redaction, and revocation controls.
- Version history must preserve enough evidence for restore, audit, disputes, and compliance while respecting retention boundaries.
- Export/import must preserve portability and avoid proprietary lock-in.
- Comments and notifications use Messaging Center refs for mentions/private communication; Workspace stores only workspace comment state.

## Metering And Accounting

- Emit usage refs for workspace/object creation, reads, edits, coediting sessions, version snapshots, comments, share changes, search handoffs, AI assist routes, context retrieval, imports, exports, storage, bandwidth, sync, replay, and retention.
- Link usage to actor, org, workspace, object, version, share, search, AI request, import/export job, Overmeter refs, and wallet receipt refs.
- Workspace and Office Suite does not maintain balances, invoices, provider payouts, grants, holds, refunds, resource rates, or ledger truth.
- Native-service economics remain structural and near-cost; collaboration and AI assist usage stays visible through Wallet/Usage Center.
- Do not encode hardcoded prices, revenue forecasts, paid collaboration tiers, or per-transaction fees.

## Observability And Operations

- Expose workspace/object counts, edit latency, coedit conflicts, version snapshot latency, share changes, revocation invalidation latency, search index lag, AI assist request latency, AI proposal apply/reject rate, export/import duration, permission denial rate, storage usage, and usage emission status.
- Alert on unauthorized access attempts, stale vault grants after revocation, search indexing of private fields, AI context permission violations, export failures, version conflict spikes, public-link abuse, missing usage refs, and replay gaps.
- Provide user/org-visible audit for share changes, public links, AI context access, exports, imports, and permission revocations.
- Provide operator diagnostics using refs, reason codes, and redacted summaries rather than raw document content.
- Provide replay for workspace creation, edits, versions, shares, revocations, search handoffs, AI assist, imports/exports, comments, and usage.

## Failure Modes And Recovery

- Edit conflict: create conflict refs and require merge/restore flow without losing prior version.
- Vault grant missing: deny private content read/edit/export/AI/search until grant is restored.
- Share revocation invalidation fails: mark share revocation pending and block new access at Workspace boundary while retrying downstream invalidations.
- Search handoff unavailable: keep object usable and mark search update pending if policy allows.
- AI route unavailable: return proposal-unavailable state and preserve context/permission plan without widening access.
- AI proposal invalid: reject proposal, keep original object version, and allow retry with narrower scope.
- Export job fails: preserve scope and failure refs; allow retry/cancel without weakening redaction.
- Import conflict: create draft/import review state and require owner confirmation before merging.
- Usage emission fails: mark operation usage pending and reconcile before final receipt visibility.

## Validation Plan

- Users can create, edit, share, comment, version, restore, archive, delete, import, export, search, and AI-assist workspace objects through explicit APIs.
- Workspace data stays tenant, org, role, share, vault, and public-link scoped.
- Share revocation blocks future object access, search visibility, AI context use, public links, and cached refs where required.
- Search Engine indexes only allowed fields under workspace source policy and permission snapshots.
- AI assist uses authorized context only, returns proposals, and applies edits only through versioned user-approved operations.
- Import/export preserves portability, redaction, versions where supported, and object ownership refs.
- Comments and notifications integrate with Messaging Center without turning comments into private messages.
- Usage refs flow to Overmeter and Wallet/Usage Center.
- Replay reconstructs workspace, object, edit, version, share, search, AI, import/export, comment, and usage decisions.

## Build Breakdown

1. Define workspace, folder, object, document content, table, page/presentation, editor session, share permission, version, comment, approval, search handoff, AI assist, import/export, and usage schemas.
2. Implement workspace, folder, object, editor session, version, restore, share, revoke, comment, import/export, search handoff, AI assist, and replay APIs.
3. Add Overbase structured state, Overstore object/bundle refs, Overvault private grants, Overguard policy, and Overwatch audit.
4. Add collaboration conflict handling, version snapshots, restore, comments, mentions, and approval flows.
5. Add Search Engine source contracts, permission snapshots, update/tombstone refs, and private-search controls.
6. Add Personal AI Assistant, AI Gateway Router, and Encrypted Docdex RAG Adapter integration for permissioned AI proposals.
7. Add import/export portability, redaction manifests, storage cleanup, retention, and deletion/tombstone behavior.
8. Add Overmeter usage refs, Wallet/Usage Center receipts, mobile/offline editing surfaces, and operational diagnostics.

## Handoff And Downstream Use

- Search Engine discovers public and authorized workspace objects through source contracts and permission snapshots.
- Personal AI Assistant uses workspace tools for drafting, summarization, table work, search, and edits only under explicit permission.
- Encrypted Docdex RAG Adapter supplies authorized context bundles for workspace and repo-linked work.
- Messaging Center delivers share invites, comments, mentions, approvals, and notifications.
- Wallet and Usage Center shows workspace storage, editing, export, search, AI assist, and collaboration usage plus permissions.
- Mobile SDK and Mobile Backend Gateway support mobile document access, offline drafts, sync, and notifications.
- Central AI Stewardship Interface can receive public-interest or dispute evidence refs, not private workspace content by default.

## Open Design Questions

Resolved decisions:

- Phase 12 first-class authoring formats are Overrid-native workspace records: `workspace_document_v0` for rich text blocks, headings, lists, embeds, comments, and version refs; `workspace_table_v0` for typed rows, columns, formulas, validation, views, and import/export refs; and `workspace_page_v0` for simple pages, slide-like sections, layouts, themes, and embedded Overstore media refs. These records are the canonical editable formats in Overbase with large assets in Overstore and private/sensitive material behind Overvault grants. Markdown/plain text, CSV/TSV, HTML, and PDF can be supported as low-risk export/import renderings. OOXML, ODF, legacy office binaries, macro-bearing spreadsheets, proprietary presentation packages, tracked-change/comment formats from external suites, and complex PDF editing start as import/export or attachment-conversion jobs only, with conversion manifests, redaction profiles, and no executable macros or proprietary format lock-in as the canonical state.
- The first collaborative editing model should be a versioned edit model with short-lived editor sessions, optimistic base-version checks, object or section locks for high-conflict or sensitive scopes, and append-only patch/proposal records. Co-presence, comments, mentions, approvals, and review tasks can be near-real-time through Workspace and Messaging refs, but authoritative content changes are applied as versioned operations against Overbase and rejected or branched when the base version, permission snapshot, vault grant, or retention policy is stale. Full operational-transform or CRDT-style sync is deferred until replay, conflict evidence, offline revalidation, and per-object schema semantics are proven; later OT/CRDT support must remain an internal collaboration engine under the Workspace contract rather than a new product boundary.
- Retention is classed by workspace policy, object state, and compliance/dispute holds. Named versions, published snapshots, restore refs, comments, approvals, share changes, AI proposal apply/reject refs, and import/export manifests remain while the object is active unless the workspace owner/org retention policy shortens them; transient autosaves, cursors, draft patches, and failed proposal material may expire after 30 days or after supersession. Deleted objects stop serving immediately, revoke public links, search visibility, cached refs, AI context grants, and vault grants where applicable, then keep recoverable tombstone, version, audit, and storage refs for the workspace recovery window before content purge, unless Overclaim, Compliance Boundary, legal hold, or incident policy requires sealed retention. Public links default to explicit scope, expiry, redaction, and revocation; short delivery links expire within 24 hours, ordinary share links default to 30 days, and durable public publication must be represented as an explicit public workspace/object state rather than an unbounded private-share link. Raw AI assist context bundles, private RAG snippets, and rejected proposal payloads default to 24-hour retention, while redacted audit refs can remain for replay/privacy audit; applied AI proposals become normal workspace versions.
- First-build mobile offline editing is draft-only and permission-snapshot bound. Mobile clients may cache authorized object metadata, recent content refs, comments, and version summaries, create new personal draft objects, and record local text-block edits, comment drafts, and simple table-cell patches against a known base version for objects already authorized before disconnect. They must not apply authoritative Overbase changes, create/narrow/expand shares, issue public links, restore/delete/archive objects, export private bundles, run AI assist, update search handoffs, or read newly private Overvault material while offline. Sync through Mobile Backend Gateway must revalidate device/session credentials, tenant/org role, workspace permission, object state, base version, vault grants, retention policy, and idempotency before accepting a patch; stale or revoked permissions fail closed, and version drift creates conflict branches or import-review drafts rather than silently merging.
