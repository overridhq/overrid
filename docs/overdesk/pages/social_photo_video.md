# Social Photo/Video

## Slug

`social-photo-video`

## Title

Social Photo/Video

## Navigation Group

Daily Apps

## Description

Social Photo/Video is the Overdesk surface for sharing photos, videos, albums, groups, profiles, comments, reactions, and community media. It must support real social use without addiction-driven feeds, dark patterns, hidden recommendation manipulation, or personal-data extraction.

## Primary Users

- Regular users
- Creators
- Community group owners
- Organization page owners
- Institution communities
- Moderators
- App owners sharing media updates

## Primary User Goals

- View media from people, groups, organizations, and communities the user follows.
- Upload photos or videos.
- Manage albums, groups, profiles, visibility, and comments.
- Understand feed ordering and recommendation controls.
- Report abuse, appeal moderation, or manage blocked users.
- Inspect media rights, attribution, usage, and storage state.

## Entry Points

- Daily Apps navigation.
- Home Dashboard fast app shortcut.
- Global Search result.
- Directory listing media links.
- Messaging attachment preview.
- Maps community layer handoff.
- Address bar command: `/social`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active account/profile.
- Feed mode.
- Upload state.
- Primary action: Upload.
- Secondary actions: Create Album, Create Group, Feed Controls.

### Feed Controls

Content:

- Following.
- Groups.
- Local/community.
- Organization pages.
- Saved media.
- Chronological mode.
- Recommendation mode.
- Content type filter.
- Time limit or batch size control.

Links and handoffs:

- Privacy And Permissions Center.
- Settings And Security.

### Media Feed

Content:

- Photo/video cards.
- Author/profile marker.
- Visibility marker.
- Rights/attribution marker.
- Caption.
- Comment/reaction summary.
- Save/share controls.
- Report controls.
- User-controlled load more.

### Media Detail

Content:

- Full media viewer.
- Caption and metadata.
- Author profile.
- Album/group context.
- Visibility.
- Rights/attribution refs.
- Processing state.
- Comments.
- Reactions.
- Related posts.
- Report/appeal state.

Links and handoffs:

- Identity And Profile Center.
- Messaging Center.
- Activity And Receipts Timeline.
- Disputes And Appeals.

### Upload And Composer Panel

Content:

- Media picker.
- Caption.
- Album/group destination.
- Visibility.
- Tags.
- Accessibility captions or alt text.
- Rights/attribution.
- Processing estimate.
- Storage/usage estimate.
- Publish confirmation.

Links and handoffs:

- Wallet.
- Overvault Secure Storage Center.
- Privacy And Permissions Center.

### Profile And Groups Panel

Content:

- User profile.
- Organization page.
- Group list.
- Membership state.
- Moderation role.
- Visibility settings.
- Blocked/muted users.
- Group invites.

### Comments And Reactions

Content:

- Comment thread.
- Reply controls.
- Reaction controls.
- Sort mode.
- Hidden/removed comment markers.
- Report comment action.
- Moderation action where authorized.

### Moderation And Appeals Panel

Content:

- Report media.
- Report profile/group.
- Appeal removed content.
- View reason code.
- Block/mute.
- Safety notice.
- Dispute handoff where applicable.

### Media Processing Panel

Content:

- Upload progress.
- Transcode progress.
- Thumbnail state.
- Caption generation state where allowed.
- Safety scan state.
- Failure/retry state.

## Primary Actions

- Upload.
- Open Media.
- Comment.
- React.
- Save.
- Create Album.
- Create Group.

## Secondary Actions

- Share route.
- Message author.
- Follow/unfollow.
- Mute/block.
- Report.
- Appeal.
- Change feed mode.
- Inspect rights.

## States

- Empty feed.
- Loading.
- Live.
- Offline cached feed.
- Uploading.
- Processing.
- Publish failed.
- Media unavailable.
- Permission denied.
- Content removed.
- Group access required.
- Partial media-processing outage.

## Permissions And Privacy Behavior

- Feed ranking must be transparent and user-controllable.
- Exact location metadata must not be shown or uploaded unless explicitly allowed.
- Private group media must not leak into public feeds, search snippets, notifications, or support bundles.
- Uploads must show visibility, rights, and usage implications before publish.
- Moderation and abuse internals must be redacted by viewer role.
- The page must avoid endless engagement loops and must provide user-controlled feed limits.

## Design Notes

- The page should prioritize media viewing and upload clarity, not addictive feed mechanics.
- Feed controls must be visible enough for users to understand why they see content.
- Upload progress and processing state should be durable and easy to resume.
- Comment controls should be compact but moderation/report paths must remain accessible.
