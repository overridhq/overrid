# Admin UI Shell

This package slice contains the Phase 4 TypeScript operator shell artifacts, Phase 5 read-only operational summary panel artifacts, Phase 6 trace-linked workload timeline artifacts, Phase 7 policy, verification, dispute, and incident evidence artifacts, Phase 8 usage, ORU, Seal Ledger, receipt, grant, and rights artifacts, Phase 9 signed admin action and receipt artifacts, and Phase 10 product reliability, diagnostics, accessibility, contract validation, and handoff artifacts from `docs/build_plan/sub_build_plan_001_admin_developer_ui.md`.

The shell is a client-side surface only. It consumes generated TypeScript projections from `packages/schemas/admin_ui/generated/typescript`, sends admin reads through Overgate, and keeps local view presets scoped to actor and tenant. It must not become the source of truth for platform contracts or a privileged path into core Overrid services.

Phase 4 artifacts:

- `src/overgate_client.ts`: Overgate-only request wrapper, trace propagation, stable error decoding, and cursor pagination helpers.
- `src/session_context.ts`: Session context loading states, capability checks, stale-state handling, and environment selection helpers.
- `src/operator_shell.ts`: Dense operator navigation and table metadata for the required Phase 4 panels.
- `src/redaction_primitives.ts`: Redaction-aware field rendering primitives for tables and detail views.
- `src/view_presets.ts`: Actor/tenant-scoped local preset validation and reset helpers.

Phase 5 artifacts:

- `src/operational_summary_panels.ts`: Read-only panel/query helpers for tenant, identity, key, node, package, workload, and queue summaries, including dependency-scoped disablement and stale-age state.
- `fixtures/valid/operational_summary_panels.valid.json`: Tenant-scoped fixture coverage for every Phase 5 panel query, per-panel summary fields, dependency health, node states, product workload families, and bounded query behavior.
- `fixtures/invalid/operational_summary_*.invalid.json`: Negative fixtures for cross-tenant rows, key material exposure, and unbounded queries.

Phase 6 artifacts:

- `src/workload_timeline.ts`: Read-only workload timeline helpers for trace assembly, gap surfacing, policy and usage overlays, copy-safe diagnostic bundles, and Overgate-only follow-mode planning.
- `fixtures/valid/workload_timeline_phase6.valid.json`: Fixture coverage for every required timeline stage, terminal outcome, partial dependency gap case, overlay layer, diagnostic bundle field, and follow-mode option.
- `fixtures/invalid/workload_timeline_*.invalid.json`: Negative fixtures for direct Overwatch connections, incomplete gap diagnostics, and private payload exposure.

Phase 7 artifacts:

- `src/policy_evidence_views.ts`: Read-only evidence-view helpers for policy decisions, verification evidence, dispute/correction cases, disabled incident readiness, and stable evidence-link checks.
- `fixtures/valid/policy_evidence_phase7.valid.json`: Fixture coverage for policy denial cases, verification states, dispute holds/corrections, disabled break-glass readiness, and stable evidence links.
- `fixtures/invalid/policy_evidence_*.invalid.json`: Negative fixtures for private payload exposure, missing stable refs, and enabled break-glass or ledger-mutation behavior.

Phase 8 artifacts:

- `src/accounting_usage_views.ts`: Read-only accounting view helpers for usage rollups, ORU and Seal Ledger read models, receipt/invoice refs, Overgrant visibility, Overasset rights bindings, and accounting role access states.
- `fixtures/valid/accounting_usage_phase8.valid.json`: Fixture coverage for every required ORU dimension, usage grouping, ledger state, receipt/invoice evidence, grant and rights refs, and accounting role visibility.
- `fixtures/invalid/accounting_*.invalid.json`: Negative fixtures for private payload exposure, pricing/revenue assumptions, direct ledger mutation, direct storage access, and blockchain or NFT ownership framing.

Phase 9 artifacts:

- `src/admin_actions.ts`: Framework-neutral helpers for signed action drafting, local signing handoff, Overgate action submission envelopes, receipt panels, stale-state protection, and Phase 9 high-risk denylist gates.
- `fixtures/valid/admin_actions_phase9.valid.json`: Fixture coverage for bounded supported actions, idempotency keys, signing handoff, receipt outcomes, stale-state blocking, and high-risk denylist enforcement.
- `fixtures/invalid/admin_action_*.invalid.json`: Negative fixtures for missing reason/state/policy/audit data, unsigned or malformed signatures, high-risk route reachability, and tenant mismatch.

Phase 10 artifacts:

- `src/phase10_reliability.ts`: Framework-neutral helpers for product reliability coverage, security/redaction probes, accessibility evidence, Overgate contract validation, diagnostic bundles, and Phase 7/13 handoff readiness gates.
- `fixtures/valid/phase10_reliability.valid.json`: Fixture coverage for Docdex, Mcoda, Codali, SDK, and CLI reliability outcomes, safe security probes, accessibility checks, contract checks, validation commands, and disabled/readiness-only handoff surfaces.
- `fixtures/invalid/phase10_*.invalid.json`: Negative fixtures for leaked sensitive diagnostics, incomplete reliability coverage, and enabled high-risk handoff behavior.

Validation is handled by `scripts/validate_admin_ui_phase4.py`, `scripts/validate_admin_ui_phase5.py`, `scripts/validate_admin_ui_phase6.py`, `scripts/validate_admin_ui_phase7.py`, `scripts/validate_admin_ui_phase8.py`, `scripts/validate_admin_ui_phase9.py`, `scripts/validate_admin_ui_phase10.py`, and the suite wrapper at `scripts/validate_admin_ui.py`.
