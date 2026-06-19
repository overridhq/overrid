# Phase 12: Native Application Layer

## Objective

Build native public utilities on top of Overrid as ordinary clients of the same identity, policy, storage, routing, metering, privacy, and accounting rails.

Native apps are non-profit oriented public utilities. They should charge for resource usage where needed, keep cost and revenue close, and route surplus through central AI stewardship rules rather than private extraction.

## Depends On

- Phase 9 deployment platform.
- Phase 8 data, storage, vault, namespace, and route substrate.
- Phase 5 ORU, Seal Ledger, and Overbill.
- Phase 4 policy and abuse controls.
- Phase 11 public low-sensitivity pool where appropriate.

## Build Order

1. Wallet and usage center.
2. Personal native AI assistant.
3. Workspace and office suite.
4. Directory listings.
5. Search engine.
6. Messaging center.
7. Social photo/video app.
8. Maps and navigation.
9. Central AI stewardship interface and governance console.
10. Mobile service layer for approved native and third-party mobile apps.

## Workstream 1: Wallet And Usage Center

Build first because every native service needs:

- ORU balances.
- Resource usage.
- Holds.
- Grants.
- Sponsored credits.
- Receipts.
- Refunds.
- Account statements.
- Privacy controls.
- App permissions.

This becomes the user's control panel for the ecosystem.

## Workstream 2: Personal Native AI Assistant

Build the assistant using:

- Central AI coordination mechanism.
- Encrypted Docdex indexes for personal, organization, and repo RAG.
- Gateway request classification.
- ADES-style entity extraction where useful.
- Smaller model routing for simple classification.
- Larger model routing for complex work.
- Available Overrid model resources.
- User permission and audit controls.
- ORU metering per model/resource operation.

The assistant should direct calls when needed rather than forcing every request through one expensive model.

## Workstream 3: Workspace And Office Suite

Build:

- Documents.
- Spreadsheets or structured tables.
- Presentations or shareable pages.
- Team folders.
- Permissions.
- Version history.
- AI-assisted editing.
- Search across workspace.
- Export and import.

This exercises Overbase, Overstore, Overvault, encrypted Docdex RAG, namespace routes, sharing, and billing.

## Workstream 4: Directory Listings

Build a Craigslist-like native directory:

- Classifieds.
- Local services.
- Jobs.
- Housing.
- Events.
- Community groups.
- Organization and business pages.
- Reputation and dispute records.
- Messaging handoff.
- Local discovery.

Start with directory listings before broad social media because the abuse surface is smaller and the utility is immediate.

## Workstream 5: Search Engine

Build search across:

- Public Overrid content.
- Native directory listings.
- Public app pages.
- Public-interest datasets.
- User-authorized private content.
- Workspace content where permitted.

Search must respect permissions. Ranking should not become an ad trap where small businesses are forced to pay escalating marketplace fees for visibility.

## Workstream 6: Messaging Center

Build a username-addressed replacement for fragmented email, phone, and chat identities:

- Direct username addressing.
- Organization inboxes.
- App notifications.
- Encrypted personal messages where appropriate.
- Attachments through Overstore.
- Identity verification markers.
- Spam and abuse controls.
- AI assistant triage with permission.

Messaging should be a protocol-level utility, not a lock-in product.

## Workstream 7: Social Photo/Video

Build only after stronger moderation and storage controls exist:

- Photo sharing.
- Video sharing.
- Feed and follow graph.
- Private groups.
- Media processing.
- Rights and attribution.
- Abuse controls.
- Age and safety controls where legally required.
- Transparent recommendations.

Avoid addiction-driven design. The app should serve people and communities, not maximize compulsive engagement.

## Workstream 8: Maps And Navigation

Build:

- Place records.
- Routes.
- Local discovery.
- Business and organization listings.
- Community map layers.
- Transit or mobility integrations where available.
- Privacy-preserving location controls.
- Offline or cached map areas where feasible.

Maps should integrate with directory listings, search, and messaging.

## Workstream 9: Central AI Stewardship Interface

Build an interface for:

- Project donations.
- Public-interest pools.
- Abuse and fraud evidence.
- Grant recommendations.
- System health.
- Native app surplus routing.
- Appeals and disputes.
- Governance reporting.

Central AI governance must be bounded by evidence, privacy rules, and appeal paths.

## Workstream 10: Mobile Service Layer

Build the mobile access layer using:

- Mobile backend gateway.
- Mobile SDK.
- Device registration and revocation.
- Mobile sessions through Overrid identity and credential rails.
- Sync cursors and compact delta responses.
- Offline command intake and idempotent replay.
- Push notification refs with redacted payload rules.
- Media upload session coordination.
- Wallet and usage readers.
- Personal AI, AI gateway, and encrypted Docdex RAG handoffs.
- Abuse, fraud, rate-limit, and mobile replay evidence.

Mobile apps should use Overrid as a backend/resource plane without bypassing Overgate, Overpass, Overtenant, Overkey, Overguard, Overwatch, Overmeter, ORU, Seal Ledger, storage, native app services, or AI routing.

## Validation

- Each native app uses normal Overrid APIs rather than private shortcuts.
- Wallet shows usage, holds, grants, refunds, and receipts.
- Personal AI assistant uses encrypted Docdex RAG and model/resource routing.
- Directory listing supports identity, listing, search, messaging handoff, and dispute basics.
- Native app surplus routing is represented structurally without speculative financial assumptions.
- Mobile SDK and Mobile Backend Gateway support session, sync, offline replay, push, media, wallet/usage, and AI/RAG flows through normal Overrid rails.

## Exit Gate

Phase 12 is complete when the first native services operate as real Overrid applications using shared rails for identity, storage, policy, usage, accounting, and governance.

## Handoff To Phase 13

Phase 13 hardens governance, compliance, security, operations, and scale so public participation can grow responsibly.
