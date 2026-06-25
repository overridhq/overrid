# SUB BUILD PLAN #74 - Wallet and Usage Center

Attached SDS: [SDS #74 - Wallet and Usage Center](../sds/native_apps/wallet_usage_center.md)

## Purpose

This sub-build plan turns SDS #74 into an implementation sequence for Wallet and Usage Center. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Wallet and Usage Center is the Phase 12 native account-visibility and permission-control app for ORU balances, usage dashboards, receipts, statements, grants, holds, refunds, provider earnings, app permissions, privacy audits, dispute handoffs, notifications, usage refs, audit refs, and replay projections. It does not own ORU balance truth, Seal Ledger entries, billing settlement, payment secrets, grant issuance, payout eligibility, resource-rate policy, final dispute outcomes, or usage measurement truth.

It must make ORU-first settlement understandable to users: bought ORU and earned ORU can be spent inside Overrid when policy allows, provider payout eligibility is a separate state, and apps/native services should not present separate in-system payment rails.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #74: Wallet and Usage Center](../sds/native_apps/wallet_usage_center.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering, operations, failure modes, validation, build breakdown, downstream handoffs, and resolved open-question decisions. |
| [Wallet and Usage Center service plan](../service_catalog/native_apps/wallet_usage_center.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, signed envelopes, trace ids, idempotency keys, stable errors, local stack, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant scopes, Overkey signing refs, Overgate ingress, Overregistry service records, Overwatch audit refs, and Overqueue command/event primitives. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy decisions, Overclaim dispute records, policy dry-runs, deny-by-default checks, evidence refs, stable reason codes, and replayable decisions. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies Overmeter rollups, ORU Account Service projections, Seal Ledger checkpoints, Overbill receipts/statements/refund refs, Overgrant grant refs, Overmark resource-card refs, and the rule that Wallet displays authoritative refs but never mutates accounting truth. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Supplies AI Gateway Router, Personal AI Assistant, encrypted RAG, adapter, SDK, CLI, admin, and runtime-bridge groundwork used for permissioned wallet explanations, dispute drafts, and bounded native-app tool handoffs. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase wallet/app state, Overstore export/artifact refs, Overvault private grants and sensitive permission refs, Universal Namespace refs, retention, backup/restore, and replay substrates. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Controls the first build point for Wallet and Usage Center and its first useful balance, usage, receipt, statement, permission, privacy audit, dispute, notification, usage, audit, and replay slice. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies formal wallet privacy, accounting-display, permission-revocation, statement/export, custody-boundary, compliance, incident response, threat review, public reporting, reliability, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #74 first build work aligned to master Phase 12, with Phase 13 as governance/security/compliance/reporting hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services and contracts, Tokio where bounded workers are needed, Axum/Tower/Hyper-style HTTP where a service boundary exists, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, TypeScript for client surfaces only, native Overrid service boundaries, and no conventional database/object-store/vault/search/social platform, Kubernetes-first, blockchain, NFT, speculative token, external payment processor, hardcoded pricing, revenue, customer-count, hidden profiling, private-ledger bypass, or mutable accounting authority drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 6, 8, 12, and 13 | Attach SDS #74, preserve Phase 12 as first build, record prerequisites, and freeze Wallet authority boundaries. |
| 2 | Master Phases 0, 1, 4, 5, 8, and 12 | Define Rust contracts, canonical schemas, lifecycle enums, event surfaces, stable errors, and deterministic fixtures. |
| 3 | Master Phases 1, 4, 5, and 12 | Implement authorized account selectors, balance projections, hold/grant/refund/correction display, stale markers, and source-checkpoint reads. |
| 4 | Master Phases 1, 4, 5, 8, 12, and 13 | Implement usage dashboards, receipt collections, statements, exports, redaction profiles, and source-service delivery refs. |
| 5 | Master Phases 1, 4, 5, 6, 8, 12, and 13 | Implement app permission inventory, narrowing, revocation routing, privacy audit views, fail-closed high-risk permission behavior, and owner-service handoffs. |
| 6 | Master Phases 4, 5, 6, 12, and 13 | Implement dispute handoffs, user-visible explanations, notification preferences, Personal AI Assistant wallet tools, and support-safe diagnostics. |
| 7 | Master Phases 6, 8, 12, and 13 | Implement mobile-friendly APIs, read-only offline snapshots, native-app permission/receipt handoffs, SDK/CLI/admin views, and client projections. |
| 8 | Master Phases 5, 8, 12, and 13 | Implement wallet usage refs, Overmeter handoffs, audit exports, replay bundles, observability, pending-usage reconciliation, and operations metrics. |
| 9 | Master Phases 4, 5, 8, 12, and 13 | Implement source outage recovery, ledger mismatch handling, retention, compliance holds, export expiry, redacted support flows, and Phase 13 hardening gates. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, accounting/privacy/permission boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Wallet and Usage Center uses Rust-first shared contracts and service-facing APIs for wallet profiles, account selectors, balance views, usage dashboards, receipt collections, statement/export jobs, app permission controls, revocation requests, privacy audit views, dispute handoffs, notification preferences, usage refs, audit refs, and replay bundles. TypeScript is acceptable for generated client bindings and native/web UI surfaces, but it must call Overrid APIs and must not become a privileged accounting or permission authority.
- Wallet profiles, account selectors, balance views, usage summaries, receipt collections, statement/export jobs, permission records, privacy audit views, dispute handoffs, notification prefs, events, fixtures, redaction profiles, retention classes, export manifests, and replay bundles use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf may be generated later only through the shared contract layer.
- Mutating APIs require signed actor/service envelopes, tenant/account/org/app scope where applicable, trace id, idempotency key, account/view authorization, owner-service refs, policy refs, redaction profile refs, reason codes, schema version, stable errors, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for statement/export manifests, receipt collections, source checkpoint displays, redacted audit bundles, replay bundles, fixture inputs, and deterministic comparisons.
- Structured state, balance truth, ledger entries, receipts/statements, grants, rollups, disputes, permissions, private grants, identity, tenancy, key status, policy, audit, usage, accounting refs, diagnostics, mobile snapshots, and replay must use native Overrid service boundaries such as Overbase, Overstore, Overvault, Overpass, Overtenant, Overkey, Overgate, Overguard, Overwatch, Overmeter, ORU Account Service, Seal Ledger, Overbill, Overgrant, Overmark, Overclaim, Personal AI Assistant, AI Gateway Router, Central AI Stewardship Interface, Mobile SDK, Mobile Backend Gateway, SDK, CLI, and Admin/Developer UI.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Elasticsearch, OpenSearch, Solr, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, external payment processors, speculative token mechanics, tradable-currency framing, hardcoded charges, hardcoded pricing, revenue forecasts, customer-count assumptions, raw card/payment secrets, vault secrets, fraud internals, provider-sensitive payout details, final accounting truth, final usage truth, final dispute outcomes, or direct balance/ledger/grant/refund/hold/payout mutation the Wallet boundary.
- Wallet may link to credit funding and provider payout flows, but it must present external rails as boundary flows and ORU as the internal payment medium for apps, native services, subscriptions, one-time charges, resource usage, and machine-to-machine calls.

## Phase 1: SDS Attachment, Phase 12 Scope, And Wallet Authority Boundary

### Work Items

- **1.1 Attach the build plan to SDS #74.**
  - Design: Link this document from the Wallet and Usage Center SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/native_apps/wallet_usage_center.md`, `docs/service_catalog/native_apps/wallet_usage_center.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #74 returns both the Wallet and Usage Center SDS and this sub-build plan.

- **1.2 Preserve master Phase 12 as the first build point.**
  - Design: Keep first implementation in Phase 12 because Wallet depends on identity, tenancy, policy, metering, accounting, storage/private refs, AI/tool rails, mobile rails, and audit foundations from earlier phases.
  - Output: Phase-gate note that Phases 0, 1, 4, 5, 6, and 8 supply prerequisites; Phase 12 builds the first useful wallet utility; Phase 13 hardens accounting-display, custody-boundary, permission, privacy, statement/export, incident, reporting, compliance, and scale.
  - Validation: Review proves the plan does not move Wallet into Phase 5 accounting ownership, Phase 8 storage ownership, Phase 13-only governance, payment processing, grant issuance, payout, or final dispute authority.

- **1.3 Freeze the Wallet ownership boundary.**
  - Design: Record that Wallet owns display preferences, account selectors, wallet-safe balance views, usage dashboards, receipt collections, statement/export requests, permission control records, revocation requests, privacy audit views, dispute handoff records, notification prefs, usage refs, audit refs, and replay projections.
  - Output: Ownership checklist for architecture, API, UI, implementation, operations, and review gates.
  - Validation: Review confirms Wallet does not own ORU projections, ledger entries, billing settlement, external payment rails, grant eligibility, payout eligibility, resource-rate policy, usage measurement truth, final claims, or source-service private grants.

- **1.4 Carry forward resolved SDS #74 decisions.**
  - Design: Preserve read-only offline snapshot rules, audience-classed redaction, immediate revocation for high-risk permissions, queued revocation only for low-risk cleanup, and dispute overlays that do not rewrite accounting truth.
  - Output: Resolved-decision checklist covering 30-second live staleness markers, 24-hour cached personal summaries, immutable receipt/statement expiry, audience redaction classes, immediate high-risk fail-closed revocation, queued low-risk revocation, contested/held/provisional overlays, and append-only accounting refs.
  - Validation: Review rejects offline budget/spend decisions, Wallet-owned refunds/corrections/releases, redaction bypass, delayed high-risk private-data revocation, and disputed totals that imply Wallet rewrote Seal Ledger or ORU truth.

- **1.5 Define upstream and downstream service boundaries.**
  - Design: Record how ORU Account Service, Seal Ledger, Overbill, Overgrant, Overmeter, Overclaim, Overmark, Overpass, Overtenant, Overkey, Overguard, Overwatch, native apps, Personal AI Assistant, Mobile SDK, Mobile Backend Gateway, SDK, CLI, Admin/Developer UI, and Central AI Stewardship Interface interact through refs.
  - Output: Boundary matrix naming allowed reads, owned writes, handoff refs, denied direct mutation, redaction rules, usage refs, audit refs, replay refs, support paths, owner-service finality, and downstream handoffs.
  - Validation: Review confirms each upstream/downstream service keeps its authority and Wallet exchanges refs/events rather than copying private internals or inventing canonical truth owned elsewhere.

## Phase 2: Contracts, Schemas, Events, Stable Errors, And Fixtures

### Work Items

- **2.1 Create the Wallet Rust contract module.**
  - Design: Add contract types for wallet profiles, account selectors, balance views, usage dashboards, usage line items, receipt collections, statement/export jobs, app permission controls, revocation requests, privacy audit views, dispute handoffs, notification prefs, usage refs, redaction profiles, retention classes, and stable errors.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, redaction-class enums, permission-risk enums, export-state enums, dispute-overlay enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from balance truth, ledger mutation, payment settlement, grant issuance, usage measurement, final claims, and source-service private grants.

- **2.2 Define canonical JSON and JSON Schema contracts.**
  - Design: Model every Wallet record with trace ids, idempotency keys where mutating, actor/account/org/app refs, tenant scope, state, source-service refs, policy refs, redaction profile refs, audit refs, reason codes, usage refs where applicable, and stable schema versions.
  - Output: JSON Schema files, valid examples, invalid examples, signed command examples, event examples, replay examples, export examples, revocation examples, privacy audit examples, and dispute overlay examples.
  - Validation: Schema tests reject records without required actor/account/source refs, state, trace id, idempotency key for mutations, policy refs, audit refs, redaction class, reason code, and schema version.

- **2.3 Define Wallet event and replay contracts.**
  - Design: Model profile update, balance view, usage view, receipt view, statement request/ready, permission listing, permission narrow/revoke, privacy audit, dispute handoff, notification update, usage emission, export, retention, and replay events without private payment, fraud, provider, unrelated-user, or source-service secret internals.
  - Output: Event schema set, replay bundle schema, redacted projection schema, BLAKE3 display hash rules, stable ordering, and fixture-backed event streams.
  - Validation: Tests prove events include necessary refs and reason codes while excluding card/payment secrets, vault secrets, provider-sensitive payout details, fraud internals, raw private app data, and unauthorized ledger internals.

- **2.4 Define stable error taxonomy.**
  - Design: Preserve SDS stable errors and add implementation-ready mapping for account visibility, stale projections, rollup unavailability, receipt visibility, statement/export scope, permission owner, revocation denial, privacy audit denial, invalid dispute targets, missing source service, and wallet state conflicts.
  - Output: Stable error registry, HTTP/API mapping, client-facing messages, support-safe diagnostics, retryability flags, redaction behavior, and replay refs.
  - Validation: Tests prove denials are deterministic, support-safe, tenant-safe, audience-safe, and replayable without exposing hidden account membership, fraud signals, provider internals, or private app data.

- **2.5 Create deterministic fixtures and golden traces.**
  - Design: Build fixtures for profile update, account list, balance view, stale checkpoint, usage dashboard, receipts, holds, grants, refunds, statements, exports, permission list, immediate revoke, queued revoke, privacy audit, dispute handoff, notification prefs, usage emission, retention, and replay.
  - Output: Fixture directory, canonical inputs, expected states, expected events, expected errors, BLAKE3 hashes, usage refs, audit refs, redacted projections, and replay bundles.
  - Validation: Fixture tests produce stable ids, hashes, lifecycle states, reason codes, usage refs, audit refs, redacted outputs, stale markers, denied counts, and replay output across repeated runs.

## Phase 3: Account Selectors, Balance Views, Holds, Grants, And Projection Reads

### Work Items

- **3.1 Implement authorized account selectors.**
  - Design: Create account selector reads for users, organizations, delegated roles, app owners, and service views through Overpass/Overtenant role refs and Overguard policy refs.
  - Output: `wallet_account_selector` schema, `GET /wallet/accounts`, viewer scope filters, redaction class selection, stale-state handling, and audit events.
  - Validation: Tests prove account lists are deny-by-default, role-scoped, tenant-scoped, redacted by audience class, and unable to reveal unrelated account existence.

- **3.2 Implement wallet-safe balance views.**
  - Design: Read ORU Account Service projections and Seal Ledger checkpoint refs, then render available/reserved/held/spent/earned/sponsored/refunded/corrected summaries without creating balance-changing state.
  - Output: `wallet_balance_view` schema, `GET /wallet/accounts/{account_id}/balances`, stale markers, checkpoint refs, source-service refs, and projection explanation records.
  - Validation: Tests prove balance views require authoritative source refs, mark stale projections, hide review-required mismatch details, and cannot create ledger entries or balance mutation commands.

- **3.3 Implement holds, grants, refunds, and corrections display.**
  - Design: Display active reservations, holds, grants, sponsored credits, refunds, corrections, provisional amounts, and release/correction refs from ORU Account Service, Overgrant, Overbill, Overclaim, and Seal Ledger.
  - Output: Holds/grants/refunds read model, `GET /wallet/accounts/{account_id}/holds`, overlay records, source refs, redaction rules, and user-visible explanation fields.
  - Validation: Tests prove Wallet distinguishes available balances from contested/held/provisional amounts and does not apply releases, refunds, corrections, or grant eligibility itself.

- **3.4 Implement source checkpoint reconciliation markers.**
  - Design: Add source checkpoint, rollup, statement, and ledger mismatch markers so users see stale or review-required state without exposing private ledger or fraud internals.
  - Output: Stale/checkpoint marker model, retry refs, review-required state, source-owner diagnostics refs, and support-safe messages.
  - Validation: Tests simulate unavailable ORU projections, Seal Ledger checkpoint mismatch, and partial source failure while preserving privacy and routing remediation to owner services.

- **3.5 Implement balance-view audit and usage hooks.**
  - Design: Emit audit and usage refs for account selector reads, balance reads, stale retries, and source-checkpoint inspections.
  - Output: Overwatch audit events, Overmeter usage refs, wallet receipt refs where applicable, and replay links for balance-view decisions.
  - Validation: Tests prove each balance/display operation has traceable audit and usage refs and that failed usage emission creates a pending reconciliation marker.

## Phase 4: Usage Dashboards, Receipts, Statements, Exports, And Redaction

### Work Items

- **4.1 Implement usage dashboard reads.**
  - Design: Read Overmeter rollups and source-service refs, grouping usage by actor, organization, account, app, service, native app, model route, storage, compute, network, data, operation class, and time window.
  - Output: `usage_dashboard` and `usage_line_item` schemas, `GET /wallet/accounts/{account_id}/usage`, filters, aggregation levels, partial-source states, and audit/usage refs.
  - Validation: Tests prove dashboards cite Overmeter/source refs, handle missing rollups as partial usage, and cannot invent usage truth or hidden pricing.

- **4.2 Implement receipt collections.**
  - Design: Read Overbill and source-service receipt refs, then display receipt status, refund/correction refs, claim refs, export refs, app/service refs, and redaction class.
  - Output: `receipt_collection` schema, `GET /wallet/accounts/{account_id}/receipts`, receipt pending states, dispute refs, refund/correction refs, and support-safe diagnostics.
  - Validation: Tests prove receipt views respect account visibility, hide provider-sensitive/fraud-sensitive internals, and preserve source refs for unavailable receipts.

- **4.3 Implement statement and export requests.**
  - Design: Create statement/export jobs that route authoritative statement creation to Overbill/source services while Wallet owns request state, redaction selection, delivery refs, expiry, and replay.
  - Output: `statement_export_job` schema, `POST /wallet/accounts/{account_id}/statements`, `GET /wallet/statements/{statement_id}`, cancellation/retry states, and export manifests.
  - Validation: Tests prove statements require live authorization, redaction profiles, source refs, idempotency keys, and Overwatch audit refs, and do not leak private ledger, provider, fraud, or app internals.

- **4.4 Implement audience-classed redaction profiles.**
  - Design: Encode user, organization administrator, delegated accountant, app owner, support, and steward redaction classes using source-service enforced policies.
  - Output: Redaction profile schema, profile fixtures, redacted examples, denied-field markers, and export-safe summaries.
  - Validation: Tests prove each audience sees only allowed projections, receipts, statements, usage, permission summaries, and dispute refs, with unrelated-user, provider-sensitive, fraud-sensitive, and secret-bearing details hidden.

- **4.5 Implement export delivery, expiry, and replay.**
  - Design: Add delivery refs, expiry refs, immutable receipt/statement refs, export manifest hashes, retry/cancel behavior, and replay for statement/export jobs.
  - Output: Export delivery records, BLAKE3 manifest hashes, Overstore artifact refs where applicable, expiry cleanup refs, and replay bundles.
  - Validation: Tests prove exports expire as configured, delivery retries are idempotent, replay reconstructs request scope and redaction decisions, and Wallet does not become an object-store or billing-settlement authority.

## Phase 5: Permission Inventory, Revocation, Narrowing, And Privacy Audit

### Work Items

- **5.1 Implement app and service permission inventory.**
  - Design: List app/service/tool/native-app permissions visible to the viewer from owning-service refs, Overpass roles, Overtenant scopes, Overvault grants where applicable, Overguard decisions, and Overwatch audit refs.
  - Output: `app_permission_control` schema, `GET /wallet/permissions`, permission class, scope, purpose, expiry, owner-service refs, revocation state, redaction class, and audit refs.
  - Validation: Tests prove permission views are viewer-scoped, source-service authoritative, redacted by audience class, and unable to expose private grants or unrelated app details.

- **5.2 Implement permission narrowing requests.**
  - Design: Route scope-narrowing requests to the owning service with signed actor envelopes, requested scope, policy refs, owner-service refs, idempotency key, pending state, and result refs.
  - Output: `POST /wallet/permissions/{permission_id}/narrow`, request records, owner-service command envelopes, pending/result states, stable errors, and audit events.
  - Validation: Tests prove Wallet records requests and outcomes but does not directly edit source-service private grants or app-specific access policy.

- **5.3 Implement permission revocation requests.**
  - Design: Apply immediate fail-closed routing for high-risk permissions and queued owner-service revocation only for low-risk cleanup-oriented permissions with expiry, pending state, and fail-closed escalation.
  - Output: `permission_revocation_request` schema, `POST /wallet/permissions/{permission_id}/revoke`, high-risk permission taxonomy, queued low-risk state, expiry, owner-service refs, and result refs.
  - Validation: Tests prove high-risk private-data, spend, credential, secret, AI/RAG, location/contact/message/workspace/private-media, payment/grant, push-sensitive, child/safety, and compromised permissions deny new privileged use immediately.

- **5.4 Implement privacy audit views.**
  - Design: Display app/service/tool access, context/data access refs, assistant/tool refs, permission refs, audit event refs, redaction class, and time window without exposing unrelated users or fraud internals.
  - Output: `privacy_audit_view` schema, `GET /wallet/privacy-audit`, filters, redacted audit summaries, source-service refs, and replay links.
  - Validation: Tests prove privacy audit views require authorization, preserve redaction, route disputes to source owners, and cannot expose raw private payloads or secret-bearing refs.

- **5.5 Implement permission and privacy replay.**
  - Design: Add replay bundles for permission listing, narrowing, revocation, owner-service results, privacy audit reads, and denied/private details.
  - Output: Replay schemas, audit refs, reason codes, source-service refs, redaction snapshots, and deterministic fixture traces.
  - Validation: Tests prove replay reconstructs visible decisions and denial reasons without recovering private payloads, vault secrets, hidden fraud signals, or unrelated-user data.

## Phase 6: Dispute Handoffs, Explanations, Notifications, And Assistant Tools

### Work Items

- **6.1 Implement dispute handoff records.**
  - Design: Create dispute handoffs for usage, receipts, holds, grants, refunds, statements, and permissions while routing final claims and remedies to Overclaim, Overbill, ORU Account Service, Seal Ledger, Overgrant, or source services.
  - Output: `wallet_dispute_handoff` schema, `POST /wallet/disputes`, draft/submitted/source_validated/claim_opened states, evidence refs, owner-service refs, and Overclaim refs.
  - Validation: Tests prove Wallet does not apply refunds, releases, corrections, or final balance changes and always links to append-only owner-service refs.

- **6.2 Implement dispute overlays on wallet summaries.**
  - Design: Display `disputed`, `hold_pending`, `held`, `awaiting_source`, `under_review`, `appeal_window`, and `finality_pending` overlays without rewriting source accounting records.
  - Output: Overlay model, balance/usage/receipt integration, claim deadlines, finality markers, contested amount/dimension display, and redaction behavior.
  - Validation: Tests prove available balance remains distinct from contested/held/provisional amounts and disputed records preserve original usage, receipt, grant, hold, and ledger refs.

- **6.3 Implement user-visible explanations.**
  - Design: Provide source-backed explanations for balances, holds, grants, refunds, corrections, receipts, permissions, privacy audit entries, disputes, stale markers, denied states, and source failures.
  - Output: Explanation schema, support-safe reason codes, source refs, redaction class, retry guidance, and replay links.
  - Validation: Tests prove explanations are understandable, deterministic, audience-safe, and do not expose private payment, fraud, provider, ledger, or app internals.

- **6.4 Implement notification preferences and alerts.**
  - Design: Add notification prefs for balance/hold/grant/refund/receipt/statement/permission/privacy/dispute events with thresholds, quiet hours, delivery route refs, and source-service constraints.
  - Output: `wallet_notification_pref` schema, `PATCH /wallet/profile` support, alert templates, delivery refs, quiet-hour enforcement, and audit/usage refs.
  - Validation: Tests prove notification changes are idempotent, scoped, revocable, redacted, and do not leak sensitive details through delivery channels.

- **6.5 Implement Personal AI Assistant wallet tools.**
  - Design: Allow the assistant to summarize usage, explain receipts, suggest permission cleanup, and draft disputes only through explicit user permission, Wallet tool refs, AI Gateway Router route refs, redaction classes, expiry, usage refs, and replay.
  - Output: Assistant tool permission contracts, wallet summary refs, dispute draft refs, permission-cleanup proposal refs, route refs, and audit refs.
  - Validation: Tests prove assistant actions are read/proposal oriented unless explicitly approved, cannot mutate accounting truth, and honor permission revocation immediately for high-risk contexts.

## Phase 7: Mobile, Offline Read Models, Client Surfaces, And Native App Handoffs

### Work Items

- **7.1 Implement mobile-friendly Wallet APIs.**
  - Design: Add compact balance, usage, receipt, permission, privacy, notification, statement, and dispute endpoints through Mobile Backend Gateway and Mobile SDK while preserving signatures, trace ids, redaction, usage, and replay.
  - Output: Mobile response contracts, sync cursors, compact deltas, pagination, offline-safe markers, SDK bindings, and API examples.
  - Validation: Tests prove mobile APIs use normal Overrid rails and do not bypass Overgate, Overguard, source-service permissions, or Wallet redaction.

- **7.2 Implement read-only offline snapshots.**
  - Design: Allow offline snapshots only for previously authorized account selectors, last successful balance projections, hold/grant/refund summaries, receipt/statement refs, permission inventory, notification prefs, and privacy-audit summaries.
  - Output: Offline snapshot schema, stale markers, 30-second live source checkpoint rule, 24-hour cached personal summary limit, export/cache expiry rules, and reconnect validation hooks.
  - Validation: Tests prove offline mode cannot perform budget prechecks, spend decisions, statement issuance, dispute submission, permission expansion, or accounting-changing actions without live revalidation.

- **7.3 Implement native app handoff contracts.**
  - Design: Let native apps submit or reference usage, receipts, permission refs, privacy audit refs, and revocation results to Wallet without transferring source-service ownership.
  - Output: Native app handoff schema, source-service refs, wallet receipt refs, permission-control refs, redaction class, usage refs, and replay refs.
  - Validation: Tests prove native apps can link to Wallet projections while their app-owned data, permissions, private grants, and side effects remain in owning services.

- **7.4 Implement SDK, CLI, and admin views.**
  - Design: Add generated client bindings and operator/developer views for account selectors, balance/usage/receipts, statements, permissions, privacy audit, disputes, replay, diagnostics, and redaction testing.
  - Output: Rust SDK bindings, TypeScript generated bindings where appropriate, CLI read/debug commands, admin support views, and documentation examples.
  - Validation: Tests prove client surfaces call Wallet APIs, preserve stable errors, avoid privileged backdoors, and keep TypeScript as a client layer only.

- **7.5 Implement Central AI Stewardship and reporting handoffs.**
  - Design: Expose aggregate stewardship, surplus/donation, usage-health, and redacted privacy/compliance refs to Central AI Stewardship Interface through accounting/governance services rather than wallet-private records.
  - Output: Aggregate handoff refs, redaction profiles, stewardship-safe summaries, owner-service refs, and report-source manifests.
  - Validation: Tests prove stewardship views cannot read private wallet records directly and public reporting uses aggregate/redacted records only.

## Phase 8: Accounting Integration, Usage Emission, Audit, Replay, And Operations

### Work Items

- **8.1 Implement Wallet usage refs.**
  - Design: Emit usage refs for balance reads, usage dashboard reads, receipt reads, statement/export jobs, permission list/narrow/revoke actions, privacy audit views, dispute handoffs, notification delivery, replay, compute, storage, and bandwidth.
  - Output: `wallet_usage_ref` schema, Overmeter handoff events, wallet receipt refs, source operation classes, and pending reconciliation states.
  - Validation: Tests prove every billable or material Wallet operation emits or reconciles usage refs without creating charges, resource rates, balances, or ledger entries.

- **8.2 Implement Overwatch audit exports.**
  - Design: Emit traceable audit refs for mutating and sensitive reads, including account/statement/permission/privacy/dispute operations, source-service failures, redaction decisions, and owner-service handoffs.
  - Output: Audit event schemas, redacted audit exports, support-safe summaries, reason codes, and replay links.
  - Validation: Tests prove audit records are append-only, tenant-scoped, actor-scoped, redacted, and complete enough for support and compliance without exposing private internals.

- **8.3 Implement replay endpoints and bundles.**
  - Design: Reconstruct wallet profile, balance view, usage view, receipt collection, statement/export, permission, revocation, privacy audit, dispute, notification, usage, and redaction decisions from stored refs.
  - Output: `GET /wallet/replay/{record_id}`, replay bundle schema, fixture traces, redaction snapshots, source refs, and deterministic hashes.
  - Validation: Tests prove replay reconstructs decisions deterministically and excludes unauthorized private payloads, secret-bearing refs, fraud internals, and provider-sensitive details.

- **8.4 Implement observability metrics and alerts.**
  - Design: Track balance view latency, projection staleness, usage-rollup latency, receipt lookup failures, export duration, revocation success/failure, privacy audit volume, dispute volume, source-service failures, usage emission status, and replay backlog.
  - Output: Metrics schema, alert rules, dashboard refs, SLO notes, degraded-state summaries, and operator diagnostics.
  - Validation: Tests and drills prove alerts fire for stale projections, missing checkpoints, unexpected held display, revocation failures, export failures, audit denials, dispute handoff failures, missing usage refs, and replay backlog.

- **8.5 Implement operations runbooks.**
  - Design: Add runbooks for source outage, ledger mismatch, rollup missing, receipt unavailable, owner-service revocation outage, statement/export failure, invalid dispute target, pending usage emission, and replay failures.
  - Output: Runbook docs, owner-service escalation refs, retry/cancel guidance, redaction-safe support scripts, and incident linkage.
  - Validation: Review proves each failure path names an owner service, user-visible behavior, retry behavior, audit refs, usage refs, and no unauthorized direct correction by Wallet.

## Phase 9: Failure Recovery, Compliance Holds, Retention, And Governance Hardening

### Work Items

- **9.1 Implement source outage recovery flows.**
  - Design: Preserve last authorized views with stale markers, partial usage states, receipt pending states, queued low-risk revocations, and source-owner retry refs when ORU, Seal Ledger, Overmeter, Overbill, Overgrant, Overclaim, or permission owners are unavailable.
  - Output: Recovery state model, retry queues, stale markers, owner-service refs, user-visible messages, and audit events.
  - Validation: Tests prove outage recovery fails closed for sensitive permissions and accounting-changing actions while preserving read-only authorized context.

- **9.2 Implement compliance holds and retention classes.**
  - Design: Add retention classes for wallet profiles, views, receipts, statements, exports, permission history, privacy audits, disputes, notifications, usage refs, redacted audit summaries, and compliance holds.
  - Output: Retention schema, hold refs, expiry refs, cleanup jobs, export tombstones, redaction behavior, and replay preservation rules.
  - Validation: Tests prove retention cleanup does not delete source accounting truth, sealed holds preserve required refs, and private cached snapshots expire according to policy.

- **9.3 Implement payment and custody boundary hardening.**
  - Design: Add explicit guardrails that Wallet cannot store payment secrets, settle payments, issue ledger entries, calculate taxes, mutate balances, grant credits, release holds, execute payouts, or expose ORU as speculative token/currency mechanics.
  - Output: Boundary assertions, deny tests, compliance checklist, support scripts, and documentation notes.
  - Validation: Tests and reviews reject direct balance/ledger/grant/refund/hold/payout/payment-secret mutations and reject blockchain/NFT/speculative-token/per-transaction-fee framing.

- **9.4 Implement threat-model and security-review gates.**
  - Design: Add Phase 13 gates for account visibility confusion, redaction bypass, stale balance misuse, permission revocation delay, assistant misuse, statement/export leakage, dispute overlay confusion, custody-boundary drift, usage gaps, audit gaps, and replay gaps.
  - Output: Threat model entries, security review checklist, remediation issue templates, accepted-risk records, and validation fixtures.
  - Validation: Review proves each risk has mitigation, tests, monitoring, or explicit accepted risk before broad release.

- **9.5 Implement public reporting and compliance summaries.**
  - Design: Produce aggregate/redacted reports for wallet usage, permission revocation behavior, privacy-audit access, statement/export health, dispute handoff outcomes, usage reconciliation, retention cleanup, and replay health.
  - Output: Reporting schema, redaction profiles, source refs, public-safe summaries, governance handoff refs, and report replay refs.
  - Validation: Tests prove reports are specific enough for trust while excluding private account details, payment-provider refs, fraud internals, provider-sensitive payout details, and unrelated-user data.

## Phase 10: Validation, Link Alignment, Queue, Index, And Handoff Readiness

### Work Items

- **10.1 Validate sub-build plan structure.**
  - Design: Check title prefix, attached SDS link, ten phase headings numbered 1 through 10, five work items per phase, and Design/Output/Validation fields.
  - Output: Structure validation evidence for `docs/build_plan/sub_build_plan_074_wallet_usage_center.md`.
  - Validation: Scripted checks pass for phase count, work-item count, numbering, and required fields.

- **10.2 Validate cross-document alignment.**
  - Design: Confirm SDS, service catalog entry, master plan, crosswalk, Phase 12, Phase 13, progress doc, and tech-stack guardrails all agree that Wallet is Phase 12-first with Phase 13 hardening.
  - Output: Alignment checklist and updated backlinks across changed docs.
  - Validation: Local Markdown link checks pass and review finds no mismatch with master Phase 0 through Phase 13 order.

- **10.3 Validate stack and authority guardrails.**
  - Design: Scan changed docs for prohibited external product boundaries, conventional database/object-store/vault/queue/search authority drift, speculative token language, hardcoded prices, revenue/customer-count assumptions, and Wallet-owned accounting mutation.
  - Output: Guardrail scan evidence and corrected wording where needed.
  - Validation: Matches are either absent or explicit negative-control lines rejecting the prohibited assumptions.

- **10.4 Validate Docdex retrieval, impact, and index state.**
  - Design: Use Docdex impact, symbols, diagnostics, search, DAG export, and targeted index refresh for the new plan and linked docs.
  - Output: Impact evidence, symbols/Markdown structure evidence, search result evidence, DAG export evidence, and updated index stats.
  - Validation: Docdex search for SDS #74 returns the new sub-build plan and backlinks; impact diagnostics remain empty; targeted index refresh succeeds.

- **10.5 Validate implementation handoff readiness.**
  - Design: Update queue/progress evidence and confirm builders can start with contracts, fixtures, account/balance reads, usage/receipt/statement reads, permissions/privacy/dispute flows, mobile/client surfaces, usage/audit/replay, and hardening gates.
  - Output: Queue/progress update, blocker notes, validation command notes, and handoff summary.
  - Validation: `docdexd hook pre-commit --repo /Users/bekirdag/Documents/apps/overrid` passes; `docdexd run-tests --repo /Users/bekirdag/Documents/apps/overrid` result is recorded, including the known missing test-runner blocker if unchanged.
