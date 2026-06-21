# Protocol Improvement Proposals

## Slug

`protocol-improvement-proposals`

## Title

Protocol Improvement Proposals

## Navigation Group

Governance And Public Trust

## Description

Protocol Improvement Proposals is the Overdesk page for reading, following, commenting on, reviewing, and tracing Overrid protocol and service-contract changes. It presents PIPs as evidence-backed change records with affected services, rationale, review requirements, implementation evidence, migration plans, rollback plans, decisions, corrections, and publication history. The PIP Registry owns proposal truth; Overdesk owns desktop presentation and signed user/steward drafts.

## Primary Users

- Regular users
- Stewards
- Protocol maintainers
- Service owners
- App owners
- Developers
- Compliance reviewers
- Security reviewers
- Public-interest reviewers

## Primary User Goals

- Understand proposed protocol, accounting, security, compliance, governance, migration, and service-contract changes.
- See which services, native apps, SDKs, ledgers, wallets, or public-network flows are affected.
- Follow PIPs and receive updates.
- Comment where the proposal allows public or scoped participation.
- Review required evidence before supporting, blocking, or requesting changes.
- Trace accepted PIPs to implementation, migration, release, and rollback evidence.
- Confirm that non-trivial changes have explicit rollback and reversibility plans.

## Entry Points

- Governance Center.
- Governance And Public Trust navigation.
- Release And Rollback Manager.
- Developer Console.
- App Detail governance tab.
- Security And Compliance Reviews.
- Stewardship Reports.
- Address bar command: `/pips`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active scope.
- Open proposal count.
- Followed proposal count.
- Assigned review count.
- Accepted pending implementation count.
- Comment windows closing soon.
- Primary action: Browse Proposals.
- Secondary actions: Followed, Assigned, Draft Comment, Export Public Index.

Links and handoffs:

- Governance Center.
- Stewardship Reports.
- Security And Compliance Reviews.

### Proposal List

Content:

- Proposal id.
- Title.
- Summary.
- Proposal class.
- Author/submitter ref.
- Affected services.
- Status.
- Review phase.
- Comment window.
- Required reviewer classes.
- Migration marker.
- Rollback marker.
- Follow state.

Links and handoffs:

- App Detail.
- Native App Catalog.
- Release And Rollback Manager.

### Proposal Detail

Content:

- Problem statement.
- Proposed change.
- Non-goals.
- Affected contracts.
- Affected APIs.
- Affected data classes.
- Affected security/compliance boundaries.
- Backward compatibility summary.
- User-visible impact.
- Owner-service authority labels.

Links and handoffs:

- Service/tool detail when available.
- Security And Compliance Reviews.
- Developer Console.

### Evidence And Rationale

Content:

- Evidence bundle refs.
- Metrics refs.
- Incident refs.
- Support/dispute refs where allowed.
- Central AI analysis refs where allowed.
- Alternatives considered.
- Risk summary.
- Privacy impact summary.
- Cost/resource impact summary.
- Public redaction profile.

Links and handoffs:

- Stewardship Reports.
- Central AI Stewardship.
- Incident Reports.
- Wallet.

### Review Requirements

Content:

- Required steward review classes.
- Security review requirement.
- Compliance review requirement.
- Accounting/ledger review requirement.
- Migration review requirement.
- Native app owner review requirement.
- Public comment requirement.
- Conflict-of-interest markers.
- Missing evidence markers.
- Readiness score.

Links and handoffs:

- Security And Compliance Reviews.
- Governance Center steward queue.
- Stewardship Reports.

### Comments And Steward Review

Content:

- Public comments where allowed.
- Scoped organization/institution comments where allowed.
- Steward review drafts.
- Comment policy summary.
- Moderation/redaction state.
- Required disclosure markers.
- Support/oppose/request-changes controls.
- Reply thread refs.
- Comment receipt.

Links and handoffs:

- Messaging Center.
- Activity And Receipts Timeline.
- Disputes And Appeals.

### Implementation And Migration Plan

Content:

- Implementation owner refs.
- Linked workstream refs.
- Release plan.
- Migration plan refs.
- Data migration requirements.
- Compatibility window.
- SDK/client impact.
- Overdesk impact.
- Test/evidence requirements.
- Deployment gate status.

Links and handoffs:

- Release And Rollback Manager.
- Developer Console.
- App Detail.
- Updates And Release Notes.

### Rollback And Reversibility

Content:

- Rollback plan.
- Reversibility class.
- Cutover criteria.
- Fallback service path.
- Data recovery refs.
- User-impact warning.
- Post-rollback validation.
- Founder-hardware dependency marker where relevant.
- Public communication requirement.

Links and handoffs:

- Release And Rollback Manager.
- Incident Reports.
- Stewardship Reports.

### Decision And Publication History

Content:

- Decision status.
- Decision refs.
- Accepted/rejected/deferred reason.
- Required changes.
- Superseded proposal refs.
- Implementation evidence refs.
- Migration evidence refs.
- Correction/retraction refs.
- Publication timeline.

Links and handoffs:

- Stewardship Reports.
- Updates And Release Notes.
- Governance Center.

## Primary Actions

- Follow proposal.
- Open proposal detail.
- Submit eligible comment.
- Draft steward review.
- Request changes where authorized.
- Open implementation evidence.
- Open migration evidence.
- Open rollback plan.

## Secondary Actions

- Filter by status.
- Filter by affected service.
- Filter by proposal class.
- Copy public PIP link.
- Export public proposal metadata.
- Open related incident/report/review.
- Ask AI to summarize proposal impact.

## States

- Loading.
- Live.
- No proposals.
- Draft.
- Submitted.
- Under review.
- Comment open.
- Comment closed.
- Changes requested.
- Accepted.
- Rejected.
- Deferred.
- Implementing.
- Migrating.
- Superseded.
- Withdrawn.
- Action denied.

## Permissions And Privacy Behavior

- The PIP Registry owns proposal records, status, decision state, and publication history.
- Overdesk may submit signed comment/review drafts only within the user's allowed scope.
- Private evidence is shown only as redacted refs unless the viewer has a specific review permission.
- Non-trivial protocol changes must remain tied to a PIP, accepted PIPs must link implementation and migration evidence, and rollback plans must be explicit.
- Comments must respect audience, moderation, redaction, and conflict-of-interest rules.
- Central AI summaries must be labeled as analysis or recommendation, not final governance decisions.

## Design Notes

- Use a two-pane list/detail layout on desktop and a stacked list/detail pattern on narrow screens.
- Make affected services scannable through chips, not long prose.
- Use persistent status rails for proposal state, required reviews, migration state, and rollback readiness.
- Keep public comments readable but visually secondary to evidence and review requirements.
- Highlight missing rollback, missing migration evidence, and missing required review coverage.
- Do not expose private evidence names or hidden reviewer identities through counts or empty states.
