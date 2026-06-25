# SUB BUILD PLAN #84 - Overdesk Desktop Client

Attached SDS: [SDS #84 - Overdesk Desktop Client](../sds/native_apps/overdesk_desktop_client.md)

## Purpose

This sub-build plan turns SDS #84 into an implementation sequence for Overdesk, the installable desktop front face for Overrid. It stays aligned with the master build plan, the service catalog, the SDS layer, the Overdesk page map, and the accepted Rust-first tech stack.

Overdesk is a desktop client shell, node-onboarding helper, resource-rule editor, Overrid browser, native-app host, wallet/credit entry point, owned-app dashboard, deployment wizard, Overasset inventory view, and desktop product surface for workspace, directory listings, app catalog, identity/profile, namespace management, privacy, vault, Docdex/RAG indexes, disputes, provider payouts, grants, activity receipts, node fleets, developer tools, release/rollback, and governance. It must stay an ordinary Overrid client. It never becomes the authority for identity, policy, node capability, scheduling, usage measurement, accounting, app records, namespace truth, deployment finality, search ranking, messaging truth, map truth, AI routing, vault secrets, RAG context, payout truth, release truth, governance truth, or asset ownership.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #84: Overdesk Desktop Client](../sds/native_apps/overdesk_desktop_client.md) | Controls product scope, page model, actors, dependencies, owned state, APIs, events, workflows, security, metering, offline behavior, validation, and resolved open-question decisions. |
| [Overdesk implementation plan](../service_catalog/native_apps/overdesk_desktop_client.md) | Controls service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Overdesk page map](../overdesk/page_map.md) and [UI/UX rules](../overdesk/ui_ux_rules.md) | Control the first desktop information architecture, navigation groups, page states, interaction rules, privacy behavior, and dense utility-first UI expectations. |
| [Master build plan](master_plan.md) | Keeps Overdesk in the Phase 12 native application layer with earlier node, policy, accounting, namespace, deployment, AI/RAG, and public-abuse-control prerequisites plus Phase 13 hardening. |
| [Phase 2: Seed Private Swarm](phase_02_seed_private_swarm.md) | Supplies node installer, hardware discovery, benchmarks, provider profile, and seed resource pool primitives. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies leases, execution, raw usage, retry/cancel, and node operation behaviors that Overdesk displays. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard, dry-run previews, workload classes, verification, challenges, disputes, and stable denial reasons. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies wallet, credit-purchase, usage, receipt, balance, provider-earning, and accounting boundaries. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Supplies product-client integration, generated SDK discipline, AI/RAG adapters, and runtime bridge groundwork used by native surfaces. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase, Overstore, Overvault, Universal Namespace, Overmesh route resolution, and private-ref boundaries. |
| [Phase 9: Overpack Deployment Platform](phase_09_overpack_deployment_platform.md) | Supplies app deployment, package validation, deployment planning, release strategies, and rollback paths. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public-provider, fraud, reputation, and abuse-control constraints that desktop access must not bypass. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Controls the first full Overdesk product build as a native app shell over owner-service APIs. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies formal desktop privacy, support-bundle, signed-update, incident, compliance, threat-review, reporting, release, governance, reliability, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #84 discoverable in the numbered service set and tied to Phase 12 plus Phase 13 hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, generated contracts, TypeScript/web only for client/UI surfaces, canonical JSON plus JSON Schema where appropriate, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional cloud product boundary, blockchain, NFT, ad-tech, hardcoded pricing, revenue, or customer-count assumptions. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 6, 8, 12, and 13 | Attach SDS #84, freeze desktop-client ownership, preserve Phase 12 as the first full build point, and map hardening to Phase 13. |
| 2 | Master Phases 0, 1, 6, 8, 12, and 13 | Build the Tauri-style shell boundary, generated SDK surface, session scope, local encrypted cache, and diagnostics without creating privileged desktop authority. |
| 3 | Master Phases 2, 3, 4, 5, 12, and 13 | Build Add This Computer onboarding over Node Installer, Hardware Discovery, Overcell, Oververify, Overguard, Overregistry, Overwatch, and usage/accounting projections. |
| 4 | Master Phases 2, 3, 4, 5, 10, 11, 12, and 13 | Build resource sharing, access rules, private UUID grants, policy previews, provider safety, and public-provider abuse constraints. |
| 5 | Master Phases 1, 4, 6, 8, 11, 12, and 13 | Build Overrid Browser resolution and embedded native-app sessions over identity, policy, namespace, routing, owner-service, and public-abuse-control rails. |
| 6 | Master Phases 5, 8, 9, 12, and 13 | Build wallet, credits, owned-app, and Overasset surfaces over accounting, rights, deployment, storage, and namespace refs. |
| 7 | Master Phases 1, 4, 6, 8, 11, 12, and 13 | Build workspace, directory, catalog, identity, and namespace surfaces as permission-safe owner-service projections. |
| 8 | Master Phases 4, 5, 6, 8, 10, 11, 12, and 13 | Build privacy, vault, encrypted RAG, disputes, payouts, grants, and activity surfaces over owner-service grants, evidence, receipts, and replay refs. |
| 9 | Master Phases 7, 9, 12, and 13 | Build deployment, fleet, developer, release, rollback, and governance views over deployment, grid, recovery, incident, and stewardship services. |
| 10 | Master Phase 12 with Phase 13 hardening | Validate end-to-end desktop flows, offline/reconnect behavior, local security, signed packaging, support bundles, accessibility, docs links, and launch readiness. |

## Tech Stack Guardrails

- Overdesk should use a Tauri-style desktop shell as the default first implementation candidate: Rust at the privileged desktop boundary, generated Overrid SDK contracts for service calls, and TypeScript/web UI surfaces for the desktop experience. Electron or a fully separate native platform split requires explicit threat-model, resource-cost, and maintenance review before adoption.
- Overdesk calls Overgate-facing APIs and generated SDK bindings. It must not call databases, object stores, vaults, queues, ledgers, node internals, package stores, or owner-service private stores directly.
- Package APIs, local models, stable errors, replay bundles, and docs-facing examples should use canonical JSON plus JSON Schema where applicable. Compact Protobuf may appear only through the shared contract layer.
- Mutating actions require signed command envelopes, actor refs, tenant/org refs, device refs where relevant, trace id, idempotency key, schema version, source refs, policy refs where required, usage refs, stable reason codes, and audit/replay refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for request hashes, package refs, cache validation, fixture hashes, replay evidence, and deterministic comparison artifacts. BLAKE3 must not be described as encryption.
- Local storage must be encrypted where the platform supports it, scoped by account/tenant/device, bounded by retention class, user-clearable, and safe to discard without corrupting owner-service truth.
- Overdesk must not store raw private keys, seed phrases, payment secrets, vault secrets, decrypted private data, raw private messages, raw RAG context, precise location trails, raw social/media payloads beyond owner-service cache policy, fraud internals, provider-sensitive internals, or unnecessary analytics data.
- Planning and implementation must not introduce conventional ad-tech tracking, hidden telemetry, hidden fees, fake urgency, addictive engagement loops, blockchain, NFT, per-transaction fee mechanics, hardcoded prices, customer-count assumptions, or revenue projections.

## Phase 1: SDS Attachment, Product Charter, And Boundary Map

### Work Items

- **1.1 Attach the build plan to SDS #84.**
  - Design: Keep this plan reachable from the Overdesk SDS, service catalog plan, master build plan, Phase 12 plan, Phase 13 hardening plan, build-plan crosswalk, Overdesk page map, and tech-stack decision.
  - Output: Stable links between this file, `docs/sds/native_apps/overdesk_desktop_client.md`, `docs/service_catalog/native_apps/overdesk_desktop_client.md`, `docs/build_plan/master_plan.md`, `docs/build_plan/phase_12_native_application_layer.md`, `docs/build_plan/phase_13_governance_compliance_scale_hardening.md`, `docs/build_plan/service_catalog_alignment.md`, and `docs/overdesk/page_map.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #84 returns the SDS, service catalog plan, and this sub-build plan.

- **1.2 Preserve Phase 12 as the first full build point.**
  - Design: Keep full Overdesk delivery in Phase 12 because it depends on identity, tenant scope, signing, policy, metering, wallet, storage/private refs, namespace routes, native app APIs, deployment APIs, AI/RAG handoffs, and public-abuse controls.
  - Output: Phase-gate note that earlier master phases provide prerequisites while the first complete desktop product starts in Phase 12 and formal security/compliance/scale hardening continues in Phase 13.
  - Validation: Review confirms the plan does not move full desktop delivery before Phase 12 or formal compliance/security hardening before Phase 13.

- **1.3 Freeze Overdesk ownership boundaries.**
  - Design: Record that Overdesk owns shell state, local device profile state, route history, page view state, local encrypted caches, drafts, diagnostics, notifications, and embedded app session refs, while owner services own final truth.
  - Output: Ownership checklist covering shell, session, cache, onboarding drafts, resource/access drafts, browser state, app-session refs, wallet intents, deployment drafts, assets, privacy/RAG/dispute/payout/grant/activity views, fleet/developer/release/governance view state, and support bundles.
  - Validation: Checklist review rejects any local identity truth, node truth, scheduler truth, policy finality, ORU/Seal Ledger mutation, deployment finality, namespace truth, search/messaging/map/AI authority, vault/RAG secret ownership, payout truth, release truth, governance truth, or Overasset ownership truth.

- **1.4 Map the Overdesk page model before implementation.**
  - Design: Use the Overdesk page map and UI/UX rules to group routes by fast access, daily apps, wallet/ownership, network contribution, app operations, identity/privacy/data, governance, and system/help.
  - Output: Route inventory with page id, navigation group, owner service, primary users, permission class, cache class, live/stale/offline states, high-risk actions, and denial view.
  - Validation: Review confirms every SDS page has an owner-service mapping, empty/live/stale/offline/restricted/error states, deep-link behavior where allowed, and no marketing-style or hidden-authority page pattern.

## Phase 2: Desktop Shell, Session, Local State, And Diagnostics

### Work Items

- **2.1 Create the Overdesk desktop package boundary.**
  - Design: Establish the Tauri-style desktop package with Rust-side privileged boundary, generated Overrid SDK clients, TypeScript/web UI components, feature flags, local platform adapters, and build metadata.
  - Output: Product package scaffold, SDK binding module, platform adapter boundary, feature-state loader, app version metadata, update-channel placeholder, and deterministic desktop fixture set.
  - Validation: Build checks prove UI code cannot bypass the Rust/SDK boundary to access local filesystem, node-agent control, wallet, vault, RAG, or local-network authority directly.

- **2.2 Build shell navigation, address bar, command palette, and notifications.**
  - Design: Implement the global shell required by SDS #84 and the page map: persistent address bar, account/scope switcher, grouped navigation rail, notification center, command palette, status strip, and route denied/not-found surfaces.
  - Output: Shell route registry, navigation groups, account switcher shell slot, command palette actions, notification preference shell, activity/status indicators, and consistent denied-state components.
  - Validation: Every route has stable id, visible account/scope, source owner, permission state, live/cache/offline state, usage class, and high-risk action treatment.

- **2.3 Implement desktop session and account scope handling.**
  - Design: Use Overpass, Overtenant, Overkey, Overgate, and Overwatch refs for login, selected account, tenant/org scope, delegated scope, session refresh, logout, revoke-device, and restricted/offline states.
  - Output: `desktop_session`, account selector, credential-provider refs, session state machine, session diagnostics, scope-change invalidation, and revocation cleanup.
  - Validation: Tests reject stale sessions, wrong tenant, revoked device, unsupported app version, missing credential provider, expired scope, and restricted account state before mutation.

- **2.4 Implement local encrypted cache and redacted diagnostics.**
  - Design: Store shell prefs, non-secret session refs, route history, bookmarks, drafts, page view state, diagnostics, and support-bundle drafts by retention class while excluding raw sensitive payloads.
  - Output: Cache provider interface, retention policy, clear-cache flow, logout/revoke cleanup, corruption recovery, diagnostic event model, redactor, support-bundle preview, and export flow.
  - Validation: Cache and diagnostics tests prove secret material, payment secrets, raw private content, raw messages, raw RAG context, precise location trails, fraud internals, and provider-sensitive internals are never written to plain local files or default support bundles.

## Phase 3: Device Onboarding And Local Node Service Handoff

### Work Items

- **3.1 Build the Add This Computer system-check flow.**
  - Design: Guide users through OS, architecture, CPU, GPU, RAM, storage, network, virtualization/sandbox readiness, battery/thermal profile, disk-encryption visibility, local permissions, installer status, and selected account/tenant scope.
  - Output: `node_onboarding_flow`, system-check screen, platform requirement explanations, installer readiness state, tenant scope selection, unsupported-device denial state, and retry path.
  - Validation: A device cannot advance to install, benchmark, or registration when required platform, permission, session, tenant, installer, or compatibility facts are missing.

- **3.2 Integrate Node Installer, Hardware Discovery, and Benchmark Runner displays.**
  - Design: Treat Node Installer, Hardware Discovery, and Benchmark Runner as owner-service/local-agent boundaries that return desktop-safe projections rather than local truth owned by Overdesk.
  - Output: Installer handoff, hardware summary cache, benchmark display, capability refs, verification-readiness refs, missing-fact warnings, and redacted diagnostic hooks.
  - Validation: Tests prove partial hardware discovery is visible as incomplete, required capability gaps block sharing, and Overdesk does not invent node capability or benchmark truth locally.

- **3.3 Submit signed onboarding through normal Overrid rails.**
  - Design: Combine tenant scope, device refs, installer refs, capability refs, first sharing preset, access-rule draft, policy preview refs, and user confirmation into a signed onboarding command.
  - Output: Final review screen, signed onboarding command builder, Overgate/Overregistry/Overcell/Overguard/Oververify/Overwatch handoff, joined/pending/failed state view, and replay refs.
  - Validation: A computer is not marked joined until authoritative owner-service registration, policy, audit, and node-health refs are returned.

- **3.4 Build local device settings, update, pause, drain, and uninstall surfaces.**
  - Design: Let users inspect local service status, update channel, node-agent health, permissions, local logs, cache, pause/drain status, signed update status, and uninstall readiness without embedding Overcell in the UI process.
  - Output: Local Device Settings, Updates and Release Notes, service status projection, pause/drain request drafts, signed update handoff, uninstall handoff, and support-bundle links.
  - Validation: Update, pause, drain, restart, and uninstall actions route through Node Installer/Overcell owner boundaries and fail closed when signatures, session freshness, or policy checks are missing.

## Phase 4: Resource Sharing, Access Rules, And Provider Safety

### Work Items

- **4.1 Build Resource Sharing Rules.**
  - Design: Support master enable/disable, day/night schedules, exact hour windows, date ranges, resource percentages, CPU/GPU/RAM/storage/network caps, concurrent lease count, thermal/battery safety, idle-only rules, workload-class allow/deny, and pause windows.
  - Output: `resource_share_rule`, schedule editor, resource cap editor, workload-class controls, safety overrides, local conflict validation, and signed rule-update flow.
  - Validation: Overdesk never schedules workloads locally, and every rule mutation routes through owner services with policy refs, usage refs, stable reason codes, and audit refs.

- **4.2 Build Access Rules and private UUID grants.**
  - Design: Let providers allow institutions, organizations, users, tags, purpose tags, and opaque private UUIDs; add deny rules, expiry, periodic review, temporary blocks, and emergency stop behavior.
  - Output: `resource_access_policy`, institution/org/user/tag/purpose-tag allowlists, private UUID handling, deny-rule editor, expiry/review controls, masked display, and signed policy-update flow.
  - Validation: Private UUIDs are treated as sensitive grants, masked in UI, never logged raw, never reused across unrelated scopes, and blocked when tenant/provider/purpose/expiry scope is invalid.

- **4.3 Build policy dry-run and scheduling preview.**
  - Design: Preview Overguard, Purpose Tag Registry, Workload Classifier, Oververify, Fraud Control, Oversched, Overlease, Overmeter, and public-provider constraints before signed rule changes.
  - Output: Policy preview session, scheduling preview, allowed/denied/missing-prerequisite display, reason-code panel, safe-alternative suggestions, and replay links.
  - Validation: Denied, risky, under-specified, expired, or unsupported policies cannot be presented as active or enforceable.

- **4.4 Build provider dashboard, health, earnings, and emergency controls.**
  - Design: Show joined state, active leases, usage, projected earnings, holds, denials, incidents, suspicious workload reports, disputes, payout refs, update refs, pause/drain controls, and emergency stop.
  - Output: Provider dashboard widgets, node health summary, earnings projection view, dispute/report handoffs, pause/drain/emergency-stop flow, and Wallet/Provider Payout handoff.
  - Validation: Earnings and usage views cite Overmeter, Overmark, ORU, Seal Ledger, Overlease, Overcell, Overwatch, Overclaim, and Provider Payout refs; local projections are clearly marked when stale or cached.

## Phase 5: Overrid Browser And Native-App Session Host

### Work Items

- **5.1 Implement address-bar parsing and namespace resolution.**
  - Design: Accept `/hugo`, `overrid:/hugo`, namespace routes, app routes, document routes, message refs, search queries, and commands; normalize input and call Universal Namespace Service, Overmesh, Overgate, and Overguard.
  - Output: Address parser, namespace resolution client, route-state view, app-session precheck, not-found view, denied view, and Search/Directory fallback.
  - Validation: Route opening is denied when namespace, route, policy, identity, tenant, app-session, or owner-service checks fail.

- **5.2 Implement tabs, history, bookmarks, trust markers, and privacy drawers.**
  - Design: Keep browsing state local, account-scoped, retention-controlled, source-attributed, and clearable while making namespace owner refs, route refs, verification markers, and active identity visible.
  - Output: Tab model, history model, bookmark model, namespace info drawer, route trust markers, privacy drawer, external-link preview, and clear-browsing-data flow.
  - Validation: Clearing browsing data removes local state without changing namespace truth, owner-service truth, app records, or audit refs.

- **5.3 Implement embedded native-app session host.**
  - Design: Open native app pages through owner-service route contracts and isolate app session state from shell state, local node authority, wallet authority, vault authority, and broad filesystem access.
  - Output: `embedded_app_session`, native-app frame, permission gates, error boundary, app-local usage refs, replay links, app-open events, and denied-state handling.
  - Validation: Owner-service permissions are checked before previews, snippets, private context, exact location, wallet data, asset details, or AI/RAG context are displayed.

- **5.4 Host first daily-app sessions.**
  - Design: Support Messaging, Search, Personal AI Assistant, Social, Maps, Wallet, Workspace, Directory, Native App Catalog, Identity/Profile, Namespace Manager, Central AI Stewardship, and Overasset sessions through owner APIs.
  - Output: Route adapters, app-specific permission prompts, usage/audit ref capture, stale/offline state handling, and per-app diagnostic events.
  - Validation: Messaging truth, search ranking, AI routing, map/place truth, social post state, workspace document truth, directory listing truth, wallet truth, catalog truth, identity truth, namespace truth, stewardship truth, and asset truth remain in owner services.

## Phase 6: Wallet, Credits, Owned Apps, And Overasset

### Work Items

- **6.1 Integrate Wallet and Usage Center.**
  - Design: Provide account selector integration, balance projections, usage dashboards, receipts, holds, grants, refunds, corrections, statements, permissions, privacy audit, and disputes through wallet/accounting owner APIs.
  - Output: Wallet page host, usage/ref filters, receipt and statement views, permission cleanup entry points, dispute handoffs, usage refs, and stale-state handling.
  - Validation: Overdesk never creates ledger entries, mutates balances, settles payments, issues statements, or changes grants locally.

- **6.2 Build Buy Credits and payment handoff.**
  - Design: Create credit-purchase intents through Wallet/Overbill with ORU projection refs, Seal Ledger checkpoint display, payment-provider handoff state, failure/retry/refund display, and clear separation of external payment rails from internal ORU accounting.
  - Output: Credit purchase form, wallet precheck, purchase-intent state machine, external handoff view, receipt view, refund/cancel state, and support/replay refs.
  - Validation: Payment secrets are never stored locally, failed payments never credit the local wallet, and Overdesk never hardcodes payment providers or pricing.

- **6.3 Build Owned Apps dashboard and app detail pages.**
  - Design: Show app owner views for namespace routes, deployment state, credit usage, credit earnings, resource costs, visitors/source-safe analytics, incidents, disputes, support inbox refs, release timeline, package refs, and app assets.
  - Output: Owned-app list, filters, app detail, analytics export handoff, support inbox handoff, incident/dispute links, release/deployment links, and replay refs.
  - Validation: Visitor analytics are redacted and source-safe; accounting, usage, deployment, release, incident, support, and asset projections cite owner-service refs.

- **6.4 Build Overasset inventory and action drafts.**
  - Design: Display owned rights, capacity claims, service/app ownership, namespace-bound assets, delegations, transfer constraints, holds, disputes, expiry, evidence refs, and related apps/resources.
  - Output: Asset list, asset detail, action drafts, delegation/revocation/transfer/bind/export/dispute handoffs, warning states, and app/resource binding handoffs.
  - Validation: Overasset remains the authority for ownership, delegation, transfer, hold, dispute, and evidence state; Overdesk only drafts and displays allowed actions.

## Phase 7: Workspace, Directory, Catalog, Identity, And Namespace

### Work Items

- **7.1 Host Workspace and Office Suite surfaces.**
  - Design: Provide workspace switchers, document/table/page/folder/object views, recent/shared/offline-marked items, comments, approvals, versions, import/export, search handoffs, sharing/revocation controls, and AI assist prompts through Workspace owner APIs.
  - Output: Workspace route adapter, object list/detail views, share/revoke drafts, import/export handoffs, AI context grant prompts, usage views, and offline/read-only state markers.
  - Validation: Workspace objects, editor sessions, versions, permissions, comments, approvals, imports, exports, document truth, and AI proposal application remain owner-service authority.

- **7.2 Host Directory Listings and Native App Catalog.**
  - Design: Expose local/public listings, organization pages, category/purpose tags, locality scope, listing drafts, moderation/report states, catalog app details, install/open/pin/update/remove/review actions, and permission summaries without ad-trap ranking.
  - Output: Directory route adapter, listing draft flow, catalog route adapter, app detail projection, safety labels, search/map/messaging/social handoffs, and report/dispute links.
  - Validation: Directory and catalog owner services keep listing truth, catalog truth, ranking, moderation, app records, package state, releases, incidents, and policy finality.

- **7.3 Build Identity/Profile and Namespace Manager surfaces.**
  - Design: Display personal/organization/institution/app-owner profiles, handles, verified markers, sessions, devices, credential-provider status, delegated scopes, owned/requested/delegated names, route bindings, verification markers, disputes, and tombstones.
  - Output: Identity/Profile route adapter, Namespace Manager route adapter, profile/session/device action drafts, namespace create/bind/release/transfer/delegation/dispute handoffs, and route history views.
  - Validation: Overpass, Overtenant, Overkey, Universal Namespace Service, and owner profile services retain identity, membership, credential, session, profile, namespace, delegation, transfer, dispute, and tombstone truth.

- **7.4 Apply Overdesk UI/UX and state rules across these surfaces.**
  - Design: Ensure every page follows dense utility-first layout, contextual side panels, stable tables/lists/cards, explicit account scope, live/stale/offline markers, deep links where allowed, permission-denied explanations, and accessible controls.
  - Output: Page-state checklist, common page header and status strip components, action confirmation patterns, keyboard/focus rules, permission and replay side-panel patterns, and responsive dense-layout fixtures.
  - Validation: UI review rejects nested decorative cards, hidden authority, unclear account scope, overlapping text, color-only risk states, inaccessible controls, and cached data that looks authoritative.

## Phase 8: Privacy, Vault, RAG, Disputes, Payouts, Grants, And Activity

### Work Items

- **8.1 Build Privacy and Permissions Center.**
  - Design: Consolidate app, service, device, workspace, message, location, RAG, vault, directory, social/media, and node/provider grants with revoke, expire, narrow, review, renew, explain, denied-attempt, and retention behavior.
  - Output: Permission inventory, high-risk permission alerts, revocation/narrow/expire/renew drafts, data-use explanations, retention views, audit refs, and owner-service status.
  - Validation: Permission truth stays with owner services, and privacy cleanup suggestions never auto-revoke without user confirmation unless owner-service policy requires fail-closed action.

- **8.2 Build Overvault Secure Storage and Docdex/RAG Index Manager.**
  - Design: Display vault item refs, grants, sealed references, rotation, recovery, encrypted index refs, scope, freshness, allowed model refs, RAG grants, retrieval receipts, and redaction decisions without exposing raw secrets or RAG context.
  - Output: Vault route adapter, grant/revoke/rotate/reveal-request/recovery/export handoffs, RAG index route adapter, connect/sync/disconnect/grant/revoke/inspect-receipt handoffs, and usage/replay refs.
  - Validation: Overvault, Overkey, Encrypted Docdex RAG Adapter, Personal AI Assistant, AI Gateway Router, and owner data services retain encryption, grants, sealed refs, retrieval, model routing, and context-access truth.

- **8.3 Build Disputes, Provider Payouts, and Grants surfaces.**
  - Design: Provide case lists, evidence refs, receipt refs, remedy requests, owner-service messages, provider earning summaries, holds, payout refs, compliance refs, grant catalogs, applications, sponsored credits, public-interest project status, and stewardship refs.
  - Output: Disputes/Appeals route adapter, Provider Earnings/Payout Center, Grants/Public-Interest Projects view, dispute/appeal/payout/grant/evidence drafts, correction paths, and replay links.
  - Validation: Overclaim, Provider Payout Service, Overgrant, ORU, Seal Ledger, Overbill, Compliance Boundary, Central AI Service, Stewardship Reporting, and owner services keep case, payout, correction, grant, and funding truth.

- **8.4 Build Activity, Receipts, Replay, and export surfaces.**
  - Design: Aggregate redacted wallet receipts, credit purchases, node leases, provider rule changes, permission changes, app opens, deployments, releases, namespace changes, vault grants, RAG receipts, disputes, payouts, grants, governance actions, support exports, and security prompts.
  - Output: Activity timeline, filters, receipt export, replay view, service ref drawer, dispute/report handoff, local retention controls, and support-bundle links.
  - Validation: Overwatch and owner services retain event and receipt truth; Overdesk aggregation is role-redacted and does not leak protected facts across account, organization, service, or viewer-role boundaries.

## Phase 9: Deployment, Fleet, Developer, Release, And Governance

### Work Items

- **9.1 Build Deploy New App wizard.**
  - Design: Guide builders through source selection, manifest validation, resource estimation, policy dry-run, namespace selection, wallet/grant precheck, deployment graph, release strategy, final confirmation, launch monitor, and rollback request.
  - Output: `deployment_wizard_session`, source selector, manifest viewer, validation results, resource estimate, policy preview, namespace selector, wallet precheck, deployment graph preview, final signed submission, monitor, rollback request, and replay refs.
  - Validation: Invalid manifests, unsafe policies, insufficient wallet/grant state, unauthorized namespace use, missing release strategy, or unsupported package state cannot be signed.

- **9.2 Build Node Fleet Manager and Developer Console.**
  - Design: Manage provider fleet projections, node health, bulk rule/access drafts, staged updates, maintenance windows, tags, private UUID pools, projects, manifests, validation refs, policy dry-runs, local dev environments, logs, and test fixtures.
  - Output: Fleet list, node detail, bulk action drafts, developer project list, manifest viewer, validation results, policy preview, log/replay links, local-stack status, SDK/CLI links, and support-bundle handoffs.
  - Validation: Bulk node actions, package validation, developer diagnostics, test-environment actions, namespace drafts, and deployment previews remain signed owner-service requests or read-only projections.

- **9.3 Build Release and Rollback Manager.**
  - Design: Display release state, rollout state, health gates, backup/restore readiness, failover refs, rollback points, route state, namespace bindings, incidents, migration links, and blocked-release explanations.
  - Output: Release list, release detail, promote/pause/resume/rollback/route-shift/health-review drafts, backup/restore links, failover links, migration links, and incident handoffs.
  - Validation: Release, rollback, route shift, backup/restore, failover, migration, and health-gate truth remains with Release Strategy Service, Deployment Planner, Backup/Restore, Failover/Recovery, Overmesh, Compliance Boundary, Overwatch, Package Validator, and owner services.

- **9.4 Build Governance Center and public-trust views.**
  - Design: Expose PIPs, stewardship reports, central-AI recommendations, public-interest funding reports, compliance summaries, security review status, incident summaries, migration evidence, correction notices, comments, follows, appeals, and steward review queues.
  - Output: Governance dashboard, PIP/report views, central-AI recommendation views, security/compliance/incident views, comment/review/correction/appeal drafts, publication-review handoffs, and redaction profile display.
  - Validation: PIP Registry, Stewardship Reporting, Central AI Service, Central AI Stewardship Interface, Compliance Boundary, Threat Modeling/Security Review Tracker, Incident Response, Migration Tooling, and owner services retain governance, publication, incident, and compliance truth.

## Phase 10: Offline, Security, Packaging, Validation, And Handoff

### Work Items

- **10.1 Implement offline and reconnect behavior.**
  - Design: Allow safe drafts, read-only cached projections, local browsing state, support-bundle drafts, and queued low-risk requests while requiring live revalidation for authority-changing actions.
  - Output: Offline state reducer, reconnect validator, expiry handler, conflict view, queue policy, stale marker components, invalidation triggers, and local clear-cache behavior.
  - Validation: Spend, ledger changes, credit settlement, deployment finality, resource-rule authority, access-policy authority, asset transfer, public publication, messaging delivery finality, map correction, search index changes, AI execution, and provider payout changes cannot complete offline.

- **10.2 Harden desktop security and privacy.**
  - Design: Threat-model local storage, installer handoffs, embedded app sessions, web content, external links, payment handoffs, exact location, AI/RAG grants, support bundles, app updates, deeplinks, and local service controls.
  - Output: Threat model entries, mitigation checklist, secure defaults, content isolation rules, local permission rules, support-bundle redaction rules, signed-update verification, and Phase 13 security review package.
  - Validation: Security review blocks broad local file access, unrestricted webviews, unsafe deeplinks, unsigned updates, silent telemetry, raw secret storage, payment-secret storage, raw RAG context capture, and local node-agent authority in the UI process.

- **10.3 Package, release, update, rollback, and support Overdesk.**
  - Design: Build signed installers, update channels, rollback path, uninstall path, cache clear, session revoke, node pause/drain safety, accessibility support, localization hooks, help surfaces, and operator runbooks.
  - Output: Signed desktop package, updater, rollback test, uninstall flow, compatibility matrix, accessibility checklist, diagnostics/help docs, support runbook, and release notes.
  - Validation: Install, update, rollback, uninstall, clear-cache, session revoke, support-bundle export, node pause/drain, and help entry points pass on supported platforms.

- **10.4 Validate end-to-end readiness and documentation alignment.**
  - Design: Run link checks, schema/fixture checks, shell route coverage, owner-service boundary review, page map coverage, local cache/security review, and representative user journeys for users, providers, app owners, builders, institutions, support, and stewards.
  - Output: Validation evidence for Add This Computer, Resource Sharing, Access Rules, `/hugo` browse, native app opens, Wallet/Credits, Owned Apps, Deploy, Assets, Workspace, Directory, Catalog, Identity, Namespace, Privacy, Vault, RAG, Disputes, Payouts, Grants, Activity, Fleet, Developer, Releases, Governance, Offline, Updates, and Support Bundles.
  - Validation: Docs and implementation readiness checks prove the plan remains aligned with SDS #84, service catalog, master Phase 0 through Phase 13 order, Phase 12 native app layer, Phase 13 hardening, Overdesk page map, and `docs/overrid_tech_stack_choice.md`.

## Exit Gates

- Overdesk has a complete desktop shell route map that matches the SDS and Overdesk page map.
- Add This Computer onboards a computer through normal owner-service APIs without terminal work.
- Resource sharing and access rules are visible, previewable, signed, and enforceable by owner services.
- Private UUID access grants are scoped, masked, expiry-bound, and never logged raw.
- `/hugo` and other namespace inputs resolve through Universal Namespace Service and Overmesh or fail clearly with Search/Directory fallback.
- Wallet, Credits, Owned Apps, Deploy, Assets, Messaging, Search, AI, Social, Maps, Workspace, Directory, App Catalog, Identity, Namespace, Privacy, Vault, RAG, Disputes, Provider, Grants, Activity, Fleet, Developer, Releases, and Governance pages are reachable.
- No page mutates accounting, deployment, node, asset, search, messaging, map, AI, vault, RAG, payout, grant, release, governance, namespace, profile, credential, or identity truth directly.
- Offline mode supports safe drafts and cached reads only; reconnect revalidates before authority-changing actions.
- Local cache, diagnostics, support bundle, payment handoff, webview isolation, installer handoff, signed updater, rollback, and package paths pass Phase 13 security review.
- The plan remains aligned with the master Phase 0 through Phase 13 order, Phase 12 native app layer, Phase 13 hardening, service-catalog crosswalk, Overdesk page map, and accepted Rust-first tech stack.

## Downstream Handoff

After this plan exits, Overdesk becomes the primary desktop distribution surface for Overrid. Later work should deepen native app features, institution-managed desktop fleets, app-owner growth tooling that stays privacy-safe, encrypted RAG/assistant workflows, release/governance participation, richer offline support, and public launch hardening through Phase 13 governance, compliance, incident, reporting, reliability, and scale processes.
