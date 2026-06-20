SDS #69

# Directory Listings SDS

## Purpose

Build a Craigslist-like native utility for classifieds, services, jobs, housing, events, community groups, organization pages, local discovery, and disputes.

Directory Listings is a native public utility app. It owns listing records, categories, locality scopes, organization/business pages, listing lifecycle, moderation refs, search handoffs, map/place handoffs, messaging handoffs, abuse reports, dispute refs, and usage refs. It is not an ad marketplace, escrow service, payment authority, messaging system, search engine, maps database, or social feed. Its design should make useful local discovery possible without forcing small businesses and individuals into escalating ad spend or addictive engagement loops.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [directory_listings.md](../../service_catalog/native_apps/directory_listings.md) |
| Sub-build plan | [SUB BUILD PLAN #69 - Directory Listings](../../build_plan/sub_build_plan_069_directory_listings.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md) |

## Service Family

- Family: Native applications
- Owning layer: Native public utility application layer for local listings and community discovery
- Primary data scope: listings, category/locality refs, organization/business pages, media refs, contact/messaging handoff refs, search-index update refs, map/place refs, moderation refs, scam/abuse reports, dispute refs, reputation refs, usage refs, and audit refs
- First build phase from service plan: [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md)

## Problem Statement

People need a simple way to find and publish local goods, services, jobs, housing, events, community groups, and organization pages without feeding an extractive marketplace that manipulates visibility through ads and data harvesting. A useful directory also has real abuse risk: scams, spam, impersonation, prohibited goods, housing/job discrimination, fake business pages, unsafe meetings, and disputes.

Directory Listings must therefore be a narrow native utility built on Overrid's shared identity, namespace, storage, search, messaging, maps, policy, metering, and dispute rails. The key design fix is to separate listing truth from search ranking, messaging, payments, maps, and reputation while preserving enough refs for those services to interoperate.

## Goals

- Define category, locality, listing, organization/business page, moderation, and dispute records.
- Support listing create, draft, publish, edit, pause, renew, expire, archive, report, dispute, and remove flows.
- Integrate with Search Engine for permission-aware and anti-ad-trap discovery.
- Integrate with Messaging Center for contact handoff without exposing private contact information by default.
- Integrate with Maps and Navigation for place/locality display without making maps the listing authority.
- Integrate with Overclaim, Fraud Control, Reputation and Anti-Sybil Service, and Overwatch for abuse, scams, impersonation, and dispute evidence.
- Keep resource usage visible through Wallet/Usage Center and Overmeter without encoding financial projections.
- Provide an immediate Phase 12 native app that exercises identity, storage, search, messaging, moderation, disputes, and local discovery before broader social media.

## Non-Goals

- Do not become an advertising marketplace where visibility depends on escalating paid promotion.
- Do not own global search ranking algorithms, map/place truth, direct messages, payment settlement, escrow, identity verification, or final reputation scores.
- Do not bypass Overpass, Overtenant, Overkey, Universal Namespace Service, Overguard, Overwatch, Overmeter, Wallet/Usage Center, Overclaim, or Fraud Control.
- Do not store raw private messages, payment details, vault secrets, exact private locations, or unrelated user profiling data.
- Do not create addictive feed mechanics, dark patterns, surveillance tracking, or hidden data resale flows.
- Do not add pricing, customer-count, revenue, blockchain, NFT, or per-transaction fee assumptions.

## Primary Actors And Clients

- Individual users posting, editing, searching, saving, reporting, or disputing listings.
- Organizations and small businesses managing pages, service listings, events, jobs, or local offers.
- Moderators, stewards, and operators reviewing prohibited content, scams, impersonation, and disputes.
- Personal AI Assistant helping users search, draft listings, summarize replies, or report abuse with permission.
- Search Engine indexing public/authorized listing summaries and returning listing refs.
- Messaging Center creating contact threads or organization inbox handoffs.
- Maps and Navigation showing place/locality and route links for permitted listings.
- Overpass, Universal Namespace Service, Overstore, Overbase, Overvault, Overguard, Overwatch, Overmeter, Overclaim, Fraud Control, Reputation and Anti-Sybil Service, Wallet/Usage Center, and Central AI Stewardship Interface.

## Dependencies

- [Overpass](../control_plane/overpass.md), [Overtenant](../control_plane/overtenant.md), [Overkey](../control_plane/overkey.md), and [Universal Namespace Service](../data_storage_namespace/universal_namespace_service.md) for identities, organization pages, usernames, namespace routes, and verification markers.
- [Overbase](../data_storage_namespace/overbase.md), [Overstore](../data_storage_namespace/overstore.md), and [Overvault](../data_storage_namespace/overvault.md) for structured listing state, media refs, private contact refs, and secret/private data.
- [Overguard](../trust_policy_verification/overguard.md), [Workload Classifier](../trust_policy_verification/workload_classifier.md), [Overwatch](../control_plane/overwatch.md), and [Overclaim](../trust_policy_verification/overclaim.md) for policy, classification, audit, reports, disputes, and corrections.
- [Reputation and Anti-Sybil Service](../trust_policy_verification/reputation_anti_sybil_service.md) and [Fraud Control Service](../federation_public/fraud_control_service.md) for risk recommendations and evidence packages.
- [Search Engine](search_engine.md), [Messaging Center](messaging_center.md), and [Maps and Navigation](maps_navigation.md) for search indexing, contact handoff, and local/place display.
- [Overmeter](../execution_scheduling/overmeter.md), [ORU Account Service](../accounting/oru_account_service.md), [Seal Ledger](../accounting/seal_ledger.md), and [Wallet and Usage Center](wallet_usage_center.md) for resource usage visibility and receipts.

## Owned Responsibilities

Directory Listings owns:

- Listing category taxonomy bindings for Phase 12 app categories and per-jurisdiction policy refs.
- Listing records, draft/published/expired/removed lifecycle, edit history, and version refs.
- Locality scopes, delivery/service-area refs, and coarse location records for listings.
- Organization/business/community page refs owned by the directory app, linked to Overpass/namespace truth.
- Listing media refs and attachment metadata, with canonical binary storage in Overstore.
- Contact handoff records to Messaging Center without storing conversation content.
- Search index update records and ranking constraint refs for Search Engine.
- Map/place handoff records for Maps and Navigation.
- Abuse report, moderation action, scam signal, impersonation signal, and dispute handoff records.
- User-visible audit summaries, usage refs, and wallet handoff records for listing operations.

Directory Listings does not own direct messages, payment flows, escrow, final identity truth, map/place canonical truth, search ranking authority, final reputation scores, or final dispute resolution.

## Data Model

- `listing_category`: category id, parent category refs, allowed listing types, required fields, prohibited fields, policy refs, moderation rules, and jurisdiction/tenant applicability.
- `locality_scope`: locality id, place/region refs, service-area refs, route/map refs, visibility radius class, privacy level, and freshness refs.
- `listing_record`: listing id, owner actor/org refs, category refs, title, summary, structured attributes, media refs, locality scope, contact preference refs, state, version, moderation state, expiry, and audit refs.
- `listing_version`: listing ref, version number, changed fields, editor refs, previous version ref, policy checks, and rollback/correction refs.
- `organization_page`: organization ref, namespace refs, verification markers, description refs, service categories, location/place refs, contact handoff refs, page state, and claim/dispute refs.
- `contact_handoff`: listing ref, requester refs, seller/provider refs, Messaging Center thread/inbox refs, spam-risk refs, consent refs, and state.
- `search_index_update`: listing/page refs, index visibility class, redaction summary, locality/category tokens, anti-ad-trap ranking constraints, and update state.
- `map_place_handoff`: listing/page refs, place refs, coarse/exact location class, route-display permission, offline-cache allowance, and Maps integration state.
- `abuse_report`: reporter refs, listing/page refs, report type, evidence refs, risk summary, Fraud Control refs, moderation refs, claim refs, and state.
- `moderation_action`: action type, target refs, policy refs, reason codes, reviewer refs, duration, correction path, and audit refs.
- `listing_dispute_ref`: Overclaim ref, parties, target refs, evidence refs, hold/hide state, correction refs, finality refs, and state.
- `directory_usage_ref`: create/edit/publish/search-handoff/media/contact/moderation/report/export usage, Overmeter refs, and wallet receipt refs.

Common envelope fields: `id`, `tenant_id`, `actor_id`, `organization_id`, `trace_id`, `idempotency_key`, `schema_version`, `state`, `created_at`, `updated_at`, `policy_refs`, and `audit_refs`.

## API Surface

- `POST /directory/listings/drafts`: creates a listing draft with category, locality, media, and contact preference refs.
- `POST /directory/listings/{listing_id}/publish`: publishes a validated listing after policy, ownership, category, and locality checks.
- `PATCH /directory/listings/{listing_id}`: creates a new listing version for allowed edits.
- `POST /directory/listings/{listing_id}/pause`: pauses visibility without deleting history.
- `POST /directory/listings/{listing_id}/renew`: renews an eligible listing with current policy checks.
- `POST /directory/listings/{listing_id}/expire`: expires a listing explicitly or by scheduled lifecycle.
- `POST /directory/listings/{listing_id}/archive`: archives a completed or stale listing.
- `GET /directory/listings/{listing_id}`: returns listing details according to visibility, role, and redaction rules.
- `GET /directory/listings`: searches/listings by category, locality, organization, state, and visibility class; Search Engine remains the broad search authority.
- `POST /directory/listings/{listing_id}/contact`: creates a Messaging Center handoff.
- `POST /directory/listings/{listing_id}/reports`: records abuse/scam/prohibited-content/impersonation reports.
- `POST /directory/listings/{listing_id}/disputes`: creates or attaches an Overclaim dispute ref.
- `POST /directory/organization-pages`: creates an organization/business/community page linked to identity/namespace refs.
- `PATCH /directory/organization-pages/{page_id}`: updates page content and verification refs.
- `GET /directory/replay/{record_id}`: reconstructs listing lifecycle, moderation, report, dispute, search, map, contact, and usage decisions.

Mutating APIs require actor identity, tenant/org context, trace id, idempotency key, category refs, policy refs, and ownership refs. Stable errors include `category_required`, `locality_required`, `owner_not_authorized`, `listing_policy_denied`, `media_ref_invalid`, `contact_handoff_denied`, `prohibited_listing`, `moderation_required`, `search_visibility_denied`, `map_location_too_precise`, `dispute_required`, and `listing_state_conflict`.

## Event Surface

- `directory_listings.draft_created`: listing draft created.
- `directory_listings.listing_published`: listing published with visibility refs.
- `directory_listings.listing_updated`: new listing version recorded.
- `directory_listings.listing_paused`: listing paused.
- `directory_listings.listing_renewed`: listing renewed after checks.
- `directory_listings.listing_expired`: listing expired.
- `directory_listings.listing_archived`: listing archived.
- `directory_listings.contact_handoff_created`: Messaging Center handoff created.
- `directory_listings.search_update_requested`: index update requested.
- `directory_listings.map_handoff_created`: map/place handoff created.
- `directory_listings.abuse_reported`: abuse/scam/prohibited-content report recorded.
- `directory_listings.moderation_action_recorded`: moderation action recorded.
- `directory_listings.dispute_opened`: Overclaim dispute ref attached.
- `directory_listings.usage_emitted`: usage refs emitted.

Events include listing/page refs, actor/org refs, category/locality refs, state, reason codes, policy refs, moderation refs, search/map/messaging refs, claim refs, audit refs, and usage refs. Events must not include raw private messages, exact private locations, payment details, private contact data, or unrelated user profile data.

## Core Workflow

1. User or organization starts a listing draft and selects category, locality, contact preference, media refs, and optional organization page refs.
2. Directory validates identity, ownership, category requirements, locality privacy, media refs, prohibited content checks, and policy refs.
3. Publishing creates immutable listing version refs, audit refs, search update refs, and map/place handoff refs where allowed.
4. Search Engine indexes the redacted public/authorized listing summary and returns listing refs through search surfaces.
5. Interested user contacts the owner through Messaging Center handoff; Directory stores handoff refs, not messages.
6. Listing owner edits, pauses, renews, expires, archives, or deletes according to lifecycle and policy rules.
7. Abuse reports, scam signals, impersonation reports, or disputes create evidence refs for moderation, Fraud Control, Overclaim, and Central AI Stewardship where appropriate.
8. Usage refs flow to Wallet/Usage Center and accounting visibility.

## State Machine

Listing lifecycle:

1. `draft`
2. `validation_pending`
3. `policy_checked`
4. `published`
5. `limited_visibility`
6. `paused`
7. `under_moderation`
8. `under_dispute`
9. `renewal_pending`
10. `expired`
11. `archived`
12. `removed`
13. `corrected`

Organization page lifecycle:

1. `draft`
2. `identity_pending`
3. `active`
4. `verification_pending`
5. `limited_visibility`
6. `disputed`
7. `suspended`
8. `archived`

Report/moderation lifecycle:

1. `submitted`
2. `triaged`
3. `evidence_requested`
4. `action_pending`
5. `action_applied`
6. `rejected`
7. `under_claim`
8. `corrected`
9. `closed`

State changes are append-only. Removed listings retain tombstone and dispute/audit refs where policy requires.

## Policy And Security

- Listing publication is deny-by-default when identity, ownership, category, locality, media, contact, or policy facts are missing.
- Category policies must handle prohibited goods/services, unsafe content, impersonation, spam, discrimination-sensitive listings, scams, and jurisdiction-specific restrictions.
- Exact private locations are not exposed by default; listings use coarse locality or place refs unless owner grants stricter visibility.
- Contact happens through Messaging Center handoff; direct private contact data is not public by default.
- Search and map handoffs carry redaction and visibility constraints; downstream services cannot index private listing fields.
- Abuse and fraud internals are redacted by audience; public users see only allowed summaries.
- Organization pages require identity/namespace refs and claim/dispute paths for impersonation handling.
- Ranking must not be designed as an ad trap or pay-to-survive marketplace dependency.

## Metering And Accounting

- Emit usage refs for listing creation, edits, media storage, publish/renew, search index updates, map handoffs, contact handoffs, abuse reports, moderation work, dispute work, and exports.
- Link usage to actor, organization, tenant, listing id, category, locality, media refs, search/map/messaging refs, report/dispute refs, and wallet receipt refs.
- Directory Listings does not maintain balances, escrow, payments, bids, ads, invoices, provider payouts, or ledger truth.
- Native-service economics remain structural and near-cost; any surplus routing is represented by stewardship refs outside listing ranking.
- Do not encode hardcoded prices, revenue forecasts, paid-placement mechanics, or per-transaction fees.

## Observability And Operations

- Expose listing counts by category/locality/state, publish latency, policy denial rate, search update lag, map handoff lag, contact handoff failure rate, report volume, moderation backlog, scam signal rates, dispute rates, and usage by operation.
- Alert on spam bursts, impersonation spikes, prohibited category attempts, repeated contact abuse, search update failures, map privacy violations, moderation backlog age, and missing usage refs.
- Provide moderator views with redacted evidence, reason codes, prior actions, linked disputes, and correction paths.
- Provide replay for listing lifecycle, publish decisions, search/map/messaging handoffs, reports, moderation, disputes, and usage.
- Support retention/tombstone behavior for expired, archived, removed, and disputed listings.

## Failure Modes And Recovery

- Missing required category/locality fields: keep listing in draft and return field-level reason codes.
- Policy denies content: prevent publish and show correction path where allowed.
- Media ref unavailable or unsafe: block publish until replaced or moderation clears it.
- Search Engine unavailable: publish listing but mark search update pending with retry refs if policy allows direct URL visibility.
- Messaging handoff fails: keep listing visible but show contact unavailable and record retry/failure refs.
- Map/place handoff denied: publish without map display or require coarser locality.
- Abuse report false positive: record correction refs and restore visibility if owning review allows.
- Dispute opens: move listing/page to limited visibility or hold state according to policy.
- Usage emission fails: mark operation usage pending and reconcile before final receipts.

## Validation Plan

- Users and organizations can create, validate, publish, edit, pause, renew, expire, archive, search, and view listings through explicit APIs.
- Listing publication enforces category, locality, ownership, media, contact, and policy requirements.
- Search and map handoffs include redaction/visibility constraints and never expose private fields.
- Messaging handoff creates thread/inbox refs without storing message content in Directory Listings.
- Abuse reports and disputes create evidence refs and Overclaim/Fraud Control handoffs.
- Ranking and discovery tests prove the first build does not depend on paid placement or ad-trap mechanics.
- Tenant, owner, and role isolation tests prevent unauthorized listing edits, private contact reads, and report evidence leaks.
- Replay reconstructs listing versions, policy decisions, moderation actions, handoffs, disputes, and usage refs.

## Build Breakdown

1. Define listing category, locality, listing, organization page, contact handoff, search update, map handoff, report, moderation, dispute, and usage schemas.
2. Implement draft, validate, publish, edit, pause, renew, expire, archive, read, and replay APIs.
3. Add Overpass/namespace ownership, Overstore media refs, Overbase listing state, and Overguard policy checks.
4. Add Search Engine index-update handoff with anti-ad-trap ranking constraints and redaction refs.
5. Add Messaging Center contact handoff and organization inbox refs.
6. Add Maps and Navigation handoff with locality/place privacy controls.
7. Add abuse reporting, moderation, Fraud Control, Reputation/Anti-Sybil, and Overclaim dispute handoffs.
8. Add usage metering, wallet visibility, moderator diagnostics, and Phase 12 native-app UI flows.

## Handoff And Downstream Use

Directory Listings is an early native utility that exercises identity, namespace, storage, search, messaging, maps, policy, moderation, disputes, and usage accounting without full social-network scale.

Search Engine, Messaging Center, Maps and Navigation, Personal AI Assistant, Wallet/Usage Center, and Central AI Stewardship Interface should integrate through documented APIs/events. They should not read listing storage directly or invent listing truth.

## Open Design Questions

Resolved decisions:

- Phase 12 should start with low-compliance, high-utility categories: community events, community groups, organization/business pages, general household classifieds, free/giveaway posts, and non-regulated local services. Jobs, housing, childcare, medical/legal/financial services, transportation, weapons, regulated goods, high-value asset sales, age-restricted categories, and anything requiring jurisdiction-specific screening remain disabled or review-gated until Compliance Boundary, category policy, Fraud Control, and Overclaim workflows can supply explicit boundary markers, evidence refs, and appeal paths.
- Locality uses four default classes. Public place or venue refs are allowed for organization pages, public events, and businesses that intentionally publish a location. Approximate locality is the default for individual listings and services, using city/neighborhood/region or bounded service-area refs without exposing a private address. Private-to-contact exact pickup, route, or address refs are released only through an authorized Messaging Center or Maps handoff after both listing policy and recipient contact policy pass. Hidden locality applies by default to sensitive categories, private homes, vulnerable-person contexts, disputes, active moderation, and any listing whose location evidence is missing or too precise for the caller.
- Contact handoff starts as a Messaging Center first-contact flow, not public phone/email exposure. Directory creates a `contact_handoff` with listing refs, requester refs, owner/org inbox refs, consent refs, spam-risk refs, rate-limit state, and report/block affordances; Messaging owns the thread, delivery, attachments, first-contact rules, and encrypted payload refs. Individual listings should use two-step contact requests and redacted owner identity until policy allows more detail, while verified organizations can route to organization inboxes. Off-platform payment pressure, repeated template spam, unsafe attachments, and private-contact harvesting create fraud/report refs without blocking ordinary low-risk messages by default.
- Directory ranking constraints are source-policy inputs for Search Engine, not a Directory-owned ad auction. The allowed signals are category match, locality fit, text/attribute relevance, freshness within capped renewal windows, verified organization or namespace markers, completeness, safety/moderation state, owner diversity, and user-selected filters. Forbidden signals are bids, paid placement, hidden promotion, native-service surplus routing, payment history, engagement addiction metrics, private profile targeting, or opaque suppression of small users. Search explanations must expose non-ad signal classes and fairness/rotation constraints while hiding abuse thresholds and private fraud internals.
- Visibility changes are immediate only for high-confidence safety, privacy, legality, identity, or policy failures: prohibited category attempts, missing required ownership/category/locality facts, exact-location leaks, credible impersonation, malware/scam payloads, discrimination-sensitive violations, child-safety or violence threats, active Overguard denial, or Fraud Control high-confidence source-trusted evidence. Lower-confidence abuse reports, duplicate/spam suspicions, category corrections, quality problems, routine buyer/seller disputes, and contested organization claims move to limited visibility, contact throttling, warning labels, or review queues according to policy. Overclaim disputes attach evidence and challenge/appeal refs; final removal, correction, restoration, or tombstone behavior comes from the owning resolution path rather than ad hoc Directory mutation.
