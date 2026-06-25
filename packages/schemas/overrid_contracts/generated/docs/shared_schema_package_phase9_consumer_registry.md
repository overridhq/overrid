# Shared Schema Package Phase 9 Consumer Registry

Generated artifact status: schema-derived consumer-registry projection.

Source of truth: `packages/schemas/overrid_contracts/v0/shared_schema_package.schema.json`

Source manifest: `packages/schemas/overrid_contracts/codegen_manifest.json`

Owning SDS: `docs/sds/foundation/shared_schema_package.md`

Build-plan phase gate: `docs/build_plan/sub_build_plan_007_shared_schema_package.md#phase-9-package-release-ci-enforcement-and-consumer-registry`

Tech-stack authority: `docs/overrid_tech_stack_choice.md`

Authority: `non_authoritative_projection`

## Contract Consumer Surfaces

| Consumer surface | Generated contract required | Untyped payloads | Private schema forks | Ad hoc string parsing |
| --- | --- | --- | --- | --- |
| service | yes | blocked | blocked | blocked |
| sdk | yes | blocked | blocked | blocked |
| cli | yes | blocked | blocked | blocked |
| worker | yes | blocked | blocked | blocked |
| node_agent | yes | blocked | blocked | blocked |
| ui | yes | blocked | blocked | blocked |
| adapter | yes | blocked | blocked | blocked |
| test | yes | blocked | blocked | blocked |

## Product Hardening Surfaces

| Product surface | Generated contracts | Overgate envelope | Stable errors | Privacy classification | Redaction | Bypass |
| --- | --- | --- | --- | --- | --- | --- |
| docdex | required | required | required | required | required | blocked |
| mcoda | required | required | required | required | required | blocked |
| codali | required | required | required | required | required | blocked |
| admin_developer_ui | required | required | required | required | required | blocked |
| cli | required | required | required | required | required | blocked |
| sdk | required | required | required | required | required | blocked |
| adapters | required | required | required | required | required | blocked |
| ai_gateway | required | required | required | required | required | blocked |
| encrypted_docdex_rag | required | required | required | required | required | blocked |

## Handoff Rule

Consumers may request new shared schemas through the package, but runtime authority stays with owning services. The shared schema package must not become a runtime registry, policy engine, audit authority, accounting authority, storage authority, or secret-custody boundary.

## Review Checklist

- Use canonical JSON Schema and generated Rust-first validation outputs.
- Keep TypeScript/web outputs generated second.
- Keep Protobuf internal-only when later compact service/RPC/event contracts justify it.
- Preserve stable reason codes, trace ids, tenant ids, actor refs, privacy classes, redaction rules, and fixture evidence for public boundaries.
