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

Later TypeScript/web, mobile, Python, Swift, or Kotlin bindings must be generated from the same contracts and pass shared fixture checks before release.
