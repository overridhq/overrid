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

- Workload manifests: `build_workload_manifest()` validates workload class, typed resource/data/policy declarations, egress declarations, output declarations, secret refs, and schema version locally while keeping `runtime_acceptance_claimed` false.
- Workload submission: `submit_workload()` wraps manifest validation, `build_command()`, `phase5_signature_ref()`, `sign_request()`, and `prepare_overgate_submission()` so mutating workload requests still route through Overgate command envelopes.
- Submission decoding: `decode_workload_submission_response()` preserves accepted, denied, duplicate, retry-wait, and terminal failure responses from Overgate, marks pending queue reached only from explicit queue evidence, and never invents completed runtime state from a local helper.
- Status, result, and cancellation helpers: `build_command_status_request()`, `build_workload_status_request()`, `build_job_status_request()`, `build_workload_result_request()`, `build_cancellation_status_request()`, `SdkWorkloadStatusRecord::from_service()`, and `build_workload_cancellation_request()` use public control-plane paths and require service evidence before failed, cancelled, timed-out, duplicate, dead-letter, or completed states are treated as true.
- Policy dry-run: `build_policy_dry_run_request()` is gated by `negotiate_sdk_capability()` for `SdkOptionalHelper::PolicyDryRun`; `decode_policy_dry_run_result()` returns matched policy refs, reason codes, placement class, and correction fields without mutating runtime state and must never cache dry-run output as policy truth.
- Runtime authority boundary: `sdk_phase6_authority_review()` and `validate_phase6_authority_review()` document that workload, manifest, status, result, cancellation, and dry-run helpers are wrappers only; they do not become schedulers, policy engines, direct storage readers, metering truth, or bypasses around Overgate, Overguard, Overqueue, Overrun, Overmeter, or Overwatch.

## Phase 7 Usage, Receipt, ORU, And Dispute Readers

The Phase 7 SDK gate adds read-only accounting helper descriptors while keeping accounting truth in owning Overrid services:

- Accounting read requests: `build_accounting_read_request()` emits read-only descriptors for usage receipts, usage rollups, ORU charge previews, Seal Ledger refs, dispute refs, receipt refs, holds, refund/correction refs, grants, and asset refs behind `SdkOptionalHelper::AccountingReaders`.
- Readiness gating: `validate_accounting_reader_readiness()` fails closed until Phase 5 accounting APIs, Phase 6 product-integration readiness, supported schema versions, supported SDK majors, and accounting service capabilities are present.
- Usage receipt views: `decode_usage_receipt_view()` requires service-returned receipt evidence and preserves usage rollup refs, ORU dimension totals, Seal Ledger refs, Overbill refs, Overgrant refs, Overasset refs, policy refs, audit refs, dispute windows, trace ids, and redaction profiles without embedding charge tables.
- Usage rollup views: `decode_usage_rollup_view()` requires service-returned Overmeter rollup evidence, preserves ORU dimension totals and accounting refs unchanged, rejects invalid accounting windows, and stays read-only.
- ORU charge previews: `decode_oru_charge_preview()` consumes only service-returned preview objects and keeps client-side settlement decisions disabled.
- Seal Ledger and receipt refs: `decode_seal_ledger_reference_view()` and `decode_receipt_reference_view()` preserve service-returned ledger, receipt, idempotency, audit, and trace refs without mutating accounting state.
- Dispute and correction refs: `decode_dispute_reference_view()` and `build_accounting_error_surface()` surface dispute refs, correction refs, challenge windows, provider payout hold refs, refund refs, and denied-settlement reason refs without masking them behind generic errors.
- Ref preservation: `verify_accounting_refs_unchanged()` rejects SDK rewrites of Overmeter, ORU Account Service, Seal Ledger, Overbill, Overgrant, Overasset, policy, and audit refs.
- Accounting authority boundary: `sdk_phase7_authority_review()` and `validate_phase7_authority_review()` reject direct payment-provider calls, embedded charge tables, client-side settlement decisions, and SDK mutation of accounting state.

## Phase 8 Fixtures, Contract Tests, And Validation Artifacts

The Phase 8 SDK gate makes fixture and validation evidence explicit while keeping it local/test-only:

- Local fixtures: `sdk_phase8_local_fixture_corpus()` and `validate_phase8_local_fixture_corpus()` cover tenants, actors, credential refs, command envelopes, manifests, signature refs, idempotency entries, errors, usage refs, and audit refs with deterministic seeds, reset markers, redaction profiles, schema versions, local environment markers, and no production defaults.
- Fixture artifacts: `packages/sdk/fixtures/phase8/local_sdk_fixture_set.valid.json`, `golden_cross_language_corpus.valid.json`, and `validation_artifacts_manifest.valid.json` are source-controlled evidence inputs checked by `validate_phase8_fixture_artifact_files()` and `scripts/validate_sdk_phase8.py` with deep field checks, not marker-only scans.
- Contract tests: `sdk_phase8_contract_tests()` and `validate_phase8_contract_tests()` describe signed command submission, duplicate idempotency, stable error preservation, and status-read checks against public Overgate/control-plane routes; the validation-artifact manifest mirrors these descriptors, and missing local-stack owner services are recorded as blockers instead of replaced by internal service mocks.
- Golden fixtures: `sdk_phase8_golden_fixtures()` and `validate_phase8_golden_fixtures()` gate request envelopes, canonical signing inputs, response errors, manifest validation, idempotency behavior, and redaction behavior. Golden cases link back to local fixture ids and assert no private material, raw payloads, or signature values are present. Rust must pass first; TypeScript/web and later bindings remain blocked until they pass the same corpus.
- Security checks: `sdk_phase8_security_redaction_checks()` and `validate_phase8_security_redaction_checks()` cover production/test separation, fake signer isolation, secret-ref redaction, request-body rejection, signature-value rejection, private payload log rejection, credential lifecycle failures, unsupported schemas, and unsafe downgrade attempts.
- Validation artifacts: `sdk_phase8_validation_artifacts()` and `validate_phase8_validation_artifacts()` describe schema generation, contract tests, signing golden checks, idempotency behavior, redaction checks, compatibility checks, and docs alignment as CI/build evidence. These artifacts are Docdex-indexable progress evidence, not Overwatch runtime events.

## Phase 9 TypeScript/Web Bindings And Product Integration Hardening

The Phase 9 SDK gate records generated TypeScript/web binding projections and product hardening descriptors while preserving Rust-first runtime authority:

- TypeScript/web bindings: `sdk_phase9_typescript_web_bindings()` and `validate_phase9_typescript_web_bindings()` describe generated models, validators, error objects, request helpers, manifest helpers, and idempotency helpers from `packages/schemas/overrid_contracts`. They must pass the Phase 8 golden corpus, match Rust request envelopes/errors/idempotency behavior, and remain browser/client projections with no runtime authority or handwritten public objects.
- Binding artifacts: `packages/sdk/bindings/phase9/typescript_web_binding_manifest.valid.json`, `product_convenience_modules.valid.json`, and `binding_readiness_gates.valid.json` are source-controlled evidence inputs checked by `validate_phase9_artifact_files()` and `scripts/validate_sdk_phase9.py`.
- Adapter/UI safety: `sdk_phase9_adapter_ui_safety_boundary()` and `validate_phase9_adapter_ui_safety_boundary()` require generated clients over approved Overgate/admin/control-plane/accounting public APIs and reject privileged internal endpoints, private key storage, hidden service shortcuts, direct worker/storage APIs, and browser runtime authority.
- Product convenience modules: `sdk_phase9_product_convenience_modules()` and `validate_phase9_product_convenience_modules()` cover Docdex encrypted RAG jobs, Mcoda agent workloads, Codali code-agent workloads, and package validation with capability checks, authorized refs, job status, cancellation, results, explicit usage rollups, receipts, and failure cases for successful jobs, retryable/final failures, cancellation, timeout, policy denial, budget exhaustion, node disconnect, and disputed usage.
- Mobile boundary: `sdk_phase9_mobile_extension_interfaces()` and `validate_phase9_mobile_extension_interfaces()` expose only shared extension points for session refresh, secure storage adapters, offline queue interfaces, sync cursors, conflict states, and redacted diagnostics. Phase 12 Mobile SDK owns OS secure storage, push/background behavior, media upload state, offline queues, and mobile packaging.
- Binding readiness: `sdk_phase9_binding_readiness_gates()` and `validate_phase9_binding_readiness_gates()` keep TypeScript/web and later Swift/iOS, Kotlin/Android, Python, or other bindings blocked on schema compatibility, golden request-envelope fixtures, generated error objects, cross-language contract tests, no handwritten public-object drift, and Rust behavior parity.

## Phase 10 Validation, Documentation Alignment, And Downstream Handoff

The Phase 10 SDK gate closes the build-plan loop with validation evidence and downstream handoff rules while keeping the SDK a thin client package:

- Structure validation: `sdk_phase10_structure_validation()` and `validate_phase10_structure_validation()` cover the sub-build-plan title prefix, attached SDS link, phase headings, work-item structure, Design/Output/Validation fields, and exit gate.
- Alignment checklist: `sdk_phase10_alignment_checklist()` and `validate_phase10_alignment_checklist()` verify Rust-first SDK generation, TypeScript/web as the generated second target, language-neutral schema authority, Overgate-only mutating calls, credential-provider signing, and Phase 1 plus Phase 6 master-plan alignment.
- Handoff rules: `sdk_phase10_downstream_handoff_rules()` and `validate_phase10_downstream_handoff_rules()` document how CLI, admin/developer UI, Docdex, Mcoda, Codali, native apps, mobile services, Overpack deployment, accounting, policy, and future bindings consume SDK primitives without bypassing owner services.
- Evidence artifacts: `packages/sdk/handoff/phase10/structure_validation.valid.json`, `alignment_checklist.valid.json`, and `downstream_handoff_rules.valid.json` are source-controlled closure evidence checked by `validate_phase10_artifact_files()` and `scripts/validate_sdk_phase10.py`.
- Runtime boundary: Phase 10 adds no new SDK dependencies and does not make the SDK a scheduler, policy authority, accounting authority, secret store, deployment authority, or storage owner.
