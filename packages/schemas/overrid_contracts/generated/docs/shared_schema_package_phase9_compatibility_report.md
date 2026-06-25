# Shared Schema Package Phase 9 Compatibility Report

Generated artifact status: schema-derived compatibility projection.

Source of truth: `packages/schemas/overrid_contracts/v0/shared_schema_package.schema.json`

Source manifest: `packages/schemas/overrid_contracts/codegen_manifest.json`

Owning SDS: `docs/sds/foundation/shared_schema_package.md`

Build-plan phase gate: `docs/build_plan/sub_build_plan_007_shared_schema_package.md#phase-9-package-release-ci-enforcement-and-consumer-registry`

Tech-stack authority: `docs/overrid_tech_stack_choice.md`

Authority: `non_authoritative_projection`

## Coverage Matrix

| Service family | Object family | Master phase | Privacy class | Validation status | Generated targets | Release status |
| --- | --- | --- | --- | --- | --- | --- |
| Overgate | command_envelope | master_phase:1 | tenant_private | fixtures_validated | rust, generated_docs | validated |
| Overwatch | audit_event | master_phase:1 | redacted_diagnostic | fixtures_validated | rust, generated_docs | validated |
| Overpack | package_manifest | master_phase:3 | tenant_private | compatibility_checked | rust, generated_docs | validated |
| Seal Ledger | ledger_ref | master_phase:5 | regulated | compatibility_checked | rust, generated_docs | validated |
| Admin/developer UI | view_model | master_phase:6 | redacted_diagnostic | schema_linted | rust, typescript_web, generated_docs | validated |

## Compatibility Gates

- Draft contract consumption is not allowed at public boundaries.
- Release requires schema lint, Rust projection, fixtures, compatibility, redaction, generated docs, and consumer-impact evidence.
- Generated targets are consumers of canonical JSON Schema and must not become independent authority.
- TypeScript/web projection remains second to canonical JSON Schema and Rust-first validation.
- Protobuf remains limited to future compact internal service/RPC/event contracts.

## Consumer Impact

The consumer registry covers service, SDK, CLI, worker, node-agent, UI, adapter, and test surfaces. Any consumer that needs a public object must consume generated contracts and stable reason-code/error objects from canonical schema sources.

## Residual Risk

This report is metadata-backed until downstream owning services implement their full public APIs. Later service phases must replace placeholder coverage counts with service-owned fixture evidence while preserving the same source-of-truth and authority boundaries.
