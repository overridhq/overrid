# SUB BUILD PLAN #84 - Overdesk Desktop Client

Attached SDS: [SDS #84 - Overdesk Desktop Client](../sds/native_apps/overdesk_desktop_client.md)

## Purpose

This sub-build plan turns SDS #84 into an implementation sequence for Overdesk, the installable desktop front face for Overrid.

Overdesk is a desktop client shell, node-onboarding helper, resource-rule editor, Overrid browser, native-app host, wallet/credit entry point, owned-app dashboard, deployment wizard, and Overasset inventory view. It must stay an ordinary Overrid client. It never becomes the authority for identity, policy, node capability, resource scheduling, usage measurement, accounting, app records, namespace truth, deployment finality, search ranking, messaging truth, map truth, AI routing, or asset ownership.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #84: Overdesk Desktop Client](../sds/native_apps/overdesk_desktop_client.md) | Controls product scope, page model, actors, dependencies, owned state, APIs, events, workflows, security, metering, offline behavior, validation, and open questions. |
| [Overdesk implementation plan](../service_catalog/native_apps/overdesk_desktop_client.md) | Controls service-catalog objective, dependencies, development order, contracts, validation, and design alignment. |
| [Master build plan](master_plan.md) | Keeps Overdesk in the Phase 12 native application layer with earlier node/accounting/namespace/deployment prerequisites and Phase 13 hardening. |
| [Phase 2: Seed Private Swarm](phase_02_seed_private_swarm.md) | Supplies node installer, hardware discovery, benchmarks, provider profile, and seed resource pool primitives. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies leases, execution, raw usage, retry/cancel, and node operation behaviors that Overdesk displays. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies wallet, credit-purchase, usage, receipt, balance, and accounting boundaries. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase, Overstore, Overvault, Universal Namespace, and Overmesh route resolution. |
| [Phase 9: Overpack Deployment Platform](phase_09_overpack_deployment_platform.md) | Supplies app deployment, package validation, release strategies, and deployment planning. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Controls the first full Overdesk product build. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies desktop security review, compliance, incident, support-bundle, retention, update, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #84 discoverable in the numbered service set. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 6, 8, 12, and 13 | Establish desktop shell contracts, generated bindings, identity/session refs, namespace awareness, local encrypted cache, diagnostics, and security gates. |
| 2 | Master Phases 2, 3, 4, 5, 11, and 12 | Build Add This Computer onboarding, hardware discovery display, node health, resource sharing rules, access rules, and provider safety controls. |
| 3 | Master Phases 8, 12, and 13 | Build Overrid Browser address resolution, namespace refs, route refs, tabs, bookmarks, and fallback behavior. |
| 4 | Master Phases 5, 8, 12, and 13 | Embed native app pages, wallet, credits, owned apps, app analytics, and Overasset views through owner-service APIs. |
| 5 | Master Phases 9, 12, and 13 | Build Deploy New App wizard over package validation, policy dry-run, deployment planning, namespace binding, wallet precheck, and release strategy. |
| 6 | Master Phase 12 with Phase 13 hardening | Harden offline behavior, local encryption, support bundles, app updates, platform packaging, accessibility, and operational readiness. |

## Tech Stack Guardrails

- Overdesk is a desktop product and should prefer the accepted Rust-first Overrid contracts. A Tauri-style shell is the default candidate because it keeps Rust close to the client boundary; Electron or native platform implementations require explicit threat-model and resource-cost review.
- The desktop app calls generated SDK bindings and Overgate-facing APIs. It must not call databases, object stores, vaults, queues, ledgers, node internals, package stores, or owner-service private stores directly.
- Local storage must be encrypted where the platform supports it, bounded by retention class, and safe to clear.
- Signed actions require actor refs, tenant refs, device refs where relevant, trace id, idempotency key, schema version, policy refs where required, usage refs, and audit refs.
- Overdesk must not store raw private keys, seed phrases, payment secrets, vault secrets, raw private messages, raw RAG context, precise location trails, raw social media beyond owner-service cache policy, or unnecessary analytics data.
- No conventional ad-tech tracking, hidden telemetry, blockchain, NFT, per-transaction fee mechanics, hardcoded prices, customer-count assumptions, or revenue projections belong in this plan.

## Phase 1: Product Shell, Session, And Local State

### Work Items

- **1.1 Create Overdesk product package and route model.**
  - Design: Define app shell routes for Home, Browse, Messaging, Search, AI, Social, Maps, Wallet, Credits, Owned Apps, Deploy, Assets, Resource Sharing, Access Rules, and Settings.
  - Output: Route registry, navigation rail, top address bar, account switcher, notification center, command palette, and page permission gates.
  - Validation: Every route has a stable id, owner-service mapping, required permission class, usage class, and denied-state view.

- **1.2 Implement desktop session and account scope handling.**
  - Design: Use Overpass, Overtenant, Overkey, Overgate, and Overwatch refs for login, selected account, tenant/org scope, delegated scope, session refresh, logout, and revoke-device.
  - Output: `desktop_session`, account selector, credential-provider refs, session state machine, and redacted session diagnostics.
  - Validation: Tests reject stale sessions, wrong tenant, revoked device, unsupported app version, and missing credential provider before mutation.

- **1.3 Implement local encrypted cache interfaces.**
  - Design: Store shell prefs, non-secret session refs, route history, bookmarks, resource-rule drafts, access-rule drafts, deployment drafts, asset view cache, and diagnostics by retention class.
  - Output: Cache provider interface, retention policy, clear-cache flow, logout/revoke cleanup, and corruption recovery.
  - Validation: Cache tests prove secret material and private payloads are never stored in plain local files.

- **1.4 Implement redacted diagnostics and support bundle baseline.**
  - Design: Track stable ids, app version, OS, device refs, route failures, policy denials, installer errors, cache errors, deployment errors, and owner-service refs.
  - Output: Diagnostic event model, redactor, support-bundle preview, and export flow.
  - Validation: Support bundle review shows no secrets, payment data, raw private content, raw messages, raw RAG context, or precise location trails by default.

## Phase 2: Device Onboarding, Resource Sharing, And Access Rules

### Work Items

- **2.1 Build Add This Computer flow.**
  - Design: Guide the user through OS/system checks, installer status, Overcell state, Hardware Discovery output, benchmark readiness, tenant scope, and first sharing preset.
  - Output: `node_onboarding_flow`, installer handoff, capability summary, benchmark display, first sharing rule draft, and final confirmation.
  - Validation: A device cannot be marked joined until owner services return authoritative registration/audit refs.

- **2.2 Build Resource Sharing Rules.**
  - Design: Support day/night schedules, exact hour windows, date windows, resource percentages, CPU/GPU/RAM/storage/network caps, idle-only rules, thermal/battery safety, workload classes, pause, drain, and emergency stop.
  - Output: Rule editor, policy preview, scheduling preview, pause/drain actions, and rule state display.
  - Validation: Overdesk never schedules workloads locally and every rule change routes through owner services.

- **2.3 Build Access Rules.**
  - Design: Let providers allow institutions, organizations, users, tags, purpose tags, and private UUIDs; add deny rules, expiry, review, and dry-run previews.
  - Output: Access policy editor, private UUID handling, policy dry-run display, reason-code display, and audit refs.
  - Validation: Private UUIDs are treated as sensitive grants, never logged raw, and never reused across unrelated scopes.

- **2.4 Build provider health and earnings display.**
  - Design: Show this computer's joined state, leases, usage, projected earnings, policy denials, suspicious workload reports, disputes, and payout refs through owner-service projections.
  - Output: Provider dashboard widgets and wallet handoff.
  - Validation: Earnings and usage views cite Overmeter, ORU, Seal Ledger, Overmark, Overlease, Overcell, and payout refs.

## Phase 3: Overrid Browser And Native App Host

### Work Items

- **3.1 Implement address-bar resolution.**
  - Design: Accept `/hugo`, names, app routes, document routes, message refs, search queries, and commands; normalize input and call Universal Namespace Service, Overmesh, Overgate, and Overguard.
  - Output: Address parser, namespace resolution client, route-state view, not-found view, denied view, and search/directory fallback.
  - Validation: Route opening is denied when policy or app-session checks fail.

- **3.2 Implement tabs, history, bookmarks, and trust markers.**
  - Design: Keep browsing state local, retention-controlled, account-scoped, and source-attributed.
  - Output: Tab model, history model, bookmark model, namespace info drawer, privacy drawer, and route trust markers.
  - Validation: Clearing browsing data removes local state without changing namespace or owner-service truth.

- **3.3 Implement embedded native-app session host.**
  - Design: Open Messaging, Search, AI, Social, Maps, Wallet, Workspace, Directory, Central AI, and Overasset pages through owner-service route contracts.
  - Output: `embedded_app_session`, native-app frame, app permission gates, error boundary, app-local usage refs, and replay links.
  - Validation: Owner-service permissions are checked before previews, snippets, private context, exact location, wallet data, or asset details are displayed.

## Phase 4: Wallet, Credits, Owned Apps, And Assets

### Work Items

- **4.1 Integrate Wallet and Usage Center.**
  - Design: Provide balance projections, usage, receipts, grants, holds, statements, permissions, privacy audit, and disputes through Wallet owner APIs.
  - Output: Wallet page host, account selector integration, wallet usage refs, and permission cleanup entry points.
  - Validation: Overdesk never creates ledger entries or balance truth locally.

- **4.2 Build Buy Credits page.**
  - Design: Create credit-purchase intents through Wallet/Overbill with ORU projection refs and Seal Ledger checkpoint display.
  - Output: Credit purchase form, payment handoff state, receipt view, failure/retry/refund display, and wallet refresh.
  - Validation: Payment secrets are never stored locally and failed payments never credit the local wallet.

- **4.3 Build Owned Apps dashboard and app detail page.**
  - Design: Show owned apps, namespace routes, deployment state, credit usage, credit earnings, resource costs, visitors/source-safe analytics, incidents, disputes, and app assets.
  - Output: Owned-app list, filters, app detail view, analytics export handoff, support inbox handoff, and replay refs.
  - Validation: Visitor analytics are redacted and source-safe; accounting and usage projections cite owner-service refs.

- **4.4 Build Overasset inventory.**
  - Design: Display owned rights, capacity claims, app/service ownership, namespace-bound assets, delegations, transfer constraints, disputes, and expiry.
  - Output: Asset list, asset detail, action drafts, export handoff, dispute handoff, and app/resource binding handoff.
  - Validation: Overasset remains the authority for ownership, delegation, transfer, hold, and dispute state.

## Phase 5: Deploy New App Wizard

### Work Items

- **5.1 Build source selection and manifest validation.**
  - Design: Accept package, repo/import, template, existing version, or local build output and validate through Overpack and Package Validator.
  - Output: Source selector, manifest viewer, validation results, stable reason codes, and fixture packages.
  - Validation: Invalid manifests cannot advance to deployment planning.

- **5.2 Build resource estimation and policy dry-run.**
  - Design: Estimate compute, GPU, RAM, storage, bandwidth, data classes, budget, grants, compliance markers, and workload class using owner services.
  - Output: Resource estimate page, wallet precheck, Overguard dry-run, Workload Classifier output, and Package Validator output.
  - Validation: Unsafe, underfunded, unauthorized, or unsupported deployments cannot be signed.

- **5.3 Build namespace, release, and deployment plan flow.**
  - Design: Choose namespace route, app route, visibility, release strategy, health checks, backup/restore refs, rollback plan, and deployment graph.
  - Output: Namespace selector, release strategy selector, deployment graph preview, final confirmation, launch monitor, and rollback request.
  - Validation: Deployment finality comes only from Deployment Planner, Release Strategy Service, Overgate, Overguard, Overregistry, Overmeter, and owner-service refs.

## Phase 6: Offline, Security, Packaging, And Release

### Work Items

- **6.1 Implement offline and reconnect behavior.**
  - Design: Allow safe drafts and read-only cached projections; revalidate all authoritative changes on reconnect.
  - Output: Offline state reducer, reconnect validator, expiry handling, and local-conflict views.
  - Validation: Spend, ledger changes, deployment finality, asset transfer, public publication, or resource-rule authority cannot complete offline.

- **6.2 Harden desktop security.**
  - Design: Threat-model local storage, installer handoffs, embedded app sessions, web content, external links, payment handoffs, exact location, AI context grants, support bundles, and app updates.
  - Output: Threat model entries, mitigations, secure defaults, and Phase 13 review checklist.
  - Validation: Security review blocks broad local file access, unrestricted web views, silent telemetry, unsafe deeplinks, raw secret storage, and unsigned updates.

- **6.3 Package and release Overdesk.**
  - Design: Build signed installers, update channel, rollback path, platform permissions, local service management, accessibility checks, and release notes.
  - Output: Signed desktop package, updater, rollback test, compatibility matrix, and operator runbook.
  - Validation: Install, update, rollback, uninstall, cache clear, session revoke, and node pause/drain all pass on supported platforms.

## Exit Gates

- Overdesk adds a computer to the network through normal owner-service APIs.
- Resource sharing and access rules are visible, previewable, signed, and enforceable by owner services.
- `/hugo` and other namespace inputs resolve through Universal Namespace Service and Overmesh or fail clearly.
- Wallet, Credits, Owned Apps, Deploy, Assets, Messaging, Search, AI, Social, and Maps pages are reachable.
- No page mutates accounting, deployment, node, asset, search, messaging, map, or AI truth directly.
- Local cache, diagnostics, support bundle, payment handoff, and updater paths pass security review.

## Downstream Handoff

After this plan exits, Overdesk becomes the primary desktop distribution surface for Overrid. Later work should add deeper native app features, institution-managed desktop fleets, app-owner growth tooling that stays privacy-safe, richer offline support, and public launch hardening through Phase 13 governance, compliance, incident, reporting, and scale processes.
