# Help

## Slug

`help`

## Title

Help

## Navigation Group

System And Help

## Description

Help is the Overdesk page for searchable product guidance, contextual help, workflow checklists, keyboard shortcuts, accessibility support, troubleshooting routes, support contact paths, and safe handoffs into diagnostics, incidents, releases, settings, wallet, AI, apps, nodes, and governance pages. It is a practical assistance surface, not an authority layer: owner services, support systems, diagnostics, incident response, release systems, identity, wallet, vault, and governance services keep their own records and decisions.

## Primary Users

- Regular users
- New Overdesk users
- Developers
- App owners
- Provider operators
- Organization admins
- Institution admins
- Stewards
- Support operators

## Primary User Goals

- Find the right page, action, or support path quickly.
- Search help topics without leaving the current workflow.
- Learn common tasks through short checklists and contextual entry points.
- See keyboard shortcuts and accessibility support.
- Diagnose common problems and route to the right troubleshooting page.
- Contact support with a redacted diagnostics path when needed.
- Understand whether a question belongs to settings, wallet, apps, AI, resources, governance, incidents, or local device support.
- Avoid exposing sensitive data while asking for help.

## Entry Points

- System And Help navigation.
- Address bar command: `/help`.
- Global Search result.
- Empty states and error panels across Overdesk.
- Diagnostics And Support Bundles.
- Settings And Security.
- Updates And Release Notes.
- App Incidents And Support.
- Contextual help buttons on individual pages.
- Keyboard shortcut overlay.

## Sections To Have

### Page Header

Content:

- Page title.
- Search field.
- Active scope marker.
- Current page/context marker when opened contextually.
- Recent help topics.
- Open support case count.
- Diagnostics readiness marker.
- Primary action: Search Help.
- Secondary actions: Shortcuts, Troubleshooting, Contact Support, Diagnostics.

Links and handoffs:

- Global Search.
- Diagnostics And Support Bundles.
- App Incidents And Support.
- Settings And Security.

### Help Search

Content:

- Search input.
- Topic results.
- Page results.
- Action results.
- Error-code results.
- Release-note results.
- Support-case results.
- Relevance markers.
- Safe query guidance.
- No-result handoff.

Links and handoffs:

- Global Search.
- Updates And Release Notes.
- Diagnostics And Support Bundles.

### Start Here

Content:

- First-run checklist.
- Account and profile checklist.
- Wallet basics checklist.
- Native apps checklist.
- Add this computer checklist.
- App deployment checklist.
- AI/RAG checklist.
- Privacy and security checklist.
- Governance/public trust checklist.

Links and handoffs:

- Identity And Profile Center.
- Wallet.
- Native App Catalog.
- Add This Computer To Overrid.
- Deploy New App.
- Personal AI Assistant.
- Privacy And Permissions Center.
- Governance Center.

### Common Tasks

Content:

- Buy credits.
- Send a message.
- Search the Overrid net.
- Open a native app.
- Deploy a new app.
- Add a computer to the network.
- Set resource sharing rules.
- Manage privacy grants.
- Create a diagnostics bundle.
- Review an incident notice.

Links and handoffs:

- Buy Credits.
- Messaging Center.
- Overrid Browser.
- Deploy New App.
- Resource Sharing Rules.
- Privacy And Permissions Center.
- Incident Reports.

### Contextual Help

Content:

- Current page name.
- Current selected object type.
- Relevant help topics.
- Relevant actions.
- Common errors for that context.
- Required permissions.
- Owner-service authority labels.
- Suggested next page.
- Ask AI about this context action.

Links and handoffs:

- Current page.
- Personal AI Assistant.
- Activity And Receipts Timeline.
- Diagnostics And Support Bundles.

### Troubleshooting Router

Content:

- Problem category selector.
- Device problem path.
- Login/session problem path.
- Wallet/credit problem path.
- Messaging problem path.
- AI/RAG problem path.
- App deployment problem path.
- Cache/offline problem path.
- Incident/security problem path.
- Update problem path.
- Support bundle recommendation.

Links and handoffs:

- Local Device Settings.
- Settings And Security.
- Wallet.
- Messaging Center.
- Personal AI Assistant.
- Developer Console.
- Local Cache And Offline Sync.
- Incident Reports.
- Updates And Release Notes.

### Error And Status Reference

Content:

- Error code search.
- Status code search.
- Plain-language meaning.
- Affected feature.
- User action.
- Admin/developer action.
- Support handoff.
- Related diagnostic category.
- Related owner service.

Links and handoffs:

- Diagnostics And Support Bundles.
- Developer Console.
- App Incidents And Support.
- Activity And Receipts Timeline.

### Keyboard Shortcuts

Content:

- Address bar shortcut.
- Search shortcut.
- Back/forward shortcut.
- Command palette shortcut.
- Page tabs shortcut.
- Accessibility navigation shortcut.
- Copy safe ref shortcut.
- Help shortcut.
- Customization link.

Links and handoffs:

- Settings And Security.
- Local Device Settings.

### Accessibility And Language Help

Content:

- Screen reader support.
- Focus behavior.
- Reduced motion support.
- High contrast support.
- Text size and density settings.
- Language and region settings.
- Keyboard-only operation.
- Accessibility diagnostics.
- Feedback/support path.

Links and handoffs:

- Settings And Security.
- Local Device Settings.
- Diagnostics And Support Bundles.

### Support Contact

Content:

- Support options by issue class.
- Self-service recommendation.
- Support case creation.
- Required safe refs.
- Optional diagnostics bundle.
- Redaction status.
- Expected response channel.
- Open support cases.
- Case history.

Links and handoffs:

- App Incidents And Support.
- Diagnostics And Support Bundles.
- Messaging Center.
- Activity And Receipts Timeline.

### Learning And Release Changes

Content:

- New feature summaries.
- Recent release changes.
- Changed workflow notes.
- Deprecated feature notices.
- Migration help.
- Security update notes.
- Governance/PIP references where relevant.
- Known issues.

Links and handoffs:

- Updates And Release Notes.
- Protocol Improvement Proposals.
- Governance Center.
- Security And Compliance Reviews.

## Primary Actions

- Search help.
- Open help topic.
- Open contextual help.
- Open troubleshooting router.
- View keyboard shortcuts.
- Start a common task.
- Create support case.
- Create redacted diagnostics bundle.
- Ask AI for help with current context.

## Secondary Actions

- Filter topics by role.
- Filter topics by feature area.
- Copy safe help link.
- Open related page.
- Open release note.
- Open incident notice.
- Open support history.
- Change language/accessibility settings.

## States

- Loading.
- Live.
- Searching.
- No results.
- Context available.
- Context unavailable.
- Troubleshooting selected.
- Diagnostics recommended.
- Support case ready.
- Support case created.
- Offline.
- Permission required.
- Action denied.

## Permissions And Privacy Behavior

- Help may route users to owner pages and support flows, but it must not mutate identity, wallet, vault, governance, incident, release, app, node, cache, or diagnostics state directly.
- Contextual help must use safe refs, page names, object classes, and redacted status summaries rather than private payloads.
- Support contact flows must recommend or create redacted diagnostics bundles and must never attach raw logs, secrets, key material, vault contents, payment details, private RAG/source content, private UUIDs, sensitive incident evidence, or broad host details.
- Help search should not reveal hidden private records through result counts, topic names, or unavailable placeholders.
- AI help must be clearly scoped to the user's allowed context and should route sensitive troubleshooting to Diagnostics And Support Bundles or the relevant owner page.
- Accessibility and language settings are managed through Settings And Security; Help only presents and routes.

## Design Notes

- Use a compact support-center layout: search at the top, common tasks below, and contextual panels only when they are relevant.
- Keep text practical and short; avoid marketing-style explanations.
- Use page/action links rather than long instructions when a workflow is already represented elsewhere.
- Put troubleshooting results into clear next-step cards with one primary route each.
- Use icons for shortcuts, diagnostics, support, release notes, incidents, settings, wallet, AI, and apps.
- Make contextual help feel like a router to the right page, not a separate manual users must read before acting.
