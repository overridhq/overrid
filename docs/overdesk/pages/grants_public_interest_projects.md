# Grants And Public-Interest Projects

## Slug

`grants-public-interest-projects`

## Title

Grants And Public-Interest Projects

## Navigation Group

Wallet, Credits, And Ownership

## Description

Grants And Public-Interest Projects is the Overdesk surface for discovering grant programs, sponsored credits, purpose-scoped resource allocations, public-interest pools, research and education programs, project eligibility, usage reports, and outcome refs. It should make public-good resource support understandable without making Overdesk the grant authority.

## Primary Users

- Researchers
- Students and academics
- Open-source maintainers
- Nonprofits and public-service teams
- Public-interest project owners
- Sponsors and contributors
- Organization and institution admins
- Stewards and reviewers

## Primary User Goals

- Find active grant programs and public-interest pools.
- Understand eligibility, purpose tags, quotas, and reporting requirements.
- Apply for resource support.
- Track active grants, sponsored credits, allocations, throttles, renewals, and revocations.
- See usage and outcome reporting without exposing private workload data.
- Open correction, appeal, or more-evidence paths.
- Move between grants, Wallet, Central AI Stewardship, Directory, and Workspace.

## Entry Points

- Wallet grant or sponsored-credit section.
- Home Dashboard active grants.
- Central AI Stewardship project view.
- Directory Listings public-interest category.
- Native App Catalog grant-eligible app marker.
- Notifications Center grant alert.
- Personal AI Assistant grant explanation.
- Address bar command: `/grants`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active account/scope.
- Viewer role.
- Grant/public-interest data freshness.
- Primary action: Find Grants.
- Secondary actions: My Grants, Public-Interest Pools, Reports.

### Grant And Pool Discovery

Content:

- Search input.
- Purpose tag filter.
- Program type filter.
- Eligibility filter.
- Resource dimension filter.
- Sponsor/stewardship source marker.
- Active/retired/suspended state.
- Public-interest pool cards.
- Grant program cards.

Links and handoffs:

- Directory Listings.
- Central AI Stewardship.
- Native App Catalog.

### Grant Detail Panel

Content:

- Program name.
- Purpose tags.
- Sponsor/source summary.
- Eligible parties.
- Eligible workload classes.
- Resource dimensions.
- Quota windows.
- Fairness rules.
- Reporting requirements.
- Abuse/throttle policy summary.
- Current program state.

Links and handoffs:

- Wallet.
- Privacy And Permissions Center.
- Disputes And Appeals.

### Eligibility And Application Panel

Content:

- Applicant account/scope.
- Required evidence refs.
- Purpose tag evidence state.
- Workload/app/project refs.
- Requested resource dimensions.
- Time window.
- Policy preview.
- Eligibility result.
- Draft, submit, withdraw, and revise controls.

Links and handoffs:

- Workspace.
- Overasset Assets.
- App Detail.
- Personal AI Assistant.

### My Grants And Allocations

Content:

- Active grants.
- Pending applications.
- Denied applications.
- Expired grants.
- Suspended or revoked grants.
- Remaining allocation projections.
- Usage refs.
- Reporting deadlines.
- Renewal actions.

Links and handoffs:

- Wallet.
- Activity And Receipts Timeline.

### Public-Interest Pools

Content:

- Pool list.
- Contribution source summary.
- Purpose scope.
- Eligible grantee rules.
- Pool quota.
- Fairness window.
- Pool exhaustion marker.
- Abuse throttle marker.
- Outcome report links.

Links and handoffs:

- Central AI Stewardship.
- Stewardship Reports.

### Usage And Outcome Reports

Content:

- Usage summary.
- ORU/Seal Ledger refs.
- Overmeter refs.
- Grant authorization refs.
- Outcome report refs.
- Public/private redaction marker.
- Missing report warnings.
- Export where allowed.

### Appeals And Corrections

Content:

- Denial reason codes.
- Throttle reason codes.
- Revocation refs.
- Correction refs.
- Appeal deadlines.
- Open appeal action.
- More-evidence request state.

Links and handoffs:

- Disputes And Appeals.
- Central AI Stewardship.

## Primary Actions

- Search grants.
- Open grant.
- Check eligibility.
- Apply.
- Submit evidence.
- View allocation.
- Open report.

## Secondary Actions

- Save grant.
- Follow project.
- Renew grant.
- Withdraw application.
- Open appeal.
- Export report.
- Ask AI to explain requirements.

## States

- Empty discovery.
- Loading.
- Live.
- No eligible grants.
- Eligibility pending.
- Eligible.
- Not eligible.
- Application draft.
- Application submitted.
- Authorization denied.
- Grant active.
- Quota exhausted.
- Fairness window exhausted.
- Abuse throttle active.
- Grant suspended.
- Grant revoked.
- Reporting overdue.
- Partial owner-service outage.
- Offline cached view.

## Permissions And Privacy Behavior

- Grant and pool pages must show only redacted source, sponsor, evidence, usage, and outcome data unless the viewer is authorized for more detail.
- Purpose tag evidence, private project evidence, private workload data, and raw accounting records must not leak through public views.
- Overdesk must not authorize grants, mutate ORU balances, append ledger entries, define purpose tags, schedule workloads, or decide central AI stewardship priorities.
- Overgrant owns grant authorization refs; Public-Interest Pool Service owns pool definitions and allocation request records; Purpose Tag Registry owns purpose tag definitions; ORU Account Service, Seal Ledger, Overmeter, Overguard, Central AI Stewardship, and owner services own final truth.

## Design Notes

- Use discovery cards for public browsing and tables for active allocations.
- Put eligibility and application controls in a right-side panel so users can compare grants quickly.
- Keep purpose tags, quota windows, reporting requirements, and denial reason codes visible.
- Separate public project promotion from private application evidence.
- Do not use donation or grant language that implies guaranteed funding before owner services authorize it.
