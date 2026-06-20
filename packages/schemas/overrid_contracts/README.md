# Overrid Contract Schemas

This package contains the Phase 2 through Phase 5 CLI contract source plus
Phase 6 hardening contracts, Phase 7 execution contracts, Phase 8
policy/package/accounting read contracts, Phase 9 product integration and CI
automation contracts, Phase 10 release-readiness/security/handoff validation
contracts, Integration Test Harness Phase 2 schema contracts, and the Rust
projection for `SUB BUILD PLAN #2 - CLI` and shared harness contract checks.

The JSON Schema files under `v0/` are the canonical docs-facing source for this
slice. The Rust crate in `src/` is the initial generated/projection layer that
CLI and SDK code consume while later phases mature schema generation.

Generated or projected code must not become the contract authority. Changes to
CLI output envelopes, trace context, idempotency records, API errors,
diagnostic bundles, profile records, credential references, confirmation
policy, or signer handoff start in the canonical schema and manifest before
parser, SDK, or runtime behavior changes.

Phase 3 profile and credential contracts are intentionally secret-free. Profile
records carry environment class, endpoint fingerprint, tenant/actor ids,
credential namespace, allowed credential-reference classes, fixture allowance,
confirmation policy, and schema pins. Credential references point to approved
stores or signing agents and must not contain raw key material.

Phase 5 bootstrap contracts define `phase1_bootstrap_command`,
`signed_command_envelope`, `bootstrap_acceptance_record`,
`manifest_bootstrap_ref`, and `synthetic_workload_pending_state`. These keep
Phase 1 control-plane bootstrap commands on the SDK/Overgate path, require
signature refs instead of key material for mutations, and preserve synthetic
workloads as pending-only records without implying real execution.

Phase 6 hardening contracts define `canonical_idempotency_fingerprint`,
`retry_timeout_policy`, `error_decode_record`, and
`local_idempotency_cache_record`. These lock deterministic retry-safe command
keys, bounded SDK retry/timeout metadata, stable reason-code decoding, and
owner-only cache records that do not contain private payloads.

Phase 7 seed private swarm and execution contracts define `node_status_record`,
`workload_execution_state`, `execution_timeline`, `execution_log_bundle`,
`execution_result_ref`, `polling_plan`, and `execution_diagnostic_event`.
These keep node and workload execution CLI output on authorized refs with
profile-scoped credential checks, bounded streaming/polling, trace-linked
diagnostics, and no direct node, queue, runner, or object-store paths.

Phase 8 policy, package, usage, receipt, ledger, and dispute contracts define
`policy_dry_run_decision`, `package_validation_summary`, `usage_oru_rollup`,
`receipt_ledger_read`, and `dispute_read_model`. These keep policy/package
validation and accounting reads on SDK/Overgate contracts, expose only
authorized refs, forbid direct meter/ledger/dispute/package/policy access, and
exclude pricing, revenue, customer-count, market-volume, blockchain, and NFT
assumptions.

Phase 9 product and automation contracts define `product_workflow_recipe` and
`ci_automation_profile`. Product recipes cover Docdex, Mcoda, and Codali
workflow commands through CLI/SDK/Overgate only, including authorized workload,
log, artifact, usage, receipt, budget, model/resource, tool-boundary, and retry
refs without direct internal APIs, raw HTTP, private storage paths, hardcoded
model/node/provider choices, or paid-service assumptions. CI automation
profiles require explicit `profile_kind=ci`, short-lived or mounted credential
refs, stable secret-free JSON, non-interactive confirmation behavior, and no
ambient persistent keychain defaults.

Phase 10 validation contracts define `cli_security_review_report`,
`cli_phase_availability_matrix`, and `cli_release_readiness_report`. They lock
release-readiness evidence for contract snapshots, help/output compatibility,
exit-code classes, reason-code families, secret-free security review,
available/read-only/denied command availability, integration reliability,
automation compatibility, and Phase 7/13 handoff gates. These records must keep
SDK/Overgate-only routing and must not authorize high-risk operations before
owning contracts exist.

Integration Test Harness Phase 2 contracts define `fixture_manifest`,
`fixture_identity`, `fixture_key`, resource card refs, workload/package/local
ledger/policy refs, `scenario_manifest`, `scenario_step`, `test_run_record`,
`assertion_result`, `golden_trace`, and `artifact_bundle`. These records are
canonical JSON Schema sources for SDS #3 harness compatibility and keep
fixtures test-only, scenarios phase-gated, run records terminal and
reason-coded, golden traces exact or DAG-safe, and artifacts redacted with
secret-free reproduction commands.
