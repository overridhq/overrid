# Overdesk UI/UX Rules

## Purpose

These rules define the shared design system for every Overdesk page. The goal is a desktop product where regular users, providers, app owners, builders, institutions, and stewards can reach the right action quickly without learning separate tools.

## Navigation Rules

- Keep the top address bar always visible for names, routes, search, commands, and quick jumps.
- Keep the account/scope switcher always visible when the page can show personal, organization, institution, app-owner, or delegated data.
- Keep the primary navigation grouped by user intent: fast access, daily apps, wallet/ownership, network contribution, app operations, identity/privacy/data, governance, and system/help.
- Keep common pages within one click from the primary shell.
- Put specialized or risky actions one level deeper, behind clear context and confirmation.
- Every page must support deep links when the owning service allows it.
- Every page must show where the user is, which account/scope is active, and whether data is live, cached, stale, offline, or restricted.

## Page Layout Rules

- Use the same high-level structure on all pages: page header, primary content, contextual side panel, activity/status area, and footer/status strip where needed.
- Page header must include title, active scope, live/stale/offline state, primary action, secondary actions, and page-level search/filter where useful.
- Primary content should favor dense, scannable information over marketing-style panels.
- Contextual side panels should show details, permissions, receipts, replay refs, selected item metadata, or explainers without navigating away.
- Repeated items should use compact cards, tables, or lists with stable dimensions.
- Avoid nested cards and decorative sections; use clear bands, lists, tables, drawers, tabs, and split views.
- Use icons for common tools and actions where the meaning is familiar; pair with labels when risk, ambiguity, or accessibility requires it.

## Interaction Rules

- Common read actions should be one click.
- Common write actions should be one click plus confirmation only when they are risky, irreversible, costly, public, privacy-sensitive, or authority-changing.
- High-risk actions must show affected account, object, device/app/service, policy result, usage/cost estimate where available, audit/replay refs, and rollback/dispute path.
- Destructive actions must require explicit confirmation and must not rely only on color.
- Bulk actions must show selected count, affected scopes, preview result, and failure handling before submit.
- Long-running actions must show queued, running, partial, failed, and completed states with retry/cancel where owner services allow it.
- Empty states should offer the next useful action, not generic explanation.

## Information Architecture Rules

- Start each page with the user outcome, then the operational details.
- Keep authority boundaries visible where they matter: Overdesk displays and drafts; owner services decide final truth.
- Show source refs, service refs, policy refs, usage refs, audit refs, and replay refs in contextual details, not as clutter in the main scan path.
- Use consistent naming for the same concepts across pages: account, scope, app, node, namespace, receipt, grant, dispute, policy, route, replay.
- Do not expose internal fraud, security, or provider-sensitive facts outside the viewer's role.

## State Rules

- Every page must define loading, empty, live, stale, offline-limited, restricted, permission-denied, partial, error, and retry states.
- Cached data must be marked clearly and must never look authoritative when stale.
- Offline mode may allow safe drafts and cached reads, but authority-changing actions must be revalidated online.
- Permission-denied states should explain what is missing without leaking protected data.
- Partial states must identify which owner service is unavailable and what still works.

## Privacy And Safety Rules

- Do not use dark patterns, fake urgency, addictive loops, ad-trap ranking, hidden fees, hidden telemetry, or manipulative engagement prompts.
- Do not show raw secrets, raw private keys, payment secrets, vault secrets, raw RAG context, raw private messages, precise location trails, or fraud internals.
- Ask for explicit permission before using private context, exact location, AI/RAG context, vault grants, message data, workspace data, or sensitive app data.
- Make revoke, narrow, expire, and inspect-permission paths easy to find.
- Support bundles and diagnostics must be redacted by default and reviewed before export.

## Visual System Rules

- Use a quiet, utility-first visual style suitable for repeated daily work.
- Prioritize contrast, clear hierarchy, readable type, stable spacing, and fast scanning.
- Do not use oversized hero sections, decorative background effects, one-note palettes, or marketing-page composition inside the app.
- Use tables for comparison-heavy data, lists for event feeds, split views for browsing plus details, tabs for sibling views, segmented controls for modes, toggles for binary settings, sliders/inputs for numeric rules, and menus for option sets.
- Keep page titles short and literal.
- Keep button labels action-oriented and consistent.
- Text must fit inside containers at desktop and compact widths without overlap.

## Accessibility Rules

- Every interactive control must have keyboard access, visible focus, accessible label, and tooltip where meaning is not obvious.
- Do not convey risk or state by color alone.
- Support reduced motion, high contrast, readable density, and screen-reader landmarks.
- Lists and tables must preserve logical reading order.
- Confirmation dialogs must return focus to the triggering control after close.

## Page Brief Template

Every page detail document should include:

- Slug
- Title
- Navigation group
- Description
- Primary users
- Primary user goals
- Entry points
- Sections to have
- Content each section will have
- Links and handoffs inside the page
- Primary actions
- Secondary actions
- States
- Permissions and privacy behavior
- Design notes
