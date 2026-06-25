# Shared Schema Package Phase 9 Migration Notes

Generated artifact status: schema-derived migration projection.

Source of truth: `packages/schemas/overrid_contracts/v0/shared_schema_package.schema.json`

Source manifest: `packages/schemas/overrid_contracts/codegen_manifest.json`

Owning SDS: `docs/sds/foundation/shared_schema_package.md`

Build-plan phase gate: `docs/build_plan/sub_build_plan_007_shared_schema_package.md#phase-9-package-release-ci-enforcement-and-consumer-registry`

Tech-stack authority: `docs/overrid_tech_stack_choice.md`

Authority: `non_authoritative_projection`

## Current Migration State

Phase 9 introduces release workflow, CI enforcement, consumer registry, schema coverage, generated documentation publishing, and Phase 6 product-hardening metadata. It does not migrate runtime authority into the schema package and does not create a deployed registry service.

## Required Migration Rules

- Public contract consumers must migrate away from untyped payloads, private schema forks, and ad hoc string parsing before release.
- Public boundary coverage must name service family, object family, master phase, privacy class, validation status, fixture coverage, generated targets, and release status.
- Product integrations for Docdex, Mcoda, Codali, admin/developer UI, CLI, SDK, adapters, AI gateway, and encrypted Docdex RAG must stay on generated contracts and Overgate envelope paths.
- Generated docs remain non-authoritative and must link back to canonical schema, source manifest, owning SDS, build-plan phase gate, and tech-stack authority.

## Authority-Sensitive Changes

Any future Phase 9 change that affects release gates, consumer requirements, coverage status, or product hardening must carry a compatibility report, rollback guidance, and consumer-impact notes before release. The schema package records contract shape and validation metadata only; runtime policy, audit, accounting, registry, storage, and secret authority remain with owning Overrid services.

## Non-Migration Notes

No migration to TypeScript/web authority, Protobuf-only public contracts, PostgreSQL, Redis, S3, Vault, blockchain, NFT, pricing, revenue, or conventional SaaS product-boundary assumptions is allowed by this phase.
