SDS #84

# Overdesk Desktop Client SDS

## Purpose

Build Overdesk, the installable desktop front face for Overrid.

Overdesk is the end-user desktop client that lets a person add a computer to the Overrid network, decide how much of that computer can be used, choose who may use those resources, browse Overrid addresses such as `/hugo`, open native Overrid apps, buy credits, manage the wallet, deploy apps, inspect owned apps, view owned Overasset records, use productivity and public-utility apps, manage identity and namespace surfaces, control privacy and encrypted storage, connect encrypted Docdex/RAG indexes, review disputes and payouts, inspect activity receipts, manage node fleets, use developer tools, control releases, and participate in governance.

Overdesk owns desktop shell state, local device profile state, resource sharing preferences, access-rule drafts, embedded native-app sessions, address-bar history, app-owner dashboard preferences, deployment wizard state, credit-purchase intents, notification preferences, surface-specific view state, and local encrypted caches. It does not own node-agent truth, policy finality, ORU balance truth, Seal Ledger entries, app business data, search ranking truth, messaging truth, map truth, AI route finality, workspace document truth, directory listing truth, namespace truth, vault secret truth, payout truth, governance truth, release authority, or asset ownership truth.

## Product Name

- Product name: Overdesk
- Formal service name: Overdesk Desktop Client
- Service number: SDS #84
- Product position: the desktop entry point for regular users, resource providers, app owners, and builders

The name is intentionally short, practical, and product-facing. It signals that the app is the desktop surface for the whole Overrid environment, not a single backend service.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [overdesk_desktop_client.md](../../service_catalog/native_apps/overdesk_desktop_client.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md) |
| Sub-build plan | [SUB BUILD PLAN #84 - Overdesk Desktop Client](../../build_plan/sub_build_plan_084_overdesk_desktop_client.md) |
| Desktop app page map | [page_map.md](../../overdesk/page_map.md) |

## Service Family

- Family: Native applications
- Owning layer: desktop-native Overrid client shell and user-facing control surface
- Primary data scope: local device profile, desktop session, node-onboarding flow, resource sharing rules, access policy drafts, embedded app sessions, namespace address history, credit-purchase intents, app-owner dashboard preferences, deployment wizard state, asset inventory views, workspace/directory/catalog/profile/namespace/privacy/vault/RAG/dispute/payout/grant/activity/fleet/developer/release/governance view state, notifications, usage refs, and audit refs
- First build phase from service plan: [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md), with Phase 2 and Phase 3 node onboarding dependencies and Phase 13 hardening

## Problem Statement

Overrid cannot become normal public infrastructure if joining the network requires terminal commands, scattered dashboards, manual node configuration, separate wallet views, separate app deployment tools, and separate native app clients. A person needs one understandable desktop product that makes the network usable.

The same product must serve two audiences without confusing them:

- Resource providers who want to contribute a computer, GPU, storage, or bandwidth under clear day/night/hour/resource-percentage rules.
- Users, app owners, and builders who want to browse the Overrid net, message people, search, use AI, use social/maps/workspace/wallet apps, buy credits, deploy apps, inspect owned apps, and manage Overasset rights.

The design risk is over-centralizing Overdesk. It must be a desktop shell and orchestration client, not a privileged bypass around Overgate, Overguard, Overcell, Overmeter, ORU, Seal Ledger, Wallet, Search, Messaging, Maps, AI, Deployment Planner, or Overasset.

## Goals

- Provide a guided "Add this computer to Overrid" flow that installs or verifies required node components and registers the device through normal Overrid rails.
- Let users define resource sharing rules for day, night, exact hours, resource percentages, idle-only behavior, bandwidth caps, thermal/battery constraints, and pause windows.
- Let users define who may use their resources through institutions, organizations, tags, purpose tags, explicit users, private UUID allowlists, and deny rules.
- Provide an Overrid browser with a top address bar where addresses such as `/hugo` resolve through Universal Namespace Service and Overmesh route handoffs.
- Provide first-class desktop pages for Messaging Center, Search Engine, Personal AI Assistant, Social Photo/Video App, Maps and Navigation, Wallet and Usage Center, credit buying, owned apps, app deployment, and Overasset-owned assets.
- Provide first-class desktop pages for Workspace, Directory Listings, Native App Catalog, Identity and Profile Center, Namespace Manager, Privacy and Permissions Center, Overvault Secure Storage Center, Docdex/RAG Index Manager, Disputes and Appeals Center, Provider Earnings and Payout Center, Grants and Public-Interest Projects, Audit and Receipts Timeline, Node Fleet Manager, Developer Console, Release and Rollback Manager, and Governance Center.
- Make every account's wallet visible and actionable without letting Overdesk mutate ORU or Seal Ledger truth directly.
- Provide a simple credit-buy screen that creates explicit purchase intents and routes settlement through Overbill, ORU Account Service, and Seal Ledger.
- Show owned Overrid apps with credit usage, credit earnings, resource costs, visitors, sources, analytics, versions, deployment state, abuse/dispute refs, and usage refs.
- Let builders deploy new apps through Deployment Planner, Overpack, Package Validator, Release Strategy Service, namespace binding, wallet prechecks, and policy dry-run.
- Provide a local encrypted cache for desktop usability while keeping authoritative truth in the owning services.
- Keep all native-app surfaces non-profit-oriented, near-cost, transparent, and hostile to dark patterns, addiction loops, ad-trap ranking, hidden fees, and personal-data extraction.

## Non-Goals

- Do not be an operating system, custom desktop environment, browser engine, direct ledger authority, external payment processor, node-agent authority, app store monopoly, analytics surveillance product, or privileged admin console.
- Do not mutate ORU balances, append Seal Ledger entries, settle payments, price resources, calculate taxes, issue provider payouts, or release accounting holds directly.
- Do not bypass Overgate, Overpass, Overtenant, Overkey, Overguard, Overwatch, Overmeter, Overcell, Oversched, Overlease, Overrun, Overpack, Overbase, Overstore, Overvault, or owner-service APIs.
- Do not own messaging records, search indexes, social posts, map/place truth, AI model routing decisions, workspace documents, directory listings, or Overasset ownership truth.
- Do not store card/payment secrets, raw private keys, seed phrases, vault secrets, decrypted private data, raw RAG context, private messages, precise location trails, or unnecessary tracking data.
- Do not introduce blockchain, NFT, speculative-token framing, per-transaction fee mechanics, hardcoded pricing, customer-count assumptions, or revenue projections.

## Primary Actors And Clients

- Regular users browsing Overrid addresses, messaging, searching, using AI, social, maps, wallet, assets, and native apps.
- Resource providers adding a desktop, workstation, server, GPU rig, lab computer, or institution machine to the Overrid network.
- App owners monitoring app usage, credits, visitors, analytics, versions, deployment health, disputes, and owned assets.
- Builders deploying new Overrid apps from packages, manifests, repos, or templates.
- Institutions and organizations managing allowed resource usage through tags, private UUIDs, purpose tags, and scoped access policies.
- Support and operators viewing redacted diagnostics, local logs, node health summaries, replay bundles, and incident refs with explicit permission.
- Native services used through Overdesk: Wallet and Usage Center, Personal AI Assistant, Messaging Center, Search Engine, Social Photo/Video App, Maps and Navigation, Workspace and Office Suite, Directory Listings, Central AI Stewardship Interface, Native App Catalog, Identity/Profile surfaces, Namespace Manager, Privacy/Permissions views, Overvault views, Docdex/RAG Index Manager, Disputes and Appeals, Provider Earnings and Payouts, Grants and Public-Interest Projects, Activity/Receipt Timeline, Node Fleet Manager, Developer Console, Release and Rollback Manager, Governance Center, and Overasset views.

## Dependencies

- [Overgate](../control_plane/overgate.md), [Overpass](../control_plane/overpass.md), [Overtenant](../control_plane/overtenant.md), [Overkey](../control_plane/overkey.md), and [Overwatch](../control_plane/overwatch.md) for API ingress, identity, tenant scope, credentials, signatures, sessions, audit, and replay refs.
- [Node Installer](../execution_scheduling/node_installer.md), [Hardware Discovery](../execution_scheduling/hardware_discovery.md), [Overcell](../execution_scheduling/overcell.md), [Benchmark Runner](../execution_scheduling/benchmark_runner.md), [Oversched](../execution_scheduling/oversched.md), [Overlease](../execution_scheduling/overlease.md), [Overrun](../execution_scheduling/overrun.md), [Overmeter](../execution_scheduling/overmeter.md), [Overmesh](../execution_scheduling/overmesh.md), and [Overcache](../execution_scheduling/overcache.md) for device onboarding, resource capability discovery, sharing, scheduling, execution, usage, routing, and cache behavior.
- [Overguard](../trust_policy_verification/overguard.md), [Policy Dry-Run API](../trust_policy_verification/policy_dry_run_api.md), [Workload Classifier](../trust_policy_verification/workload_classifier.md), [Oververify](../trust_policy_verification/oververify.md), [Challenge Task Service](../trust_policy_verification/challenge_task_service.md), [Overclaim](../trust_policy_verification/overclaim.md), and [Reputation and Anti-Sybil Service](../trust_policy_verification/reputation_anti_sybil_service.md) for resource-use policy, previews, verification, disputes, and abuse control.
- [ORU Account Service](../accounting/oru_account_service.md), [Seal Ledger](../accounting/seal_ledger.md), [Overbill](../accounting/overbill.md), [Overmark](../accounting/overmark.md), [Overgrant](../accounting/overgrant.md), [Provider Payout Service](../accounting/provider_payout_service.md), and [Overasset](../accounting/overasset.md) for wallet views, purchase intents, credit usage, provider earnings, grants, rights, and asset ownership refs.
- [Overbase](../data_storage_namespace/overbase.md), [Overstore](../data_storage_namespace/overstore.md), [Overvault](../data_storage_namespace/overvault.md), and [Universal Namespace Service](../data_storage_namespace/universal_namespace_service.md) for app state refs, object refs, private encrypted refs, address resolution, route binding, and namespace history.
- [Deployment Planner](../deployment_grid/deployment_planner.md), [Grid-Resident Service Packager](../deployment_grid/grid_resident_service_packager.md), [Package Validator](../deployment_grid/package_validator.md), and [Release Strategy Service](../deployment_grid/release_strategy_service.md) for app deployment and owned-app release management.
- [Purpose Tag Registry](../federation_public/purpose_tag_registry.md), [Public Provider Onboarding](../federation_public/public_provider_onboarding.md), and [Fraud Control Service](../federation_public/fraud_control_service.md) for resource sharing constraints, public provider controls, and misuse detection.
- [Personal AI Assistant](../ai_rag_model_routing/personal_ai_assistant.md), [AI Gateway Router](../ai_rag_model_routing/ai_gateway_router.md), [Central AI Service](../ai_rag_model_routing/central_ai_service.md), [ADES Enrichment Adapter](../ai_rag_model_routing/ades_enrichment_adapter.md), [Encrypted Docdex RAG Adapter](../ai_rag_model_routing/encrypted_docdex_rag_adapter.md), and [Lightweight Classifier](../ai_rag_model_routing/lightweight_classifier.md) for AI assistant, model routing, tool calls, RAG, entity extraction, and lightweight request classification.
- Native apps: [Wallet and Usage Center](wallet_usage_center.md), [Messaging Center](messaging_center.md), [Search Engine](search_engine.md), [Social Photo/Video App](social_photo_video_app.md), [Maps and Navigation](maps_navigation.md), [Workspace and Office Suite](workspace_office_suite.md), [Directory Listings](directory_listings.md), and [Central AI Stewardship Interface](central_ai_stewardship_interface.md).
- Governance and operations services: [Protocol Improvement Proposal Registry](../governance_ops/pip_registry.md), [Stewardship Reporting Service](../governance_ops/stewardship_reporting_service.md), [Compliance Boundary Service](../governance_ops/compliance_boundary_service.md), [Threat Modeling and Security Review Tracker](../governance_ops/threat_modeling_security_review_tracker.md), [Incident Response Service](../governance_ops/incident_response_service.md), [Backup and Restore Service](../deployment_grid/backup_restore_service.md), and [Failover and Recovery Coordinator](../deployment_grid/failover_recovery_coordinator.md) for governance, incident, release, backup, recovery, reporting, and compliance-facing desktop views.
- [SDK](../foundation/sdk.md), [CLI](../foundation/cli.md), [Shared Schema Package](../foundation/shared_schema_package.md), and [mSwarm Runtime Bridge](../adapters/mswarm_runtime_bridge.md) for generated contracts, signed calls, offline/local-first helpers, runtime session handoffs, and diagnostics.

## Owned Responsibilities

Overdesk owns:

- Desktop app shell, navigation, page routing, local session state, account switcher, window state, notification preferences, and theme/accessibility preferences.
- Local device profile, node onboarding flow state, installer handoff state, hardware summary cache, benchmark display cache, and node health display preferences.
- Resource sharing rule drafts and user-facing rule records for time windows, day/night schedules, resource percentages, idle-only behavior, bandwidth caps, and pause/snooze controls.
- Resource access policy drafts and user-facing records for institutions, organizations, users, tags, purpose tags, private UUIDs, allow/deny rules, and emergency stop behavior.
- Overrid browser shell state, address-bar history, bookmarks, tabs, resolved namespace refs, route refs, app-open refs, and denied-resolution explanations.
- Embedded native-app session refs for Messaging, Search, AI Assistant, Social, Maps, Workspace, Directory, Wallet, Central AI Stewardship Interface, and Overasset views.
- View state and action drafts for Workspace, Directory, Native App Catalog, Identity/Profile, Namespace Manager, Privacy/Permissions, Overvault, Docdex/RAG indexes, Disputes/Appeals, Provider Earnings/Payouts, Grants, Activity/Receipts, Node Fleet, Developer Console, Release/Rollback, and Governance Center.
- Credit purchase intent drafts, wallet-safe confirmation views, receipt refs, and post-purchase display state.
- Owned-app dashboard preferences, app analytics filters, visitor/source-safe summaries, credit usage and earnings projections, release status views, deployment wizard state, and app detail page state.
- Overasset inventory views, asset filters, delegation request drafts, transfer request drafts, dispute handoff refs, and local display state.
- Local encrypted cache, redacted diagnostics, support bundle exports, replay refs, and usage refs for Overdesk itself.

Overdesk does not own:

- Identity truth, tenant membership truth, key custody, policy finality, node capability truth, lease placement truth, execution truth, usage measurement truth, balance truth, ledger truth, app deployment finality, search ranking truth, message delivery truth, AI route finality, map truth, social post truth, workspace document truth, directory listing truth, or asset ownership truth.

## UI Page Model

### Global Shell

The global shell is always available after login unless the user is in a restricted onboarding state.

Required elements:

- Top address bar that accepts Overrid names, routes, and commands, including `/hugo`.
- Account switcher for personal accounts, organization accounts, institution scopes, app-owner scopes, and delegated scopes.
- Left navigation rail for Home, Browse, Workspace, Directory, App Catalog, Messaging, Search, AI, Social, Maps, Wallet, Credits, Owned Apps, Deploy, Assets, Identity, Namespace, Privacy, Vault, RAG, Disputes, Provider, Grants, Activity, Fleet, Developer, Releases, Governance, Resource Sharing, Access Rules, and Settings.
- Notification center for wallet alerts, node health, policy denials, deployment events, messages, app-owner alerts, asset updates, and security prompts.
- Activity indicator for local node state, connection state, sync state, current account, and pending high-risk actions.
- Command palette for opening apps, resolving names, starting deployment, searching, asking the assistant, and jumping to settings.
- Redacted diagnostics entry point for support bundles and replay refs.

Shell rules:

- Never display hidden fees, dark patterns, fake urgency, addictive engagement prompts, or ad-trap placement.
- High-risk actions require a clear confirmation screen with source refs, affected account/device/app, policy result, usage estimate where available, and rollback or dispute path.
- Pages must be deep-linkable through Overrid address refs where the owning service allows it.

### Home Dashboard

Purpose: give the user a clear starting point.

Core sections:

- Account summary: selected identity, tenant/org scope, wallet summary, active grants, usage warning bands, and permission alerts.
- This computer: node joined/not joined state, resource sharing state, health summary, last benchmark refs, current lease count, and pause button.
- Native apps: shortcuts for Messaging, Search, AI, Social, Maps, Wallet, Workspace, Directory, App Catalog, Governance, and Central AI views where allowed.
- App owner summary: owned app count, credit usage, credit earnings, visitor/source-safe summaries, deployment alerts, and active incidents.
- Asset summary: Overasset count, delegated rights, pending disputes, expiring rights, and app/resource ownership refs.
- Recent activity: messages, credit receipts, resource leases, search sessions, assistant sessions, workspace edits, listing updates, namespace changes, vault grants, RAG index syncs, disputes, payout status, grants, deployment events, release events, governance events, and asset updates, all redacted by viewer role.

### Add This Computer To Overrid

Purpose: let a user add a computer to the Overrid network without terminal work.

Page sections:

- System check: OS, architecture, CPU, GPU, RAM, storage, network, virtualization/sandbox readiness, battery/thermal profile, and disk encryption status where visible.
- Identity and tenant scope: which account or institution the node will belong to.
- Installer check: Overcell, Node Installer, Hardware Discovery, benchmark runner, updater, local service status, and required permissions.
- Capability discovery: detected compute, GPU, memory, storage, bandwidth, uptime profile, and safety limits.
- Benchmark and verification: optional benchmark run, Oververify requirements, challenge-task readiness, and reputation bootstrap state.
- First sharing rule: quick presets for "idle only", "night only", "office hours", "institution only", "private UUID only", and "paused until I enable".
- Final review: resource caps, allowed users/institutions/tags, policy preview, expected usage dimensions, node name, audit refs, and start button.

Owned state:

- `node_onboarding_flow`
- `desktop_device_profile`
- `installer_handoff_ref`
- `hardware_summary_cache`
- `first_share_rule_draft`

Authority boundary:

- Overdesk may start the install/update flow and submit signed onboarding commands. Node Installer, Overcell, Hardware Discovery, Oververify, Overguard, Overregistry, and Overwatch own authoritative registration, capability, policy, and audit state.

### Resource Sharing Rules

Purpose: let the provider decide when and how much of the computer can be used.

Controls:

- Master enable/disable switch.
- Schedules: day, night, specific hours, weekdays, weekends, date ranges, holidays, one-time windows, recurring windows.
- Resource percentages: CPU percentage, GPU percentage, RAM cap, storage cap, network bandwidth cap, I/O cap, concurrent lease count, thermal cap, battery cap, and idle-only threshold.
- Device behavior: pause when on battery, pause when screen active, pause for selected local apps, pause during calls/meetings, pause on high fan/temperature, pause on network metered mode.
- Workload classes: allowed classes, denied classes, public low-sensitivity only, institution/private only, AI/model jobs allowed/denied, storage allowed/denied, bandwidth-only allowed/denied.
- Spending and payout display: projected usage and earnings are display-only projections from Overmeter, Overmark, ORU, Seal Ledger, and Provider Payout Service.
- Emergency controls: pause now, drain leases, stop accepting new leases, report suspicious workload, open dispute, export support bundle.

State:

- `resource_share_rule`
- `resource_schedule_window`
- `resource_cap_profile`
- `provider_pause_state`
- `provider_safety_override`

Authority boundary:

- Overdesk edits provider preferences and sends signed rule updates. Overguard, Oversched, Overlease, Overcell, Overmeter, and Oververify decide admission, placement, leases, measurement, and verification.

### Access Rules

Purpose: let the provider choose who may use the resource.

Controls:

- Institution allowlist: universities, labs, schools, companies, public-interest pools, or approved federation templates.
- Organization allowlist: tenant/org refs and role scopes.
- User allowlist: explicit Overpass identity refs.
- Private UUID allowlist: opaque private resource-access UUIDs for controlled pilots, advisor networks, institution labs, or closed beta deployments.
- Tags and purpose tags: Purpose Tag Registry tags for public-interest, academic, research, local community, AI/RAG, low-sensitivity public work, and institution-private work.
- Deny rules: denied orgs, users, tags, workload classes, jurisdictions, or risk bands.
- Expiry and review: access rule expiry, periodic reapproval, one-time grants, and temporary emergency blocks.
- Dry run: policy preview with allowed, denied, missing prerequisites, and reason codes.

State:

- `resource_access_policy`
- `resource_allowed_institution`
- `resource_allowed_private_uuid`
- `resource_allowed_tag`
- `resource_deny_rule`
- `policy_preview_session`

Authority boundary:

- Overdesk stores user-facing drafts and submits rules. Overguard, Overtenant, Purpose Tag Registry, Federation Template Service, Oververify, and Fraud Control own authoritative policy and eligibility facts.

### Overrid Browser

Purpose: browse the Overrid net from a desktop address bar.

Core behavior:

- User enters `/hugo`, `overrid:/hugo`, a namespace route, an app route, a document route, a message thread ref, a search query, or a command.
- Overdesk normalizes the address and asks Universal Namespace Service for resolution.
- Overmesh returns route hints where a live route is available.
- Overgate checks identity, tenant, policy, and app/session requirements.
- Overdesk opens the target in an embedded native-app view, app window, browser-like tab, or denied-resolution explanation page.
- If the namespace is not found, Overdesk offers Search Engine fallback and Directory Listings fallback where appropriate.

Page elements:

- Address bar with history, bookmarks, recently opened apps, route state, and trust markers.
- Tabs or split view for multiple Overrid pages.
- Namespace info drawer showing owner refs, target refs, verification marker, route status, source policy, and dispute/referral links.
- Privacy drawer showing which identity/account is being used for this address.
- Safe preview for external web links when a route points outside Overrid.

State:

- `namespace_address_visit`
- `address_bar_session`
- `overrid_tab`
- `route_resolution_view`
- `bookmark_ref`

Authority boundary:

- Overdesk does not own namespace truth, DNS-like resolution truth, route truth, search ranking truth, or app authorization. Universal Namespace Service, Overmesh, Overgate, Overguard, and source apps own those decisions.

### Messaging Center Page

Purpose: desktop surface for username-addressed messaging.

Expected capabilities:

- Inbox list for person, organization, app, service, and system-notification inboxes.
- Direct username compose through Overpass and Universal Namespace Service.
- Threads, attachments, reactions where supported, message search, archive, mute, block, report, recall/tombstone status, and read state.
- Organization inbox routing with assignment, internal notes, escalation, and role-aware visibility.
- AI triage entry point that requires explicit permission and redaction scope.
- Wallet-visible usage refs for storage, attachments, AI triage, notifications, and large sends.

Authority boundary:

- Messaging Center owns messaging truth. Overdesk hosts the view, local cache, notifications, and desktop actions through Messaging APIs.

### Search Page

Purpose: permission-aware search across Overrid content.

Expected capabilities:

- Search box, source filters, public/private scope toggle, directory/app/workspace/social/maps filters, saved searches, and recent queries.
- Result cards with permission-safe snippets, source refs, ranking explanations, omitted/denied counts, and direct app handoffs.
- Private search permission prompts before using private workspace, message metadata, vault refs, or RAG-backed content.
- Abuse/spam report flow and source correction request flow.

Authority boundary:

- Search Engine owns indexing, ranking, permission filtering, source policy, and result set truth. Overdesk owns desktop search session display and local history subject to retention rules.

### AI Assistant Page

Purpose: desktop interface for the Personal AI Assistant.

Expected capabilities:

- Chat/workbench view with model route transparency, context scope selector, tool permission drawer, usage estimate display, and receipt refs.
- Context sources: current page, selected app, wallet usage, workspace docs, messages, owned app analytics, Overasset refs, or encrypted Docdex RAG bundles, only with explicit permission.
- Call routing through AI Gateway Router, Lightweight Classifier, ADES Enrichment Adapter, Encrypted Docdex RAG Adapter, Central AI Service, and available model resources.
- Tool handoffs: open app, search, draft message, explain usage, prepare deployment, inspect node health, review app analytics, summarize assets, or draft dispute.
- Replay view for AI route decision, model resource, context grants, tool calls, redactions, usage refs, and results.

Authority boundary:

- Overdesk does not decide model routing or own AI memory. Personal AI Assistant and AI Gateway Router own route decisions, with owner services controlling data access.

### Social Photo/Video Page

Purpose: desktop surface for Overrid social media without addiction-driven extraction.

Expected capabilities:

- Feed, profile, groups, albums, uploads, comments, reactions, follows, media processing state, moderation notices, rights/attribution refs, report/appeal flows, and transparent recommendation controls where enabled.
- Upload tool for photos and videos through Overstore, processing jobs, safety scans, accessibility captions, and usage estimates.
- Visibility controls and Overvault-backed private group access.
- No infinite-scroll dark patterns by default; user-controlled sorting and limits must be available.

Authority boundary:

- Social Photo/Video App owns posts, media refs, feed policy, visibility, moderation, and rights state. Overdesk hosts the desktop view and local upload/session state.

### Maps And Navigation Page

Purpose: desktop maps, places, routes, and local discovery.

Expected capabilities:

- Search for places, directory listings, organizations, public-interest map layers, and community map layers.
- Route planning for supported route classes, with privacy-preserving location controls.
- Exact location permission prompts with purpose, expiry, revocation, and audit.
- Offline area or route-corridor pack management where enabled.
- Correction/report flow for map/place issues and directory/listing handoffs.

Authority boundary:

- Maps and Navigation owns place/route/map-layer truth and location permission records. Overdesk owns map view state, local cache preferences, and desktop route/session display.

### Wallet Page

Purpose: every account has a wallet and every user needs clear usage visibility.

Expected capabilities:

- Account selector for personal, organization, institution, app-owner, and delegated accounts.
- Balance projection by ORU dimension.
- Usage dashboard by app, service, model route, storage, compute, bandwidth, and time window.
- Receipts, holds, grants, sponsored credits, refunds, corrections, statements, exports, permissions, privacy audit, and disputes.
- Provider resource contribution summary for this computer and all owned nodes.
- Permission cleanup view and AI explanation view.

Authority boundary:

- Wallet and Usage Center owns wallet views. ORU Account Service, Seal Ledger, Overbill, Overgrant, Overmeter, Overclaim, and owner services own authoritative accounting and dispute state.

### Buy Credits Page

Purpose: make credit buying easy while preserving Seal Ledger and ORU boundaries.

Expected capabilities:

- Select account and ORU credit dimension or bundle where allowed by current product policy.
- Show payment method refs and payment-provider handoff state without storing payment secrets in Overdesk.
- Create `credit_purchase_intent` through Wallet/Overbill.
- Confirm amount, fees if any external payment provider imposes them, target account, expected ORU crediting path, and cancellation/refund path.
- Display Overbill receipt refs, ORU projection update refs, Seal Ledger checkpoint refs, and failure/retry states.
- Avoid per-operation payment friction: credits are purchased into the account and resource usage settles internally through ORU and Seal Ledger.

Authority boundary:

- Overdesk creates purchase-intent drafts and displays status. Overbill, ORU Account Service, Seal Ledger, and external payment integrations own settlement, crediting, receipts, refunds, and reconciliation.

### Owned Apps Page

Purpose: let app owners understand app performance, credit usage, earnings, and health.

Expected capabilities:

- App list grouped by owner account, organization, namespace, release state, risk class, and environment.
- Per-app summary: app name, namespace route, version, deployment state, usage, earnings, cost, visitors, source refs, errors, incidents, disputes, grants, and asset refs.
- Analytics: visitor/source-safe summary, traffic sources, regions/coarse locality where policy allows, resource usage by dimension, credit earnings, credit costs, wallet receipts, deployment health, search referrals, directory referrals, social referrals, and app-to-app handoffs.
- App detail page: usage charts, earning/cost projections, release timeline, active deployments, package refs, namespace refs, permissions, data/storage usage, support inbox, abuse reports, disputes, and replay refs.
- Export: privacy-safe app analytics export and statement handoff.

Authority boundary:

- Overdesk displays app-owner projections and analytics from owning services. It does not own app records, deployment truth, metering truth, visitor identity truth, or accounting truth.

### Deploy New App Page

Purpose: let builders deploy Overrid apps without manually touching every backend service.

Wizard steps:

1. Select source: Overpack manifest, repo/package import, template, existing app version, or local build output.
2. Validate manifest: package identity, permissions, resource needs, data classes, storage refs, namespace needs, workload class, and policy refs.
3. Estimate resources: compute, GPU, RAM, storage, bandwidth, expected usage dimensions, budget precheck, and grant availability.
4. Policy dry run: Overguard, Workload Classifier, Package Validator, Oververify, Compliance Boundary where needed, and risk reason codes.
5. Namespace and routes: choose `/name`, app route, subroutes, visibility, public/private access, and dispute info.
6. Deployment plan: Deployment Planner graph, release strategy, rollback plan, health checks, backup/restore refs, and usage refs.
7. Final confirmation: signed command, account, wallet precheck, source refs, risk refs, and audit refs.
8. Launch monitor: deployment state, logs, health, errors, rollback button, and support/replay bundle.

Authority boundary:

- Deployment Planner, Overpack, Package Validator, Release Strategy Service, Overguard, Overregistry, Overgate, Overbase, Overstore, Overvault, Overmeter, and owner services own deployment authority. Overdesk owns wizard state and desktop display.

### Overasset Assets Page

Purpose: give users and app owners a clear view of owned operational assets.

Expected capabilities:

- Asset inventory by account, organization, app, namespace, capacity right, service ownership, app ownership, grant right, delegation, and dispute state.
- Asset detail: evidence refs, owner refs, scope, expiry, delegation, revocation, transfer constraints, dispute refs, usage refs, and related apps/resources.
- Actions: request delegation, revoke delegation, request transfer where allowed, bind to app, bind to namespace, open dispute, view audit, export asset record.
- Warnings for non-transferable, disputed, expired, suspended, held, or policy-restricted assets.

Authority boundary:

- Overasset owns asset truth. Overdesk owns inventory display, filters, action drafts, and app-page handoffs.

### Workspace Page

Purpose: desktop productivity surface for documents, tables, pages, files, comments, approvals, and AI-assisted work.

Expected capabilities:

- Workspace switcher for personal, organization, institution, app-owner, and delegated workspaces.
- Document, table, page, folder, and object views with recent files, shared-with-me, offline-marked items, approvals, comments, versions, import/export, and search handoffs.
- Explicit sharing and revocation controls with role, expiry, public/private link state, vault grant state, AI context eligibility, and audit refs.
- AI assist entry points for summarizing, drafting, table cleanup, import review, and document Q&A only after context permission is granted.
- Usage view for storage, export, AI assist, indexing, collaboration, and offline sync.

Authority boundary:

- Workspace and Office Suite owns workspace objects, editor sessions, versions, permissions, comments, approvals, import/export, and document truth. Overdesk owns route state, local cache policy, desktop notifications, and user-visible action drafts.

### Directory Listings Page

Purpose: local and public listing surface for marketplace-style discovery without ad-trap ranking.

Expected capabilities:

- Listings for local services, jobs, housing, events, community posts, organization pages, resource offers, university/public-interest pools, and app/service pages.
- Create/edit listing drafts, listing media uploads, category/tag/purpose tag selection, locality scope, expiry, renewal, report, dispute, and moderation/appeal state.
- Search, map, social, messaging, wallet, and namespace handoffs that preserve viewer permissions and source refs.
- Transparent sorting and filtering by relevance, locality, freshness, category, trust markers, and public-interest status without paid-placement monopoly behavior.

Authority boundary:

- Directory Listings owns listing truth, visibility, moderation, ranking, disputes, and category policy. Overdesk owns desktop view state, drafts, cached cards, and handoffs.

### App Store And Native App Catalog

Purpose: discover, inspect, install/open, and review Overrid-native apps and service surfaces.

Expected capabilities:

- Catalog of native apps, public utilities, workspace tools, developer tools, institution apps, organization apps, and user-owned deployed apps.
- App detail pages with owner refs, namespace refs, versions, permissions, data classes, credit usage model, resource usage, reviews, incidents, support links, privacy summary, and source/package refs where public.
- Install/open/pin actions, update notices, permission review, app removal, default app selection, and app-to-app handoffs.
- Safety labels for experimental, institution-only, private-beta, restricted, deprecated, disputed, suspended, or incident-affected apps.

Authority boundary:

- App owner services, Deployment Planner, Package Validator, Release Strategy Service, Universal Namespace Service, Overguard, and Overwatch own app records, deployability, package state, releases, policy, and incident truth. Overdesk owns catalog display, local pins, open sessions, and desktop update prompts.

### Identity And Profile Center

Purpose: manage the user's identities, profiles, organization scopes, public handles, and account-facing trust markers.

Expected capabilities:

- Personal profile, public profile, organization profile, institution profile, app-owner profile, delegated scope, and recovery/contact state.
- Username/handle display, verified markers, profile fields, avatar/media refs, privacy visibility, blocked users, trusted devices, sessions, and credential-provider status.
- Identity handoffs to Overpass, Overtenant, Overkey, Universal Namespace Service, Messaging Center, Directory Listings, and Wallet.
- Account safety prompts for device revoke, session revoke, profile visibility changes, delegated role changes, and recovery changes.

Authority boundary:

- Overpass, Overtenant, Overkey, Universal Namespace Service, and owning profile services own identity, membership, key, credential, session, and profile truth. Overdesk owns desktop presentation and confirmation flows.

### Namespace Manager

Purpose: manage Overrid names, app routes, profile routes, directory routes, service aliases, and namespace disputes.

Expected capabilities:

- Owned names, requested names, delegated names, app-bound routes, organization aliases, service routes, route history, verification markers, and tombstone/dispute state.
- Create/request/update namespace route drafts, bind names to apps or profiles, set public/private visibility, add route metadata, rotate route target refs, and release names where allowed.
- Search fallback, directory fallback, conflict explanation, proof/evidence display, and dispute/appeal handoff.

Authority boundary:

- Universal Namespace Service owns normalized names, owner/target refs, delegation, transfer, route bindings, tombstones, verification markers, and disputes. Overdesk owns manager view state and action drafts.

### Privacy And Permissions Center

Purpose: give users one place to understand and revoke app permissions, data access, AI context grants, location grants, vault grants, notifications, and retention settings.

Expected capabilities:

- Permission inventory by app, service, device, workspace, message scope, location scope, RAG scope, vault scope, directory listing, social/media scope, and node/provider scope.
- Revoke, expire, narrow, review, renew, and explain grants with immediate/queued owner-service status.
- Data-use explanations, redaction classes, retention periods, audit refs, denied access attempts, and active high-risk permissions.
- Privacy health checks and cleanup suggestions that do not auto-revoke without user confirmation unless owner-service policy requires it.

Authority boundary:

- Owner services, Overguard, Overpass, Overtenant, Overkey, Overvault, Wallet, Search, AI, Maps, Workspace, Messaging, and Social own permission truth. Overdesk owns consolidated display and signed revocation requests.

### Overvault Secure Storage Center

Purpose: desktop surface for private encrypted records, vault grants, recovery refs, and app secret storage visibility.

Expected capabilities:

- Vault item categories by account, app, organization, device, workspace, RAG index, deployment, credential ref, and backup/recovery ref.
- Grant review, grant request, grant revoke, rotation status, sealed reference display, recovery status, and export/backup eligibility where allowed.
- Warnings for expired, stale, unreviewed, high-risk, shared, disputed, unrecoverable, or incident-affected vault grants.
- No plaintext secret display unless an owner-service flow explicitly requires a short-lived reveal and fresh credential verification.

Authority boundary:

- Overvault and Overkey own encryption, grants, sealed refs, recovery, and secret access. Overdesk owns safe projection, local encrypted cache policy, and action drafts.

### Docdex And RAG Index Manager

Purpose: connect encrypted Docdex indexes to the personal AI assistant, workspaces, repositories, and apps without leaking raw private context.

Expected capabilities:

- List personal, organization, repo, workspace, and app-scoped encrypted indexes with scope, freshness, embedding/index state, allowed models, RAG grants, and source refs.
- Create/connect/sync/disconnect encrypted index refs, choose RAG eligibility, review model/tool permissions, and inspect redacted retrieval receipts.
- Route requests through Personal AI Assistant, AI Gateway Router, Encrypted Docdex RAG Adapter, Lightweight Classifier, ADES Enrichment Adapter, and available model resources.
- Display usage, freshness, denied-source counts, redaction decisions, and replay refs without storing raw private RAG context in Overdesk.

Authority boundary:

- Encrypted Docdex RAG Adapter, Personal AI Assistant, AI Gateway Router, owner repos/workspaces/apps, and permission services own index truth, routing, retrieval, and context access. Overdesk owns manager UI and explicit grants.

### Disputes And Appeals Center

Purpose: one desktop place for usage disputes, asset disputes, namespace disputes, moderation appeals, payout holds, provider complaints, fraud reports, and incident-linked appeals.

Expected capabilities:

- Case list by account, app, node, listing, namespace, asset, wallet receipt, provider earning, content moderation event, abuse report, or governance decision.
- Case detail with status, evidence refs, receipts, affected services, deadlines, requested remedy, owner-service messages, appeal options, and public/private redaction class.
- Create dispute/appeal drafts from wallet receipt, app analytics, asset detail, listing, namespace route, provider lease, message/report, or incident ref.
- Timeline of owner-service updates and final decisions with replay and correction paths.

Authority boundary:

- Overclaim, Overasset, Wallet/Overbill/Seal Ledger, Directory, Social, Universal Namespace Service, Provider Payout Service, Incident Response Service, and owner services own dispute truth. Overdesk owns consolidated case display and draft handoffs.

### Provider Earnings And Payout Center

Purpose: show resource providers what their nodes earned, what is pending, what is held, and what can be paid out.

Expected capabilities:

- Provider earning summaries by node, device, organization, resource class, lease, workload class, time window, ORU dimension, gross usage, costs, holds, corrections, disputes, and payout state.
- Payout method refs, tax/compliance requirement refs, minimum/hold state, payout schedule, payout history, failed payout state, and appeal/dispute links.
- Node contribution analytics tied to Resource Sharing Rules and Node Fleet Manager.

Authority boundary:

- Provider Payout Service, Overmeter, Overmark, ORU Account Service, Seal Ledger, Overbill, Overclaim, Compliance Boundary Service, and owner services own earning, hold, payout, correction, and compliance truth. Overdesk owns display and signed payout-action drafts.

### Grants And Public-Interest Projects

Purpose: connect users, providers, builders, academics, institutions, and public projects to grants and central-AI/stewardship-funded work.

Expected capabilities:

- Grant catalog, active grants, sponsored credits, public-interest pools, academic/research projects, institution programs, donation/surplus-funded projects, eligibility, application status, and usage reports.
- Apply, accept, renew, report progress, attach evidence refs, link apps/nodes/workspaces/listings/assets, and view grant receipts.
- Central AI recommendation view with evidence, confidence, review status, public report links, and appeal/correction paths.

Authority boundary:

- Overgrant, Central AI Service, Central AI Stewardship Interface, Stewardship Reporting Service, ORU Account Service, Seal Ledger, Wallet, and owner services own grant truth and funding decisions. Overdesk owns desktop discovery, drafts, and status display.

### Audit, Activity, And Receipts Timeline

Purpose: give each actor a replayable, role-redacted timeline of important Overrid actions.

Expected capabilities:

- Timeline across wallet receipts, credit purchases, node leases, provider rule changes, permission grants/revokes, app opens, deployments, releases, namespace changes, vault grants, RAG retrieval receipts, disputes, payouts, grants, governance actions, support exports, and security prompts.
- Filters by account, org, device, app, service, asset, namespace, time, risk class, receipt type, usage dimension, and redaction class.
- Open replay, export user-visible receipts, report issue, open dispute, revoke permission, or jump to owner-service detail where authorized.

Authority boundary:

- Overwatch, owner services, Wallet, ORU, Seal Ledger, Overmeter, Overbill, Overguard, Overvault, AI Gateway, Deployment Planner, and governance services own event and receipt truth. Overdesk owns aggregation view and local filters.

### Node Fleet Manager

Purpose: manage more than one contributed computer from the same desktop product.

Expected capabilities:

- Fleet list by personal, organization, institution, lab, class, office, server room, GPU rig, or cloud/edge group.
- Node health, hardware summary, resource sharing state, access rules, current leases, earnings, updates, incidents, benchmark freshness, pause/drain state, and policy denials.
- Bulk rule drafts, staged updates, maintenance windows, tags, private UUID pools, institution scopes, and support bundle exports.

Authority boundary:

- Node Installer, Overcell, Hardware Discovery, Overregistry, Oververify, Overguard, Oversched, Overlease, Overrun, Overmeter, Overwatch, and Provider Payout Service own node, scheduling, execution, health, usage, and earning truth. Overdesk owns fleet display and signed bulk-action drafts.

### Developer Console

Purpose: make Overrid app development, integration, testing, and deployment reachable without turning Overdesk into a privileged admin tool.

Expected capabilities:

- Project list, app manifests, local dev environments, SDK/CLI links, package validation, policy dry-run, permissions, secrets refs, namespace route drafts, test fixtures, deployment previews, logs, and replay bundles.
- Overpack build/import status, Package Validator results, Deployment Planner graph preview, release strategy preview, wallet/grant precheck, and app-owner handoff.
- Developer diagnostics for local stack, generated contract version, API health, app sessions, test accounts, and redacted support bundles.

Authority boundary:

- SDK, CLI, Overpack, Package Validator, Deployment Planner, Release Strategy Service, Overguard, Overbase, Overstore, Overvault, Universal Namespace Service, and owner services own build/deploy authority. Overdesk owns developer UX and local diagnostics.

### Release And Rollback Manager

Purpose: let app owners and service owners see release state, staged rollouts, health gates, rollback readiness, and recovery refs.

Expected capabilities:

- Release list by app/service/environment, version, package ref, rollout strategy, health status, incident status, backup/restore readiness, rollback point, route state, and namespace binding.
- Promote, pause, resume, rollback request, route-shift request, health gate review, migration link, backup/restore link, and support bundle handoff where the actor is authorized.
- Clear explanations for blocked releases, unsafe rollbacks, stale backups, failed health checks, missing reviews, and compliance boundaries.

Authority boundary:

- Release Strategy Service, Deployment Planner, Backup and Restore Service, Failover and Recovery Coordinator, Overmesh, Overwatch, Package Validator, Compliance Boundary Service, and owner services own release and rollback authority. Overdesk owns display and signed action drafts.

### Governance Center

Purpose: expose protocol, stewardship, central-AI recommendation, incident, compliance, and public-interest governance surfaces to users and stewards.

Expected capabilities:

- Protocol Improvement Proposals, stewardship reports, public-interest funding reports, native-service surplus reports, central-AI recommendations, compliance boundary summaries, security review status, incident summaries, migration evidence, and public correction/retraction notices.
- Read public reports, follow proposals, submit comments where allowed, view affected services, see required reviews, inspect redacted evidence, and open appeal/correction paths.
- Steward views for assigned reviews, pending decisions, publication readiness, redaction status, conflict-of-interest markers, and required review coverage.

Authority boundary:

- PIP Registry, Stewardship Reporting Service, Central AI Service, Central AI Stewardship Interface, Compliance Boundary Service, Threat Modeling and Security Review Tracker, Incident Response Service, Migration Tooling, and owner services own governance truth. Overdesk owns user/steward desktop presentation and signed comment/review drafts.

### Settings And Security

Required settings:

- Accounts, tenants, organizations, institutions, app-owner scopes, and delegated scopes.
- Credentials, device refs, session state, key-provider status, logout/revoke device.
- Local cache policy, encrypted local store status, clear cache, export diagnostics, support bundle.
- Notification preferences and quiet hours.
- Address-bar history retention, search history retention, AI session retention, app analytics retention, map/location retention, and social/media upload retention.
- Accessibility, language, theme, display density, and keyboard shortcuts.
- Developer mode for signed test environments, local Overrid stack, manifest preview, and debug logs.

Security rules:

- Secret material remains in approved platform credential providers or Overkey-controlled refs, not in Overdesk plain storage.
- Local caches are encrypted where platform support exists, bounded by retention policy, user-clearable, and safe to discard.
- High-risk actions require fresh session/credential verification.
- Support bundles are redacted by default and include explicit user review before export.

## Data Model

- `overdesk_device_profile`: local device ref, owner refs, OS, architecture, app version, node status, installer refs, hardware summary refs, privacy class, and state.
- `desktop_session`: actor refs, tenant refs, selected account, session refs, credential refs, active scope, local cache refs, and state.
- `node_onboarding_flow`: step, device refs, installer refs, hardware discovery refs, benchmark refs, policy preview refs, rule draft refs, error refs, and state.
- `resource_share_rule`: provider/device refs, schedule refs, resource caps, workload class allow/deny, safety overrides, pause state, policy refs, audit refs, and state.
- `resource_access_policy`: provider/device refs, allowed institution refs, organization refs, user refs, private UUID refs, purpose tags, deny rules, expiry, policy refs, and state.
- `namespace_address_visit`: viewer refs, raw input, normalized address, namespace refs, route refs, app refs, denial refs, history/bookmark flags, and retention class.
- `embedded_app_session`: app/service refs, viewer refs, route refs, permission refs, local window/tab state, cached view refs, usage refs, audit refs, and state.
- `credit_purchase_intent`: account refs, amount/dimension refs, Overbill refs, payment-provider refs, ORU projection refs, Seal Ledger checkpoint refs, receipt refs, failure refs, and state.
- `owned_app_dashboard`: owner refs, app refs, namespace refs, release refs, usage refs, earnings refs, visitor/source-safe analytics refs, incident refs, dispute refs, and filters.
- `app_detail_view`: app refs, release refs, deployment refs, package refs, policy refs, usage/earning/cost refs, analytics refs, support inbox refs, and replay refs.
- `deployment_wizard_session`: source refs, manifest refs, validation refs, resource estimate refs, policy dry-run refs, namespace refs, deployment graph refs, release refs, wallet precheck refs, and state.
- `asset_inventory_view`: owner refs, asset refs, type filters, delegation refs, transfer refs, dispute refs, related app/resource refs, and display state.
- `workspace_surface_view`: workspace refs, object refs, document/table/page refs, share refs, version refs, comment/approval refs, AI context grant refs, offline refs, and display state.
- `directory_surface_view`: listing refs, category refs, locality refs, media refs, moderation refs, report/dispute refs, search/map/message handoff refs, and display state.
- `app_catalog_view`: app refs, package refs, release refs, namespace refs, permission refs, usage model refs, incident refs, review/support refs, and local pin/open state.
- `identity_profile_view`: identity refs, profile refs, tenant/org refs, handle refs, credential/session/device refs, visibility refs, recovery refs, and display state.
- `namespace_manager_session`: namespace refs, route refs, target refs, verification refs, delegation refs, transfer refs, tombstone/dispute refs, and action draft state.
- `privacy_permission_dashboard`: actor refs, app/service refs, permission grant refs, revocation refs, retention refs, AI/location/vault/RAG grant refs, denied-attempt refs, and review state.
- `secure_storage_view`: vault item refs, grant refs, sealed reference refs, rotation refs, recovery refs, backup/export eligibility refs, and redacted display state.
- `rag_index_connection`: index refs, owner/source refs, freshness refs, RAG grant refs, allowed model refs, retrieval receipt refs, redaction refs, and sync state.
- `dispute_appeal_dashboard`: case refs, evidence refs, receipt refs, affected service refs, deadline refs, remedy refs, decision refs, and appeal state.
- `provider_payout_view`: provider refs, node refs, earning refs, hold refs, correction refs, payout refs, compliance refs, tax/profile refs, and display state.
- `grant_public_interest_view`: grant refs, project refs, eligibility refs, application refs, sponsored credit refs, usage report refs, stewardship refs, and display state.
- `activity_timeline_view`: receipt refs, event refs, replay refs, service refs, redaction refs, filters, export refs, and dispute handoff refs.
- `node_fleet_view`: fleet refs, node refs, health refs, rule refs, access refs, lease refs, earning refs, incident refs, update refs, and bulk action draft state.
- `developer_console_session`: project refs, app refs, manifest refs, package validation refs, policy dry-run refs, deployment preview refs, namespace drafts, logs, and test environment refs.
- `release_rollback_view`: release refs, rollout refs, health gate refs, package refs, backup refs, restore refs, rollback refs, route refs, incident refs, and action draft state.
- `governance_center_view`: PIP refs, stewardship report refs, central-AI recommendation refs, compliance/security/incident refs, review refs, comment refs, publication refs, and display state.
- `overdesk_notification_pref`: actor/org refs, notification classes, delivery prefs, quiet hours, redaction prefs, and state.
- `overdesk_usage_ref`: desktop shell, node onboarding, resource-rule edit, access-policy edit, address resolution, native-app view, credit purchase, app analytics, deployment, asset view, diagnostic export, compute, storage, and bandwidth usage refs.

Common envelope fields: `id`, `tenant_id`, `actor_id`, `organization_id`, `device_id`, `app_id`, `trace_id`, `idempotency_key`, `schema_version`, `state`, `created_at`, `updated_at`, `policy_refs`, `usage_refs`, and `audit_refs`.

## API Surface

Overdesk APIs are desktop-client support APIs and gateway-facing client calls. They do not replace owner-service APIs.

- `GET /overdesk/profile`: returns desktop profile, session state, node state, selected account, and allowed desktop capabilities.
- `PATCH /overdesk/profile`: updates local display, notification, retention, shell, and accessibility prefs.
- `POST /overdesk/devices/onboarding`: starts or resumes device onboarding.
- `GET /overdesk/devices/{device_id}`: returns desktop-safe node and hardware summary projection.
- `POST /overdesk/devices/{device_id}/install`: starts Node Installer or update handoff.
- `POST /overdesk/devices/{device_id}/benchmark`: starts benchmark handoff where allowed.
- `GET /overdesk/resource-rules`: lists user-visible resource sharing rules.
- `POST /overdesk/resource-rules`: submits a signed rule change through owner services.
- `POST /overdesk/resource-rules/{rule_id}/preview`: runs a policy dry-run and scheduling preview.
- `POST /overdesk/resource-rules/{rule_id}/pause`: pauses or drains sharing through owner service APIs.
- `GET /overdesk/access-rules`: lists access policies and private UUID rules.
- `POST /overdesk/access-rules`: submits a signed access-rule change through owner services.
- `POST /overdesk/access-rules/{policy_id}/preview`: runs policy dry-run for allowed institutions/tags/private UUIDs.
- `POST /overdesk/browser/resolve`: resolves an address-bar input through namespace, route, policy, and optional search fallback.
- `GET /overdesk/apps/sessions`: lists embedded native-app sessions.
- `POST /overdesk/apps/open`: opens a native app, route, namespace, or app-owned page.
- `POST /overdesk/wallet/credit-purchase-intents`: creates a credit-purchase intent through Wallet/Overbill.
- `GET /overdesk/owned-apps`: lists owned apps and app-owner dashboard summaries.
- `GET /overdesk/owned-apps/{app_id}`: returns app detail analytics projection.
- `POST /overdesk/deployments/wizard`: starts a deployment wizard session.
- `POST /overdesk/deployments/wizard/{session_id}/validate`: validates package/manifest and policy preview.
- `POST /overdesk/deployments/wizard/{session_id}/submit`: submits signed deployment intent through Deployment Planner.
- `GET /overdesk/assets`: returns Overasset inventory projections.
- `POST /overdesk/assets/{asset_id}/actions`: drafts or submits allowed delegation, revocation, transfer, bind, export, or dispute handoff actions.
- `GET /overdesk/workspaces`: returns Workspace projections, recent objects, share state, offline state, and usage refs.
- `POST /overdesk/workspaces/{workspace_id}/actions`: drafts allowed share, revoke, import, export, approval, AI-assist, or open-object handoffs.
- `GET /overdesk/directory`: returns Directory Listing projections, filters, map/search handoffs, moderation state, and listing usage refs.
- `POST /overdesk/directory/listings`: drafts or submits a listing create/update request through Directory Listings.
- `GET /overdesk/catalog/apps`: returns Native App Catalog projections, app detail summaries, permission labels, usage labels, and release/incident state.
- `POST /overdesk/catalog/apps/{app_id}/open`: opens, pins, updates, removes, or reviews an app through owner-service route contracts where authorized.
- `GET /overdesk/identity-profile`: returns profile, identity, session, device, handle, visibility, and delegated-scope projections.
- `POST /overdesk/identity-profile/actions`: drafts or submits profile, session, device, visibility, or delegated-scope actions through owner services.
- `GET /overdesk/namespaces`: returns owned, delegated, requested, disputed, and app-bound namespace projections.
- `POST /overdesk/namespaces/actions`: drafts or submits namespace create, bind, release, transfer, delegation, verification, or dispute handoffs.
- `GET /overdesk/privacy-permissions`: returns permission, grant, retention, location, AI, RAG, vault, notification, and denied-access projections.
- `POST /overdesk/privacy-permissions/actions`: drafts or submits revoke, narrow, expire, renew, review, or export-permission actions.
- `GET /overdesk/vault`: returns Overvault-safe projections for vault items, grants, sealed refs, rotation, recovery, and backup eligibility.
- `POST /overdesk/vault/actions`: drafts or submits grant, revoke, rotate, reveal-request, recovery, export, or backup handoffs.
- `GET /overdesk/rag-indexes`: returns encrypted Docdex/RAG index projections, freshness, model eligibility, grants, and retrieval receipts.
- `POST /overdesk/rag-indexes/actions`: drafts or submits connect, sync, disconnect, grant, revoke, or inspect-receipt actions.
- `GET /overdesk/disputes`: returns disputes and appeals visible to the actor.
- `POST /overdesk/disputes`: drafts or submits a dispute or appeal through the owning dispute service.
- `GET /overdesk/provider/earnings`: returns provider earning, hold, correction, payout, compliance, and node-contribution projections.
- `POST /overdesk/provider/payout-actions`: drafts or submits payout-method, payout-request, hold-appeal, correction, or export actions.
- `GET /overdesk/grants`: returns grants, public-interest projects, sponsored credits, applications, and stewardship report refs.
- `POST /overdesk/grants/actions`: drafts or submits grant apply, accept, renew, report, link-resource, or evidence actions.
- `GET /overdesk/activity`: returns redacted activity, receipt, audit, replay, and export projections.
- `GET /overdesk/fleet/nodes`: returns node fleet projections, health, rules, leases, earnings, incidents, and updates.
- `POST /overdesk/fleet/actions`: drafts or submits allowed bulk rule, access, pause, drain, update, tag, or support-bundle actions.
- `GET /overdesk/developer`: returns developer projects, manifests, validation refs, deployment previews, namespace drafts, logs, and local environment state.
- `POST /overdesk/developer/actions`: drafts or submits package validate, policy dry-run, manifest update, namespace draft, test, or deployment-preview actions.
- `GET /overdesk/releases`: returns release, rollout, health gate, rollback, backup/restore, route, migration, and incident projections.
- `POST /overdesk/releases/actions`: drafts or submits release pause, resume, promote, rollback request, route-shift request, or health-review actions.
- `GET /overdesk/governance`: returns PIP, stewardship, central-AI recommendation, compliance, security, incident, migration, and public-report projections.
- `POST /overdesk/governance/actions`: drafts or submits allowed comment, follow, review, correction, appeal, or publication-review actions.
- `GET /overdesk/replay/{record_id}`: reconstructs desktop flow, owner-service refs, usage refs, policy refs, and audit refs visible to the actor.

Stable errors include `desktop_session_required`, `device_not_supported`, `installer_permission_denied`, `node_onboarding_denied`, `hardware_discovery_unavailable`, `benchmark_denied`, `resource_rule_invalid`, `access_policy_invalid`, `private_uuid_invalid`, `policy_preview_denied`, `namespace_not_found`, `route_denied`, `app_session_denied`, `wallet_precheck_failed`, `purchase_intent_denied`, `owned_app_not_visible`, `analytics_scope_denied`, `deployment_manifest_invalid`, `deployment_policy_denied`, `asset_not_visible`, `asset_action_denied`, `workspace_scope_denied`, `listing_action_denied`, `catalog_action_denied`, `profile_action_denied`, `namespace_action_denied`, `permission_action_denied`, `vault_action_denied`, `rag_index_action_denied`, `dispute_action_denied`, `payout_action_denied`, `grant_action_denied`, `activity_scope_denied`, `fleet_action_denied`, `developer_action_denied`, `release_action_denied`, `governance_action_denied`, `local_cache_unavailable`, and `overdesk_state_conflict`.

## Event Surface

- `overdesk.desktop_session_started`
- `overdesk.desktop_session_restricted`
- `overdesk.device_onboarding_started`
- `overdesk.device_onboarding_completed`
- `overdesk.device_onboarding_failed`
- `overdesk.resource_rule_previewed`
- `overdesk.resource_rule_updated`
- `overdesk.provider_pause_requested`
- `overdesk.access_policy_previewed`
- `overdesk.access_policy_updated`
- `overdesk.private_uuid_rule_updated`
- `overdesk.address_resolved`
- `overdesk.address_resolution_denied`
- `overdesk.embedded_app_opened`
- `overdesk.credit_purchase_intent_created`
- `overdesk.owned_app_dashboard_viewed`
- `overdesk.deployment_wizard_started`
- `overdesk.deployment_intent_submitted`
- `overdesk.asset_inventory_viewed`
- `overdesk.asset_action_submitted`
- `overdesk.workspace_surface_opened`
- `overdesk.directory_listing_action_submitted`
- `overdesk.app_catalog_opened`
- `overdesk.identity_profile_action_submitted`
- `overdesk.namespace_action_submitted`
- `overdesk.permission_action_submitted`
- `overdesk.vault_action_submitted`
- `overdesk.rag_index_action_submitted`
- `overdesk.dispute_appeal_submitted`
- `overdesk.provider_payout_action_submitted`
- `overdesk.grant_action_submitted`
- `overdesk.activity_timeline_viewed`
- `overdesk.node_fleet_action_submitted`
- `overdesk.developer_action_submitted`
- `overdesk.release_action_submitted`
- `overdesk.governance_action_submitted`
- `overdesk.support_bundle_exported`
- `overdesk.usage_emitted`

Events include actor refs, tenant/org refs, device refs, app refs, namespace refs, policy refs, owner-service refs, usage refs, audit refs, redaction class, and stable reason codes. Events must not include raw payment secrets, raw private keys, vault secrets, decrypted private data, raw messages, raw social media, precise location trails beyond explicit map permission, raw RAG context, provider-sensitive internals, or fraud internals outside the viewer's role.

## Core Workflows

### Add A Computer To The Network

1. User opens Overdesk and chooses "Add this computer".
2. Overdesk verifies session, tenant/org scope, local permissions, OS compatibility, and installer status.
3. Hardware Discovery produces a desktop-safe capability summary.
4. User chooses first resource sharing preset and access rule.
5. Overdesk runs policy dry-run and displays reason codes.
6. User confirms signed onboarding command.
7. Node Installer, Overcell, Overregistry, Oververify, Overguard, Overwatch, and Overmeter receive normal service calls.
8. Overdesk displays joined state, health, rules, and usage refs.

### Set Usage Rules

1. Provider opens Resource Sharing Rules.
2. Provider adjusts schedules, resource percentages, caps, workload classes, and safety overrides.
3. Overdesk validates locally for obvious conflicts.
4. Overdesk runs Policy Dry-Run API and scheduling preview.
5. Provider signs update.
6. Owner services update authoritative rules.
7. Overdesk updates display, emits usage/audit refs, and shows active/pending state.

### Set Allowed Users, Institutions, Tags, And Private UUIDs

1. Provider opens Access Rules.
2. Provider adds institution, organization, user, tag, purpose tag, or private UUID allowlist entries.
3. Provider adds deny rules and expiry/review settings.
4. Overdesk runs Overguard and Purpose Tag Registry previews.
5. Provider signs update.
6. Owner services enforce authoritative policy.
7. Overdesk displays resulting rule state and denial explanations for future workloads.

### Browse `/hugo`

1. User types `/hugo` into the address bar.
2. Overdesk normalizes the input and requests namespace resolution.
3. Universal Namespace Service returns target refs or not-found state.
4. Overmesh returns route hints where live routes exist.
5. Overgate/Overguard validate actor, tenant, policy, app session, and data access.
6. Overdesk opens the app/page or displays a denial/not-found page with search/directory fallback.

### Buy Credits

1. User opens Credits page.
2. User selects account, ORU dimension, and purchase amount allowed by current policy.
3. Overdesk requests wallet/account precheck.
4. Overdesk creates credit-purchase intent through Wallet/Overbill.
5. External payment handoff happens through approved payment integration without Overdesk storing payment secrets.
6. Overbill reconciles payment refs, ORU Account Service updates projections, and Seal Ledger records accounting entries.
7. Overdesk displays receipt, projection update, and failure/refund paths.

### Inspect Owned App

1. App owner opens Owned Apps.
2. Overdesk requests app-owner summary from owner services.
3. User opens one app detail page.
4. Overdesk displays usage, costs, earnings, visitors/source-safe analytics, deployment state, version history, namespace refs, incident refs, support inbox refs, and disputes.
5. Actions such as pause release, deploy update, export analytics, or open dispute route to owner services.

### Deploy A New App

1. Builder opens Deploy New App.
2. Builder selects package, repo/import, template, or manifest.
3. Overdesk runs validation and resource estimate.
4. Overdesk runs policy dry-run and package validation.
5. Builder chooses namespace route and release strategy.
6. Wallet precheck confirms budget or grant availability.
7. Builder signs deployment intent.
8. Deployment Planner and related services execute the deployment plan.
9. Overdesk monitors deployment and rollback state.

### Manage Overasset-Owned Assets

1. User opens Assets.
2. Overdesk fetches Overasset inventory projection.
3. User filters by app, namespace, capacity right, service ownership, delegation, or dispute state.
4. User opens an asset detail page and reviews evidence refs, owner refs, expiry, rights, related apps/resources, and policy constraints.
5. User drafts delegation, revocation, transfer, binding, export, or dispute action.
6. Overasset and owner services validate and record authoritative state.

### Use Workspace, Directory, Catalog, Identity, And Namespace Surfaces

1. User opens Workspace, Directory, App Catalog, Identity, or Namespace from the shell or address bar.
2. Overdesk resolves the selected account, organization, institution, app-owner, and delegated scope.
3. Overdesk fetches a permission-safe projection from the owning service.
4. User drafts the desired action: open document, share/revoke workspace access, create/edit listing, open/pin app, update profile visibility, or bind a namespace.
5. Overdesk shows source refs, permission implications, usage refs, and denial/fallback paths.
6. User confirms where the action is high-risk or mutating.
7. Owner services record authoritative state and return audit/replay refs.

### Manage Privacy, Vault, RAG, Disputes, Payouts, And Grants

1. User opens Privacy, Vault, RAG, Disputes, Provider, or Grants.
2. Overdesk builds a consolidated projection from owner services without copying raw private content, secrets, RAG context, or fraud internals.
3. User reviews active grants, vault access, index connections, disputes, holds, payouts, grants, and public-interest project status.
4. User drafts a revoke, rotate, connect, sync, dispute, appeal, payout, grant, or evidence-submission action.
5. Overdesk runs local validation for missing fields and requests policy/precheck decisions where needed.
6. User confirms signed action with affected account, service, refs, and rollback/dispute path.
7. Owner services update authoritative state and Overdesk records view/update refs only.

### Manage Activity, Fleet, Developer, Releases, And Governance

1. User opens Activity, Fleet, Developer, Releases, or Governance.
2. Overdesk fetches redacted projections for receipts, nodes, projects, releases, PIPs, reports, incidents, reviews, and central-AI recommendations.
3. User filters by account, organization, device, app, namespace, service, risk class, or time window.
4. User opens replay, exports receipts, creates bulk node-rule drafts, validates packages, previews deployments, requests release actions, follows PIPs, or submits comments/reviews where authorized.
5. High-risk actions require fresh credential verification and owner-service prechecks.
6. Owner services record final state; Overdesk keeps display state, replay refs, and local diagnostics.

## State Machines

Desktop session lifecycle:

1. `unconfigured`
2. `configured`
3. `login_required`
4. `active`
5. `restricted`
6. `refresh_required`
7. `offline_limited`
8. `logged_out`
9. `revoked`

Node onboarding lifecycle:

1. `not_started`
2. `system_check`
3. `installer_ready`
4. `hardware_discovered`
5. `benchmark_optional`
6. `policy_previewed`
7. `signed`
8. `registering`
9. `joined`
10. `failed`
11. `paused`

Resource rule lifecycle:

1. `draft`
2. `local_validated`
3. `policy_previewed`
4. `signed`
5. `pending_owner_service`
6. `active`
7. `restricted`
8. `paused`
9. `denied`
10. `archived`

Address resolution lifecycle:

1. `input_received`
2. `normalized`
3. `namespace_lookup`
4. `route_lookup`
5. `policy_check`
6. `app_session_check`
7. `opened`
8. `search_fallback`
9. `denied`
10. `not_found`

Credit purchase lifecycle:

1. `draft`
2. `wallet_prechecked`
3. `intent_created`
4. `payment_handoff`
5. `payment_pending`
6. `settled`
7. `credited`
8. `receipt_ready`
9. `failed`
10. `cancelled`
11. `refunded`

Deployment wizard lifecycle:

1. `draft`
2. `source_selected`
3. `manifest_validated`
4. `resource_estimated`
5. `policy_previewed`
6. `namespace_selected`
7. `wallet_prechecked`
8. `signed`
9. `submitted`
10. `deploying`
11. `healthy`
12. `failed`
13. `rollback_requested`
14. `rolled_back`

## Policy And Security

- Every mutating action uses signed request envelopes, actor refs, tenant/org refs, trace id, idempotency key, schema version, source refs, policy refs where required, and Overwatch audit refs.
- Desktop local cache is encrypted where the platform supports it, scoped by account/tenant/device, bounded by retention class, and user-clearable.
- Resource sharing defaults to off until the user explicitly enables a rule.
- Newly added resource nodes default to the safest available sharing preset, not broad public execution.
- Public resource use requires explicit workload class, policy, provider, verification, and abuse-control readiness.
- Private UUIDs are treated as sensitive access grants. They are never displayed broadly, logged in raw form, or reused across unrelated scopes.
- Address-bar history, search history, assistant history, map/location history, and app analytics history are retention-controlled and clearable.
- Credit-buy screens never store card details or payment secrets and must clearly separate external payment processing from internal ORU/Seal Ledger accounting.
- Exact location, private messages, private workspace data, encrypted RAG context, private media, and asset transfer actions require explicit purpose-bound permission.
- High-risk actions require fresh credential/session checks and clear source-service confirmation.
- Support bundles are redacted by default and require user review before export.

## Metering And Accounting

- Overdesk emits usage refs for desktop shell operations, address resolution, native-app sessions, node onboarding, resource-rule previews, access-policy previews, credit purchase intents, owned-app analytics, deployment wizard actions, asset inventory views, support bundles, local cache sync, compute, storage, and bandwidth.
- Owner services emit authoritative usage and accounting facts. Overdesk displays those facts through Wallet and Usage Center projections.
- Overdesk must show credit usage and earnings for owned apps only through ORU, Seal Ledger, Overmeter, Overbill, Provider Payout Service, and owner-service refs.
- Provider resource usage for this computer is displayed as a projection and must cite Overmeter, Overlease, Overcell, Overmark, ORU, Seal Ledger, and payout refs.
- Overdesk must avoid per-transaction external payment prompts for normal machine-to-machine usage. Internal ORU and Seal Ledger accounting keeps low-friction usage settlement possible.

## Local Storage And Offline Behavior

- Allowed offline state: encrypted local session refs, non-secret credential refs, cached shell prefs, account selector cache, resource rule drafts, access rule drafts, address history where retained, bookmarks, native-app view cache where owner-service policy allows, deployment wizard drafts, asset inventory snapshots, diagnostics, and support-bundle drafts.
- Denied offline finality: credit purchase settlement, ledger mutation, app deployment finality, resource-rule authority, access-policy authority, asset transfer finality, messaging delivery finality, social publication finality, map correction finality, search index changes, AI execution, and provider payout changes.
- Offline mode may allow safe drafts, local previews, read-only cached views, and queued low-risk requests. Reconnect must revalidate identity, tenant, policy, account, device, owner-service state, and expiry before authoritative mutation.
- Cache invalidation triggers include logout, device revoke, credential revoke, tenant switch, permission revoke, policy change, app removal, asset hold/dispute, local clear-cache request, and owner-service tombstone.

## Observability And Operations

- Desktop diagnostics track app version, platform, session state, device state, installer state, node health summary, rule preview failures, route resolution failures, native-app session failures, deployment wizard failures, asset action failures, credit purchase state, cache errors, and support-bundle exports.
- Metrics are redacted and local-first by default. No hidden analytics pipeline is allowed.
- Support bundles include stable ids, trace ids, source refs, policy reason codes, usage refs, and audit refs, but not secrets or private payloads.
- Crash recovery restores draft state and pending signed requests only where idempotency and expiry make replay safe.
- Auto-updates must be signed, rollback-capable, and compatible with node-agent update rules where the local desktop app manages Overcell/Node Installer handoffs.

## Failure Modes

- Installer unavailable: show required permissions, platform requirements, and retry path without marking node joined.
- Hardware discovery partial: allow read-only summary with missing capability warnings and block sharing until required facts exist.
- Policy preview denied: show stable reason codes and suggested safe alternatives without bypassing Overguard.
- Node joined but paused: display paused state and current reason.
- Address not found: show search/directory fallback and namespace claim/dispute info where allowed.
- Route denied: show account/permission/policy reason and owner-service support path.
- Credit purchase failed: show payment/refund/retry state from Overbill without local crediting.
- Deployment validation failed: show package, manifest, policy, namespace, wallet, or compatibility reason codes.
- App analytics denied: show missing role/scope rather than leaking visitor or revenue data.
- Asset action denied: show Overasset reason code and dispute path where allowed.
- Local cache corrupt: discard cache, preserve server truth, and offer support bundle.

## Build Breakdown

### Phase 1: Product Shell And Contracts

- Create Overdesk product package, route model, account selector, session model, shell navigation, notification model, local cache interfaces, and generated API bindings.
- Define page route contracts for all native app surfaces.
- Add stable errors, fixtures, and redacted diagnostics baseline.

### Phase 2: Device Onboarding And Resource Provider UX

- Implement Add This Computer flow, installer handoff, hardware discovery display, benchmark display, first sharing preset, node health summary, and pause/drain controls.
- Implement Resource Sharing Rules with schedules, resource caps, workload classes, and policy previews.
- Implement Access Rules with institutions, organizations, tags, purpose tags, users, private UUIDs, deny rules, and expiry.

### Phase 3: Browser And Native App Host

- Implement top address bar, namespace resolution, route resolution, tabs, bookmarks, history, trust markers, route-denied views, and search/directory fallback.
- Embed initial Wallet, Messaging, Search, AI Assistant, Social, Maps, Workspace, Directory, App Catalog, Identity/Profile, Namespace, Central AI, and Overasset app sessions through owner-service contracts.

### Phase 4: Wallet, Credits, Owned Apps, Assets, Privacy, And RAG

- Implement Wallet page embedding, credit purchase intent flow, owned app dashboard, app detail analytics, app-owner exports, asset inventory, Privacy and Permissions Center, Overvault Secure Storage Center, Docdex/RAG Index Manager, Disputes and Appeals Center, Provider Earnings and Payout Center, Grants and Public-Interest Projects, and Audit/Receipts Timeline.
- Ensure all accounting, privacy, RAG, dispute, payout, grant, and rights actions route to ORU, Seal Ledger, Overbill, Wallet, Provider Payout Service, Overmeter, Overclaim, Overgrant, Overvault, AI Gateway/Docdex adapter, and Overasset owner APIs.

### Phase 5: Developer, Fleet, Deployment, Release, And Governance

- Implement source selection, manifest validation, resource estimation, policy dry-run, namespace selection, wallet precheck, deployment graph preview, signed submission, monitor, rollback request, and support bundle.
- Implement Node Fleet Manager, Developer Console, Release and Rollback Manager, and Governance Center projections and action drafts.
- Keep developer, release, fleet, and governance actions behind owner-service checks, release gates, policy dry-runs, security/compliance refs, and replayable audit refs.

### Phase 6: Security, Offline, Packaging, And Release

- Harden local encryption, retention, clear-cache, support-bundle review, crash recovery, signed update, rollback, platform packaging, accessibility, localization hooks, and Phase 13 threat/security review inputs.

## Validation

- A new user can install Overdesk, sign in, and add the computer to Overrid without terminal commands.
- A provider can set day/night/hour schedules, resource percentages, idle-only behavior, and emergency pause.
- A provider can restrict resource use by institutions, organizations, tags, purpose tags, users, and private UUIDs.
- `/hugo` resolves through namespace/route/policy flow or shows a clear not-found/denied/fallback state.
- Messaging, Search, AI, Social, Maps, Workspace, Directory, App Catalog, Wallet, Credits, Owned Apps, Deploy, Assets, Identity, Namespace, Privacy, Vault, RAG, Disputes, Provider, Grants, Activity, Fleet, Developer, Releases, and Governance pages are reachable from the shell.
- Credit buying creates a purchase intent and displays Overbill/ORU/Seal Ledger refs without local ledger mutation.
- Owned app detail pages show credit usage, credit earnings, resource costs, visitors/source-safe analytics, deployment state, and disputes from owner-service refs.
- Deployment wizard rejects invalid manifests, unsafe policies, insufficient wallet/precheck state, and unauthorized namespace use.
- Overasset page displays asset inventory and routes all rights-changing actions to Overasset.
- Privacy, Vault, RAG, Disputes, Provider, Grants, Activity, Fleet, Developer, Releases, and Governance pages display owner-service projections and route every mutating action through authoritative services.
- Local cache can be cleared without corrupting authoritative state.
- Support bundles are redacted and reviewed before export.

## Open Questions

Resolved decisions:

- The first implementation should use a Tauri-style desktop shell with Rust core bindings and generated Overrid SDK contracts. This matches the accepted Rust-first stack while still allowing TypeScript/web client components for desktop UI surfaces. Electron is not the default first build because of resource and sandboxing cost, and a fully native platform split waits until the generated SDK, mobile bindings, signing envelopes, and desktop threat model are stable enough to justify separate platform clients.
- Overdesk should install, verify, start, stop, update, pause, drain, and inspect Overcell as a separately signed supervised local service through Node Installer. Overcell must not be embedded into the Overdesk UI process, because it owns node-agent lifecycle, heartbeat, capability, assignment, local audit-spool, and lease-bound handoff behavior. Overdesk may present service status and signed action drafts, but Node Installer and Overcell remain the authoritative installer and node-agent boundaries.
- The first credit-purchase intent flow should use an Overbill-managed provider adapter allowlist, not a provider hardcoded into Overdesk. For the first target jurisdiction, only sandbox/dev providers and one Compliance Boundary-approved external fiat rail adapter may be enabled, with Overbill recording payment-provider refs, reconciliation refs, refund/chargeback refs, and compliance refs. Overdesk never stores card data, bank credentials, raw provider tokens, payment secrets, tax secrets, or local balance truth, and crypto/token/NFT-style payment mechanics stay out of the first flow.
- Closed resource-access pilots should use opaque `private_access_uuid_v1` records backed by random 128-bit UUIDv4-compatible values, stored and compared by scoped digest where possible and displayed only in masked form. Each UUID is scoped to tenant/account, provider or node group, resource policy, purpose tag, expiry, and policy refs; UUIDs are never reused across unrelated scopes. Default pilot grants expire in 30 days or less, rotate immediately on suspected exposure, membership change, policy narrowing, device compromise, or pilot closure, and leave revoked/tombstoned audit refs for Overguard, Overwatch, Overclaim, and Access Rules replay.
- First-release local native/Tauri views are the Overdesk-owned shell and system pages: Home, Overrid Browser chrome, Global Search entry, Notifications, Activity/Receipts shell, Add This Computer, Resource Sharing Rules, Access Rules, Local Device Settings, Local Cache and Offline Sync, Diagnostics and Support Bundles, Updates, Settings/Security shell, and Help. Wallet, Buy Credits, Messaging, Personal AI Assistant, Workspace, Directory, Social, Maps, Native App Catalog, Central AI Stewardship, Owned Apps, App Detail, Deploy, Developer, Release/Rollback, Governance, Profile, Namespace, Privacy, Vault, RAG, Disputes, Provider Earnings, Grants, Overasset, Fleet, and related detail pages are embedded owner-service app sessions rendered inside Overdesk through generated contracts and signed action envelopes. Remote or sandboxed web surfaces are allowed only for external provider handoffs, app-owned public content, or temporary compatibility views, and they must use strict webview/external-browser isolation with no broad filesystem, node-agent, wallet, vault, RAG, or local-network authority.

## Handoff

Overdesk is SDS #84 because it turns the existing Overrid service ecosystem into a usable desktop product. It should be built after the wallet, identity, node onboarding, policy, accounting, namespace, deployment, and native-app contracts are stable enough to expose through one client shell. The first product release should focus on joining the network, setting resource rules, browsing Overrid addresses, wallet/credits, messaging/search/AI, workspace/directory/catalog, privacy/vault/RAG, owned apps, deployment, assets, disputes, payouts, grants, activity, fleet, developer, release, and governance views, then deepen each embedded surface as its owner service matures.
