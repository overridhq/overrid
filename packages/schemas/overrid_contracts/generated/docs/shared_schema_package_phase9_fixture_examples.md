# Shared Schema Package Phase 9 Fixture Examples

Generated artifact status: schema-derived fixture projection.

Source of truth: `packages/schemas/overrid_contracts/v0/shared_schema_package.schema.json`

Source manifest: `packages/schemas/overrid_contracts/codegen_manifest.json`

Owning SDS: `docs/sds/foundation/shared_schema_package.md`

Build-plan phase gate: `docs/build_plan/sub_build_plan_007_shared_schema_package.md#phase-9-package-release-ci-enforcement-and-consumer-registry`

Tech-stack authority: `docs/overrid_tech_stack_choice.md`

Authority: `non_authoritative_projection`

## Valid Fixture

- `packages/schemas/overrid_contracts/fixtures/valid/shared_schema_package_phase9.valid.json`

The valid fixture covers release workflow gates, generated-contract consumption lints, schema coverage reports, documentation publishing items, product hardening checks, source hash inputs, and non-authoritative Rust projection metadata.

## Invalid Fixtures

- `packages/schemas/overrid_contracts/fixtures/invalid/shared_schema_package_phase9_release_gate_missing.invalid.json`
- `packages/schemas/overrid_contracts/fixtures/invalid/shared_schema_package_phase9_untyped_payload.invalid.json`
- `packages/schemas/overrid_contracts/fixtures/invalid/shared_schema_package_phase9_coverage_missing.invalid.json`
- `packages/schemas/overrid_contracts/fixtures/invalid/shared_schema_package_phase9_docs_backlink_missing.invalid.json`
- `packages/schemas/overrid_contracts/fixtures/invalid/shared_schema_package_phase9_product_bypass.invalid.json`
- `packages/schemas/overrid_contracts/fixtures/invalid/shared_schema_package_phase9_projection_authority.invalid.json`

These negative fixtures prove release gates, generated-contract consumption, coverage reports, generated-doc backlinks, product-hardening checks, and Rust projection authority fail closed.

## Validation Command

```bash
python3 scripts/validate_shared_schema_package_phase9.py
```

The focused validator also runs `cargo test -p overrid-contracts shared_schema_phase9`.

## Fixture Safety

Fixtures must remain deterministic, secret-free, and docs-facing. They must not contain raw private payloads, credentials, private key material, tenant-private user content, production endpoints, pricing assumptions, revenue assumptions, blockchain mechanics, or NFT mechanics.
