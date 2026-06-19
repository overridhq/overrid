SDS #2

# CLI SDS

## Purpose

Let developers, operators, product integrators, and automation use Overrid from the terminal without manually calling internal APIs.

This SDS defines the CLI as a schema-checked client on top of the SDK and Overgate. The CLI must wrap signing, idempotency, trace ids, retries, error decoding, and machine-readable output so humans and scripts do not reimplement protocol details.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [cli.md](../../service_catalog/foundation/cli.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md), [Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md) |
| SDS sub-build plan | [SUB BUILD PLAN #2 - CLI](../../build_plan/sub_build_plan_002_cli.md) |

## Service Family

- Family: Foundation and developer tooling.
- Owning layer: Developer/operator client tooling.
- Primary data scope: local CLI profiles, credential references, command envelopes, idempotency cache, output envelopes, exit codes, and local diagnostics.
- First build phase from service plan: basic commands in [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md); hardened external-product CLI in [Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md).

## Problem Statement

Overrid's first usable path must be scriptable. Phase 1 needs a terminal path to create identities, tenants, keys, manifests, and signed synthetic workload commands. Phase 6 needs the same tool hardened enough for real product integrations. Without a disciplined CLI, builders will handcraft HTTP calls, copy development secrets, skip idempotency, and produce untraceable failures.

## Goals

- Provide a stable command surface for Phase 1 control-plane bootstrap and Phase 6 product integration.
- Use the SDK and shared schemas for every request and response.
- Make signed command envelopes, idempotency keys, trace ids, retries, and error decoding automatic.
- Produce clear human output by default and stable `--json` output for automation.
- Support local, seed, staging, and production-like profiles without mixing credentials.
- Keep failure output useful: trace id, reason code, retry class, and audit refs where available.

## Non-Goals

- Do not bypass Overgate or call private service storage.
- Do not become the source of truth for identity, tenant, key, workload, usage, or ledger state.
- Do not store raw private keys or long-lived secrets in world-readable files.
- Do not add hidden developer bypasses that work against production or seed environments.
- Do not make CLI-only behavior that the SDK and Admin UI cannot reproduce through documented APIs.
- Do not encode pricing, revenue forecasts, blockchain mechanics, NFT mechanics, or per-transaction fee assumptions.

## Primary Actors And Clients

- Founder/operator managing the seed private swarm.
- Service implementers running Phase 0-6 workflows locally.
- Product integrators for Docdex, Mcoda, Codali, and later native apps.
- Automation scripts and CI jobs that need stable `--json` output.
- Incident responders collecting trace ids, audit refs, and diagnostics.

## Dependencies

- SDK for typed API calls, signing helpers, retries, and error decoding.
- Shared schema package for command, manifest, event, error, and output validation.
- Overgate for all platform API access.
- Overpass, Overtenant, and Overkey for identity, tenant, role, credential, signing, rotation, and revocation flows.
- Overregistry for resource, workload, package, provider, and native app manifests.
- Overwatch for audit trace lookup.
- Overqueue and later execution services for workload status, logs, cancellation, and results.
- Overmeter, ORU Account Service, Seal Ledger, and Overbill for usage, receipt, balance, and hold views.
- Package Validator and Policy Dry-Run API for local validation before submit.

When a dependency is not built yet, the CLI command must fail with a stable `not_available_in_phase` or equivalent reason code instead of silently using private shortcuts.

## Owned Responsibilities

CLI is responsible for:

- Defining command names, flags, arguments, profiles, output formats, and exit codes.
- Loading local profile context and credential references safely.
- Constructing signed Overrid command envelopes.
- Generating or reusing idempotency keys for mutating commands.
- Passing all API calls through the SDK/Overgate path.
- Rendering human-readable output and stable JSON output from the same response envelope.
- Preserving trace ids, reason codes, and audit refs in every failure.
- Providing local diagnostics without logging secrets or private payloads.

The CLI does not own platform state and must not create untracked side effects outside Overgate.

## Data Model

The first implementation should define:

- `cli_profile`: named environment profile with API endpoint, tenant id, actor id, key reference, default output mode, and safety flags.
- `credential_reference`: pointer to OS keychain, encrypted local key store, hardware key, agent socket, or test fixture key. It must not require raw key content in config files.
- `command_context`: resolved profile, actor, tenant, trace id, idempotency key, dry-run flag, output mode, and timeout/retry policy.
- `signed_command_envelope`: shared-schema command object with command id, tenant id, actor id, target, command type, payload type, idempotency key, trace id, signature, and schema version.
- `output_envelope`: stable response shape for `--json` with `ok`, `result`, `error`, `trace_id`, `audit_refs`, `reason_code`, `retry_class`, `schema_version`, and `warnings`.
- `exit_code_registry`: stable numeric exit codes and named classes for automation.
- `local_idempotency_cache`: optional local cache mapping command fingerprint to idempotency key for safe retries.
- `diagnostic_bundle`: redacted local context for support, including version, profile name, endpoint, trace ids, and reason codes.

Local config should live under an OS-appropriate config directory and use file permissions equivalent to owner-read/write only when file-backed.

## API Surface

The CLI surface is a command contract, not an HTTP API. Initial command groups:

- `overrid auth login`: enroll or select actor credentials.
- `overrid auth whoami`: show actor, tenant, roles, key id, and endpoint.
- `overrid tenant create|list|inspect|suspend`: tenant setup and inspection.
- `overrid identity create|list|inspect|disable`: person, organization, node, app, service-account, and system-service identity records.
- `overrid key enroll|list|rotate|revoke`: Overkey credential lifecycle.
- `overrid manifest validate|submit|inspect`: resource, workload, package, provider, and native app manifests.
- `overrid workload submit|status|timeline|logs|cancel|result`: workload lifecycle through Overgate and queue/execution services.
- `overrid node register|inspect|health`: node registration and seed private swarm checks.
- `overrid usage show`: usage rollups through Overmeter views.
- `overrid receipt show`: receipt and ledger references through Seal Ledger/Overbill views.
- `overrid policy dry-run`: policy evaluation before submit.
- `overrid package validate`: package validation before scheduling/execution.
- `overrid dev start|stop|reset|seed|smoke`: local-stack convenience wrapper once Phase 0 commands exist.

Command requirements:

- Every command supports `--profile`, `--tenant`, `--actor`, `--trace-id`, `--json`, `--verbose`, and `--no-color` where relevant.
- Mutating commands support `--idempotency-key`, `--dry-run`, `--confirm`, and `--reason` where relevant.
- Long-running commands support `--wait`, `--timeout`, `--poll-interval`, and `--follow`.
- `--json` output must remain backward-compatible within a schema version.
- Human output must include trace id for failures and terminal state summaries for successful mutating commands.

## Event Surface

The CLI must not emit platform events directly. It submits signed commands to Overgate; Overgate and downstream services emit platform events.

CLI-local diagnostics:

- `cli.command_started`: local diagnostic containing command name, profile name, trace id, and schema version; no secrets.
- `cli.command_completed`: local diagnostic containing command name, trace id, duration, exit code, and retry count.
- `cli.command_failed`: local diagnostic containing reason code, retry class, trace id, and sanitized endpoint.

Platform events caused by CLI commands include `request.accepted`, `request.denied`, `manifest.accepted`, `queue_item.created`, `state_transition.applied`, `usage.rollup.created`, or more specific service events as the platform matures.

## Core Workflow

1. Parse command, flags, environment variables, and selected profile.
2. Load credential reference and resolve actor, tenant, endpoint, and schema versions.
3. Validate command payload locally using shared schemas.
4. Generate trace id and idempotency key when needed.
5. Sign the command envelope or request an approved signer to sign.
6. Submit through SDK to Overgate.
7. Decode response, reason code, retry class, audit refs, and result records.
8. Render human output or `--json` output.
9. Preserve diagnostics for support while redacting secrets.

## State Machine

Command lifecycle:

1. `parsed`: command and flags are syntactically valid.
2. `profile_loaded`: local profile and endpoint are resolved.
3. `credential_ready`: credential reference exists and is usable.
4. `payload_validated`: request payload passed shared-schema validation.
5. `signed`: mutating command has a signed envelope.
6. `submitted`: request sent to Overgate with trace id and idempotency key.
7. `accepted`: Overgate accepted the request.
8. `waiting`: CLI is polling or following a long-running operation.
9. `completed`: terminal success rendered.
10. `denied`: request denied before side effects.
11. `failed`: transport, schema, dependency, timeout, or terminal platform failure.

Credential lifecycle:

1. `unenrolled`
2. `enrolled`
3. `active`
4. `rotating`
5. `revoked`
6. `expired`

## Policy And Security

- Keep profiles environment-scoped so local/test credentials cannot be sent to production-like endpoints accidentally.
- Store raw secrets only in approved secret stores; file-backed secrets require owner-only permissions and clear warning output.
- Never print private keys, tokens, signatures, encrypted content, or raw private payloads.
- Require explicit confirmation or `--yes` for destructive operations.
- Require `--reason` for admin-impacting operations such as cancellation, suspension, revocation, dispute annotation, or correction.
- Include expected current state when commands mutate records that may change concurrently.
- Fail closed on schema mismatch, unknown endpoint mode, revoked key, missing tenant, or ambiguous profile.
- Make dry-run behavior explicit; dry-run cannot create side effects.
- Keep test fixture bypasses locked to local-stack profiles with visible test-environment markers.

## Metering And Accounting

The CLI should not create payment behavior. It should expose and preserve accounting evidence:

- Show usage rollups, ORU balances, holds, receipts, and correction refs through read APIs.
- Include trace id and actor/tenant context for CLI-originated commands that consume resources.
- Surface budget exhaustion and quota denials with reason code and retry/correction guidance.
- Avoid external payment calls for internal resource operations.
- Keep machine-to-machine usage visible through normal Overmeter and Seal Ledger references.

## Observability And Operations

The CLI should provide:

- `overrid version` with CLI, SDK, schema, and API compatibility versions.
- `overrid doctor` for profile, endpoint, credential, schema, clock-skew, and local config checks.
- Redacted diagnostic bundle generation.
- Structured debug logs behind explicit `--verbose` or `OVERRID_DEBUG=1`.
- Stable exit codes for automation.
- Timing and retry counts in JSON output.
- Clear handling of network errors, TLS errors, schema mismatch, auth denial, policy denial, and platform terminal failures.

## Failure Modes And Recovery

- Missing profile: show profile creation command and do not guess endpoint.
- Credential unavailable: fail before constructing a command envelope.
- Revoked or expired key: show key id, reason code, and enrollment/rotation path.
- Invalid schema: fail locally with field path before submit.
- Invalid signature or tenant: Overgate denies before side effects; CLI shows reason and trace id.
- Duplicate idempotency key: show original outcome or deterministic duplicate result.
- Long-running timeout: return timeout exit code with trace id and `status` command hint.
- Partial output failure: preserve raw JSON response in debug bundle if it contains no secret-bearing fields.

## Validation Plan

Service-plan validation:

- CLI can complete the [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md) synthetic workload path.
- CLI can submit and inspect a [Phase 3: Private Execution Loop](../../build_plan/phase_03_private_execution_loop.md) real private job.
- CLI output includes trace ids for failures.

Additional SDS-level validation:

- Contract tests cover every command's `--json` output schema.
- Snapshot tests cover human output without depending on terminal color.
- Permission tests prove CLI cannot bypass Overgate or call private service storage.
- Credential tests cover file permissions, missing key, revoked key, rotation, and local fixture isolation.
- Idempotency tests prove retried mutating commands do not duplicate side effects.
- Exit-code tests lock automation behavior.
- Phase 6 product tests prove Docdex, Mcoda, or Codali can submit, inspect, cancel, and retrieve jobs through CLI/SDK without direct internal API calls.

## Build Breakdown

1. Define command naming, profile file shape, output envelope, and exit-code registry.
2. Build `auth`, `tenant`, `identity`, `key`, and `manifest` commands for Phase 1.
3. Add signing, idempotency, trace id, and SDK/Overgate client middleware.
4. Add `workload submit/status/timeline/cancel/result` for synthetic queued work.
5. Add local-stack helpers for Phase 0 developer workflows.
6. Add node registration/health commands for Phase 2.
7. Add logs/results/follow behavior for Phase 3 execution.
8. Add policy dry-run and package validation commands for Phase 4/9.
9. Add usage, receipt, balance, and dispute views for Phase 5/6.
10. Harden `--json`, diagnostics, retries, and product-integration workflows for Phase 6.

## Handoff And Downstream Use

The CLI becomes the first practical operator and developer workflow before the Admin and Developer UI is mature. The SDK and CLI should share transport, signing, error, retry, and schema code so product integrations do not fork behavior.

Later native apps and adapters should treat CLI behavior as an executable example of correct Overgate usage, not as a private backdoor.

## Open Design Questions

Resolved decisions:

- Default credential storage should be platform-native where possible and explicit where not. macOS defaults to the system Keychain. Linux desktop defaults to Secret Service/libsecret when available, while headless Linux may use an Overrid-owned encrypted local key store or signing-agent socket with owner-only file permissions and visible warnings. CI must not have an ambient persistent keychain default; it should use explicit short-lived service-account, fixture, hardware-token, signing-agent, or mounted-secret references supplied by the CI environment, marked `profile_kind=ci`, and never stored as raw key material in CLI config.
- Phase 1 should expose only commands backed by the Phase 1 control-plane skeleton: `version`, `doctor`, profile management, `auth login|whoami`, credential enrollment, `tenant create|list|inspect`, `identity create|list|inspect|disable`, `key enroll|list|rotate|revoke`, `manifest validate|submit|inspect`, and synthetic `workload submit|status|timeline` commands that stop at durable pending queue state. Local-only `dev start|stop|reset|seed|smoke` may be available for Phase 0/1 profiles. Node registration and health commands remain Phase 2; logs, result fetch, follow, cancellation, and real execution status remain Phase 3; policy dry-run remains Phase 4; usage, balance, receipt, and dispute views remain Phase 5/6; package validation and deployment commands remain Phase 9. Normal help should hide commands whose backing service contract is not present, while `--all-phases` or docs may list planned commands that fail with `not_available_in_phase`.
- Mutating commands should generate idempotency keys from a canonical command fingerprint by default after local schema validation. The fingerprint should include environment class, endpoint identity, tenant id, actor id, command type, target ref, canonical payload hash, expected current state, reason, and schema version. This makes retries deterministic and prevents duplicate side effects. Operators may pass `--idempotency-key` to resume a known operation or `--new-idempotency-key`/changed payload to intentionally request a new operation. Read-only commands do not need idempotency keys.
- The first stable exit-code registry should be small, numeric, and schema-versioned before external automation depends on it: `0` success, `2` CLI usage or flag error, `3` profile/config/env error, `4` credential or authentication error, `5` local schema/validation error, `6` authorization or policy denial returned by Overgate/Overguard, `7` phase or dependency unavailable, `8` idempotency conflict or expected-state mismatch, `9` retryable transport/platform unavailable, `10` timeout while waiting or following, `11` terminal platform failure, and `12` local I/O or diagnostic-bundle failure. `--json` output must always include the named exit class, trace id where one exists, reason code, retry class, and audit refs where available.
- CLI profiles must enforce environment separation before any request is built. A profile should carry `environment_class` (`local`, `seed`, `staging`, `production_like`, `ci`), endpoint fingerprint, tenant id, actor id, credential namespace, allowed credential classes, fixture allowance, default confirmation policy, and schema compatibility pins. Fixture credentials are valid only for loopback/local-stack endpoints and must be rejected for seed, staging, production-like, and CI profiles unless the profile is explicitly marked as a test harness profile. Seed and production-like profiles must require distinct credential namespaces, endpoint verification, no silent endpoint override from environment variables, explicit profile selection for mutating commands, and an extra confirmation or `--confirm-profile` when a command would cross from local/test into seed or production-like infrastructure.
