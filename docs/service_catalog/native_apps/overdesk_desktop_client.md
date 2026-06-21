# Overdesk Desktop Client Implementation Plan

## Objective

Build Overdesk, the installable desktop front face for Overrid. It lets users add a computer to the network, set resource sharing and access rules, browse Overrid addresses, use native apps, buy credits, manage wallet/account views, inspect owned apps, deploy new apps, manage Overasset-owned assets, and operate the main desktop surfaces for workspace, directory listings, app catalog, identity/profile, namespaces, privacy, vault, Docdex/RAG indexes, disputes, provider payouts, grants, activity receipts, node fleets, developer workflows, release/rollback, and governance.

## First Build Phase

[Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md), with Phase 2/3 node onboarding dependencies and Phase 13 hardening.

## Dependencies

- Overgate, Overpass, Overtenant, Overkey, and Overwatch.
- Node Installer, Hardware Discovery, Overcell, Benchmark Runner, Oversched, Overlease, Overrun, Overmeter, and Overmesh.
- Overguard, Policy Dry-Run API, Workload Classifier, Oververify, Overclaim, and Reputation/Anti-Sybil.
- ORU Account Service, Seal Ledger, Overbill, Overmark, Overgrant, Provider Payout Service, and Overasset.
- Overbase, Overstore, Overvault, Universal Namespace Service, Deployment Planner, Package Validator, Release Strategy Service, and Grid-Resident Service Packager.
- Wallet and Usage Center, Messaging Center, Search Engine, Personal AI Assistant, Social Photo/Video App, Maps and Navigation, Workspace and Office Suite, Directory Listings, and Central AI Stewardship Interface.
- Protocol Improvement Proposal Registry, Stewardship Reporting Service, Compliance Boundary Service, Threat Modeling and Security Review Tracker, Incident Response Service, Backup and Restore Service, Failover and Recovery Coordinator, Encrypted Docdex RAG Adapter, AI Gateway Router, ADES Enrichment Adapter, and Lightweight Classifier.

## Development Order

1. Build the Overdesk shell, account selector, session handling, local encrypted cache, navigation, notification center, and address bar.
2. Build Add This Computer onboarding with installer handoff, hardware discovery display, benchmark display, and node health summary.
3. Build Resource Sharing Rules for day/night/hour schedules, resource percentages, idle-only mode, bandwidth caps, safety limits, and pause/drain controls.
4. Build Access Rules for institutions, organizations, users, tags, purpose tags, private UUID allowlists, deny rules, expiry, and policy previews.
5. Build Overrid Browser resolution for addresses such as `/hugo`, namespace refs, route refs, tabs, bookmarks, and search/directory fallback.
6. Embed native app views for Messaging, Search, Personal AI Assistant, Social, Maps, Wallet, Workspace, Directory, Native App Catalog, Identity/Profile, Namespace Manager, Central AI stewardship, and Overasset.
7. Build Wallet, Credits, and credit-purchase intent screens through Wallet/Overbill/ORU/Seal Ledger owner APIs.
8. Build Owned Apps dashboards with credit usage, credit earnings, resource costs, visitors/source-safe analytics, deployment health, incidents, and disputes.
9. Build Privacy and Permissions, Overvault Secure Storage, Docdex/RAG Index Manager, Disputes and Appeals, Provider Earnings and Payouts, Grants and Public-Interest Projects, and Audit/Receipts Timeline through owner-service projections and signed action drafts.
10. Build Node Fleet Manager, Developer Console, Deploy New App wizard, Release and Rollback Manager, and Governance Center through Overpack, Package Validator, Policy Dry-Run API, Deployment Planner, namespace binding, wallet precheck, Release Strategy Service, PIP Registry, Stewardship Reporting, Incident Response, Backup/Restore, and Failover/Recovery owner APIs.
11. Harden local cache, support bundles, offline behavior, signed updates, rollback, accessibility, and platform packaging.

## Contracts And Interfaces

- Desktop shell route contract.
- Local device profile contract.
- Node onboarding and installer handoff contract.
- Resource sharing rule contract.
- Resource access policy and private UUID contract.
- Address-bar namespace resolution contract.
- Embedded native-app session contract.
- Credit purchase intent contract.
- Owned-app analytics projection contract.
- Deployment wizard session contract.
- Overasset inventory and action-draft contract.
- Workspace, directory, app catalog, identity/profile, and namespace projection contracts.
- Privacy/permissions, vault, encrypted Docdex/RAG, disputes, payout, grant, and activity timeline projection contracts.
- Node fleet, developer console, release/rollback, and governance projection contracts.
- Redacted support bundle and replay contract.

## Validation

- User can add a desktop computer to Overrid without terminal commands.
- User can set day/night/hour/resource-percentage sharing rules and pause sharing.
- User can restrict usage by institution, tag, purpose tag, user, organization, and private UUID.
- `/hugo` resolves through namespace/route/policy flow or gives a clear fallback.
- Messaging, Search, AI Assistant, Social, Maps, Workspace, Directory, App Catalog, Wallet, Credits, Owned Apps, Deploy, Assets, Identity, Namespace, Privacy, Vault, RAG, Disputes, Provider, Grants, Activity, Fleet, Developer, Releases, and Governance are reachable from the app shell.
- Credit purchase creates an owner-service purchase intent and never mutates ORU or Seal Ledger truth locally.
- Owned app pages show usage, earnings, costs, visitors/source-safe analytics, deployment state, and disputes from authoritative refs.
- Deployment wizard rejects invalid, unsafe, unauthorized, or underfunded deployment attempts.
- Privacy, vault, RAG, dispute, payout, grant, activity, fleet, developer, release, and governance actions route through owner services and do not become local Overdesk authority.
- Local diagnostics and support bundles are redacted by default.

## Handoff

Overdesk should become the default desktop entry point for Overrid users, providers, app owners, builders, institutions, and stewards. It must stay an ordinary Overrid client that routes authority to owning services instead of becoming a privileged bypass.

## Detailed SDS

The detailed design contract is [Overdesk Desktop Client SDS](../../sds/native_apps/overdesk_desktop_client.md).

## Sub-Build Plan

- [SUB BUILD PLAN #84 - Overdesk Desktop Client](../../build_plan/sub_build_plan_084_overdesk_desktop_client.md)

## Design Alignment

- Treat Overdesk as the product shell and desktop orchestration client for Overrid, not as a ledger, policy engine, node authority, app database, search engine, AI router, payment processor, or asset registry.
- Make adding a computer, setting provider rules, buying credits, browsing Overrid addresses, and deploying apps obvious to regular users.
- Keep all native-app, accounting, privacy, RAG, dispute, payout, grant, activity, fleet, developer, release, and governance actions routed through owner-service APIs with usage refs, audit refs, policy refs, and stable denials.
- Avoid blockchain, NFT, per-transaction fee, hardcoded pricing, customer-count, revenue projection, ad-trap, hidden tracking, or addiction-loop assumptions.
