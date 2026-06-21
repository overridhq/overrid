# Incident Reports

## Slug

`incident-reports`

## Title

Incident Reports

## Navigation Group

Governance And Public Trust

## Description

Incident Reports is the Overdesk page for public and scoped incident visibility across operational, abuse, fraud, security, compliance, reporting, central AI, wallet, vault, provider, and native-app incidents. It shows incident state, affected services, timeline, containment, recovery, communication, evidence class, post-incident review, correction notices, and follow-up work while keeping Incident Response Service, owner services, Compliance Boundary Service, Stewardship Reporting Service, and Security Review Tracker as the authoritative systems.

## Primary Users

- Regular users
- Stewards
- Incident responders
- Security reviewers
- Compliance reviewers
- App owners
- Provider operators
- Organization admins
- Support operators

## Primary User Goals

- See incidents that affect the active user, app, institution, provider, service, or public ecosystem.
- Understand severity, current state, affected services, and required user action.
- Follow containment, recovery, communication, and post-incident review progress.
- Open related stewardship reports, security reviews, PIPs, app support cases, and release/rollback records.
- Review public or scoped incident details without exposing private evidence.
- Submit allowed updates, acknowledgements, or follow-up review drafts.
- Track correction, retraction, and supersession notices tied to incident reporting.

## Entry Points

- Governance Center.
- Governance And Public Trust navigation.
- App Incidents And Support.
- Security And Compliance Reviews.
- Stewardship Reports.
- Release And Rollback Manager.
- Activity And Receipts Timeline.
- Notifications Center.
- Address bar command: `/incidents`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active audience/scope.
- Open incident count.
- High-severity incident count.
- Incidents requiring user action.
- Public notice count.
- Post-incident reviews due.
- Primary action: Review Incidents.
- Secondary actions: Followed, My Apps, My Devices, Public Notices, Export Public Index.

Links and handoffs:

- Governance Center.
- App Incidents And Support.
- Security And Compliance Reviews.
- Stewardship Reports.

### Incident List

Content:

- Incident id.
- Title.
- Incident class.
- Severity.
- Status.
- Affected services/apps/devices.
- User impact marker.
- Public/private/audience-limited marker.
- Last update time.
- Required action marker.
- Follow state.

Links and handoffs:

- App Detail.
- Node Detail.
- Wallet.
- Overvault Secure Storage Center.

### Incident Detail

Content:

- Summary.
- Declared state.
- Severity rationale.
- Incident class.
- Detection source.
- Affected service refs.
- Affected user/app/provider refs where allowed.
- Current owner role refs.
- Public user impact.
- Current next step.

Links and handoffs:

- Governance Center.
- App Incidents And Support.
- Security And Compliance Reviews.

### Timeline And Roles

Content:

- Detection time.
- Declaration time.
- Triage milestones.
- Role assignments.
- Containment requests.
- Containment acknowledgements.
- Recovery steps.
- Communications.
- Report publication events.
- Follow-up events.
- Replay refs.

Links and handoffs:

- Activity And Receipts Timeline.
- Stewardship Reports.
- Diagnostics And Support Bundles.

### Impact And Affected Surfaces

Content:

- Affected service refs.
- Affected app refs.
- Affected node/provider refs.
- Affected wallet/accounting refs.
- Affected vault/key refs where allowed.
- Affected RAG/AI refs where allowed.
- Regional/institutional scope where allowed.
- Data class markers.
- Availability/integrity/confidentiality markers.
- User action guidance.

Links and handoffs:

- Wallet.
- Docdex And RAG Index Manager.
- Node Fleet Manager.
- Owned Apps.

### Containment And Recovery

Content:

- Containment requests.
- Owning service acknowledgements.
- Recovery plan refs.
- Recovery step state.
- Rollback refs.
- Migration/failover refs.
- Validation evidence refs.
- Reopen criteria.
- Remaining risk summary.

Links and handoffs:

- Release And Rollback Manager.
- Security And Compliance Reviews.
- Local Device Settings.

### Communications And Public Notices

Content:

- Public notice status.
- Audience-limited notice status.
- User notification state.
- Communication owner.
- Publication refs.
- Correction/retraction refs.
- External communication marker where allowed.
- User acknowledgement state.

Links and handoffs:

- Notifications Center.
- Messaging Center.
- Stewardship Reports.
- Updates And Release Notes.

### Evidence And Redaction

Content:

- Evidence bundle refs.
- Evidence class.
- Redaction profile.
- Private evidence marker.
- Sensitive incident marker.
- Reviewer visibility state.
- Missing evidence warnings.
- Evidence retention policy.
- Export-safe metadata.

Links and handoffs:

- Security And Compliance Reviews.
- Stewardship Reports.
- Diagnostics And Support Bundles.

### Post-Incident Review

Content:

- Review status.
- Findings summary.
- Root-cause summary where publishable.
- Follow-up action refs.
- Security review refs.
- Compliance review refs.
- PIP refs.
- Stewardship report refs.
- Publication readiness.

Links and handoffs:

- Protocol Improvement Proposals.
- Security And Compliance Reviews.
- Stewardship Reports.

### Follow-Up And Corrections

Content:

- Open follow-up tasks.
- Owners.
- Due dates.
- Remediation evidence.
- Correction notices.
- Retraction notices.
- Supersession refs.
- User compensation/appeal refs where relevant.
- Closure criteria.

Links and handoffs:

- Disputes And Appeals.
- Wallet.
- App Incidents And Support.
- Governance Center.

## Primary Actions

- Open incident.
- Follow incident.
- Acknowledge user-impact notice.
- Submit allowed update.
- Draft responder note where authorized.
- Open containment/recovery refs.
- Open post-incident review.
- Open public notice.

## Secondary Actions

- Filter by severity.
- Filter by affected service.
- Filter by status.
- Filter by audience.
- Copy public incident link.
- Export public incident metadata.
- Open related report/review/PIP/release.
- Ask AI to explain allowed incident context.

## States

- Loading.
- Live.
- No visible incidents.
- Suspected.
- Triaging.
- Confirmed.
- Containment requested.
- Contained.
- Recovering.
- Monitoring.
- Resolved.
- Post-incident review pending.
- Correction published.
- Retracted.
- Reopened.
- Action denied.

## Permissions And Privacy Behavior

- Incident Response Service owns incident state, roles, containment requests, recovery steps, communication refs, post-incident review refs, and follow-up refs.
- Owner services execute containment and recovery; Overdesk cannot directly mutate service, ledger, vault, policy, payout, or deployment state.
- Public users see public or audience-allowed notices only.
- Sensitive incident evidence, private user data, exploit details, fraud evidence, child-safety material, vault refs, secret material, and payment/payout details remain hidden unless the viewer has explicit review authority.
- Public reports route through Stewardship Reporting Service and required redaction.
- Severe security, privacy, compliance, ledger, vault, public-provider, central-AI, or public-report cases cannot be softened in UI labels without evidence-backed state changes.

## Design Notes

- Use a timeline-first detail view so incident evolution is easy to scan.
- Make severity, status, affected surface, public visibility, and required action visible in every list row.
- Keep private evidence counts and hidden affected-user counts from leaking through public UI.
- Put user action guidance above responder-only controls.
- Use status chips consistently with Security And Compliance Reviews and App Incidents And Support.
- Clearly distinguish public notices from responder notes and private evidence.
