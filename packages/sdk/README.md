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
- Version reporting: `sdk_version_report()` reports SDK name, semantic version, schema set, generated-contract revision, supported feature flags, language binding, and Phase 3 capability profile.
