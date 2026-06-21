# Docdex And RAG Index Manager

## Slug

`docdex-rag-index-manager`

## Title

Docdex And RAG Index Manager

## Navigation Group

Identity, Privacy, And Data

## Description

Docdex And RAG Index Manager is the Overdesk page for encrypted Docdex/RAG index projections, repository/person/workspace context sources, index freshness, leakage profiles, context grants, model eligibility, retrieval receipts, redaction refs, sync state, and AI/app handoffs. It lets users control private context for AI and search without exposing raw RAG content, decrypted repositories, or broad storage access.

## Primary Users

- Regular users
- Developers
- Researchers
- App owners
- Organization admins
- Institution admins
- Workspace owners
- Delegated RAG/context managers

## Primary User Goals

- See connected Docdex/RAG indexes by owner, source, freshness, and scope.
- Register or remove encrypted index refs where policy allows.
- Review leakage profiles for encrypted-index modes.
- Grant, narrow, renew, or revoke RAG context access.
- See which models, apps, assistants, or code agents may use each index.
- Inspect retrieval receipts, context bundle refs, redaction refs, and usage refs.
- Diagnose stale, degraded, revoked, or unavailable indexes.
- Keep AI context permissioned, bounded, auditable, and efficient.

## Entry Points

- Identity, Privacy, And Data navigation.
- Personal AI Assistant context source selector.
- Personal AI Assistant RAG panel.
- Developer Console project context section.
- Workspace AI assist context controls.
- Privacy And Permissions Center AI/RAG grants.
- Overvault Secure Storage Center key/grant detail.
- Wallet usage and privacy audit.
- Address bar command: `/rag-indexes`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active account/scope.
- Index count.
- Active grant count.
- Stale/degraded index count.
- Retrieval receipt count.
- Primary action: Connect Index.
- Secondary actions: Grants, Sync, Receipts, Export Audit.

### Index Inventory

Content:

- Index ref.
- Source type: person, organization, workspace, repository, app, or project.
- Owner scope.
- Encryption mode.
- Key ref marker.
- Last indexed time.
- Freshness state.
- Capability snapshot.
- Leakage profile marker.
- Grant count.
- Sync state.

Links and handoffs:

- Personal AI Assistant.
- Workspace.
- Developer Console.
- Overvault Secure Storage Center.

### Connect Or Update Index

Content:

- Source selector.
- Owner scope.
- Repository/workspace/person refs.
- Docdex index ref.
- Encryption mode.
- Key/grant ref marker.
- Capability snapshot.
- Leakage profile acknowledgement.
- Policy dry-run result.
- Connect/update draft.

Links and handoffs:

- Overvault Secure Storage Center.
- Privacy And Permissions Center.

### Leakage And Capability Profile

Content:

- Plaintext term-index status.
- Visible path/token-count class.
- Disabled structural features.
- Semantic/web/full-open capability flags.
- Result limits.
- Key status.
- Residual leakage notes.
- Accepted/not accepted marker.
- Capability degradation reason codes.

Links and handoffs:

- Security And Compliance Reviews.
- Activity And Receipts Timeline.

### Context Grants

Content:

- Grant id.
- Actor refs.
- Allowed app/service/model/client refs.
- Allowed indexes and scopes.
- Purpose refs.
- Max result count.
- Max snippet size.
- Redaction profile.
- Expiry.
- Revocation state.
- Renew/narrow/revoke controls.

Links and handoffs:

- Privacy And Permissions Center.
- Personal AI Assistant.

### Retrieval Receipts

Content:

- Retrieval request refs.
- Caller app/service.
- Query hash/ref.
- Requested scope.
- Allowed/denied result.
- Result count.
- Redaction status.
- Context bundle refs.
- Usage refs.
- Replay refs.

Links and handoffs:

- Activity And Receipts Timeline.
- Wallet.

### Context Bundles And Redaction

Content:

- Context bundle refs.
- Selected result refs.
- Redaction plan.
- Context-window budget.
- Prompt-inclusion rules.
- Expiry.
- Retention class.
- Consumed/revoked marker.
- Downstream route refs.

Links and handoffs:

- Personal AI Assistant.
- AI assistant route detail where available.

### Sync And Health

Content:

- Index sync state.
- Last successful sync.
- Pending sync jobs.
- Adapter availability.
- Key material availability.
- Stale index warning.
- Retrieval failures.
- Reindex request action where allowed.
- Removal/deprovision state.

Links and handoffs:

- Diagnostics And Support Bundles.
- Developer Console.

## Primary Actions

- Connect index.
- Update index ref.
- Run policy dry-run.
- Grant RAG access.
- Revoke RAG access.
- Request sync/reindex.
- Open retrieval receipt.
- Export RAG audit.

## Secondary Actions

- Filter indexes.
- Open source ref.
- Copy index ref.
- View leakage profile.
- View context bundle metadata.
- Open wallet usage.
- Ask AI to explain RAG usage.

## States

- Empty inventory.
- Loading.
- Live.
- Index proposed.
- Owner verified.
- Key checked.
- Capability loaded.
- Leakage profiled.
- Active.
- Degraded.
- Stale.
- Sync running.
- Retrieval denied.
- Grant revoked.
- Context bundle expired.
- Adapter unavailable.
- Permission denied.
- Partial owner-service outage.
- Offline cached view.
- Error with retry.

## Permissions And Privacy Behavior

- Docdex And RAG Index Manager displays encrypted-index projections and drafts actions. Encrypted Docdex RAG Adapter owns index authorization, context grants, retrieval requests, leakage profiles, context bundle refs, usage refs, and replay evidence.
- Docdex Adapter and Docdex runtime own Docdex indexing/search/retrieval execution. AI Gateway Router owns final model/resource routing.
- Overdesk must not display raw repository content, raw workspace documents, raw messages, decrypted files, raw snippets outside authorized context, prompts containing private data, vault secrets, or unredacted context bundles.
- Leakage profiles must be visible and honest; encrypted search must not be described as zero-leakage when paths, term indexes, result counts, token counts, or metadata remain visible.
- Grant revocation must make future retrieval fail closed and show downstream invalidation state where owner services provide it.
- Offline mode may show cached metadata and receipts, but connect, sync, grant, revoke, retrieve, and export actions require online revalidation.

## Design Notes

- Use an index inventory table with freshness, leakage, grants, and sync state as primary columns.
- Make leakage profile and grant scope visually prominent before users authorize an index.
- Keep retrieval receipts metadata-first; raw context should be behind explicit authorized flows if ever available.
- Place RAG usage and privacy audit near grant controls so users understand the cost and privacy effect together.
