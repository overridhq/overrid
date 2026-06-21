# Developer Console

## Slug

`developer-console`

## Title

Developer Console

## Navigation Group

Apps, Deployment, And Operations

## Description

Developer Console is the Overdesk page for app projects, manifests, local development environments, package validation, policy dry-runs, test fixtures, namespace drafts, logs, replay bundles, SDK/CLI handoffs, and deployment previews. It should make app development reachable from Overdesk without turning the desktop app into privileged infrastructure authority.

## Primary Users

- Developers
- App owners
- Delegated app managers
- Organization engineering admins
- Institution app teams
- Support engineers with authorized developer views
- SDK/CLI users

## Primary User Goals

- See active app projects and their environment state.
- Validate app manifests and packages quickly.
- Run policy dry-runs before deployment.
- Manage local dev environment status and SDK/CLI links.
- Review permissions, secrets refs, namespace route drafts, and test fixtures.
- Preview deployment plans without signing them.
- Inspect logs, replay links, app sessions, generated contract versions, and API health.
- Hand off to Deploy New App, App Detail, or Release And Rollback Manager.

## Entry Points

- Apps, Deployment, And Operations navigation.
- Owned Apps developer action.
- App Detail developer action.
- Deploy New App source selection and plan steps.
- Release And Rollback Manager release row.
- Diagnostics And Support Bundles developer section.
- Address bar command: `/developer`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active owner/developer scope.
- Project count.
- Local dev status.
- Validation alert count.
- Deployment preview count.
- Primary action: New Project.
- Secondary actions: Import Project, SDK/CLI, Validate, Deploy.

### Project List

Content:

- Project name.
- Linked app.
- Environment.
- Manifest state.
- Package validation state.
- Policy dry-run state.
- Namespace draft state.
- Local dev state.
- Last build/import state.
- Deployment preview state.
- Open actions.

Links and handoffs:

- App Detail.
- Deploy New App.
- Owned Apps.

### Manifest And Package Panel

Content:

- Manifest path/ref.
- Package identity.
- App version.
- Permission declarations.
- Resource declarations.
- Data class declarations.
- Storage refs.
- Workload class.
- Overpack build/import state.
- Package Validator results.
- Stable reason codes.

Links and handoffs:

- Deploy New App.
- Privacy And Permissions Center.
- Security And Compliance Reviews.

### Local Development Environment

Content:

- Local stack state.
- SDK version.
- CLI version.
- Generated contract version.
- Test account state.
- Fixture package state.
- Local app session state.
- API health.
- Required local permissions.
- Developer diagnostics.
- Update/remediation actions.

Links and handoffs:

- Local Device Settings.
- Updates And Release Notes.
- Diagnostics And Support Bundles.

### Policy Dry Run

Content:

- Selected app/project.
- Selected environment.
- Overguard output.
- Workload Classifier output.
- Package Validator output.
- Wallet/grant precheck.
- Compliance markers.
- Permission blockers.
- Reason codes.
- Replay refs.

Links and handoffs:

- Deploy New App.
- Security And Compliance Reviews.

### Namespace And Route Drafts

Content:

- Proposed namespace route.
- Proposed app route.
- Subroutes.
- Visibility class.
- Route conflict state.
- Route trust marker.
- Dispute/contact info.
- Route preview.
- Draft save state.

Links and handoffs:

- Namespace Manager.
- Overrid Browser.

### Deployment Preview

Content:

- Deployment Planner graph preview.
- Resource estimate.
- Release strategy preview.
- Health check plan.
- Backup/restore refs.
- Rollback preview.
- Wallet/grant precheck.
- Blockers and warnings.
- Open Deploy New App action.

Links and handoffs:

- Deploy New App.
- Release And Rollback Manager.

### Logs, Replay, And Support

Content:

- Build logs with redaction.
- Validation logs.
- Policy dry-run logs.
- App session logs.
- Replay links.
- Support bundle preview.
- Error reason codes.
- Export support bundle action.

Links and handoffs:

- Diagnostics And Support Bundles.
- Activity And Receipts Timeline.
- App Incidents And Support.

## Primary Actions

- Create project.
- Import project.
- Validate manifest.
- Build/import package.
- Run policy dry-run.
- Preview deployment.
- Open Deploy New App.
- Open SDK/CLI docs or handoff.

## Secondary Actions

- Open app detail.
- Copy refs.
- Refresh local dev status.
- Generate fixture package.
- Open namespace draft.
- Export redacted diagnostics.
- Ask AI to explain validation errors.

## States

- Empty project list.
- Loading.
- Live.
- Local dev unavailable.
- SDK/CLI update required.
- Manifest missing.
- Manifest invalid.
- Package valid.
- Policy dry-run running.
- Policy blocked.
- Deployment preview ready.
- Build/import failed.
- Logs redacted.
- Permission denied.
- Partial owner-service outage.
- Offline local-only view.
- Error with retry.

## Permissions And Privacy Behavior

- Developer Console owns developer UX, local diagnostics, and draft/previews. SDK, CLI, Overpack, Package Validator, Deployment Planner, Release Strategy Service, Overguard, Overbase, Overstore, Overvault, Universal Namespace Service, Wallet, and owner services own build/deploy authority and truth.
- Secrets must be referenced through approved credential providers or Overvault refs; raw secrets must not appear in manifests, logs, support bundles, local caches, or exports.
- Policy dry-runs and deployment previews must be clearly marked as previews until owner services return authoritative refs.
- Test accounts and fixtures must be visibly separated from production scopes.
- Offline mode may show local project state and cached logs but cannot sign deployment, reserve namespaces, validate authority, or mutate owner-service state.

## Design Notes

- Use a project table plus a persistent project detail drawer.
- Keep validation, policy, and deployment preview as separate tabs to avoid mixing build errors with policy denials.
- Use stable reason-code presentation so developers can fix problems without reading raw service logs.
- Make SDK/CLI actions handoffs, not hidden shell-only flows.
