# Shared Schema Package Phase 9 Schema Reference

Generated artifact status: schema-derived reference projection.

Source of truth: `packages/schemas/overrid_contracts/v0/shared_schema_package.schema.json`

Source manifest: `packages/schemas/overrid_contracts/codegen_manifest.json`

Owning SDS: `docs/sds/foundation/shared_schema_package.md`

Build-plan phase gate: `docs/build_plan/sub_build_plan_007_shared_schema_package.md#phase-9-package-release-ci-enforcement-and-consumer-registry`

Tech-stack authority: `docs/overrid_tech_stack_choice.md`

Authority: `non_authoritative_projection`

## Object Families

- `phase9_release_ci_contract`: package-release, CI, consumer-registry, coverage, generated-docs, and product-hardening root contract.
- `phase9_release_workflow_gate`: release blockers for schema lint, Rust projection, fixture validation, compatibility, redaction, generated docs, and consumer impact.
- `phase9_contract_consumption_lint`: consumer-surface requirements that block untyped payloads, private schema forks, and ad hoc string parsing.
- `phase9_schema_coverage_report`: coverage metadata by service family, object family, master phase, privacy class, fixture coverage, generated targets, and release status.
- `phase9_documentation_publishing_item`: generated documentation artifact metadata with source-schema, owning-SDS, build-plan, backlink, and non-authority requirements.
- `phase9_product_hardening_check`: Phase 6 product-integration guardrails for generated contract use, Overgate envelopes, stable errors, privacy classes, and redaction.
- `phase9_rust_projection`: Rust projection path and validator entrypoint metadata.

## Release Gates

- `schema_lint`
- `rust_generation_projection`
- `fixture_validation`
- `compatibility_report`
- `redaction_check`
- `generated_docs`
- `consumer_impact_report`

Each release gate is required before release and must block CI when missing or failing.

## Consumer Surfaces

- `service`
- `sdk`
- `cli`
- `worker`
- `node_agent`
- `ui`
- `adapter`
- `test`

Each consumer surface must use generated contracts for public objects. Untyped payloads, private schema forks, and ad hoc string parsing are blocked.

## Product Hardening Surfaces

- `docdex`
- `mcoda`
- `codali`
- `admin_developer_ui`
- `cli`
- `sdk`
- `adapters`
- `ai_gateway`
- `encrypted_docdex_rag`

These product-integration surfaces must not bypass generated contracts, Overgate command envelopes, stable error objects, privacy classification, or redaction.

## Source Inputs

- `packages/schemas/overrid_contracts/v0/shared_schema_package.schema.json`
- `packages/schemas/overrid_contracts/codegen_manifest.json`
- `docs/build_plan/sub_build_plan_007_shared_schema_package.md`
- `docs/overrid_tech_stack_choice.md`

Canonical JSON Schema remains the docs-facing and fixture-facing source of truth. Rust projection, TypeScript/web projections, generated documentation, and future Protobuf projections are consumers only.
