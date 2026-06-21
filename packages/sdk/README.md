# SDK Package Contract

`packages/sdk` owns the Rust SDK first. It provides typed client and transport helpers generated or validated from stable Overrid contracts.

Rules:

- Depend on `packages/schemas` as the contract authority.
- Preserve SDK/Overgate routing for control-plane calls; do not add direct internal service, queue, storage, node, or private-state paths.
- Keep TypeScript/web bindings as generated client projections outside this Rust SDK runtime.
- Keep test fixtures secret-free and aligned with shared schema versions.
- Expose release compatibility metadata for SDK name, semantic version, language binding, supported schema versions, service capability profile, deprecation behavior, upgrade guidance, and security-critical break handling.
- Reject unsafe SDK majors with `unsupported_sdk_version` and unsafe schema versions with `schema_version_unsupported`; never silently downgrade command-envelope, signing, tenant, policy, secret-ref, privacy, or accounting behavior.

## Phase 1 Release Gate

The Phase 1 SDK gate is intentionally narrow:

- First binding: Rust SDK only.
- Current stable major: reported by `sdk_compatibility_metadata()`.
- Previous stable major: reported only when a previous major is still supported.
- Schema authority: `packages/schemas/overrid_contracts`.
- Capability profile: `phase1-control-plane-thin-client`.
- Release checklist: `sdk_release_checklist()`.
- Resolved SDS decisions: Rust-first binding first; TypeScript/web generated second after schema stability; credential-provider-only signing; bounded idempotency retention; separate Mobile SDK boundary; current-plus-previous stable major compatibility.
- Compatibility check: `OverridSdkClient::new()` uses `check_sdk_compatibility()` and accepts only named `SDK_SUPPORTED_SCHEMA_VERSIONS`; older or unnamed schema strings fail with `schema_version_unsupported` instead of silently downgrading.

Later TypeScript/web, mobile, Python, Swift, or Kotlin bindings must be generated from the same contracts and pass shared fixture checks before release.

## Phase 2 Contract Intake And Local Data Model

The Phase 2 SDK gate keeps `packages/schemas` and docs/specs as contract authority:

- Contract intake: `sdk_contract_intake_manifest()` names the canonical JSON Schema, codegen manifest, Rust projection, docs/specs, SDK SDS, owning phase, generated output path, schema version, and freshness policy.
- Local config: `SdkConfigRecord::from_input()` requires an explicit environment, Overgate endpoint, timeout/retry policy, feature flags, client identity ref, credential ref, service capability profile, and live-endpoint confirmation for live profiles.
- Production/test separation: unknown feature flags, implicit live endpoint use, and production fixture enablement fail before network use.
- Request records: `SdkRequestContextRecord` and `SdkSignedRequestRecord` preserve actor id, tenant id, trace id, idempotency key, command type, schema version, credential id, signature metadata, body hash, timestamp, and replay window without raw private material or bearer tokens.
- Idempotency and errors: `SdkIdempotencyEntry` applies command-class retention and skips read-only cache entries; `OverridErrorRecord` preserves reason codes, trace ids, audit refs, retryability, correction fields, dependency names, policy refs, and schema version.
- Capability negotiation: `negotiate_sdk_capability()` checks service schema versions, SDK major support, signing, idempotency, policy dry-run, and accounting capability, returning `sdk_capability_unavailable` before unsafe optional helpers run.

## Phase 3 Generated Rust SDK Skeleton

The Phase 3 SDK gate keeps the first SDK binding Rust-first and explicit about generated versus handwritten ownership:

- Package boundary: `sdk_package_boundary()` records generated projection, handwritten client, and read-helper module paths, source authorities, public entrypoints, and the rule that generated SDK output is not contract authority.
- Generated models: `sdk_generated_model_descriptors()` names Phase 1 command, tenant, identity, key metadata, manifest, queue status, audit ref, and error projections with source contract names, validator symbols, schema versions, stable enum mappings, and reason-code object coverage.
- Client construction: `configure_client()` builds an immutable `ConfiguredSdkClient` from `SdkConfigRecord`, preserving retry/timeout policy, credential-provider refs, trace policy, redaction defaults, and local/test loopback protection before network use.
- Read helpers: `build_control_plane_read_request()` creates read-only request descriptors for tenant, identity, key metadata, manifest, queue status, and audit refs while preserving schema versions, pagination cursors, request ids, trace ids, and audit refs.
- Version reporting: `sdk_version_report()` reports SDK name, semantic version, schema set, generated-contract revision, supported feature flags, language binding, and Phase 3 capability profile; request builders also attach SDK metadata headers so Overgate compatibility checks can reject unsupported clients with stable upgrade guidance.

## Phase 4 Command Pipeline, Idempotency, Retry, And Errors

The Phase 4 SDK gate turns Phase 2 records and Phase 3 client construction into a safe mutating-command pipeline:

- Command envelopes: `build_command()` validates tenant id, actor id, command type, schema version, trace id, idempotency key, timestamp, signature ref, and payload fields before building a `SignedCommandEnvelope`.
- Canonical payloads: `SdkCommandPayload::canonical_payload()` sorts and escapes fields deterministically, and `SdkCommandEnvelope` carries a stable request hash for idempotency decisions.
- Overgate submission: `prepare_overgate_submission()` only prepares `POST /v1/overgate/commands` descriptors and reuses Overgate endpoint validation; it does not call private service storage, queue, worker, policy, accounting, or vault endpoints.
- Response decoding: `decode_overgate_submission()` accepts only explicit Overgate service responses for accepted, completed, denied, duplicate, retry-wait, and terminal failure states while preserving queue refs, audit refs, trace ids, retry class, duplicate refs, and errors.
- Idempotency cache: `phase4_idempotency_policy()` and `evaluate_idempotency_cache()` skip read-only caching, store in-flight retry records for the shorter of the absolute command deadline or the 2-hour SDK cap, keep 24-hour Phase 1 terminal digests, keep 7-day workload refs, keep security-sensitive terminal digests for 24 hours with no raw payload retention, detect conflicting request hashes, and expose `clear_phase4_idempotency_cache()` for local `dev reset` cleanup.
- Retry classification: `classify_phase4_retry()` retries only transport failures, timeouts, or service responses explicitly marked retryable/retry-after, and always requires the same idempotency key for safe retries.
- Stable errors: `decode_stable_overrid_error()` preserves reason code, message, trace id, audit refs, retryable flag, correction fields, dependency name, policy refs, and schema version in caller-facing `overrid_error` records.

## Phase 5 Credential Providers, Signing, And Security Guardrails

The Phase 5 SDK gate keeps signing delegated to credential-provider references while the Rust SDK prepares deterministic, secret-free signing metadata:

- Credential providers: `SdkCredentialProvider::from_config()` and `SdkCredentialProviderKind` describe file-backed local test credentials, host signing-agent sockets, platform keychains, hardware-backed devices, and Overkey-compatible refs without storing private keys, bearer tokens, seed phrases, raw payload secrets, or vault values.
- Canonical signing: `phase5_signature_ref()`, `build_canonical_signing_input()`, and `sign_request()` bind method, path, sorted headers, body hash, timestamp, schema version, credential id, replay window, tenant id, actor id, trace id, and idempotency key before signing handoff.
- Signing invariants: `validate_signed_request_invariants()` rejects mutated payloads, wrong tenants, expired timestamps, mismatched credentials, missing signing capability, and unsupported signing algorithms before a request can be treated as signed.
- Test signer separation: `validate_fixture_signer_installation()` allows fixture signers only for explicit local or CI test-fixture configuration and rejects production-like or ambiguous fixture use.
- Redacted diagnostics: `SDK_PHASE5_DIAGNOSTIC_EVENTS` and `redacted_diagnostic_event()` cover request_built, request_signed, request_sent, response_received, retry_scheduled, request_denied, request_failed, and duplicate_resolved while rendering payloads, signatures, and secret refs as redacted.
- Credential lifecycle failures: `credential_lifecycle_failure()` maps expired, revoked, rotated, missing, mismatched, unknown, insufficient, host-signer-unavailable, and retry-prohibited signing failures to terminal decisions unless Overgate supplies an explicit retryable correction path.

## Phase 6 Workload, Manifest, Status, And Policy Helpers

The Phase 6 SDK gate adds ergonomic workload helpers while keeping runtime authority in Overrid services:

- Workload manifests: `build_workload_manifest()` validates workload class, resources, data refs, policy refs, egress declarations, output declarations, secret refs, and schema version locally while keeping `runtime_acceptance_claimed` false.
- Workload submission: `submit_workload()` wraps manifest validation, `build_command()`, `phase5_signature_ref()`, `sign_request()`, and `prepare_overgate_submission()` so mutating workload requests still route through Overgate command envelopes.
- Submission decoding: `decode_workload_submission_response()` preserves accepted, denied, duplicate, retry-wait, and terminal failure responses from Overgate and never invents completed runtime state from a local helper.
- Status, result, and cancellation helpers: `build_workload_read_request()`, `SdkWorkloadStatusRecord::from_service()`, and `build_workload_cancellation_request()` use public control-plane paths and require service evidence before failed, cancelled, timed-out, duplicate, dead-letter, or completed states are treated as true.
- Policy dry-run: `build_policy_dry_run_request()` is gated by `negotiate_sdk_capability()` for `SdkOptionalHelper::PolicyDryRun`; `decode_policy_dry_run_result()` returns matched policy refs, reason codes, placement class, and correction fields without mutating runtime state and must never cache dry-run output as policy truth.
- Runtime authority boundary: `sdk_phase6_authority_review()` and `validate_phase6_authority_review()` document that workload, manifest, status, result, cancellation, and dry-run helpers are wrappers only; they do not become schedulers, policy engines, direct storage readers, metering truth, or bypasses around Overgate, Overguard, Overqueue, Overrun, Overmeter, or Overwatch.
