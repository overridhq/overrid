# Diagnostics And Support Bundles

## Slug

`diagnostics-support-bundles`

## Title

Diagnostics And Support Bundles

## Navigation Group

System And Help

## Description

Diagnostics And Support Bundles is the Overdesk page for health checks, redacted logs, device diagnostics, network checks, cache/offline reports, app/session diagnostics, support bundle assembly, redaction review, export receipts, and support handoffs. It helps users and support teams troubleshoot without leaking secrets, private user data, raw evidence, vault content, private UUIDs, or broad local host details.

## Primary Users

- Regular users
- Support operators
- Developers
- App owners
- Provider operators
- Organization admins
- Institution admins
- Security reviewers

## Primary User Goals

- Run focused diagnostics for device, network, local stack, cache, app, wallet, AI/RAG, and update issues.
- Build a support bundle with clear included/excluded data classes.
- Review redaction results before export.
- Copy safe refs and receipts for support.
- Open related incidents, app support cases, cache repair, device settings, and release notes.
- Avoid exposing secrets or private evidence while still giving support enough context.

## Entry Points

- System And Help navigation.
- Settings And Security.
- Local Device Settings.
- Local Cache And Offline Sync.
- App Incidents And Support.
- Developer Console.
- Incident Reports.
- Updates And Release Notes.
- Address bar command: `/diagnostics`.

## Sections To Have

### Page Header

Content:

- Page title.
- Device health state.
- Last diagnostic run.
- Open support bundle count.
- Redaction status.
- Support handoff state.
- Primary action: Run Diagnostics.
- Secondary actions: Create Bundle, Review Exports, Device Check, Cache Check, Network Check.

Links and handoffs:

- Local Device Settings.
- Local Cache And Offline Sync.
- App Incidents And Support.

### Diagnostic Categories

Content:

- Device diagnostics.
- Network diagnostics.
- Local stack diagnostics.
- Cache/offline diagnostics.
- App/session diagnostics.
- Wallet/credit diagnostics.
- AI/RAG diagnostics.
- Update diagnostics.
- Node/resource contribution diagnostics.
- Developer diagnostics.

Links and handoffs:

- Wallet.
- Personal AI Assistant.
- Docdex And RAG Index Manager.
- Node Detail.

### Diagnostic Run Detail

Content:

- Run id.
- Category.
- Started/finished time.
- Result.
- Failed checks.
- Warning checks.
- Safe summary.
- Related refs.
- Suggested next action.
- Retry action.

Links and handoffs:

- Activity And Receipts Timeline.
- Local Device Settings.
- Local Cache And Offline Sync.

### Support Bundle Builder

Content:

- Bundle purpose.
- Target support case.
- Included diagnostic categories.
- Included safe refs.
- Included log windows.
- Excluded data classes.
- Retention setting.
- Audience/scope.
- Bundle size estimate.
- Build action.

Links and handoffs:

- App Incidents And Support.
- Incident Reports.
- Settings And Security.

### Redaction Review

Content:

- Redaction profile.
- Secret scan status.
- Private data scan status.
- Sensitive evidence scan status.
- Private UUID/tag redaction.
- Key material exclusion.
- Payment/vault exclusion.
- User review checklist.
- Preview safe summary.
- Approve export action.

Links and handoffs:

- Security And Compliance Reviews.
- Overvault Secure Storage Center.
- Privacy And Permissions Center.

### Export And Handoff

Content:

- Bundle id.
- Destination.
- Support case ref.
- Export time.
- Export receipt.
- Expiry.
- Revocation/delete option where supported.
- Copy safe bundle ref.
- Open support conversation.

Links and handoffs:

- Messaging Center.
- App Incidents And Support.
- Activity And Receipts Timeline.

### Device And Local Stack Checks

Content:

- Hardware discovery result.
- Permission result.
- Platform compatibility.
- Local stack health.
- Endpoint reachability.
- Storage availability.
- Background permission state.
- Update compatibility.
- Repair suggestions.

Links and handoffs:

- Local Device Settings.
- Developer Console.
- Updates And Release Notes.

### Network Cache And Sync Checks

Content:

- Overgate reachability.
- Route resolution.
- Latency class.
- Offline state.
- Cache integrity.
- Offline queue state.
- Sync cursor state.
- Conflict count.
- Repair actions.

Links and handoffs:

- Overrid Browser.
- Local Cache And Offline Sync.
- Global Search.

### App Wallet AI And RAG Checks

Content:

- App session state.
- Wallet read-model state.
- Credit purchase precheck.
- AI route availability.
- RAG index availability.
- Model/gateway route refs.
- Permission denials.
- Owner-service health markers.
- Suggested handoffs.

Links and handoffs:

- Wallet.
- Personal AI Assistant.
- Docdex And RAG Index Manager.
- Native App Catalog.

### Support History

Content:

- Recent diagnostic runs.
- Recent bundles.
- Support case refs.
- Incident refs.
- Export receipts.
- Deletion/expiry state.
- Follow-up action refs.
- User notes.

Links and handoffs:

- Activity And Receipts Timeline.
- Incident Reports.
- App Incidents And Support.

## Primary Actions

- Run diagnostics.
- Build support bundle.
- Review redaction.
- Export approved bundle.
- Copy safe bundle ref.
- Open support case.
- Delete local bundle.
- Open repair action.

## Secondary Actions

- Filter diagnostic history.
- Run category-specific check.
- Compare diagnostic runs.
- Open related incident.
- Open related app/session.
- Ask AI to explain diagnostic results.
- Export public-safe metadata.

## States

- Loading.
- Live.
- Running diagnostics.
- Diagnostics passed.
- Diagnostics warnings.
- Diagnostics failed.
- Bundle building.
- Redaction pending.
- Redaction failed.
- Ready for review.
- Exported.
- Export revoked.
- Bundle expired.
- Support handoff pending.
- Action denied.

## Permissions And Privacy Behavior

- Support bundles must be redacted by default and must include explicit user review before export.
- Raw secrets, key material, recovery secrets, vault contents, payment details, private RAG/source content, sensitive incident evidence, child-safety material, private UUIDs, and broad host details must be excluded or redacted.
- Diagnostics should use safe refs, hashes, classed status, bounded log windows, and reason codes rather than raw private content.
- Advanced diagnostics require role/scope checks and may require fresh verification.
- Export receipts must be visible in Activity And Receipts Timeline.
- Overdesk can assemble and export user-approved bundles, but owner services remain authoritative for the underlying states.

## Design Notes

- Use a wizard-like support bundle flow: select, build, redact, review, export.
- Put redaction status in the header and before any export control.
- Use expandable diagnostic details with safe summaries collapsed by default.
- Keep repair actions near failed checks.
- Make "what is included" and "what is excluded" visually explicit.
- Avoid showing raw logs until they have passed redaction and user review.
