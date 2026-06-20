# SUB BUILD PLAN #75 - Workspace and Office Suite

Attached SDS: [SDS #75 - Workspace and Office Suite](../sds/native_apps/workspace_office_suite.md)

## Purpose

This sub-build plan turns SDS #75 into an implementation sequence for Workspace and Office Suite. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Workspace and Office Suite is the Phase 12 native productivity utility for workspaces, folders, documents, structured tables, pages/presentations, editor sessions, permissions, shares, public links, version history, comments, approvals, import/export jobs, search handoff refs, AI assist request/proposal/apply/reject refs, usage refs, audit refs, and replay projections. It does not own raw object storage, vault secret material, global search ranking, AI model execution, private direct messages, ORU balance truth, Seal Ledger entries, billing settlement, payment secrets, or final dispute outcomes.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #75: Workspace and Office Suite](../sds/native_apps/workspace_office_suite.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering, operations, failure modes, validation, build breakdown, downstream handoffs, and resolved open-question decisions. |
| [Workspace and Office Suite service plan](../service_catalog/native_apps/workspace_office_suite.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, signed envelopes, trace ids, idempotency keys, stable errors, local stack, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant scopes, Overkey signing refs, Overgate ingress, Overregistry service records, Overwatch audit refs, and Overqueue command/event primitives. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy decisions, Workload Classifier facts, Overclaim dispute refs, deny-by-default checks, policy dry-runs, evidence refs, reason codes, and replayable decisions. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies Overmeter usage refs, Wallet/Usage Center receipt projections, ORU/Seal Ledger/Overbill accounting truth, Overclaim dispute refs, and the rule that Workspace emits usage refs but never mutates accounting truth. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Supplies AI Gateway Router, Personal AI Assistant, encrypted RAG, adapter, SDK, CLI, admin, and mSwarm Runtime Bridge groundwork used for permissioned AI assist, local-first collaboration proposals, and bounded native-app handoffs. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase workspace state, Overstore object/bundle refs, Overvault private grants, Universal Namespace refs, retention, backup/restore, and replay substrates. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public abuse-control and low-sensitivity public-pool prerequisites for public workspace objects, public links, public templates, and source-safe discovery where applicable. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Controls the first build point for Workspace and Office Suite and its first useful workspace, object, editing, sharing, versioning, search, AI assist, import/export, mobile draft, usage, audit, and replay slice. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies formal workspace privacy, retention, export, revocation, AI context, offline sync, incident response, threat review, public reporting, reliability, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #75 first build work aligned to master Phase 12, with Phase 13 as governance/security/compliance/reporting hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services and contracts, Tokio where bounded workers are needed, Axum/Tower/Hyper-style HTTP where a service boundary exists, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, TypeScript for client surfaces only, native Overrid service boundaries, and no conventional database/object-store/vault/search/office platform, Kubernetes-first, blockchain, NFT, external payment processor, hardcoded pricing, revenue, customer-count, hidden AI training, private-data bypass, direct-message ownership, or proprietary office lock-in. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 6, 8, 11, 12, and 13 | Attach SDS #75, preserve Phase 12 as first build, record prerequisites, and freeze Workspace ownership boundaries. |
| 2 | Master Phases 0, 1, 4, 8, and 12 | Define Rust contracts, canonical schemas, authoring formats, lifecycle enums, event surfaces, stable errors, and deterministic fixtures. |
| 3 | Master Phases 1, 4, 8, and 12 | Implement workspace, folder, object, document, table, page/presentation, content refs, namespace routes, and object metadata foundations. |
| 4 | Master Phases 1, 4, 6, 8, 12, and 13 | Implement editor sessions, versioned edits, snapshots, restore, comments, approvals, locks, conflicts, and append-only proposal records. |
| 5 | Master Phases 1, 4, 8, 11, 12, and 13 | Implement share permissions, workspace roles, public links, invite refs, revocation, vault/search/cache invalidation, and audit visibility. |
| 6 | Master Phases 6, 8, 12, and 13 | Implement Search Engine handoffs, permission snapshots, Personal AI Assistant tools, AI Gateway routing, encrypted RAG context, and AI proposal/apply/reject flow. |
| 7 | Master Phases 6, 8, 12, and 13 | Implement import/export portability, redaction manifests, format conversion boundaries, mobile draft/offline sync, SDK/CLI/admin/client surfaces, and mSwarm Runtime Bridge handoffs. |
| 8 | Master Phases 5, 8, 12, and 13 | Implement Overbase/Overstore/Overvault integration, usage refs, Wallet/Usage Center handoffs, Overwatch audit, replay bundles, observability, and operations runbooks. |
| 9 | Master Phases 4, 8, 11, 12, and 13 | Implement failure recovery, retention classes, compliance holds, threat-model/security-review gates, public-link abuse controls, AI context cleanup, and governance reporting. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, workspace/privacy/export/AI/offline boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Workspace and Office Suite uses Rust-first shared contracts and service-facing APIs for workspace records, folders, objects, content refs, editor sessions, share permissions, versions, comments, approvals, search handoffs, AI assist records, import/export jobs, usage refs, audit refs, and replay bundles. TypeScript is acceptable for generated client bindings and native/web UI surfaces, but it must call Overrid APIs and must not become a privileged workspace authority.
- Workspace contracts, authoring records, events, lifecycle states, stable errors, permission snapshots, search handoffs, AI assist refs, import/export manifests, retention classes, redaction profiles, replay bundles, and deterministic fixtures use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf may be generated later only through the shared contract layer.
- Mutating APIs require signed actor/service envelopes, tenant/org/workspace/object scope where applicable, trace id, idempotency key, workspace/object authority, permission refs, vault/storage refs where applicable, policy refs, reason codes, schema version, stable errors, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for content refs, version snapshots, import/export manifests, AI proposal bundles, search handoff snapshots, replay bundles, audit exports, fixture inputs, and deterministic comparisons.
- Structured workspace state, large object assets, encrypted private grants, queueing where needed, identity, namespace, key status, policy, audit, usage, accounting refs, dispute refs, diagnostics, AI route refs, search refs, notification refs, and replay must use native Overrid service boundaries such as Overbase, Overstore, Overvault, Overqueue, Overpass, Overtenant, Overkey, Universal Namespace Service, Overgate, Overguard, Overwatch, Overmeter, ORU Account Service, Seal Ledger, Wallet/Usage Center, Search Engine, Messaging Center, Personal AI Assistant, AI Gateway Router, Encrypted Docdex RAG Adapter, mSwarm Runtime Bridge, Compliance Boundary Service, Mobile SDK, Mobile Backend Gateway, SDK, CLI, and Admin/Developer UI.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Elasticsearch, OpenSearch, Solr, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, external office suites, proprietary-only document formats, hidden AI training datasets, engagement/profiling analytics, raw document bodies in events, raw vault secrets, direct messages, global search ranking truth, AI model execution, final dispute authority, direct ORU/Seal Ledger mutation, hardcoded pricing, revenue forecasts, or customer-count assumptions the Workspace boundary.

## Phase 1: SDS Attachment, Phase 12 Scope, And Workspace Authority Boundary

### Work Items

- **1.1 Attach the build plan to SDS #75.**
  - Design: Link this document from the Workspace and Office Suite SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/native_apps/workspace_office_suite.md`, `docs/service_catalog/native_apps/workspace_office_suite.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #75 returns both the Workspace and Office Suite SDS and this sub-build plan.

- **1.2 Preserve master Phase 12 as the first build point.**
  - Design: Keep first implementation in Phase 12 because Workspace depends on identity, tenancy, policy, storage, vault, metering, accounting visibility, AI/RAG routing, search, messaging, mobile, and runtime-bridge rails from earlier phases.
  - Output: Phase-gate note that Phases 0, 1, 4, 5, 6, 8, and 11 supply prerequisites; Phase 12 builds the first useful workspace utility; Phase 13 hardens privacy, retention, export, collaboration, offline sync, AI context, incident response, reporting, compliance, and scale.
  - Validation: Review proves the plan does not move Workspace into Phase 8 storage ownership, Phase 11 public-provider controls, Phase 13-only governance, Search ranking, Messaging private messages, AI model execution, or payment/accounting ownership.

- **1.3 Freeze the Workspace ownership boundary.**
  - Design: Record that Workspace owns workspace/folder/object records, editor state, authoring records, share permissions, version history, comments, review/approval refs, import/export jobs, search handoff refs, AI assist refs, usage refs, audit refs, and replay projections.
  - Output: Ownership checklist for architecture, API, UI, implementation, operations, and review gates.
  - Validation: Review confirms Workspace does not own raw object storage, vault secret material, final search indexes, model execution, direct messages, payment/accounting truth, final fraud decisions, final dispute outcomes, or source-service private grants.

- **1.4 Carry forward resolved SDS #75 decisions.**
  - Design: Preserve Overrid-native canonical authoring formats, versioned edit model, deferred OT/CRDT boundary, classed retention, public-link defaults, AI context retention, and draft-only mobile offline editing rules.
  - Output: Resolved-decision checklist covering `workspace_document_v0`, `workspace_table_v0`, `workspace_page_v0`, short-lived editor sessions, optimistic base-version checks, object/section locks, append-only patch/proposal records, 24-hour raw AI context retention, public-link expiry, durable public publication state, and offline draft revalidation.
  - Validation: Review rejects proprietary canonical formats, executable macros, silent merge behavior, unbounded private-share links, hidden AI training, authoritative offline edits, public-link bypass, and Workspace-owned storage/vault/search/model truth.

- **1.5 Define upstream and downstream service boundaries.**
  - Design: Record how Overbase, Overstore, Overvault, Overpass, Overtenant, Overkey, Universal Namespace Service, Overguard, Overwatch, Overclaim, Compliance Boundary Service, Search Engine, Messaging Center, Personal AI Assistant, AI Gateway Router, Encrypted Docdex RAG Adapter, Overmeter, Wallet/Usage Center, Mobile SDK, Mobile Backend Gateway, SDK, CLI, and Admin/Developer UI interact through refs.
  - Output: Boundary matrix naming allowed reads, owned writes, handoff refs, denied direct mutation, redaction rules, usage refs, audit refs, replay refs, owner-service finality, and downstream handoffs.
  - Validation: Review confirms each upstream/downstream service keeps its authority and Workspace exchanges refs/events rather than copying private internals or inventing canonical truth owned elsewhere.

## Phase 2: Contracts, Schemas, Formats, Events, Stable Errors, And Fixtures

### Work Items

- **2.1 Create the Workspace Rust contract module.**
  - Design: Add contract types for workspace records, folders, objects, document content refs, structured tables, page/presentation refs, editor sessions, share permissions, versions, comments, approvals, search handoffs, AI assist records, import/export jobs, usage refs, retention classes, redaction profiles, and stable errors.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, authoring-format enums, permission-role enums, export-state enums, retention-state enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from raw storage, vault material, search ranking, model execution, direct messaging, and accounting truth.

- **2.2 Define canonical authoring schemas.**
  - Design: Model `workspace_document_v0`, `workspace_table_v0`, and `workspace_page_v0` with stable block/table/layout refs, embedded refs, Overstore refs, Overvault refs where needed, comments/version anchors, redaction profiles, search policies, and AI context policy refs.
  - Output: JSON Schema files, valid examples, invalid examples, round-trip fixture inputs, BLAKE3 snapshot hashes, migration notes, and compatibility rules for later Protobuf generation.
  - Validation: Schema tests reject missing workspace/object scope, unstable content refs, unauthorized embedded refs, missing schema version, missing audit refs, raw secret payloads, executable macro fields, and proprietary-only canonical state.

- **2.3 Define API, event, and replay contracts.**
  - Design: Model workspace/object CRUD, editor sessions, version snapshots/restores, shares, revocations, comments, import/export, search handoffs, AI assist requests/apply/reject, usage emissions, and replay events with trace ids, idempotency keys, actor/org/workspace/object refs, policy refs, reason codes, and audit refs.
  - Output: API contract set, event schema set, replay bundle schema, redacted projection schema, stable ordering rules, and fixture-backed event streams.
  - Validation: Tests prove events include necessary refs and reason codes while excluding unauthorized document bodies, private table data, vault secrets, private AI context, direct messages, payment data, and hidden model-training payloads.

- **2.4 Define stable error taxonomy.**
  - Design: Preserve SDS stable errors and map them to workspace visibility, object visibility, edit permission, share denial, public-link denial, vault grant, version conflict, lock conflict, export denial, search indexing denial, AI context permission, AI proposal applicability, and workspace state conflicts.
  - Output: Stable error registry, HTTP/API mapping, client-facing messages, support-safe diagnostics, retryability flags, redaction behavior, and replay refs.
  - Validation: Tests prove denials are deterministic, tenant-safe, audience-safe, and replayable without exposing hidden workspace membership, private content, vault grant internals, AI context payloads, or unrelated object existence.

- **2.5 Create deterministic fixtures and golden traces.**
  - Design: Build fixtures for workspace create, object create, document edit, table edit, page edit, editor session, version snapshot, restore, share, revoke, comment, approval, import, export, search handoff, AI assist, AI apply/reject, mobile draft sync, usage emission, retention, and replay.
  - Output: Fixture directory, canonical inputs, expected states, expected events, expected errors, BLAKE3 hashes, usage refs, audit refs, redacted projections, and replay bundles.
  - Validation: Fixture tests produce stable ids, hashes, lifecycle states, reason codes, usage refs, audit refs, redacted outputs, denied counts, and replay output across repeated runs.

## Phase 3: Workspace, Folder, Object, And Canonical Authoring Records

### Work Items

- **3.1 Implement workspace records and namespace routes.**
  - Design: Create personal, team, organization, and public workspace records with owner refs, namespace route refs, default permissions, retention policy, search policy, AI policy, state, and audit refs.
  - Output: `workspace_record` schema, `POST /workspace/workspaces`, `GET /workspace/workspaces/{workspace_id}`, namespace binding refs, default policy records, and lifecycle transitions.
  - Validation: Tests prove workspace creation requires signed actor/service identity, tenant/org authority, idempotency, namespace policy, and deny-by-default visibility.

- **3.2 Implement folders and object metadata.**
  - Design: Add folder records and object metadata for documents, structured tables, pages, presentations, embedded refs, inherited permissions, ordering, retention refs, and state transitions.
  - Output: `workspace_folder` and `workspace_object` schemas, folder create/update APIs, object create/read/update APIs, inherited permission refs, and tombstone behavior.
  - Validation: Tests prove moves, renames, archive/delete/restore, inherited permissions, and tombstones preserve audit refs and do not widen visibility.

- **3.3 Implement document content refs.**
  - Design: Store canonical rich text blocks, headings, lists, embeds, content chunks, redaction profile refs, current version refs, Overstore refs, and Overvault refs where private/sensitive material requires grants.
  - Output: `document_content_ref` schema, document object create/read/update fixtures, content ref validation, and BLAKE3 content snapshot hashes.
  - Validation: Tests prove document APIs never place raw unauthorized document bodies in events and reject edits without valid object permission, base version, storage/vault refs, and audit refs.

- **3.4 Implement structured table and page/presentation refs.**
  - Design: Model typed rows, columns, formulas, validation refs, views, import/export refs, layouts, slide/page refs, themes, embedded media refs, and state for first-build structured tables and pages/presentations.
  - Output: `structured_table_ref` and `page_presentation_ref` schemas, object create/edit fixtures, validation rules, formula-safety checks, and export refs.
  - Validation: Tests reject unsafe formula/macros, unauthorized embedded media, invalid view refs, missing version refs, and proprietary-only canonical state.

- **3.5 Implement object lifecycle, audit, and usage hooks.**
  - Design: Emit audit and usage refs for workspace create, folder change, object create/read/update/archive/delete/restore, metadata reads, storage refs, and retention changes.
  - Output: Overwatch audit events, Overmeter usage refs, Wallet/Usage Center receipt refs where applicable, pending-usage reconciliation markers, and replay links.
  - Validation: Tests prove every material object operation has traceable audit and usage refs and that failed usage emission creates a pending reconciliation marker without blocking safe object state.

## Phase 4: Editor Sessions, Versioned Edits, Comments, Approvals, And Conflict Handling

### Work Items

- **4.1 Implement short-lived editor sessions.**
  - Design: Start/resume editor sessions only after object permission, tenant/org role, vault grant, lock/coedit state, base version, and policy checks pass.
  - Output: `editor_session` schema, `POST /workspace/objects/{object_id}/sessions`, session expiry, cursor/presence refs, lock/coedit refs, and stable conflict errors.
  - Validation: Tests prove sessions fail closed for stale permission snapshots, revoked vault grants, object state conflicts, expired roles, and incompatible base versions.

- **4.2 Implement versioned edit operations.**
  - Design: Apply edits as append-only patch/proposal records against explicit base versions with object or section locks for high-conflict or sensitive scopes.
  - Output: Versioned edit command schema, patch/proposal refs, optimistic conflict detection, branch/import-review behavior, and user-visible merge/restore states.
  - Validation: Tests prove stale base versions reject or branch without silent merge and preserve prior versions, audit refs, and replay evidence.

- **4.3 Implement snapshots, restore, and version history.**
  - Design: Create version snapshots, diffs, restore refs, named versions, published snapshots, retention refs, and search update refs without deleting audit history.
  - Output: `workspace_version` schema, `POST /workspace/objects/{object_id}/versions`, `POST /workspace/versions/{version_id}/restore`, version list/read views, and replay support.
  - Validation: Tests prove restore requires live authorization, preserves version/audit/search refs, respects retention and compliance holds, and does not resurrect revoked shares.

- **4.4 Implement comments, mentions, reviews, and approvals.**
  - Design: Store workspace comment state, anchored comments, mention refs, review/approval refs, task refs, and Messaging Center notification refs without becoming private direct messaging.
  - Output: `workspace_comment` and `review_approval_ref` schemas, `POST /workspace/comments`, notification handoff refs, approval lifecycle, and audit refs.
  - Validation: Tests prove comments inherit object permission, notifications use Messaging Center refs, private conversations stay out of Workspace, and comment retention follows workspace policy.

- **4.5 Implement conflict and review diagnostics.**
  - Design: Provide user-visible conflict refs, branch/merge review refs, invalid proposal refs, restore candidates, and support-safe diagnostics using reason codes and redacted summaries.
  - Output: Conflict records, import-review drafts, merge/restore guidance refs, diagnostic summaries, and replay bundles.
  - Validation: Tests simulate edit conflict, invalid AI proposal, import conflict, missing vault grant, and stale lock while preserving prior versions and private content boundaries.

## Phase 5: Sharing, Permissions, Public Links, Revocation, And Audit

### Work Items

- **5.1 Implement share permissions and workspace roles.**
  - Design: Model actor/org/group/app refs, object/workspace refs, roles, scopes, expiry, public-link refs, inherited refs, revocation state, and audit refs.
  - Output: `share_permission` schema, `POST /workspace/shares`, share update/read views, role matrix, invitation refs, and permission fixtures.
  - Validation: Tests prove shares are deny-by-default, scoped, expiring, tenant-safe, app-safe, auditable, and unable to expose hidden workspace/object membership.

- **5.2 Implement public-link controls.**
  - Design: Require explicit scope, expiry, redaction, reason codes, and revocation behavior for public links, with durable public publication represented as explicit public workspace/object state.
  - Output: Public-link schema fields, short delivery link defaults, ordinary share link defaults, durable public publication records, and redaction profiles.
  - Validation: Tests prove unbounded private-share links are rejected and public links cannot serve private content, vault material, hidden comments, or unauthorized versions.

- **5.3 Implement share revocation and invalidation.**
  - Design: Revoke future object access and trigger vault, search, cache, public-link, AI context, and offline snapshot invalidation refs where required.
  - Output: `POST /workspace/shares/{share_id}/revoke`, revocation refs, downstream invalidation envelopes, pending invalidation states, and retry behavior.
  - Validation: Tests prove future access blocks at the Workspace boundary even when downstream invalidation is pending and high-risk private content fails closed immediately.

- **5.4 Implement permission audit views.**
  - Design: Provide user/org-visible audit for share changes, public links, AI context access, exports, imports, revocations, comments, and permission denials.
  - Output: Permission audit view schema, filters, redacted summaries, Overwatch refs, source refs, and replay links.
  - Validation: Tests prove audit views are viewer-scoped, redacted by audience class, and unable to reveal unrelated-user details, raw private content, vault secrets, or hidden AI context.

- **5.5 Implement permission replay and owner-service handoffs.**
  - Design: Add replay bundles for share creation, update, revocation, public-link delivery, invalidation refs, denied access, and downstream owner-service results.
  - Output: Replay schemas, redaction snapshots, reason codes, source-service refs, audit refs, and deterministic fixture traces.
  - Validation: Tests prove replay reconstructs visible decisions and denial reasons without recovering private payloads, secret-bearing refs, or unrelated-user data.

## Phase 6: Search Handoffs, AI Assist, RAG Context, And Assistant Proposals

### Work Items

- **6.1 Implement Search Engine handoff refs.**
  - Design: Emit source-policy, indexable-field, redaction-profile, permission-snapshot, freshness, tombstone, and update refs for public or authorized workspace objects.
  - Output: `workspace_search_handoff` schema, `POST /workspace/search-handoffs`, update/tombstone events, source-owner diagnostics, and search replay refs.
  - Validation: Tests prove Search indexes only allowed fields under source policy and permission snapshots and receives revocation/tombstone refs before future visibility widens.

- **6.2 Implement AI assist request records.**
  - Design: Route permissioned writing, summarization, table work, document search, and edit proposals through Personal AI Assistant, AI Gateway Router, and Encrypted Docdex RAG Adapter with explicit context permissions.
  - Output: `ai_assist_request` schema, `POST /workspace/ai-assist`, context-source refs, route refs, proposal refs, usage refs, audit refs, and expiry.
  - Validation: Tests prove AI assist denies missing object permission, vault grants, source permissions, stale shares, private RAG scope, and revoked high-risk access.

- **6.3 Implement proposal apply and reject flow.**
  - Design: Treat AI output as proposals, not authoritative edits, and apply accepted proposals only through versioned user/role/policy approval with base-version checks.
  - Output: `POST /workspace/ai-assist/{request_id}/apply`, `POST /workspace/ai-assist/{request_id}/reject`, proposal state, applied version refs, rejection refs, and audit/replay refs.
  - Validation: Tests prove invalid proposals reject safely, applied proposals become normal workspace versions, and rejected proposal payloads follow short retention.

- **6.4 Implement AI context retention and privacy controls.**
  - Design: Apply 24-hour defaults for raw AI context bundles, private RAG snippets, and rejected proposal payloads, while preserving redacted audit refs for replay/privacy audit where allowed.
  - Output: Retention fields, cleanup jobs, sealed hold overrides, redacted audit refs, context grant revocation refs, and privacy audit handoffs.
  - Validation: Tests prove AI context cleanup removes raw private context on schedule and revocation blocks future context access without deleting required redacted audit evidence.

- **6.5 Implement assistant and native app handoff boundaries.**
  - Design: Let Personal AI Assistant, Mobile clients, Search Engine, Messaging Center, Wallet/Usage Center, SDK, CLI, Admin/Developer UI, and Central AI Stewardship Interface consume bounded refs without owning Workspace state.
  - Output: Handoff contracts, redaction classes, result refs, notification refs, usage refs, audit refs, and replay refs.
  - Validation: Tests prove downstream consumers cannot mutate workspace objects outside Workspace APIs and cannot receive raw private content beyond explicit permission scope.

## Phase 7: Import, Export, Portability, Mobile Drafts, And Offline Sync

### Work Items

- **7.1 Implement import/export job records.**
  - Design: Create import/export jobs with format, object refs, redaction/export profile, Overstore bundle refs, failure refs, delivery refs, state, and replay evidence.
  - Output: `import_export_job` schema, `POST /workspace/import-jobs`, `POST /workspace/export-jobs`, delivery refs, cancellation/retry states, and BLAKE3 export manifests.
  - Validation: Tests prove export/import requires live authorization, redaction profiles, idempotency keys, audit refs, and cannot bypass vault grants or public-link policy.

- **7.2 Implement format boundaries and portability.**
  - Design: Keep Overrid-native formats canonical while supporting Markdown/plain text, CSV/TSV, HTML, PDF, and later OOXML/ODF/legacy formats as conversion jobs with manifests and no executable macros.
  - Output: Format mapping registry, conversion manifests, unsupported-format errors, redaction behavior, attachment-conversion refs, and import-review drafts.
  - Validation: Tests reject macro-bearing packages, proprietary-only canonical state, unsafe embedded content, conversion without manifest, and export that hides version history where policy requires it.

- **7.3 Implement mobile draft and offline sync contracts.**
  - Design: Allow mobile clients to cache authorized metadata/content refs, create local personal draft objects, and record local text-block/comment/table-cell patches against a known base version.
  - Output: Mobile draft schema, offline snapshot schema, sync cursor contracts, compact deltas, local patch envelopes, and Mobile Backend Gateway examples.
  - Validation: Tests prove offline mode cannot apply authoritative Overbase changes, create/narrow/expand shares, issue public links, restore/delete/archive objects, export private bundles, run AI assist, update search handoffs, or read newly private Overvault material while offline.

- **7.4 Implement reconnect revalidation and conflict handling.**
  - Design: Revalidate device/session credentials, tenant/org role, workspace permission, object state, base version, vault grants, retention policy, and idempotency before accepting mobile patches.
  - Output: Reconnect validation flow, conflict branch records, import-review drafts, stale/revoked permission errors, and replay evidence.
  - Validation: Tests prove revoked or stale permissions fail closed and version drift creates conflict branches or review drafts rather than silently merging.

- **7.5 Implement SDK, CLI, admin, and client surfaces.**
  - Design: Add generated client bindings and operator/developer views for workspace CRUD, object refs, editor sessions, versions, shares, import/export, search handoffs, AI assist, mobile sync, replay, diagnostics, and redaction testing.
  - Output: Rust SDK bindings, TypeScript generated bindings where appropriate, CLI read/debug commands, admin support views, client examples, and documentation notes.
  - Validation: Tests prove client surfaces call Workspace APIs, preserve stable errors, avoid privileged backdoors, and keep TypeScript as a client layer only.

## Phase 8: Storage/Vault Integration, Usage Emission, Audit, Replay, And Operations

### Work Items

- **8.1 Implement Overbase, Overstore, and Overvault integration.**
  - Design: Store structured workspace state in Overbase, large assets/bundles through Overstore refs, and private/sensitive material through Overvault grants and encrypted refs.
  - Output: Service integration adapters, storage/vault ref validators, grant checks, repair refs, tombstone refs, and replay hooks.
  - Validation: Tests prove Workspace cannot store raw large binary objects or secret material directly and denies private reads/edits/exports/search/AI until required vault grants are valid.

- **8.2 Implement Workspace usage refs.**
  - Design: Emit usage refs for workspace/object creation, reads, edits, coediting sessions, version snapshots, comments, share changes, search handoffs, AI assist routes, context retrieval, imports, exports, storage, bandwidth, sync, replay, and retention.
  - Output: `workspace_usage_ref` schema, Overmeter handoff events, wallet receipt refs, source operation classes, and pending reconciliation states.
  - Validation: Tests prove every billable or material Workspace operation emits or reconciles usage refs without creating hardcoded prices, balances, invoices, resource rates, or ledger entries.

- **8.3 Implement Overwatch audit exports.**
  - Design: Emit append-only audit refs for mutating and sensitive reads, share/public-link changes, revocations, AI context access, exports/imports, permission denials, source-service failures, and redaction decisions.
  - Output: Audit event schemas, redacted audit exports, support-safe summaries, reason codes, and replay links.
  - Validation: Tests prove audit records are tenant-scoped, actor-scoped, redacted, and complete enough for support/compliance without exposing raw document content or vault internals.

- **8.4 Implement replay endpoints and bundles.**
  - Design: Reconstruct workspace, object, edit, version, share, search, AI, import/export, comment, usage, retention, redaction, and offline-sync decisions from stored refs.
  - Output: `GET /workspace/replay/{record_id}`, replay bundle schema, fixture traces, redaction snapshots, source refs, and deterministic hashes.
  - Validation: Tests prove replay reconstructs decisions deterministically and excludes unauthorized private payloads, secret-bearing refs, private AI context, direct messages, and payment data.

- **8.5 Implement observability metrics and operations runbooks.**
  - Design: Track workspace/object counts, edit latency, coedit conflicts, version snapshot latency, revocation invalidation latency, search index lag, AI assist latency, export/import duration, usage emission status, and replay backlog.
  - Output: Metrics schema, alert rules, dashboard refs, degraded-state summaries, runbook docs, owner-service escalation refs, and redaction-safe support scripts.
  - Validation: Tests and drills prove alerts fire for unauthorized access attempts, stale vault grants, private search indexing, AI context permission violations, export failures, version conflict spikes, public-link abuse, missing usage refs, and replay gaps.

## Phase 9: Failure Recovery, Retention, Compliance Holds, Threat Review, And Governance Hardening

### Work Items

- **9.1 Implement failure recovery flows.**
  - Design: Preserve conflict refs, pending search updates, proposal-unavailable states, export failure refs, import review state, pending usage reconciliation, and downstream invalidation retries without widening access.
  - Output: Recovery state model, retry queues, owner-service refs, user-visible messages, audit events, and replay refs.
  - Validation: Tests simulate edit conflict, vault grant missing, share invalidation failure, search outage, AI route outage, invalid proposal, export failure, import conflict, and usage emission failure.

- **9.2 Implement retention classes and compliance holds.**
  - Design: Apply retention for active objects, named versions, published snapshots, restore refs, comments, approvals, share changes, AI proposal refs, import/export manifests, autosaves, cursors, draft patches, tombstones, deleted objects, public links, and private AI context.
  - Output: Retention schema, hold refs, expiry refs, cleanup jobs, export tombstones, redaction behavior, and replay preservation rules.
  - Validation: Tests prove retention cleanup stops serving deleted objects immediately, preserves required tombstone/audit/hold refs, expires raw private AI context, and respects Overclaim/Compliance Boundary/incident holds.

- **9.3 Implement public-link, export, and AI context hardening.**
  - Design: Add Phase 13 guardrails for public-link abuse, export leakage, import poisoning, AI context leakage, rejected proposal retention, redaction bypass, hidden training, and proprietary lock-in drift.
  - Output: Boundary assertions, deny tests, compliance checklist, support scripts, and documentation notes.
  - Validation: Tests and reviews reject unbounded private links, export without redaction, executable macros, hidden AI training, unsupported retention bypass, and canonical proprietary-only formats.

- **9.4 Implement threat-model and security-review gates.**
  - Design: Add Phase 13 gates for workspace membership confusion, share revocation delay, vault grant bypass, object permission bypass, offline sync replay abuse, editor conflict loss, AI proposal misuse, search handoff overexposure, import/export leakage, public-link abuse, retention gaps, usage gaps, audit gaps, and replay gaps.
  - Output: Threat model entries, security review checklist, remediation issue templates, accepted-risk records, and validation fixtures.
  - Validation: Review proves each risk has mitigation, tests, monitoring, or explicit accepted risk before broad release.

- **9.5 Implement public reporting and governance summaries.**
  - Design: Produce aggregate/redacted reports for workspace usage, share/revocation behavior, public-link outcomes, export/import health, AI assist permission behavior, retention cleanup, privacy/audit access, incident trends, usage reconciliation, and replay health.
  - Output: Reporting schema, redaction profiles, source refs, public-safe summaries, governance handoff refs, and report replay refs.
  - Validation: Tests prove reports are specific enough for trust while excluding private workspace content, private table data, vault secrets, direct messages, raw AI context, and unrelated-user details.

## Phase 10: Validation, Link Alignment, Queue, Index, And Handoff Readiness

### Work Items

- **10.1 Validate sub-build plan structure.**
  - Design: Check title prefix, attached SDS link, ten phase headings numbered 1 through 10, five work items per phase, and Design/Output/Validation fields.
  - Output: Structure validation evidence for `docs/build_plan/sub_build_plan_075_workspace_office_suite.md`.
  - Validation: Scripted checks pass for phase count, work-item count, numbering, and required fields.

- **10.2 Validate cross-document alignment.**
  - Design: Confirm SDS, service catalog entry, master plan, crosswalk, Phase 12, Phase 13, progress doc, and tech-stack guardrails all agree that Workspace is Phase 12-first with Phase 13 hardening.
  - Output: Alignment checklist and updated backlinks across changed docs.
  - Validation: Local Markdown link checks pass and review finds no mismatch with master Phase 0 through Phase 13 order.

- **10.3 Validate stack and authority guardrails.**
  - Design: Scan changed docs for prohibited external product boundaries, conventional database/object-store/vault/queue/search authority drift, proprietary office lock-in, hidden AI training, blockchain/NFT language, pricing/revenue/customer-count assumptions, and Workspace-owned accounting/storage/vault/search/model/direct-message authority.
  - Output: Guardrail scan evidence and corrected wording where needed.
  - Validation: Matches are either absent or explicit negative-control lines rejecting the prohibited assumptions.

- **10.4 Validate Docdex retrieval, impact, and index state.**
  - Design: Use Docdex impact, symbols, diagnostics, search, DAG export, and targeted index refresh for the new plan and linked docs.
  - Output: Impact evidence, symbols/Markdown structure evidence, search result evidence, DAG export evidence, and updated index stats.
  - Validation: Docdex search for SDS #75 returns the new sub-build plan and backlinks; impact diagnostics remain empty; targeted index refresh succeeds.

- **10.5 Validate implementation handoff readiness.**
  - Design: Update queue/progress evidence and confirm builders can start with contracts, fixtures, workspace/object foundations, editor sessions, sharing, search/AI handoffs, import/export, mobile drafts, usage/audit/replay, retention, and hardening gates.
  - Output: Queue/progress update, blocker notes, validation command notes, and handoff summary.
  - Validation: `docdexd hook pre-commit --repo /Users/bekirdag/Documents/apps/overrid` passes; `docdexd run-tests --repo /Users/bekirdag/Documents/apps/overrid` result is recorded, including the known missing test-runner blocker if unchanged.
