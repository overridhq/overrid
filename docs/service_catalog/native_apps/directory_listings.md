# Directory Listings Implementation Plan

## Objective

Build a Craigslist-like native utility for classifieds, services, jobs, housing, events, community groups, organization pages, local discovery, and disputes.

## First Build Phase

[Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md).

## Dependencies

- Overbase.
- Overstore.
- Overvault.
- Overpass namespace.
- Universal Namespace Service.
- Overguard and Overwatch.
- Search Engine.
- Messaging Center.
- Maps and Navigation.
- Fraud Control and Reputation/Anti-Sybil.
- Overclaim.
- Overmeter, ORU Account Service, Seal Ledger, and Wallet/Usage Center.

## Development Order

1. Define listing categories and listing schema.
2. Add listing creation, edit, expiration, moderation, and local discovery.
3. Add organization/business pages.
4. Add search and messaging handoff.
5. Add map/place handoff, reputation, abuse reports, moderation, disputes, usage, and replay.

## Contracts And Interfaces

- Listing schema.
- Category and locality refs.
- Organization page schema.
- Search and map handoff contracts.
- Reputation/dispute refs.
- Messaging handoff contract.
- Usage, audit, and replay contracts.

## Validation

- Users can create and search listings.
- Contact, search, and map handoffs preserve Directory-owned listing truth and downstream service authority.
- Abuse reports and disputes create evidence records.
- Directory search does not become an ad-trap ranking dependency.

## Handoff

Directory listings are an early native utility that exercises identity, search, messaging, moderation, and disputes without full social-network scale.

## Detailed SDS

The detailed design contract is [Directory Listings SDS](../../sds/native_apps/directory_listings.md).

## Sub-Build Plan

- [SUB BUILD PLAN #69 - Directory Listings](../../build_plan/sub_build_plan_069_directory_listings.md)

## Design Alignment

- Treat Directory Listings as a native public utility for listings and organization pages, not as an ad marketplace, payment service, messaging system, map database, or social feed.
- Require listing/category/locality/page records, media refs, contact handoffs, search index updates, map handoffs, abuse reports, moderation actions, dispute refs, and usage refs.
- Integrate with Search Engine, Messaging Center, Maps and Navigation, Overclaim, Fraud Control, and Wallet/Usage Center through explicit refs and events.
- Discovery and ranking must avoid ad-trap mechanics and preserve small-user visibility without hidden paid-placement dependence.
