# Overrid Contract Schemas

This package contains the Phase 2 through Phase 5 CLI contract source plus
Phase 6 hardening contracts, Phase 7 execution contracts, Phase 8
policy/package/accounting read contracts, Phase 9 product integration and CI
automation contracts, Phase 10 release-readiness/security/handoff validation
contracts, Integration Test Harness Phase 2 schema contracts, and the Rust
projection for `SUB BUILD PLAN #2 - CLI`, `SUB BUILD PLAN #3 - Integration
Test Harness`, and `SUB BUILD PLAN #4 - Local Development Stack` contract
checks.

The JSON Schema files under `v0/` are the canonical docs-facing source for this
slice. The Rust crate in `src/` is the initial generated/projection layer that
CLI and SDK code consume while later phases mature schema generation.

Shared Schema Package Phase 2 contracts define `source_layout`,
`typed_ref_primitive`, `lifecycle_primitives`, `privacy_class_rule`,
`reason_code_entry`, and `correction_field`. These keep
`packages/schemas` source roots separate from generated outputs, fixture roots,
compatibility report roots, and internal binary projection roots, require common
object references to be typed and version-aware, require mutating/external
records to carry lifecycle primitives, classify privacy/data surfaces, and make
reason-code/correction metadata stable before downstream services consume the
package.

Shared Schema Package Phase 3 contracts define the first Phase 0 and Phase 1
contract modules: `identity`, `tenant`, `command`, `api_error`, `event`,
`audit`, `workload_manifest`, `resource_manifest`, `registry_metadata`,
`queue_and_lease`, and `credential_key_metadata`. These module records require
tenant/actor refs for mutating command paths, stable trace/idempotency fields,
reason-coded API errors with correction shapes, append-only privacy-classified
events and audit records, typed secret refs in manifests, ref-only queue and
lease state, and credential/key metadata without raw secrets or private key
material.

Shared Schema Package Phase 4 contracts define the generation toolchain and
projection metadata for `rust-json-schema-projection-v0`. The canonical JSON
Schema remains the source of truth; Rust projection checks run first through
`SharedSchemaPhase4GenerationContract::canonical().validate()`, generated docs
under `generated/docs/` carry source-to-doc trace metadata, TypeScript/web
models remain blocked until Rust projection, golden fixture, docs trace, and
compatibility gates are stable, and Protobuf is internal-only for compact
service/RPC/event contracts with canonical JSON Schema source required for
every public object.

Shared Schema Package Phase 5 contracts define strict validator, parse helper,
common envelope assertion, reason-code registry, and redaction diagnostic
metadata. Sensitive command, identity, tenant, credential, signature, error,
audit, policy, usage, ORU, Seal Ledger, Overasset, dispute, and namespace
ownership families reject unknown fields by default; extension maps are allowed
only for explicitly low-risk metadata with namespace prefixes, typed values,
privacy class, and compatibility class. Rust projection checks run through
`SharedSchemaPhase5ValidationContract::canonical().validate()`, the generated
reason-code registry is exposed through the Rust
`list_reason_codes(domain)` helper, and diagnostics must fail closed before
sentinel private material can appear in generated docs, compatibility reports,
fixture reports, logs, or validation output.

Shared Schema Package Phase 6 contracts define deterministic valid and invalid
fixture-builder metadata, golden command/event/audit/usage/ledger/API error
envelope fixtures, SDS #3 integration-harness fixture reuse, SDS #4 local-stack
reset bundles, and validation artifact metadata. Rust projection checks run
through `SharedSchemaPhase6FixtureContract::canonical().validate()`. Fixture
builders must use stable seeds from a clean checkout, negative fixtures must
return stable reason codes without ambiguous parser errors, local-stack and
harness bundles must stay test-only and secret-free, and schema lint/generated
diff/fixture count/redaction/compatibility artifacts remain non-authoritative CI
evidence rather than Overwatch runtime events.

Shared Schema Package Phase 7 contracts define schema comparison,
deprecation metadata, current-plus-previous stable major support,
authority-sensitive migration gates, and consumer impact report metadata. Rust
projection checks run through
`SharedSchemaPhase7CompatibilityContract::canonical().validate()`. Breaking or
migration-required changes must carry migration metadata and compatibility
reports, deprecated fields must name replacements and active consumers, stable
major support must fail with `schema.schema_version_unsupported` instead of
silent downgrade behavior, and authority-sensitive modules must block release
without migration plans, owner signoff, rollback guidance, and consumer impact
reports.

Shared Schema Package Phase 8 contracts define downstream domain expansion
metadata for execution/scheduling, trust/policy/verification,
accounting/rights/settlement, data/storage/namespace/secret refs, and
AI/Docdex/mobile/native-app/ADES-facing schemas. Rust projection checks run
through `SharedSchemaPhase8DomainExpansionContract::canonical().validate()`.
Domain modules must name owning master phases and service families, consume
generated contracts instead of private duplicate public types, keep runtime
authority with owning services, reject raw secrets and untyped refs, forbid
conventional object-store and pricing/revenue/blockchain/NFT assumptions, and
keep TypeScript/web projections generated second from canonical schemas with
browser-safe redaction.

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

Local Development Stack Phase 2 contracts define `stack_profile`,
`service_definition`, `port_registry`, `local_env_manifest`,
`volume_registry`, `reset_plan`, `seed_manifest`, `health_snapshot`,
`local_secret_record`, and `local_diagnostic_event`. These records keep SDS #4
local stack contracts loopback-only, deterministic, local/test-scoped, and
phase-gated before Phase 3 adds Rust lifecycle code. They reject wildcard or
non-loopback endpoints, ungated future services, missing service metadata,
duplicate or out-of-range local ports, unmarked reset targets, non-test seed
fixtures, invalid health states, and diagnostics that expose raw secrets.

Integration Test Harness Phase 2 contracts define `fixture_manifest`,
`fixture_identity`, `fixture_key`, resource card refs, workload/package/local
ledger/policy refs, `scenario_manifest`, `scenario_step`, `test_run_record`,
`assertion_result`, `golden_trace`, and `artifact_bundle`. These records are
canonical JSON Schema sources for SDS #3 harness compatibility and keep
fixtures test-only, scenarios phase-gated, run records terminal and
reason-coded, golden traces exact or DAG-safe, and artifacts redacted with
secret-free reproduction commands. Artifact bundles must name redacted log,
Overwatch export, CLI output, API payload envelope, stack health, and fixture
version refs, and must reject raw secrets, private keys, tokens, signatures,
encrypted RAG content, private payloads, and fixture key material.
