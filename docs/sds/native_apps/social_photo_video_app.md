SDS #73

# Social Photo/Video App SDS

## Purpose

Build native media sharing without addiction-driven extraction, hidden data resale, or captive ad-marketplace mechanics.

Social Photo/Video App is the native media and community sharing utility for Overrid. It owns media post records, feed/follow/group state, visibility controls, comments/reactions, media processing refs, rights/attribution refs, recommendation control refs, moderation refs, abuse reports, and usage refs. It is not a private messaging system, object store, ad marketplace, payment service, identity authority, or behavioral-profiling engine. The app must serve people and communities without maximizing compulsive engagement.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [social_photo_video_app.md](../../service_catalog/native_apps/social_photo_video_app.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md) |
| Sub-build plan | [SUB BUILD PLAN #73 - Social Photo/Video App](../../build_plan/sub_build_plan_073_social_photo_video_app.md) |

## Service Family

- Family: Native applications
- Owning layer: Native public utility application layer for photo/video sharing, media communities, transparent feeds, and moderation handoffs
- Primary data scope: media post refs, upload intents, processing refs, albums/collections, follow/group refs, feed state, visibility controls, comment/reaction records, rights/attribution refs, recommendation controls, moderation refs, abuse reports, usage refs, and audit refs
- First build phase from service plan: [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md), after privacy, moderation, storage, and abuse controls mature

## Problem Statement

Photo and video sharing is valuable, but dominant social platforms often convert it into addiction loops, surveillance profiling, data resale, creator lock-in, escalating ad spend, and opaque reach manipulation. Overrid needs a native media sharing app that gives people, communities, organizations, and creators useful sharing without making attention extraction the product.

The design challenge is to separate app-owned social state from storage, identity, messaging, search, payments, and governance. Social Photo/Video should own posts, visibility, feeds, follows, groups, comments, reactions, rights/attribution, recommendation controls, and moderation refs while using Overstore for media, Messaging Center for private messages, Search for authorized discovery, and Overguard/Fraud/Reputation/Overclaim for abuse and disputes.

## Goals

- Define upload intent, media asset ref, media processing, post, album/collection, follow, group, feed, visibility, comment, reaction, rights/attribution, recommendation control, moderation, abuse report, and usage records.
- Support photo/video post creation, editing, publishing, limiting, removing, archiving, and restoring where policy allows.
- Support feeds and recommendations that are transparent, configurable, bounded, and not optimized for addiction.
- Support follows, groups, public profiles, and organization/community pages without making social graph the global identity system.
- Support media processing through Overstore/Object refs and execution resources without storing raw media inside app state.
- Support rights, attribution, consent, repost/remix permissions, takedown, and dispute refs.
- Integrate Messaging Center for private communication and notifications without storing private messages.
- Integrate Search Engine for public/authorized discovery with moderation and visibility constraints.
- Emit usage and receipt refs without pricing or revenue projections.

## Non-Goals

- Do not build addiction loops, infinite dark-pattern engagement mechanics, hidden behavioral profiling, ad targeting, paid reach dependency, or data resale.
- Do not own raw media object storage, vault secrets, global identity, direct messages, wallet balances, provider payouts, ledger entries, or final fraud/reputation verdicts.
- Do not bypass Overpass, Overtenant, Overkey, Overstore, Overvault, Overguard, Overwatch, Overmeter, Search Engine, Messaging Center, Overclaim, Fraud Control, or Reputation/Anti-Sybil.
- Do not expose private groups, posts, comments, reactions, or media refs outside visibility and consent rules.
- Do not train models on user media, comments, or social graph data unless explicit source and user permissions allow the specific purpose.
- Do not add pricing, customer-count, revenue, blockchain, NFT, or per-transaction fee assumptions.

## Primary Actors And Clients

- Users creating, publishing, viewing, commenting, reacting, following, grouping, reporting, and exporting media posts.
- Creators, community groups, organizations, and public-interest projects publishing media under explicit visibility and rights rules.
- Moderators, stewards, rights reviewers, and fraud reviewers handling abuse, unsafe content, impersonation, spam, and rights disputes.
- Personal AI Assistant helping draft captions, summarize feeds, create accessibility text, find posts, or file reports with permission.
- Search Engine indexing public/authorized posts and profiles.
- Messaging Center delivering notifications and private contact handoffs.
- Wallet and Usage Center displaying media storage, processing, feed, moderation, and export usage.
- Mobile SDK and native clients handling uploads, offline drafts, notifications, and media playback.
- Overpass, Universal Namespace Service, Overbase, Overstore, Overvault, Overguard, Overwatch, Overmeter, Overclaim, Fraud Control Service, Reputation and Anti-Sybil Service, ORU Account Service, and Seal Ledger.

## Dependencies

- [Overstore](../data_storage_namespace/overstore.md) for photo/video object refs, thumbnails, transcoded variants, accessibility assets, and export bundles.
- [Overbase](../data_storage_namespace/overbase.md) for structured post, feed, graph, comment, reaction, moderation, and usage state.
- [Overvault](../data_storage_namespace/overvault.md) for private group grants, sensitive media grants, and private metadata refs.
- [Overpass](../control_plane/overpass.md), [Overtenant](../control_plane/overtenant.md), [Overkey](../control_plane/overkey.md), and [Universal Namespace Service](../data_storage_namespace/universal_namespace_service.md) for actor/org identity, public profile refs, namespace routes, app refs, and service accounts.
- [Overguard](../trust_policy_verification/overguard.md), [Workload Classifier](../trust_policy_verification/workload_classifier.md), [Overwatch](../control_plane/overwatch.md), [Overclaim](../trust_policy_verification/overclaim.md), [Fraud Control Service](../federation_public/fraud_control_service.md), and [Reputation and Anti-Sybil Service](../trust_policy_verification/reputation_anti_sybil_service.md) for policy, audit, abuse, rights disputes, spam, and Sybil controls.
- [Search Engine](search_engine.md) for public/authorized post/profile discovery and result handoffs.
- [Messaging Center](messaging_center.md) for notifications, private contact, reports, and moderation messages.
- [Personal AI Assistant](../ai_rag_model_routing/personal_ai_assistant.md) for permissioned drafting, accessibility, summarization, and report assistance.
- [Overmeter](../execution_scheduling/overmeter.md), [ORU Account Service](../accounting/oru_account_service.md), [Seal Ledger](../accounting/seal_ledger.md), and [Wallet and Usage Center](wallet_usage_center.md) for media/storage/processing usage visibility and receipts.

## Owned Responsibilities

Social Photo/Video App owns:

- Upload intents, media processing refs, post drafts, published posts, post versions, albums/collections, and archive/tombstone state.
- Follow graph records, group membership refs, public profile refs, and feed membership state owned by the app.
- Visibility rules for posts, groups, comments, reactions, reposts, remixes, and profile surfaces.
- Feed and recommendation preference records, ranking explanation refs, and user-controlled recommendation settings.
- Comment and reaction records for public/authorized post conversations; private messages remain in Messaging Center.
- Rights, attribution, consent, source, license, takedown, remix/repost, and dispute refs.
- Moderation action records, age/safety controls where legally required, abuse reports, appeal/correction refs, and evidence handoffs.
- Search index update refs, notification refs, assistant handoff refs, and usage refs.

Social Photo/Video does not own raw object storage, private direct messages, identity truth, search ranking authority, payment balances, final disputes, final fraud verdicts, or central AI governance decisions.

## Data Model

- `media_upload_intent`: actor/org refs, client refs, media type, expected size/duration, destination visibility, processing profile, policy refs, and state.
- `media_asset_ref`: upload intent refs, Overstore object refs, thumbnail/transcode refs, hash refs, safety scan refs, accessibility asset refs, retention class, and state.
- `media_processing_job`: media refs, processing type, worker/route refs, output variant refs, failure refs, usage refs, and state.
- `media_post`: post id, author actor/org refs, media refs, caption/text refs, album/group/profile refs, visibility rules, state, version, recommendation settings, moderation refs, and audit refs.
- `post_version`: post ref, version number, changed fields, editor refs, previous version ref, policy checks, restore/correction refs, and audit refs.
- `album_collection`: owner refs, title/summary refs, post refs, visibility, ordering, collaboration refs, and state.
- `social_follow`: follower refs, followee actor/org/profile refs, visibility, notification prefs, abuse throttles, and state.
- `social_group`: group id, owner/admin refs, membership rules, visibility, posting rules, moderation policy refs, vault grants, and state.
- `group_membership`: group refs, member refs, role, invitation refs, posting/comment rights, mute/ban state, and audit refs.
- `feed_record`: feed id, actor/client refs, feed type, source refs, ranking policy, recommendation controls, denied/hidden refs, explanation refs, and state.
- `comment_record`: post refs, author refs, parent comment refs, text refs, visibility, moderation refs, edit history refs, and state.
- `reaction_record`: post/comment refs, actor refs, reaction type, visibility, abuse throttle refs, and state.
- `rights_attribution_ref`: media/post refs, creator/source refs, license/consent refs, repost/remix permissions, takedown refs, claim refs, and state.
- `recommendation_control`: actor/profile refs, allowed signals, blocked signals, time/volume limits, sensitive-topic controls, explanation settings, and revocation state.
- `social_moderation_record`: target refs, moderation type, reason codes, evidence refs, reviewer refs, appeal/correction refs, and state.
- `social_abuse_report`: reporter refs, target refs, report class, evidence refs, safety/fraud/reputation refs, claim refs, and state.
- `social_usage_ref`: upload/storage/processing/post/feed/comment/reaction/search/notification/moderation/export/replay usage, Overmeter refs, and wallet receipt refs.

Common envelope fields: `id`, `tenant_id`, `actor_id`, `organization_id`, `app_id`, `trace_id`, `idempotency_key`, `schema_version`, `state`, `created_at`, `updated_at`, `policy_refs`, and `audit_refs`.

## API Surface

- `POST /social/media/upload-intents`: creates an upload intent with media, processing, visibility, and policy refs.
- `POST /social/media/{media_id}/processing-callbacks`: records processing results from authorized workers.
- `POST /social/posts/drafts`: creates a post draft with media refs, caption refs, and visibility rules.
- `POST /social/posts/{post_id}/publish`: publishes a validated post and emits search/feed refs.
- `PATCH /social/posts/{post_id}`: creates a new post version for allowed edits.
- `POST /social/posts/{post_id}/archive`: archives a post for the owner or authorized role.
- `POST /social/posts/{post_id}/remove`: removes or tombstones a post under policy, owner, or moderation authority.
- `GET /social/posts/{post_id}`: returns a post according to visibility, role, and redaction rules.
- `GET /social/feeds/{feed_id}`: returns a feed page with ranking explanation refs and applied controls.
- `POST /social/follows`: creates or updates follow refs.
- `POST /social/groups`: creates a group with membership, posting, and moderation rules.
- `POST /social/groups/{group_id}/members`: invites, accepts, updates, removes, mutes, or bans group members.
- `POST /social/posts/{post_id}/comments`: adds a visible comment under post/group rules.
- `POST /social/posts/{post_id}/reactions`: records or updates a reaction.
- `POST /social/recommendation-controls`: creates or updates recommendation controls.
- `POST /social/reports`: records abuse, spam, rights, impersonation, unsafe content, or privacy reports.
- `POST /social/moderation-actions`: records moderation actions by authorized reviewers/services.
- `GET /social/replay/{record_id}`: reconstructs upload, post, feed, follow, group, comment, reaction, moderation, report, and usage decisions.

Mutating APIs require signed actor/service identity, tenant scope, trace id, idempotency key, owner/group role refs, media/storage refs, visibility refs, policy refs, and rights/consent refs where needed. Stable errors include `media_ref_invalid`, `processing_not_complete`, `visibility_denied`, `group_role_required`, `post_policy_denied`, `comment_policy_denied`, `rights_claim_required`, `recommendation_control_required`, `unsafe_media_denied`, `moderation_required`, `search_visibility_denied`, and `post_state_conflict`.

## Event Surface

- `social_photo_video.upload_intent_created`: upload intent created.
- `social_photo_video.media_processed`: media processing completed or failed.
- `social_photo_video.post_draft_created`: post draft created.
- `social_photo_video.post_published`: post published with visibility refs.
- `social_photo_video.post_updated`: post version updated.
- `social_photo_video.post_removed`: post removed, hidden, or tombstoned.
- `social_photo_video.feed_generated`: feed page generated with controls and explanation refs.
- `social_photo_video.follow_updated`: follow state changed.
- `social_photo_video.group_created`: group created.
- `social_photo_video.group_membership_updated`: group membership changed.
- `social_photo_video.comment_created`: comment created.
- `social_photo_video.reaction_updated`: reaction changed.
- `social_photo_video.recommendation_control_updated`: recommendation controls changed.
- `social_photo_video.rights_claim_recorded`: rights/takedown/dispute refs recorded.
- `social_photo_video.abuse_reported`: abuse/safety/spam/privacy report recorded.
- `social_photo_video.moderation_action_recorded`: moderation action recorded.
- `social_photo_video.usage_emitted`: usage refs emitted.

Events include media/post/group/feed refs, actor/org refs, visibility class, rights refs, recommendation control refs, moderation refs, search/notification refs, reason codes, audit refs, and usage refs. Events must not include unauthorized private media, private group content, vault secrets, exact private location, direct messages, payment data, or hidden behavioral profiles.

## Core Workflow

1. User creates upload intent and uploads media to Overstore under policy, size, safety, and storage constraints.
2. Media processing workers generate thumbnails, transcodes, accessibility assets, and scan refs, then return processing state.
3. User creates a post draft with media refs, caption, visibility, rights/attribution, group/album/profile refs, and recommendation controls.
4. Publishing validates identity, ownership, group role, visibility, safety, rights, policy, and media readiness.
5. Published posts update feeds and authorized search indexes while preserving redaction and visibility constraints.
6. Viewers read feeds under explicit recommendation controls, see ranking explanations where useful, and can hide/follow/mute/block/report.
7. Comments and reactions follow post/group visibility and moderation rules.
8. Abuse reports, rights claims, impersonation, spam, or unsafe content create evidence refs for moderation, Overclaim, Fraud Control, Reputation/Anti-Sybil, and Central AI Stewardship where appropriate.
9. Usage and audit records flow to Overmeter, Wallet/Usage Center, Overwatch, and accounting views.

## State Machine

Media lifecycle:

1. `upload_intent_created`
2. `uploading`
3. `uploaded`
4. `processing`
5. `ready`
6. `quarantined`
7. `failed`
8. `deleted`

Post lifecycle:

1. `draft`
2. `media_pending`
3. `policy_checked`
4. `published`
5. `limited_visibility`
6. `under_review`
7. `under_claim`
8. `archived`
9. `removed`
10. `restored`
11. `tombstoned`

Group lifecycle:

1. `draft`
2. `active`
3. `restricted`
4. `under_review`
5. `suspended`
6. `archived`
7. `deleted`

Comment/reaction lifecycle:

1. `submitted`
2. `visible`
3. `limited_visibility`
4. `edited`
5. `under_review`
6. `removed`
7. `tombstoned`

Report/moderation lifecycle:

1. `submitted`
2. `triaged`
3. `evidence_requested`
4. `action_pending`
5. `action_applied`
6. `appealed`
7. `corrected`
8. `closed`

State transitions are append-only. Removed content retains tombstone, rights, report, and audit refs where policy requires.

## Policy And Security

- Posting is deny-by-default when identity, media ownership, group role, visibility, safety, rights, or policy facts are missing.
- Media refs must pass processing and safety policy before publication or preview.
- Private groups and restricted posts require explicit membership/visibility refs and cannot be indexed or recommended outside that audience.
- Recommendation controls must be user-visible, revocable, and bounded; ranking must not optimize for compulsive engagement, outrage, paid reach, or hidden profiling.
- Rights and attribution refs must travel with media posts, reposts, remixes, exports, and takedown/dispute flows.
- Private messages stay in Messaging Center. Social comments/reactions are not a substitute for private messaging.
- Search handoffs carry visibility and redaction refs; Search cannot reveal hidden posts.
- AI assistance cannot use media/post/comment data as model training or broad context without explicit permission and source rules.
- Moderation evidence is redacted by audience and preserves appeal/correction paths.

## Metering And Accounting

- Emit usage refs for uploads, storage, media processing, thumbnail/transcode generation, post publish/edit, feed generation, comments, reactions, search updates, notifications, moderation, reports, exports, replay, compute, bandwidth, and retention.
- Link usage to actor, org, group, post, media refs, processing job, feed, comment/reaction, report/moderation refs, Overmeter refs, and wallet receipt refs.
- Social Photo/Video does not maintain balances, ad bids, paid reach, creator payout schedules, invoices, provider payouts, or ledger truth.
- Native-service economics remain structural and near-cost; surplus routing is outside feed ranking and creator visibility.
- Do not encode hardcoded prices, revenue forecasts, ad-marketplace mechanics, or per-transaction fees.

## Observability And Operations

- Expose upload success rate, processing latency, safety scan denial rate, post publish latency, feed latency, recommendation-control usage, private visibility denial rate, group moderation backlog, report volume, rights-claim volume, search update lag, notification handoff failure rate, and usage emission status.
- Alert on upload abuse, spam bursts, suspicious follow/comment/reaction patterns, sudden recommendation anomalies, rights-claim spikes, private-content leak attempts, media processing failures, missing usage refs, and moderation backlog age.
- Provide user-visible controls for feed sources, recommendation settings, hidden/muted content, privacy, export, and AI use.
- Provide moderator diagnostics using refs, reason codes, and redacted evidence instead of broad private content access.
- Provide replay for upload, processing, publish, feed generation, visibility, recommendation controls, comments, reactions, moderation, reports, and usage.

## Failure Modes And Recovery

- Upload interrupted: preserve upload intent and allow resume or cancellation.
- Media processing fails: keep post in media-pending state and surface replace/retry options.
- Safety scan quarantines media: block publish and create moderation/review refs.
- Visibility rule conflict: deny publish or limit visibility until owner corrects rule.
- Feed/recommendation unavailable: return chronological/following feed fallback without expanding visibility.
- Search update unavailable: publish post under direct URL/feed visibility and mark search update pending if policy allows.
- Rights claim opens: move target content to limited visibility, hold, or review state according to policy.
- Abuse false positive: restore content or reach through correction refs and preserve review evidence.
- Usage emission fails: mark operation usage pending and reconcile before final receipt visibility.

## Validation Plan

- Users can upload, process, publish, edit, archive, remove, restore, view, comment, react, follow, group, report, and export content through explicit APIs.
- Private groups, restricted posts, comments, and reactions obey visibility and membership rules.
- Raw media is stored through Overstore refs, not Social-owned blobs.
- Feeds expose configurable recommendation controls and do not depend on paid reach or hidden engagement optimization.
- Search indexes only public/authorized posts with redaction and moderation refs.
- Messaging handoffs send notifications and private contact through Messaging Center without copying messages.
- Rights/takedown/dispute flows create evidence refs and appeal/correction paths.
- Usage refs flow to Overmeter and Wallet/Usage Center.
- Replay reconstructs upload, media processing, post, feed, follow, group, comment, reaction, moderation, report, and usage decisions.

## Build Breakdown

1. Define upload intent, media asset, processing job, post, version, album, follow, group, membership, feed, comment, reaction, rights, recommendation control, moderation, abuse report, and usage schemas.
2. Implement upload intent, processing callback, draft/publish/edit/archive/remove/read, follow, group, comment, reaction, report, moderation, recommendation-control, and replay APIs.
3. Add Overstore media refs, Overbase app state, Overvault private group grants, Overguard policy, and Overwatch audit.
4. Add feed generation with chronological/following baseline, transparent recommendation controls, and explanation refs.
5. Add Search Engine index update refs and Messaging Center notification/contact handoffs.
6. Add rights/attribution, takedown, Overclaim, Fraud Control, Reputation/Anti-Sybil, and moderation flows.
7. Add Personal AI Assistant permissions for captions, accessibility, summaries, and report drafting.
8. Add Overmeter usage, Wallet/Usage Center receipts, export/import, mobile upload/playback, and operational diagnostics.

## Handoff And Downstream Use

- Search Engine indexes public/authorized posts and profiles using visibility and moderation refs.
- Messaging Center handles private contact, notifications, reports, moderation notices, and organization inbox handoffs.
- Personal AI Assistant uses Social tools only under explicit user/app permissions.
- Wallet and Usage Center displays media storage, processing, feed, moderation, export, and AI-assist usage.
- Directory Listings and Maps can link public organization/community profiles where owners permit.
- Central AI Stewardship Interface can inspect redacted abuse and public-interest evidence without private feed access.
- Mobile SDK and Mobile Backend Gateway use Social APIs for uploads, offline drafts, playback, notifications, and privacy controls.

## Open Design Questions

- Resolved: ship following-only and chronological feed modes first, with profile, group, and album feeds scoped by explicit visibility refs. Local/community feeds can enter limited opt-in pilots only when Directory, Maps, Search, and coarse locality refs are source-attributed and policy-compatible. Transparent recommendations are not a launch default; they require active `recommendation_control` records, explanation refs, mute/block/hide controls, volume limits, Overwatch replay evidence, and an explicit no-paid-reach/no-addiction ranking policy before broad exposure.
- Resolved: broad public launch requires a jurisdictional age and safety policy profile before the app is enabled in that jurisdiction. The minimum profile includes age/guardian/consent checks where legally required, minor-safe visibility defaults, sensitive-media labeling, upload/comment/reaction rate limits, safety scan quarantine, report and appeal paths, Messaging Center contact restrictions, Search exclusion for restricted material, and stable Overguard reason codes. If the local profile is missing, stale, or contested, Social may run private/group-only or review-gated pilots but must deny broad public discovery and recommendations.
- Resolved: reposts, remixes, and public-interest media must carry a `rights_attribution_ref` attached to the source `media_asset_ref`, `media_post`, and derived post. Reposts require source post, creator/owner, license or consent, audience, revocation, and takedown refs. Remixes require explicit derivative permission, transform provenance, source linkage, and claim refs. Public-interest media requires steward/source refs, Purpose Tag Registry tag versions, data class, license/rights evidence, Overasset or namespace refs where available, and Overguard/Compliance checks; contested ownership moves to limited visibility or metadata-only state with Overclaim and Overwatch evidence.
- Resolved: use classed retention defaults rather than one hardcoded deletion window. Owner-deleted posts stop serving immediately, revoke feed/search/notification refs, and keep redacted tombstone, rights, usage, and audit refs until Overstore retention, backup, and replay policy allows purge. Moderation-removed or unsafe media is quarantined and retained only as redacted evidence until review, appeal, or incident policy closes. Rights disputes hold source media, derived media, attribution, and takedown refs under Overclaim/Overguard policy until finality or expiry. Private-group and sensitive media revoke Overvault grants immediately and retain only the minimum sealed refs needed for audit, recovery, or legal holds.
