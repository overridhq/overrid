# Central AI Stewardship

## Slug

`central-ai-stewardship`

## Title

Central AI Stewardship

## Navigation Group

Daily Apps

## Description

Central AI Stewardship is the Overdesk surface for public and authorized views of central AI recommendations, fraud and abuse evidence summaries, grants, public-interest projects, surplus-routing proposals, appeals, system-health summaries, and stewardship reports. It must make central AI inspectable, bounded, and correctable.

## Primary Users

- Public users
- Affected users and organizations
- Stewards and reviewers
- Operators
- Public-interest project owners
- Grantees and donors
- Builders and app owners

## Primary User Goals

- Understand central AI recommendations without trusting a black box.
- Review evidence refs, confidence, proportionality, owner-service refs, and appeal paths.
- Browse public-interest projects and grant or surplus-routing status.
- Follow public reports, corrections, and withdrawals.
- Submit correction, appeal, or more-evidence requests where allowed.
- See which service can act and which service only displays state.

## Entry Points

- Daily Apps navigation.
- Home Dashboard stewardship shortcut.
- Notifications Center.
- Grants And Public-Interest Projects.
- Governance Center.
- Wallet grant or surplus ref.
- Incident Reports.
- Address bar command: `/central-ai`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active account/scope.
- Audience class: public, affected party, steward, operator, auditor, or compliance reviewer.
- Redaction state.
- System freshness state.
- Primary action: Open Recommendation.
- Secondary actions: Reports, Projects, Appeals, Work Queue.

### Recommendation Feed

Content:

- Recommendation type.
- Affected service or project.
- Confidence and proportionality summary.
- Severity or priority.
- Review state.
- Owning service.
- Appeal/correction marker.
- Public/private redaction marker.

Links and handoffs:

- Governance Center.
- Disputes And Appeals.
- Incident Reports.

### Recommendation Detail

Content:

- Recommendation title.
- Central AI recommendation ref.
- Evidence package refs.
- Model/run provenance refs.
- Policy thresholds.
- Confidence summary.
- Proportionality summary.
- Downstream owner refs.
- Review state.
- Allowed actions for current role.
- Appeal and correction path.

Links and handoffs:

- Personal AI Assistant.
- Activity And Receipts Timeline.
- Security And Compliance Reviews.

### Public-Interest Projects

Content:

- Project list.
- Purpose tags.
- Public-interest pool refs.
- Grant refs.
- Outcome refs.
- Eligibility state.
- Reporting summary.
- Donation/source summary where public.
- Follow/save state.

Links and handoffs:

- Grants And Public-Interest Projects.
- Wallet.
- Directory Listings.

### Surplus Routing View

Content:

- Native app or service ref.
- Structural surplus refs.
- Proposed pool, grant, or project refs.
- Central AI recommendation refs.
- Steward review state.
- Public reporting refs.
- No forecast or revenue assumption fields.

Links and handoffs:

- Wallet.
- Stewardship Reports.
- Protocol Improvement Proposals.

### Evidence And Redaction Panel

Content:

- Audience class.
- Evidence summary.
- Hidden/private evidence count.
- Redaction reason.
- Source refs.
- Confidence marker.
- Missing evidence warnings.
- Correction request control.

### Review And Action Panel

Content:

- Accept, reject, request more evidence, correct, retract, publish, withhold, or escalate actions where role permits.
- Required signer.
- Reason code.
- Downstream service target.
- Idempotency state.
- Policy check result.
- Owner-service acceptance or denial.

### Reports And Corrections Panel

Content:

- Published reports.
- Draft report state where authorized.
- Corrections.
- Withdrawal notices.
- Publication refs.
- Redaction review status.
- Export where allowed.

## Primary Actions

- Open recommendation.
- View evidence summary.
- Submit correction.
- Submit appeal.
- Follow project.
- Open report.
- Create review action where authorized.

## Secondary Actions

- Filter by audience.
- Filter by owner service.
- Request more evidence.
- Export public report.
- Open grant refs.
- Open system-health summary.
- Ask AI to explain public context.

## States

- Empty public feed.
- Loading.
- Live.
- Redacted view.
- Authorized view.
- Recommendation missing evidence.
- Review pending.
- Downstream owner unavailable.
- Appeal path missing.
- Report withheld.
- Report corrected.
- Report withdrawn.
- Private evidence denied.
- Partial service outage.
- Error with retry.

## Permissions And Privacy Behavior

- Every view must resolve an audience class before evidence is fetched or rendered.
- Public views must default to aggregate or redacted summaries.
- Private evidence, raw fraud signals, payment details, vault secrets, private workspace data, Docdex/RAG context, and private messages must not be exposed.
- Severe actions, grant decisions, public report publication, sanctions, and compliance-sensitive actions require role-bound signed review action envelopes.
- Central AI Service owns recommendations and analysis refs; owner services own final mutations; Overdesk owns the desktop view and signed action draft flow only.

## Design Notes

- Use a work-queue plus detail-panel layout for reviewers and a simplified public-report layout for public viewers.
- Keep confidence, evidence, review state, owner service, and appeal path visible before action buttons.
- Show "not enough evidence" as a blocking product state, not a small warning.
- Do not make central AI look like an unchecked final authority; every serious action needs owner-service and review state.
- Separate public-interest project browsing from fraud/review work queues.
