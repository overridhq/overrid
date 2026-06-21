# Security And Compliance Reviews

## Slug

`security-compliance-reviews`

## Title

Security And Compliance Reviews

## Navigation Group

Governance And Public Trust

## Description

Security And Compliance Reviews is the Overdesk page for threat models, security reviews, remediation tracking, compliance boundary summaries, launch gates, exception records, redacted evidence, release blockers, and follow-up actions. It connects developers, app owners, stewards, and reviewers to the review state required before broader deployment or public exposure, while keeping final authority in the Compliance Boundary Service, Threat Modeling and Security Review Tracker, owner services, Incident Response Service, and release/deployment systems.

## Primary Users

- Security reviewers
- Compliance reviewers
- Stewards
- Developers
- App owners
- Provider operators
- Organization admins
- Institution admins
- Release operators

## Primary User Goals

- See security and compliance review state for apps, services, PIPs, releases, providers, native apps, and public-network changes.
- Understand affected services, data classes, jurisdictions, risk classes, and required evidence.
- Track threat model coverage and remediation status.
- Review compliance boundary summaries and rule-set changes.
- Approve, reject, request changes, or accept exceptions where authorized.
- Open related incidents, PIPs, releases, deployments, and stewardship reports.
- Prepare public disclosure summaries without exposing sensitive evidence.

## Entry Points

- Governance Center.
- Governance And Public Trust navigation.
- Developer Console.
- Deploy New App.
- Release And Rollback Manager.
- App Detail governance/security tab.
- Protocol Improvement Proposals.
- Incident Reports.
- Settings And Security.
- Address bar command: `/reviews`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active review scope.
- Open review count.
- Blocking finding count.
- Exception count.
- Missing evidence count.
- Reviews due soon.
- Public disclosure pending count.
- Primary action: Open Review Queue.
- Secondary actions: Threat Models, Compliance Boundaries, Exceptions, Remediation.

Links and handoffs:

- Governance Center.
- Developer Console.
- Release And Rollback Manager.
- Incident Reports.

### Review List

Content:

- Review id.
- Target type: app, service, release, PIP, provider, integration, native app, or migration.
- Target ref.
- Review class.
- Risk class.
- Status.
- Blocking marker.
- Required evidence state.
- Assigned reviewers.
- Due date.
- Public/private marker.

Links and handoffs:

- App Detail.
- Deploy New App.
- Protocol Improvement Proposals.
- Release And Rollback Manager.

### Review Detail

Content:

- Review summary.
- Target metadata.
- Requested change or release refs.
- Review owner refs.
- Review scope.
- Data classes involved.
- User-impact summary.
- Public-provider eligibility marker.
- Current decision state.
- Next required action.

Links and handoffs:

- Developer Console.
- App Detail.
- Governance Center.

### Scope And Affected Services

Content:

- Affected service refs.
- Affected native app refs.
- API/contract refs.
- Data storage refs.
- Wallet/accounting refs.
- RAG/AI context refs.
- Public-network exposure marker.
- Provider/resource exposure marker.
- Migration/release refs.

Links and handoffs:

- Native App Catalog.
- Wallet.
- Docdex And RAG Index Manager.
- Release And Rollback Manager.

### Threat Model Coverage

Content:

- Threat model id.
- Baseline coverage state.
- Abuse/fraud/security/compliance domains.
- Attack surface summary.
- Trust boundary summary.
- Asset/data-class map.
- Control coverage.
- Open model gaps.
- Taxonomy version.
- Last update receipt.

Links and handoffs:

- Stewardship Reports.
- Incident Reports.
- Developer Console.

### Risk Findings

Content:

- Finding id.
- Severity.
- Confidence.
- Affected component.
- Evidence class.
- Redacted summary.
- Exploit-sensitive marker.
- Remediation owner.
- Due date.
- Blocker state.
- Accepted exception marker.

Links and handoffs:

- App Incidents And Support.
- Incident Reports.
- Release And Rollback Manager.

### Compliance Boundary

Content:

- Jurisdiction/profile refs.
- Domain marker.
- Privacy requirements.
- Deletion/retention requirements.
- Child-safety requirements where relevant.
- Payment/payout requirements.
- Dispute/hold requirements.
- Residency requirements.
- Owning service refs.
- Rule-set version.
- Staged comparison result.

Links and handoffs:

- Privacy And Permissions Center.
- Wallet.
- Disputes And Appeals.
- Stewardship Reports.

### Required Evidence

Content:

- Required evidence checklist.
- Submitted evidence refs.
- Missing evidence.
- Stale evidence.
- Redaction status.
- Test/drill evidence.
- Migration evidence.
- Release evidence.
- Operator approval refs.
- Audit receipt refs.

Links and handoffs:

- Activity And Receipts Timeline.
- Release And Rollback Manager.
- Diagnostics And Support Bundles.

### Approval Exceptions And Decisions

Content:

- Decision options allowed by role.
- Required approval classes.
- Approval/refusal reason codes.
- Exception request details.
- Exception expiry.
- Exception owner.
- Re-review schedule.
- Conflict-of-interest markers.
- Decision receipts.

Links and handoffs:

- Governance Center.
- Protocol Improvement Proposals.
- Stewardship Reports.

### Remediation And Follow-Up

Content:

- Remediation tasks.
- Owners.
- Due dates.
- Status.
- Release blocker linkage.
- Incident linkage.
- Regression test refs.
- Follow-up review refs.
- Closure evidence.
- Public report linkage.

Links and handoffs:

- Developer Console.
- Release And Rollback Manager.
- Incident Reports.
- Stewardship Reports.

### Disclosure And Publication

Content:

- Public disclosure eligibility.
- Redacted summary.
- Report refs.
- Correction/retraction refs.
- Sensitive-evidence exclusion marker.
- Publication readiness.
- Public user impact.
- Communication refs.

Links and handoffs:

- Stewardship Reports.
- Updates And Release Notes.
- Incident Reports.

## Primary Actions

- Open review.
- Draft reviewer decision.
- Request changes.
- Approve where authorized.
- Reject where authorized.
- Request exception.
- Open remediation task.
- Open compliance boundary.
- Open public disclosure summary.

## Secondary Actions

- Filter by review class.
- Filter by risk class.
- Filter by service/app.
- Filter by blocker state.
- Copy review ref.
- Export redacted review metadata.
- Open linked incident/PIP/release/report.
- Ask AI to summarize allowed review context.

## States

- Loading.
- Live.
- No reviews.
- Draft.
- Evidence pending.
- In review.
- Changes requested.
- Blocking.
- Exception requested.
- Exception accepted.
- Approved.
- Rejected.
- Remediation active.
- Re-review required.
- Public disclosure pending.
- Closed.
- Action denied.

## Permissions And Privacy Behavior

- Compliance Boundary Service owns compliance markers, rule-set versions, jurisdiction/profile refs, and boundary decisions.
- Threat Modeling and Security Review Tracker owns threat models, findings, remediation state, exceptions, and review status.
- Overdesk may present allowed summaries and submit signed review drafts but cannot mutate final compliance/security truth directly.
- Exploit-sensitive evidence, private user data, fraud evidence, child-safety material, secrets, payment/payout details, vault refs, and sensitive diagnostics must never be exposed in public views.
- Review lists must avoid leaking hidden private reviews through counts, names, or unavailable placeholders.
- Release-blocking findings must remain visible to authorized release operators until closure evidence is accepted.
- Public disclosure summaries must route through redaction and stewardship publication rules.

## Design Notes

- Use an operational review-queue layout, with blockers and due dates prominent.
- Keep finding summaries short and redacted, with deeper evidence visible only to authorized reviewers.
- Use severity, blocker, evidence, exception, and disclosure chips consistently.
- Put compliance boundary summaries next to security findings so reviewers see both technical and policy constraints.
- Use clear empty states that do not imply private review counts.
- Make decision controls explicit and gated by role, freshness, and conflict-of-interest status.
