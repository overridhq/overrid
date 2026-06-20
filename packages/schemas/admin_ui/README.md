# Admin UI Schemas

This package slice contains the Phase 2 Admin and Developer UI contract artifacts and Phase 3 Overgate-admin read API contract artifacts from `docs/build_plan/sub_build_plan_001_admin_developer_ui.md`.

The JSON Schema files under `v0/` are the canonical source for these passes. Generated TypeScript declarations under `generated/typescript/` are client projections only and must not become the source of truth.

The valid fixture set includes a resource summary variant matrix so every Phase 2 `resource_summary_view` kind is exercised without turning the TypeScript projection into the authority.

The Phase 3 `admin_read_api_contracts.schema.json` fixture set covers Overgate-admin route specs, list responses, workload timeline responses, authorization matrices, capability discovery, stable errors, and invalid direct-storage fallback cases.

Phase 3 read-route authorization is intentionally server-side and fail-closed: route contracts name the stable `admin_error_response` shape and require tenant, actor, role, data-class, policy-scope, redaction, and Overwatch audit-ref coverage before the TypeScript UI projection can consume them.

Validation is handled by `scripts/validate_admin_ui_phase2.py`, `scripts/validate_admin_ui_phase3.py`, and the suite wrapper at `scripts/validate_admin_ui.py`.
