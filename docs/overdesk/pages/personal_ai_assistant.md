# Personal AI Assistant

## Slug

`personal-ai-assistant`

## Title

Personal AI Assistant

## Navigation Group

Daily Apps

## Description

Personal AI Assistant is the user's AI workbench inside Overdesk. It uses the central AI mechanism, AI Gateway Router, model-routing policy, optional encrypted Docdex/RAG indexes, ADES enrichment, and owner-service permissions. The page must make AI useful without hiding model routing, context use, tool calls, usage costs, or permission boundaries.

## Primary Users

- Regular users
- Builders
- App owners
- Institution users
- Researchers
- Stewards with review tasks

## Primary User Goals

- Ask questions and complete tasks across Overrid apps.
- Choose or approve context sources.
- Use private workspace, messages, wallet, app, node, asset, or Docdex/RAG context safely.
- See model route, usage estimate, and receipts.
- Approve tool calls before side effects.
- Inspect AI replay for important answers.

## Entry Points

- Daily Apps navigation.
- Home Dashboard Ask AI action.
- Contextual Ask AI actions from Workspace, Wallet, Messaging, Developer Console, Owned Apps, Assets, and Governance.
- Command palette: `ask`.
- Address bar command: `/ai`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active account and scope.
- Current mode.
- Model route visibility.
- Usage estimate status.
- Primary action: New Chat.
- Secondary actions: Context Sources, Tool Permissions, AI History.

### Conversation Workspace

Content:

- User prompts.
- Assistant responses.
- Streaming/running state.
- Citations/source chips.
- Tool-call chips.
- Error/partial answer states.
- Message actions: copy, retry, branch, save, export where allowed.

Links and handoffs:

- Activity And Receipts Timeline.
- Global Search.
- Workspace.
- Developer Console.

### Context Source Selector

Content:

- Current page context.
- Selected files/documents.
- Workspace docs.
- Messages.
- Wallet and receipts.
- Owned app analytics.
- Node health.
- Overasset records.
- Directory listings.
- Governance reports.
- Encrypted Docdex/RAG indexes.

Links and handoffs:

- Privacy And Permissions Center.
- Docdex And RAG Index Manager.
- Overvault Secure Storage Center.

### Model And Route Panel

Content:

- Classifier result.
- Selected model/resource.
- Fallback model/resource.
- Central AI or local route marker.
- Confidence and escalation reason.
- Expected usage dimensions.
- Current run state.
- Receipt refs after completion.

Links and handoffs:

- Wallet.
- Activity And Receipts Timeline.
- Governance Center where central AI stewardship is relevant.

### Tool Permission Panel

Content:

- Available tools.
- Requested tool calls.
- Side-effect risk.
- Affected app/service.
- Required account/scope.
- Confirmation controls.
- Deny/allow once/allow with expiry.

Links and handoffs:

- Privacy And Permissions Center.
- Settings And Security.

### Task Templates

Content:

- Summarize current page.
- Draft message.
- Analyze wallet usage.
- Explain receipt.
- Prepare deployment.
- Review app analytics.
- Summarize asset.
- Draft dispute.
- Search Overrid.
- Help with workspace document.

### RAG And Memory Panel

Content:

- Connected encrypted indexes.
- Index freshness.
- Retrieval permission.
- Redacted retrieval receipts.
- Memory/session policy.
- Context expiry.
- Denied source count.

Links and handoffs:

- Docdex And RAG Index Manager.
- Overvault Secure Storage Center.

### Replay And Receipts Panel

Content:

- Prompt refs.
- Context grant refs.
- Model route refs.
- Tool call refs.
- Usage refs.
- Redaction refs.
- Final response refs.
- Retry and correction links.

## Primary Actions

- New Chat.
- Ask.
- Add Context.
- Approve Tool Call.
- Retry.
- Save Response.

## Secondary Actions

- Change mode.
- Remove context source.
- Open citation.
- Export chat.
- Open usage receipt.
- Open RAG receipt.
- Report bad answer.

## States

- Empty new chat.
- Waiting for prompt.
- Classifying.
- Routing.
- Running.
- Waiting for tool approval.
- Partial answer.
- Context permission required.
- RAG source unavailable.
- Model resource unavailable.
- Offline limited.
- Usage precheck failed.

## Permissions And Privacy Behavior

- No private source may be used without explicit permission.
- Raw RAG context, raw private messages, raw vault secrets, raw payment data, exact location trails, and hidden fraud/security internals must not be shown in the page.
- Tool calls with side effects must require confirmation.
- AI memory must be visible, bounded, and revocable.
- Usage estimates and receipts must be clear before and after costly actions.

## Design Notes

- The conversation area should be primary, with context and route panels available but not noisy.
- The user should always understand what context the assistant can see.
- Model routing should be transparent without overwhelming casual users.
- Tool approvals should look materially different from normal chat replies.
