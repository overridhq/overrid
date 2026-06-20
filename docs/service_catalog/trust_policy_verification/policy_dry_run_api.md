# Policy Dry-Run API Implementation Plan

## Objective

Let developers, native apps, and operators preview policy outcomes before submitting real work.

## First Build Phase

[Phase 4: Trust, Policy, and Verification](../../build_plan/phase_04_trust_policy_verification.md).

## Dependencies

- Overguard policy engine.
- Overregistry manifests.
- Overtenant quota facts.
- Overwatch retention.

## Development Order

1. Define dry-run request and response schema.
2. Return allow/deny, matched rules, reason codes, expected placement class, estimated reservation, and missing prerequisites.
3. Persist stable dry-run ids for audit and debugging.
4. Add SDK, CLI, and admin UI support.

## Contracts And Interfaces

- Dry-run API.
- Dry-run event record.
- Reason code and matched-rule format.
- SDK/CLI response object.

## Detailed SDS

The detailed design contract lives in [Policy Dry-Run API SDS](../../sds/trust_policy_verification/policy_dry_run_api.md).

- [SUB BUILD PLAN #35 - Policy Dry-Run API](../../build_plan/sub_build_plan_035_policy_dry_run_api.md)

## Design Alignment

- Treat Policy Dry-Run API as a side-effect-free Overguard preview surface, not a separate policy engine or admission shortcut.
- Store dry-run requests, declared inputs, fact snapshots, results, matched-rule previews, missing prerequisites, estimated reservation data, comparison records, and replay bundles.
- Return actionable reason codes, missing prerequisites, expected placement class, required trust class, secret prerequisites, and estimated reservation requirements for SDK, CLI, admin UI, native apps, and AI-generated plans.
- Prove dry runs never enqueue work, reserve resources, mount secrets, bill, settle, or mutate workload state.

## Validation

- Dry-run and real admission decisions match when inputs are unchanged.
- Missing prerequisite messages are actionable.
- Dry-runs never mutate workload or billing state.

## Handoff

Policy dry-run is required for developer ergonomics, AI-generated deployments, and native app permission previews.
