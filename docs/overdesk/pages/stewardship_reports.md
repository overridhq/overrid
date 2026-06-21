# Stewardship Reports

## Slug

`stewardship-reports`

## Title

Stewardship Reports

## Navigation Group

Governance And Public Trust

## Description

Stewardship Reports is the Overdesk page for structured public and scoped reports about system health, grants, surplus routing, fraud and abuse controls, security, incidents, compliance, migrations, PIPs, public-interest projects, and central AI stewardship. It lets users inspect reports, evidence classes, redaction profiles, correction timelines, and publication state without exposing private user data or sensitive evidence.

## Primary Users

- Regular users
- Stewards
- Central AI reviewers
- Compliance reviewers
- Security reviewers
- Public-interest project reviewers
- App owners
- Provider operators
- Organization admins

## Primary User Goals

- Read published stewardship reports in one place.
- Understand native-service surplus routing, grant recommendations, public-interest projects, and donation/project outcomes.
- Review central AI recommendation refs and their human review state.
- See report evidence classes and redaction profiles.
- Track corrections, retractions, supersessions, and publication history.
- Prepare assigned report reviews and redaction checks.
- Export public report metadata for audit or sharing.

## Entry Points

- Governance Center.
- Governance And Public Trust navigation.
- Central AI Stewardship.
- Grants And Public-Interest Projects.
- Wallet.
- Security And Compliance Reviews.
- Incident Reports.
- Protocol Improvement Proposals.
- Address bar command: `/reports`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active audience/scope.
- Published report count.
- Draft/assigned report count where allowed.
- Correction/retraction notice count.
- Native-service surplus report count.
- Public-interest report count.
- Primary action: Browse Reports.
- Secondary actions: Assigned Reviews, Corrections, Export Index.

Links and handoffs:

- Governance Center.
- Central AI Stewardship.
- Grants And Public-Interest Projects.

### Report Library

Content:

- Report id.
- Report title.
- Report class.
- Reporting period.
- Audience class.
- Publication state.
- Evidence class.
- Redaction profile.
- Correction state.
- Related service/project refs.
- Follow state.

Links and handoffs:

- Governance Center.
- Incident Reports.
- Security And Compliance Reviews.
- Protocol Improvement Proposals.

### Report Detail

Content:

- Executive summary.
- Report scope.
- Reporting period.
- Owning service refs.
- Reported metrics.
- Affected services.
- Public-interest refs.
- Native-service refs.
- Decision/recommendation refs.
- Limitations and caveats.

Links and handoffs:

- App Detail.
- Wallet.
- Central AI Stewardship.
- Grants And Public-Interest Projects.

### Evidence And Redaction

Content:

- Evidence bundle refs.
- Evidence class summary.
- Public/private data class markers.
- Redaction profile.
- Reviewer refs where allowed.
- Unpublished evidence marker.
- Leakage risk marker.
- Publication readiness.
- Missing evidence warnings.

Links and handoffs:

- Security And Compliance Reviews.
- Activity And Receipts Timeline.
- Diagnostics And Support Bundles.

### Central AI Recommendation Trace

Content:

- Central AI recommendation refs.
- Recommendation purpose.
- Evidence summary.
- Model/route ref where public.
- Human/steward review state.
- Accepted/rejected/deferred marker.
- Decision authority label.
- Appeal/correction path.
- Downstream report/proposal/grant refs.

Links and handoffs:

- Central AI Stewardship.
- Personal AI Assistant.
- Disputes And Appeals.

### Funding Surplus And Public-Interest Reports

Content:

- Native-service surplus refs.
- ORU/Seal Ledger accounting refs where public.
- Overgrant refs.
- Provider payout summary where public.
- Public-interest pool refs.
- Purpose tag refs.
- Project outcome refs.
- Donation/source refs where public.
- Routing recommendation refs.
- Publication and correction status.

Links and handoffs:

- Wallet.
- Grants And Public-Interest Projects.
- Provider Earnings And Payouts.
- Central AI Stewardship.

### Incident Compliance And Security Reports

Content:

- Incident report refs.
- Compliance report refs.
- Security review report refs.
- Threat model report refs.
- Severity/risk markers.
- Containment/recovery markers.
- Remediation summary.
- Public disclosure state.
- Follow-up action refs.

Links and handoffs:

- Incident Reports.
- Security And Compliance Reviews.
- App Incidents And Support.

### Publication And Correction Timeline

Content:

- Draft created time.
- Review milestones.
- Redaction milestones.
- Publication time.
- Correction notices.
- Retraction notices.
- Supersession refs.
- Public archive refs.
- Change receipts.

Links and handoffs:

- Activity And Receipts Timeline.
- Governance Center.
- Protocol Improvement Proposals.

### Steward Review Queue

Content:

- Assigned reports.
- Due dates.
- Required review class.
- Redaction checklist.
- Conflict-of-interest marker.
- Missing evidence markers.
- Ready-to-publish marker.
- Requested changes.
- Publication gate status.

Links and handoffs:

- Governance Center.
- Security And Compliance Reviews.
- Central AI Stewardship.

## Primary Actions

- Open report.
- Follow report class.
- Open correction notice.
- Draft report review.
- Approve redaction where authorized.
- Request report changes where authorized.
- Export public report metadata.
- Open linked grant/project/proposal/review.

## Secondary Actions

- Filter by report class.
- Filter by reporting period.
- Filter by audience class.
- Copy public report link.
- Open evidence refs where allowed.
- Ask AI to summarize a report.
- Compare report versions.

## States

- Loading.
- Live.
- No reports.
- Draft.
- In review.
- Redaction required.
- Ready to publish.
- Published.
- Corrected.
- Retracted.
- Superseded.
- Private/audience-limited.
- Evidence unavailable.
- Action denied.

## Permissions And Privacy Behavior

- Stewardship Reporting Service owns report records, publication state, correction history, and public archive refs.
- Overdesk only presents reports and submits signed review/comment drafts within allowed scope.
- Public reports must not expose private user data, raw fraud evidence, exploit details, payment details, vault refs, or sensitive diagnostics.
- Audience-limited reports must show clear visibility markers and must not leak hidden evidence through counts, titles, or empty states.
- Central AI drafts remain non-authoritative until reviewed by the required human/steward path.
- Corrections, retractions, and supersessions must remain visible and traceable.

## Design Notes

- Use a report-library pattern with strong filtering by report class and period.
- Put evidence/redaction status near the report title so users understand trust and privacy boundaries.
- Use version/timeline components for corrections and supersessions.
- Keep public-interest and surplus summaries clear but avoid pricing projections or speculative revenue assumptions.
- Use compact tables for accounting refs, report refs, and outcome refs.
- Make steward queues operational and dense, with due dates and missing evidence called out clearly.
