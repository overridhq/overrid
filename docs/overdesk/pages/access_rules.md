# Access Rules

## Slug

`access-rules`

## Title

Access Rules

## Navigation Group

Network Contribution

## Description

Access Rules is the Overdesk page for deciding who may use a provider's contributed resources. It covers institutions, organizations, explicit users, private UUID grants, purpose tags, deny rules, expiries, reviews, and policy dry runs while keeping sensitive grant data and policy internals protected.

## Primary Users

- Individual resource providers
- University lab admins
- Organization admins
- Institution resource managers
- Private pilot operators
- Public-interest pool managers
- Delegated access managers
- Support operators with authorized provider views

## Primary User Goals

- Allow trusted institutions, organizations, users, or private pilot groups to use selected resources.
- Add private UUIDs without exposing or logging them raw.
- Use tags and purpose tags to support academic, public-interest, local community, AI/RAG, and low-sensitivity work.
- Deny risky users, organizations, tags, jurisdictions, workload classes, or risk bands.
- Set expiry, review, one-time grants, and temporary emergency blocks.
- Dry-run access rules before saving.
- See safe reason codes for allowed, denied, or blocked requests.

## Entry Points

- Network Contribution navigation.
- Add This Computer To Overrid access preview step.
- Resource Sharing Rules workload-class controls.
- Node Fleet Manager bulk access action.
- Node Detail access summary.
- Provider Earnings And Payouts node contribution breakdown.
- Disputes And Appeals resource-access case.
- Notifications Center access-policy alert.
- Address bar command: `/access-rules`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active account/scope.
- Selected node or node group.
- Access policy state.
- Draft marker.
- Last dry-run state.
- Primary action: Save Access Rules.
- Secondary actions: Dry Run, Add Grant, Emergency Block, View Activity.

### Node Scope Selector

Content:

- Current computer selector.
- Personal node selector.
- Organization node group selector.
- Institution/lab node group selector.
- Private pilot pool selector.
- Current access template marker.
- Policy drift marker.
- Selected node count.

Links and handoffs:

- Node Fleet Manager.
- Node Detail.
- Resource Sharing Rules.

### Institution And Organization Allowlist

Content:

- Allowed universities.
- Allowed labs.
- Allowed schools.
- Allowed companies.
- Allowed public-interest pools.
- Approved federation templates.
- Organization/tenant refs.
- Role scopes.
- Required verification state.
- Expiry and review date.

Links and handoffs:

- Identity And Profile Center.
- Governance Center.
- Grants And Public-Interest Projects.

### User Allowlist

Content:

- Explicit Overpass identity refs.
- User role or reason label.
- Grant scope.
- Expiry.
- Last review date.
- Current eligibility state.
- Safe denial reason where applicable.
- Remove or narrow grant action.

Links and handoffs:

- Identity And Profile Center.
- Privacy And Permissions Center.

### Private UUID Grants

Content:

- Private UUID grant list with masked display.
- Grant label.
- Scope.
- Created-by ref.
- Created-at timestamp.
- Expiry.
- Last used marker where allowed.
- Rotation requirement.
- Revoke action.
- Import/add flow for controlled pilots.

Links and handoffs:

- Activity And Receipts Timeline.
- Security And Compliance Reviews.

### Tags And Purpose Tags

Content:

- Purpose Tag Registry tags.
- Public-interest tag.
- Academic/research tag.
- Local community tag.
- AI/RAG tag.
- Low-sensitivity public work tag.
- Institution-private work tag.
- Tag verification state.
- Tag conflict warnings.
- Required grant or compliance markers.

Links and handoffs:

- Grants And Public-Interest Projects.
- Central AI Stewardship.

### Deny Rules

Content:

- Denied organizations.
- Denied users.
- Denied tags.
- Denied workload classes.
- Denied jurisdictions where policy allows.
- Denied risk bands.
- Temporary emergency block rules.
- Deny rule priority.
- Expiry and review state.

### Expiry And Review

Content:

- Access rules expiring soon.
- Grants requiring review.
- One-time grants.
- Temporary grants.
- Emergency blocks.
- Reapproval requirements.
- Review owner.
- Reminder state.
- Bulk review action.

### Policy Dry Run

Content:

- Test requester selector.
- Test institution/org/user/private UUID input.
- Test workload class.
- Test purpose tag.
- Test schedule/resource interaction marker.
- Allowed, denied, or missing prerequisite result.
- Safe reason codes.
- Effective rule diff.
- Owner-service refs for dry-run output.

Links and handoffs:

- Resource Sharing Rules.
- Node Detail.
- Security And Compliance Reviews.

### Access Activity

Content:

- Saved policy versions.
- Actor refs.
- Added and removed grants.
- Private UUID rotations.
- Deny rule changes.
- Dry-run refs.
- Emergency blocks.
- Replay refs.

Links and handoffs:

- Activity And Receipts Timeline.
- Disputes And Appeals.

## Primary Actions

- Save access rules.
- Run policy dry run.
- Add institution.
- Add organization.
- Add user.
- Add private UUID.
- Add purpose tag.
- Add deny rule.
- Revoke access.
- Add emergency block.

## Secondary Actions

- Select node group.
- Apply access template.
- Copy access rules from another node.
- Mask or reveal authorized metadata.
- Rotate private UUID.
- Review expiring grants.
- Export access summary.
- Ask AI to explain access outcome.

## States

- No joined node.
- Empty policy.
- Loading.
- Live.
- Draft with unsaved changes.
- Dry run running.
- Dry run failed.
- Save pending.
- Save accepted.
- Save denied.
- Private UUID masked.
- Grant expiring.
- Review required.
- Emergency block active.
- Partial owner-service outage.
- Permission denied.
- Offline cached view.
- Error with retry.

## Permissions And Privacy Behavior

- Access Rules may store user-facing drafts and submit signed policy updates, but authoritative eligibility and policy state belongs to owner services.
- Overguard, Overtenant, Purpose Tag Registry, Federation Template Service, Oververify, and Fraud Control own policy and eligibility truth.
- Private UUIDs are sensitive grants. They must be masked by default, never logged raw, never reused across unrelated scopes, and never included in unredacted support bundles.
- Fraud, risk-band, and eligibility internals must be reduced to safe reason codes for provider-facing views.
- Offline mode may allow draft edits, but saving, dry-run authority, revocation, and emergency blocks must be revalidated online.

## Design Notes

- Use a two-column policy builder: allow rules and deny rules side by side, with dry-run results in a stable contextual panel.
- Use tokenized chips for institutions, orgs, tags, and masked private UUIDs; use clear expiry badges.
- Make revoke, narrow, expire, and review actions easy to find without making accidental revocation likely.
- Keep dry-run output short in the main panel and put detailed refs in the contextual drawer.
