# Admin UI Schemas

This package slice contains the Phase 2 Admin and Developer UI contract artifacts, Phase 3 Overgate-admin read API contract artifacts, Phase 4 TypeScript operator shell artifacts, Phase 5 read-only operational summary panel artifacts, Phase 6 trace-linked workload timeline artifacts, Phase 7 policy, verification, dispute, and incident evidence artifacts, Phase 8 usage, ORU, Seal Ledger, receipt, grant, and rights artifacts, Phase 9 signed admin action and receipt artifacts, and Phase 10 product reliability, diagnostics, accessibility, contract validation, and handoff artifacts from `docs/build_plan/sub_build_plan_001_admin_developer_ui.md`.

The JSON Schema files under `v0/` are the canonical source for these passes. Generated TypeScript declarations under `generated/typescript/` are client projections only and must not become the source of truth.

The valid fixture set includes a resource summary variant matrix so every Phase 2 `resource_summary_view` kind is exercised without turning the TypeScript projection into the authority.

The Phase 3 `admin_read_api_contracts.schema.json` fixture set covers Overgate-admin route specs, list responses, workload timeline responses, authorization matrices, capability discovery, stable errors, and invalid direct-storage fallback cases.

Phase 3 read-route authorization is intentionally server-side and fail-closed: route contracts name the stable `admin_error_response` shape and require tenant, actor, role, data-class, policy-scope, redaction, and Overwatch audit-ref coverage before the TypeScript UI projection can consume them.

Validation is handled by `scripts/validate_admin_ui_phase2.py`, `scripts/validate_admin_ui_phase3.py`, and the suite wrapper at `scripts/validate_admin_ui.py`.

The Phase 4 shell lives in `packages/admin_ui_shell`. It is a TypeScript client-surface package for Overgate-only request construction, session context state, dense operator panel metadata, redaction primitives, and actor/tenant-scoped local preset validation; `scripts/validate_admin_ui_phase4.py` keeps those artifacts tied to the generated projections without making TypeScript the contract authority.

The Phase 5 read-only operational summary panel artifacts also live in `packages/admin_ui_shell`. They add operational summary fixtures and framework-neutral helpers for tenant, identity, key, node, package, workload, queue, dependency health, bounded query behavior, and stale-age state while keeping Overgate-admin responses and canonical schemas as the authority; `scripts/validate_admin_ui_phase5.py` validates those artifacts.

The Phase 6 trace-linked workload timeline artifacts also live in `packages/admin_ui_shell`. They add workload timeline fixtures and framework-neutral helpers for timeline stage assembly, partial dependency gaps, policy and usage overlays, copy-safe diagnostic bundles, and Overgate-only follow-mode behavior while keeping Overgate-admin timeline responses and canonical schemas as the authority; `scripts/validate_admin_ui_phase6.py` validates those artifacts.

The Phase 7 policy, verification, dispute, and incident evidence artifacts also live in `packages/admin_ui_shell`. They add policy evidence fixtures and framework-neutral helpers for policy denial cases, verification states, dispute holds and corrections, disabled incident readiness, and stable evidence-link validation while keeping Overgate-admin read models and canonical schemas as the authority; `scripts/validate_admin_ui_phase7.py` validates those artifacts.

The Phase 8 usage, ORU, Seal Ledger, receipt, grant, and rights artifacts also live in `packages/admin_ui_shell`. They add accounting usage fixtures and framework-neutral helpers for usage dimensions and groupings, immutable ledger refs, receipt and invoice refs, Overgrant purpose-scoped allocation visibility, Overasset storage/namespace/route entitlement bindings, and accounting role access states while keeping Overgate-admin read models and canonical schemas as the authority; `scripts/validate_admin_ui_phase8.py` validates those artifacts.

The Phase 9 signed admin action and receipt artifacts also live in `packages/admin_ui_shell`. They add signed action fixtures and framework-neutral helpers for bounded action drafts, local signing handoff, Overgate submission envelopes, action receipt rendering, stale-state protection, and high-risk action denylist gates while keeping canonical schemas and generated contract projections subordinate to Rust-owned service contracts; `scripts/validate_admin_ui_phase9.py` validates those artifacts.

The Phase 10 product reliability, diagnostics, accessibility, contract validation, and handoff artifacts also live in `packages/admin_ui_shell`. They add `phase10_reliability.valid.json` and framework-neutral helpers for Docdex, Mcoda, Codali, SDK, and CLI outcome coverage, security/redaction probes, dense-table accessibility evidence, Overgate contract checks, copy-safe diagnostic events, and disabled/readiness-only Phase 7/13 handoff surfaces while preserving the Rust-first service boundary; `scripts/validate_admin_ui_phase10.py` validates those artifacts.
