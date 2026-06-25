# Shared Schema Package Phase 9 Reason Codes

Generated artifact status: schema-derived reason-code projection.

Source of truth: `packages/schemas/overrid_contracts/v0/shared_schema_package.schema.json`

Source manifest: `packages/schemas/overrid_contracts/codegen_manifest.json`

Owning SDS: `docs/sds/foundation/shared_schema_package.md`

Build-plan phase gate: `docs/build_plan/sub_build_plan_007_shared_schema_package.md#phase-9-package-release-ci-enforcement-and-consumer-registry`

Tech-stack authority: `docs/overrid_tech_stack_choice.md`

Authority: `non_authoritative_projection`

## Release Workflow Reasons

- `schema.release_schema_lint_failed`: schema lint did not pass for release-bound contract sources.
- `schema.release_generation_stale`: Rust projection or generated metadata is stale against canonical JSON Schema.
- `schema.release_fixture_validation_failed`: valid or invalid fixture coverage failed.
- `schema.release_compatibility_unresolved`: compatibility report is missing or unresolved.
- `schema.release_redaction_failed`: redaction checks did not prove generated artifacts are safe.
- `schema.release_docs_missing`: generated documentation artifacts are missing or lack trace metadata.
- `schema.release_consumer_impact_missing`: consumer-impact report is missing before release.

## Validation Reasons

- `schema.release_gate_missing`: a required release gate is absent or not CI-blocking.
- `schema.generated_contract_consumption_missing`: a consumer surface allows untyped payloads, private forks, or ad hoc parsing.
- `schema.schema_coverage_missing`: schema coverage is missing for a required service/object family or uses draft contracts.
- `schema.generated_docs_backlink_missing`: generated docs are missing source, SDS, build-plan, backlink, or non-authority metadata.
- `schema.product_hardening_bypass`: a Phase 6 product surface can bypass generated contracts, Overgate envelopes, stable errors, privacy classification, or redaction.
- `schema.rust_projection_authority_drift`: Rust projection metadata no longer points at the non-authoritative validator path.

## Correction Guidance

All Phase 9 validation failures are release-blocking until the canonical JSON Schema, manifest metadata, fixtures, generated docs, Rust projection, and validator evidence agree. Corrections start in canonical JSON Schema and manifest metadata before projection or consumer behavior changes.
