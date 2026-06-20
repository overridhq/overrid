# CLI Implementation Plan

## Objective

Let developers and operators use Overrid from the terminal without manually calling internal APIs.

## First Build Phase

Basic commands in [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md); hardened CLI in [Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md).

## Detailed SDS

[CLI SDS](../../sds/foundation/cli.md).

## Detailed Build Plan

[SUB BUILD PLAN #2 - CLI](../../build_plan/sub_build_plan_002_cli.md).

## Dependencies

- SDK.
- Overgate.
- Overkey credential enrollment.
- Shared schemas.

## Development Order

1. Freeze Phase 1 command availability, profile and credential boundaries, output envelopes, exit classes, idempotency rules, and phase-gate validation before runtime parser work.
2. Build the Phase 2 Rust CLI crate skeleton with generated contract imports, SDK/Overgate wrapper, parser conventions, and fixtures.
3. Implement Phase 1 runtime command groups: version, doctor, profile, auth, credential, tenant, identity, key, manifest, synthetic workload submit/status/timeline, and local-only dev helpers.
4. Add signing, idempotency, retry, trace propagation, and error-decoding hardening for Phase 1 automation and Phase 6 product paths.
5. Add later command families only when owning services are ready: Phase 2 node commands; Phase 3 real workload logs, cancellation, result, and follow; Phase 4 policy dry-run; Phase 5/6 usage, receipt, and dispute reads; Phase 9 package and deployment-adjacent commands.
6. Harden product integration, stable JSON output, diagnostics, security/redaction, and release readiness.

## Contracts And Interfaces

- Human-readable command output.
- Machine-readable `--json` output.
- Config and credential storage rules.
- Exit-code conventions.
- Signed command envelope construction through the SDK.
- Local profile and credential-reference rules.
- Stable reason-code, retry-class, trace-id, and audit-ref output.

## Design Alignment

- The CLI is a terminal client, not a private service interface.
- Every platform call must go through the SDK and Overgate path.
- Local/test fixture credentials must be isolated from seed or production-like endpoints.

## Phase 1 Implementation Gates

- Boundary gate: CLI Phase 1 work freezes the CLI as a Rust SDK/Overgate client. It may define command scope, profile shape, credential references, output envelopes, exit classes, capability checks, and diagnostics, but it may not introduce direct storage, queue, ledger, vault, object-store, node-agent, or service-local state access.
- Command gate: Phase 1 exposes only `version`, `doctor`, profile, auth, credential, tenant, identity, key, manifest, synthetic workload pending-state, and local-only `dev` helpers. Later node, real execution, policy, usage, receipt, package, deployment, governance, incident, compliance, migration, and backbone commands remain hidden, documented planned, or `not_available_in_phase` until owning-service contracts exist.
- Decision gate: implementation must carry forward SDS decisions for platform-native credential storage, deterministic idempotency fingerprints, the small numeric exit-code registry, and profile environment separation before command payload construction.
- Documentation gate: new CLI command behavior must first update the owning SDS and service-catalog contract, then the master phase or crosswalk if phase alignment changes, then the CLI sub-build plan and command docs before parser behavior is implemented.
- Validation gate: `scripts/validate_cli_phase1.py` verifies the Phase 1 gate outputs, cross-document links, command availability states, and Rust-first tech-stack constraints before later CLI phases build runtime code.

## Phase 2 Implementation Gates

- Workspace gate: `packages/cli`, `packages/sdk`, and `packages/schemas/overrid_contracts` are Cargo workspace members and keep the CLI runtime Rust-first.
- Contract gate: the CLI consumes the Rust projection from `packages/schemas/overrid_contracts`, while `packages/schemas/overrid_contracts/v0/cli_command.schema.json` and `packages/schemas/overrid_contracts/codegen_manifest.json` remain the canonical source; schema-version compatibility checks reject unknown or incompatible versions before request construction.
- SDK gate: the CLI uses `packages/sdk` for Overgate-only endpoint validation and must reject private service targets instead of calling Overbase, Overstore, Overvault, Overqueue, Overwatch, Seal Ledger, node-agent, or service-local state directly.
- Parser gate: `version`, `help`, `--json`, `--output`, `--no-color`, `--verbose`, `--profile`, and `--all-phases` are stable Phase 2 parser conventions; phase-gated command families are visible only through all-phase help and fail with `not_available_in_phase`.
- Fixture gate: CLI fixtures preserve final Overrid output-envelope and endpoint-validation shapes rather than CLI-only mock payloads; `scripts/validate_cli_phase2.py` compares actual `overrid version --json` output with the valid fixture and Cargo tests validate the gate.

## Phase 3 Implementation Gates

- Profile gate: `packages/schemas/overrid_contracts` now defines `cli_profile` fields for endpoint, endpoint fingerprint, environment class, tenant id, actor id, credential namespace, schema pins, default output mode, confirmation policy, fixture allowance, and owner-only file-backed storage policy.
- Credential gate: `credential_reference` validation supports keychain, secret service, encrypted local store, signing agent, hardware token, fixture, and CI reference classes while rejecting raw private-key material, revoked credentials, expired credentials, namespace mismatch, and disallowed fixture use.
- Environment gate: `packages/sdk` enforces profile and credential safety before request construction, rejects silent endpoint overrides for seed and production-like profiles, requires explicit profile confirmation for sensitive mutations, and requires `--reason` for admin-impacting mutations.
- Signer handoff gate: mutating credential commands return signature refs through SDK signer handoff and never print raw private keys, tokens, signatures, decrypted payloads, or private payload material.
- Validation gate: `scripts/validate_cli_phase3.py` exercises Rust tests, sanitized `profile inspect --json`, signer handoff output, missing seed confirmation, schema/manifest/docs alignment, and secret-redaction checks.

## Phase 4 Implementation Gates

- Lifecycle gate: the CLI records parsed, profile-loaded, credential-ready, payload-validated, signed, submitted, accepted, waiting, completed, denied, and failed lifecycle states in the contract layer and emits terminal lifecycle metadata in JSON output.
- Output-envelope gate: version, doctor, profile, credential, and phase-gated command paths render through one stable envelope containing result/error data, trace id, reason code, retry class, exit code, named exit class, timing, lifecycle, diagnostic bundle, capability metadata, audit refs, and warnings.
- Exit-registry gate: the contract layer locks the small numeric exit-code registry for success, usage, config, credential, schema, policy, phase, idempotency, transport, timeout, platform, and local I/O classes.
- Diagnostic gate: `overrid doctor` and all JSON envelopes expose only redacted diagnostic refs, schema versions, reason codes, retry counts, dependency status, and capability status; raw keys, tokens, signatures, private payloads, raw prompts, and private file contents are rejected.
- Capability gate: local capability discovery reports stale age, route availability, schema versions, phase support, and fail-closed behavior, with unavailable routes returning `not_available_in_phase` instead of private shortcuts.
- Validation gate: `scripts/validate_cli_phase4.py` exercises schema/manifest alignment, Rust surfaces, real CLI JSON output, fixture parity, fail-closed capability output, redaction checks, and Cargo tests.

## Phase 5 Implementation Gates

- Bootstrap parser gate: `auth`, `tenant`, `identity`, `key`, `manifest`, and `workload` command groups plus trace, idempotency, expected-state, target, manifest, workload, and dry-run flags are Phase 1 bootstrap parser surface.
- Signed envelope gate: mutating tenant, identity, key, manifest, and synthetic workload bootstrap commands build signed command envelopes only after profile, credential, SDK target, idempotency, and trace validation.
- Manifest bootstrap gate: `manifest validate|submit|inspect` returns local validation and immutable manifest refs through `sdk_overgate_contract`, without direct registry shortcuts.
- Synthetic workload gate: `workload submit|status|timeline` exposes pending-only synthetic workload state with `execution_implied:false`; real logs, cancel, result, and follow remain fail-closed phase-gated commands.
- Validation gate: `scripts/validate_cli_phase5.py` is wired into `scripts/validate_overrid.py` and validates docs, schema source, Rust surfaces, emitted CLI JSON, redaction, and Cargo tests.

## Phase 6 Implementation Gates

- Canonical idempotency gate: default mutating command keys are derived from environment class, endpoint identity, tenant, actor, command type, target ref, canonical payload hash, expected state, reason, and schema version; safe retries reuse the same key while `--new-idempotency-key` creates a new operation key.
- Retry and timeout gate: CLI commands reuse SDK bounded retry/timeout policy, exposing retryable transport/platform classes while schema, auth, policy, phase, credential, and idempotency denials remain non-retryable.
- Trace and audit gate: trace ids flow into signed envelopes and JSON output, and mutating commands render acceptance/audit refs without direct Overwatch access.
- Error decoding gate: Overgate, policy, schema, idempotency, credential, dependency, timeout, and platform failures decode into stable reason codes, retry classes, exit classes, source families, and remediation hints without raw internal errors.
- Local idempotency cache gate: `idempotency-cache inspect|reset` renders owner-only, profile/environment scoped, resettable/inspectable cache records with `contains_private_payload:false`.
- Validation gate: `scripts/validate_cli_phase6.py` is wired into `scripts/validate_overrid.py` and validates docs, schema source, Rust surfaces, emitted CLI JSON, redaction, and Cargo tests.

## Validation

- CLI can complete the [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md) synthetic workload path.
- CLI can submit and inspect a [Phase 3: Private Execution Loop](../../build_plan/phase_03_private_execution_loop.md) real private job.
- CLI output includes trace ids for failures.

## Handoff

This becomes the operator and developer workflow until richer admin UI surfaces mature.
