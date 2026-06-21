# App Incidents And Support

## Slug

`app-incidents-support`

## Title

App Incidents And Support

## Navigation Group

Apps, Deployment, And Operations

## Description

App Incidents And Support is the Overdesk page for app-owner support inboxes, abuse reports, operational incidents, policy/compliance reviews, support bundles, user-visible status, containment requests, recovery refs, communications, disputes, and post-incident follow-ups related to apps. It gives app teams an incident and support workspace while keeping Incident Response, Overwatch, Overguard, Overclaim, recovery services, and app owner services as the authorities.

## Primary Users

- App owners
- App support operators
- Developers
- Organization admins
- Institution app operators
- Incident commanders with app scope
- Security/compliance reviewers
- Stewards reviewing public-interest app incidents

## Primary User Goals

- See app support items, abuse reports, incidents, disputes, and policy reviews in one place.
- Triage issues by severity, affected app, route, release, user-visible state, and deadline.
- Open or link incident cases with evidence refs.
- Request containment, rollback, route shift, recovery, communication, or support-bundle actions through the proper owner service.
- Communicate status safely to affected users or public reports where allowed.
- Track recovery, verification, post-incident follow-up, and dispute/appeal links.
- Export redacted support or replay bundles.

## Entry Points

- Apps, Deployment, And Operations navigation.
- Owned Apps incidents/support section.
- App Detail incidents/support section.
- Release And Rollback Manager failed health gate or incident row.
- Deploy New App launch monitor failure.
- Developer Console logs/support section.
- Disputes And Appeals incident-linked case.
- Notifications Center incident/support alert.
- Activity And Receipts Timeline incident event.
- Address bar command: `/app-support`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active owner scope.
- App filter.
- Open issue count.
- Incident severity summary.
- Support SLA/deadline marker where applicable.
- Public status marker.
- Primary action: Open Support Item.
- Secondary actions: Declare Incident, Export Bundle, Filters.

### Inbox And Case List

Content:

- Support item title.
- Type: support, abuse report, operational incident, policy review, compliance review, dispute, or post-incident follow-up.
- App/service ref.
- Affected route or release.
- Severity.
- State.
- Owner/assignee.
- Deadline.
- User-visible status marker.
- Last update.
- Redaction marker.

Links and handoffs:

- App Detail.
- Release And Rollback Manager.
- Disputes And Appeals.

### Case Detail

Content:

- Case id/ref.
- App/service refs.
- Affected scope summary.
- Severity.
- Current state.
- Parties/audiences.
- Owner assignments.
- Source refs.
- Evidence refs.
- Policy refs.
- Recovery refs.
- Dispute refs.
- Replay refs.

Links and handoffs:

- Activity And Receipts Timeline.
- Incident Reports.

### Evidence And Support Bundles

Content:

- Overwatch event refs.
- Logs with redaction.
- Deployment refs.
- Release refs.
- User-submitted evidence refs.
- Support bundle preview.
- Redaction profile.
- Export eligibility.
- Evidence completeness marker.
- Missing evidence warnings.

Links and handoffs:

- Diagnostics And Support Bundles.
- Overvault Secure Storage Center.

### Incident Declaration And Severity

Content:

- Promote support item to incident action.
- Incident type.
- Severity selector.
- Initial affected-scope refs.
- Commander/owner assignment.
- Evidence requirement.
- Overguard policy check.
- Incident Response case refs.
- Declaration confirmation.

Links and handoffs:

- Incident Reports.
- Security And Compliance Reviews.

### Containment And Recovery

Content:

- Containment request drafts.
- Rollback request handoff.
- Route-shift request handoff.
- Policy block/challenge/quarantine request where authorized.
- Recovery steps.
- Verification refs.
- Health gate status.
- Owning-service response state.
- Expiry and rollback refs for containment.

Links and handoffs:

- Release And Rollback Manager.
- Governance Center.
- Activity And Receipts Timeline.

### Communications And Status

Content:

- Audience classes.
- User-visible status.
- Internal-only notes marker.
- Draft communication refs.
- Approval/redaction state.
- Sent/published/corrected/retracted markers.
- Public report handoff where required.
- Last communication timestamp.

Links and handoffs:

- Stewardship Reports.
- Incident Reports.

### Disputes, Appeals, And Follow-Ups

Content:

- Linked disputes.
- Moderation appeals.
- Billing/accounting impact refs.
- Abuse report outcomes.
- Post-incident follow-up actions.
- Corrective action owners.
- Due dates.
- Closure requirements.
- Finality markers.

Links and handoffs:

- Disputes And Appeals.
- Wallet.
- Grants And Public-Interest Projects.

## Primary Actions

- Open support item.
- Declare incident.
- Assign owner.
- Add evidence.
- Export redacted support bundle.
- Request rollback.
- Request containment.
- Publish or correct status where authorized.
- Close case.

## Secondary Actions

- Filter cases.
- Link dispute.
- Link release.
- Copy case ref.
- Open app detail.
- Open activity refs.
- Ask AI to summarize redacted case data.
- Create follow-up action.

## States

- Empty inbox.
- Loading.
- Live.
- Support item open.
- Abuse report open.
- Incident suspected.
- Incident declared.
- Triaging.
- Containment requested.
- Containment denied.
- Recovery running.
- Verification pending.
- Communication pending approval.
- Public status published.
- Dispute linked.
- Follow-up open.
- Closed.
- Permission denied.
- Partial owner-service outage.
- Offline cached view.
- Error with retry.

## Permissions And Privacy Behavior

- App Incidents And Support displays support and incident projections and drafts actions. Incident Response Service owns incident cases; Overwatch owns event evidence; Overguard owns containment authorization; owning services execute containment/recovery; Overclaim owns disputes and appeals.
- Support bundles, logs, evidence, communications, and replay exports must be redacted by default and reviewed before export or publication.
- Raw private content, vault secrets, payment data, encrypted Docdex/RAG context, exact location trails, exploit details, fraud heuristics, child-safety evidence, and unrelated-party data must not appear in broad app-support views.
- Containment, rollback, route shift, policy block, quarantine, challenge, and closure actions require signed authority, evidence refs, policy refs, and clear owner-service response state.
- Offline cached support data must be read-only except for safe local drafts that are revalidated online before submission.

## Design Notes

- Use an inbox table for triage and a case detail panel with tabs for Evidence, Timeline, Recovery, Communications, and Follow-Ups.
- Keep severity and user-visible status prominent without exposing protected evidence.
- Make redaction profile and audience class visible before exports or communications.
- Keep support items and formal incidents connected but visually distinct so normal support does not look like an emergency.
