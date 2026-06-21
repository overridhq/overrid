# Updates And Release Notes

## Slug

`updates-release-notes`

## Title

Updates And Release Notes

## Navigation Group

System And Help

## Description

Updates And Release Notes is the Overdesk page for desktop app versions, update channels, compatibility state, release notes, migration notices, security fixes, service-contract changes, rollback notices, PIP links, local update health, and user-facing change history. It informs users about what changed and what action is needed while Release Strategy, Release And Rollback Manager, PIP Registry, owner services, and update distribution systems retain operational authority.

## Primary Users

- Regular users
- Developers
- App owners
- Provider operators
- Organization admins
- Institution admins
- Support operators
- Security reviewers

## Primary User Goals

- See current Overdesk version, channel, update state, and compatibility status.
- Read release notes by version, service, native app, and impact class.
- Understand security fixes, breaking changes, migration requirements, and rollback notices.
- Open related PIPs, stewardship reports, incidents, app releases, and security reviews.
- Update Overdesk where allowed by local policy.
- Diagnose failed updates or pinned/managed versions.
- Track post-update issues and support handoffs.

## Entry Points

- System And Help navigation.
- Local Device Settings.
- Settings And Security.
- Release And Rollback Manager.
- Governance Center.
- Security And Compliance Reviews.
- Incident Reports.
- Diagnostics And Support Bundles.
- Address bar command: `/updates`.

## Sections To Have

### Page Header

Content:

- Page title.
- Current Overdesk version.
- Update channel.
- Latest available version.
- Compatibility state.
- Managed/pinned state.
- Last update check time.
- Primary action: Check For Updates.
- Secondary actions: Release Notes, Update Settings, Diagnostics, Rollback Notices.

Links and handoffs:

- Local Device Settings.
- Settings And Security.
- Diagnostics And Support Bundles.

### Current Version And Channel

Content:

- Installed version.
- Build ref.
- Signature/verification state.
- Channel: stable, preview, test, or managed.
- Managed by organization/institution marker.
- Update policy refs.
- Auto-update preference where allowed.
- Last successful update.
- Last failed update.
- Fresh verification requirement for channel changes.

Links and handoffs:

- Settings And Security.
- Local Device Settings.
- Activity And Receipts Timeline.

### Available Updates

Content:

- Version.
- Release date.
- Update size class.
- Signature state.
- Required/recommended/optional marker.
- Security fix marker.
- Migration required marker.
- Restart required marker.
- Download/install readiness.
- Policy denial reasons.

Links and handoffs:

- Diagnostics And Support Bundles.
- Security And Compliance Reviews.
- Release And Rollback Manager.

### Release Notes Feed

Content:

- Version.
- Release summary.
- Changed pages/features.
- Native app changes.
- Desktop shell changes.
- SDK/API compatibility notes.
- Bug fixes.
- Security fixes.
- Known issues.
- User action needed.

Links and handoffs:

- Native App Catalog.
- Developer Console.
- Help.

### Impact And Compatibility

Content:

- Compatibility with active services.
- Minimum required service/API versions.
- Deprecated features.
- Breaking changes.
- Migration requirements.
- Local cache compatibility.
- Offline queue compatibility.
- RAG/AI route compatibility.
- Node/resource contribution compatibility.
- App-owner impact summary.

Links and handoffs:

- Local Cache And Offline Sync.
- Docdex And RAG Index Manager.
- Node Detail.
- Owned Apps.

### Security Fixes And Compliance Notes

Content:

- Security fix summary.
- Affected component refs.
- Severity marker.
- Required update marker.
- Compliance boundary refs.
- Security review refs.
- Incident refs where public.
- Disclosure state.
- User mitigation guidance.

Links and handoffs:

- Security And Compliance Reviews.
- Incident Reports.
- Stewardship Reports.

### Migration And Rollback Notices

Content:

- Migration refs.
- Rollback notices.
- Version cutoff.
- Data migration state.
- Local cache rebuild requirement.
- Manual user step.
- Failed migration recovery.
- Rollback eligibility.
- Public communication refs.

Links and handoffs:

- Release And Rollback Manager.
- Protocol Improvement Proposals.
- Local Cache And Offline Sync.

### Governance And PIP Links

Content:

- PIP refs.
- Proposal status.
- Accepted/rejected/deferred marker.
- Implementation evidence refs.
- Migration evidence refs.
- Stewardship report refs.
- Correction/supersession refs.
- Public comment links where allowed.

Links and handoffs:

- Protocol Improvement Proposals.
- Governance Center.
- Stewardship Reports.

### Update Diagnostics

Content:

- Last update check result.
- Download state.
- Signature verification state.
- Installer permission state.
- Disk space state.
- Network reachability.
- Managed policy blockers.
- Retry state.
- Redacted diagnostic export action.

Links and handoffs:

- Diagnostics And Support Bundles.
- Local Device Settings.
- App Incidents And Support.

### Post-Update Follow-Up

Content:

- Recently installed versions.
- Post-update health check.
- Known issue matches.
- Support case refs.
- Incident refs.
- Cache rebuild state.
- User feedback action.
- Rollback/support guidance.

Links and handoffs:

- App Incidents And Support.
- Incident Reports.
- Local Cache And Offline Sync.

## Primary Actions

- Check for updates.
- Download update.
- Install update.
- Change update channel where allowed.
- Open release notes.
- Open migration notice.
- Open rollback notice.
- Run update diagnostics.

## Secondary Actions

- Filter release notes by version.
- Filter by impact class.
- Copy version/build ref.
- Open related PIP.
- Open related incident/review/report.
- Export update diagnostic metadata.
- Ask AI to explain release impact.

## States

- Loading.
- Live.
- Up to date.
- Update available.
- Update required.
- Downloading.
- Ready to install.
- Installing.
- Restart required.
- Update failed.
- Managed/pinned.
- Channel change pending.
- Migration required.
- Rollback notice active.
- Offline.
- Action denied.

## Permissions And Privacy Behavior

- Update distribution, release strategy, service deployment, rollback, and migration authority remain outside Overdesk.
- Overdesk may check, present, download, and request install actions according to local platform and managed policy.
- Channel changes, managed policy overrides, rollback-sensitive actions, and test-channel enrollment may require fresh verification.
- Release notes must not expose private incident evidence, exploit-sensitive details, secrets, private user data, hidden provider data, or unpublished compliance evidence.
- Security fixes should describe user impact and mitigation without publishing exploit instructions.
- Update diagnostics and failed-install logs must be redacted before support export.

## Design Notes

- Put current version, channel, and required action at the top.
- Use a timeline/list for release notes and a separate high-visibility area for required security or migration updates.
- Link every governance-driven change back to its PIP, stewardship report, review, or incident where public.
- Keep install controls visually distinct from release-note reading.
- Use plain status language for managed or pinned devices.
- Avoid overwhelming regular users with developer-only compatibility details unless they expand the section.
