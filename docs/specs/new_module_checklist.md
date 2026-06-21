# New Module Checklist

## Purpose

Define the required evidence before a new service, package, generated path, local profile, integration scenario root, or layout boundary is accepted into the Overrid workspace.

This checklist is a Repository Layout validation contract. It is not runtime configuration, service discovery, deployment orchestration, or a shortcut around owning SDS and service contracts.

## Required Lifecycle States

Every module addition moves through these states as applicable:

- `proposed`: the owning SDS, service catalog entry, build-plan phase, and layout rationale are documented.
- `scaffolded`: the path and workspace manifest record exist without runtime behavior being implied.
- `contracted`: service/package contract and SDS links exist.
- `wired`: root commands, local stack, integration harness, schema checks, docs checks, or package tests can discover the module.
- `validated`: relevant tests and validators have passed and the progress trail records evidence.
- `accepted`: SDS, service catalog, build plan, crosswalk, specs, module record, and validation evidence are aligned.
- `deprecated`: the old boundary has a replacement or removal note and remains traceable until consumers migrate.
- `removed`: stale docs, manifest records, tests, local stack wiring, harness scenarios, generated outputs, and Docdex references have been updated.

## Addition Checklist

Before implementation starts:

- Name the owning SDS, service catalog entry, sub-build plan, master phase, and `service_catalog_alignment` row.
- Reuse an existing top-level directory unless an SDS-backed layout change explicitly justifies a new one.
- Add or update the service/module contract under `docs/specs` or a documented equivalent contract path.
- Add canonical schema refs under `packages/schemas` or record a no-public-contract reason.
- Add `overrid.workspace.toml` module metadata: name, type, owner layer, path, master phase, public contract path, allowed dependency groups, generated-output paths, test targets, local-stack participation, documentation links, and lifecycle state.
- Add local-stack participation only when the module owns a loopback local profile, service definition, fixture source, or helper crate.
- Add integration harness participation only when the module owns scenarios, fixtures, artifact expectations, or cross-service validation.
- Add generated-output and Docdex ignore rules before generated files or local state can appear.

## Deprecation And Removal Checklist

Before a module is deprecated or removed:

- Record the replacement path or explicit removal reason.
- Update SDS, service catalog, build plan, crosswalk, specs, package docs, module records, tests, local stack records, harness scenarios, generated-output rules, and Docdex-indexed references.
- Keep deprecated modules discoverable until consumers and validation targets have migrated.
- Reject removal when stale references, missing replacement notes, or missing validation evidence remain.

## Cross-Document Maintenance

Accepted layout changes must keep these documents aligned:

- `docs/sds/foundation/repository_layout.md`
- `docs/service_catalog/foundation/repository_layout.md`
- `docs/build_plan/sub_build_plan_005_repository_layout.md`
- `docs/build_plan/master_plan.md`
- `docs/build_plan/service_catalog_alignment.md`
- `docs/specs/service_contract_template.md`
- `docs/specs/new_module_checklist.md`
- `overrid.workspace.toml`
- The matching `docs/planning/*_plan.md` and `docs/planning/*_progress.md` files

## Validation

`layout:check`, docs checks, and phase validators reject invalid lifecycle states, accepted modules without wired validation evidence, deleted modules with stale references, missing service contracts, missing test targets, and checklist-driven additions that invent new top-level folders without SDS-backed evidence.
