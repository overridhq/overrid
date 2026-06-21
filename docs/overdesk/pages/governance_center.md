# Governance Center

## Slug

`governance-center`

## Title

Governance Center

## Navigation Group

Governance And Public Trust

## Description

Governance Center is the Overdesk hub for protocol proposals, stewardship reports, central AI recommendations, compliance boundaries, security review status, incident notices, public corrections, migration evidence, and public-interest governance activity. It gives users and stewards one place to understand how Overrid is being changed and supervised, while keeping governance authority in the PIP Registry, Stewardship Reporting Service, Central AI Service, Compliance Boundary Service, Incident Response Service, security review tracker, migration tooling, and owning services.

## Primary Users

- Regular users
- Stewards
- Central AI reviewers
- Security reviewers
- Compliance reviewers
- App owners
- Provider operators
- Organization admins
- Public-interest project reviewers

## Primary User Goals

- See governance activity across proposals, reports, reviews, incidents, corrections, and public-interest funding.
- Follow protocol changes and understand affected services.
- Review central AI recommendations and their human/steward review state.
- Track public reports, native-service surplus reports, public-interest reports, and correction notices.
- Open compliance/security review summaries without exposing sensitive evidence.
- Comment on eligible proposals or reports.
- Work assigned stewardship queues from one entry point.
- See how governance decisions connect to migration, release, incident, and funding evidence.

## Entry Points

- Governance And Public Trust navigation.
- Central AI Stewardship.
- Protocol Improvement Proposals.
- Stewardship Reports.
- Security And Compliance Reviews.
- Incident Reports.
- Grants And Public-Interest Projects.
- Release And Rollback Manager.
- App Detail governance tab.
- Address bar command: `/governance`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active account/scope.
- Public/steward/admin mode marker.
- Open proposal count.
- Active report count.
- Open review count.
- Active incident notice count.
- Public correction count.
- Primary action: Review Governance Feed.
- Secondary actions: Proposals, Reports, Reviews, Incidents.

Links and handoffs:

- Protocol Improvement Proposals.
- Stewardship Reports.
- Security And Compliance Reviews.
- Incident Reports.

### Governance Overview

Content:

- Governance health summary.
- Latest accepted PIP.
- Latest public stewardship report.
- Latest central AI recommendation requiring review.
- Open compliance boundary changes.
- Security review status summary.
- Incident and correction notice summary.
- Migration evidence summary.
- Public-interest funding signal summary.

Links and handoffs:

- Central AI Stewardship.
- Grants And Public-Interest Projects.
- Release And Rollback Manager.

### Proposal Feed

Content:

- Proposal id.
- Proposal title.
- Proposal type.
- Affected services.
- Status.
- Required review coverage.
- Comment window.
- Migration and rollback markers.
- Implementation evidence marker.
- User follow state.

Links and handoffs:

- Protocol Improvement Proposals.
- App Detail.
- Release And Rollback Manager.

### Stewardship Reports

Content:

- Report id.
- Report class.
- Reporting period.
- Published/private/audience-limited marker.
- Evidence class summary.
- Redaction profile.
- Correction/retraction state.
- Native-service surplus report marker.
- Public-interest project refs.
- Central AI recommendation refs.

Links and handoffs:

- Stewardship Reports.
- Central AI Stewardship.
- Wallet.
- Grants And Public-Interest Projects.

### Central AI Recommendations

Content:

- Recommendation ref.
- Recommendation class.
- Target service/project/report/proposal ref.
- Evidence bundle marker.
- Human/steward review state.
- Confidence and limitation summary.
- Decision authority label.
- Appeal/correction path marker.
- Publication state.

Links and handoffs:

- Central AI Stewardship.
- Stewardship Reports.
- Disputes And Appeals.

### Compliance And Security Status

Content:

- Compliance boundary summary.
- Jurisdiction/profile marker.
- Pending compliance reviews.
- Security review coverage.
- Threat model status.
- Open high-severity findings.
- Accepted exceptions.
- Remediation due dates.
- Public/private disclosure state.

Links and handoffs:

- Security And Compliance Reviews.
- Developer Console.
- Deploy New App.
- Release And Rollback Manager.

### Incident And Correction Notices

Content:

- Incident notice refs.
- Severity markers.
- Affected service refs.
- Public status.
- Containment/recovery markers.
- Post-incident report marker.
- Correction/retraction/supersession notices.
- User action needed marker.

Links and handoffs:

- Incident Reports.
- App Incidents And Support.
- Activity And Receipts Timeline.

### Steward Work Queue

Content:

- Assigned proposals.
- Assigned reports.
- Assigned compliance reviews.
- Assigned security reviews.
- Pending redaction review.
- Conflict-of-interest marker.
- Due dates.
- Missing evidence markers.
- Ready-to-publish marker.

Links and handoffs:

- Protocol Improvement Proposals.
- Stewardship Reports.
- Security And Compliance Reviews.
- Central AI Stewardship.

### Public Interest And Funding Signals

Content:

- Public-interest pool refs.
- Grant/project refs.
- Purpose tag refs.
- Native-service surplus routing summaries.
- Donation/source refs where public.
- Outcome report refs.
- Central AI recommendation refs.
- Steward approval state.

Links and handoffs:

- Grants And Public-Interest Projects.
- Wallet.
- Central AI Stewardship.
- Stewardship Reports.

## Primary Actions

- Open proposal feed.
- Open stewardship reports.
- Open security/compliance reviews.
- Open incident notices.
- Follow governance item.
- Submit eligible comment.
- Open assigned review.
- Open public-interest report.

## Secondary Actions

- Filter by service.
- Filter by governance type.
- Filter by status.
- Copy public report/proposal link.
- Export public report metadata.
- Open affected app/service.
- Ask AI to explain governance item.

## States

- Loading.
- Live.
- Public mode.
- Steward mode.
- Admin mode.
- No governance activity.
- Review assignment pending.
- Missing evidence.
- Redaction required.
- Publication pending.
- Comment window closed.
- Action denied.

## Permissions And Privacy Behavior

- Public users see only public or audience-allowed governance records.
- Stewards see assigned private review refs and redacted evidence according to scope.
- Overdesk may create signed comment/review drafts but cannot accept proposals, publish reports, approve compliance boundaries, close incidents, or change security findings by itself.
- Sensitive evidence, exploit details, private user data, fraud details, child-safety material, payment data, vault refs, and raw diagnostics stay behind owner-service scopes.
- Central AI recommendations are displayed as evidence-backed recommendations, not automatic decisions.
- Correction and retraction notices must remain visible alongside superseded reports.

## Design Notes

- Treat this page as a dense governance dashboard, not a marketing page.
- Use tabs or segmented controls for Public, Followed, Assigned, and All Allowed views.
- Show authority labels on every card: PIP Registry, Stewardship Reporting, Compliance Boundary, Security Review Tracker, Incident Response, Central AI, or owning service.
- Use compact status chips for proposal state, review state, publication state, and redaction state.
- Never display hidden evidence counts in a way that leaks private activity.
- Put active user obligations at the top of steward views.
