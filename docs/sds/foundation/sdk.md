SDS #6

# SDK SDS

## Purpose

Give applications, native services, adapters, operators, and developers a safe client layer for Overrid APIs.

The SDK is a versioned developer package, not a standalone runtime service. It standardizes request signing, idempotency, trace propagation, schema validation, error decoding, and common workflow helpers so callers can use Overrid without bypassing Overgate or reimplementing security-sensitive behavior.

## Source Documents

| Source | Path |
| --- | --- |
| Sub-build plan | [sub_build_plan_006_sdk.md](../../build_plan/sub_build_plan_006_sdk.md) |
| Service implementation plan | [sdk.md](../../service_catalog/foundation/sdk.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md), [Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md) |

## Service Family

- Family: Foundation and developer tooling.
- Owning layer: Developer experience and platform foundation.
- Runtime shape: generated libraries, request builders, validators, test fixtures, and adapter helpers shipped with semantic versions.
- Primary data scope: local configuration, request envelopes, generated API clients, validation results, retry state, and typed response objects.
- First build phase from service plan: thin client in [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md); hardened SDK in [Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md).

## Problem Statement

Overrid depends on signed, tenant-scoped, idempotent commands. If every service, native app, mobile client, and adapter hand-builds those calls, the platform will drift into incompatible request formats, weak error handling, duplicated retry bugs, and unsafe shortcuts. The SDK must turn shared schemas into one predictable calling path so early hardware can run the first control-plane services and later grid-resident clients can integrate without custom privileged paths.

## Goals

- Generate typed clients from the shared schema package and the approved API descriptions.
- Provide one canonical request pipeline for trace id creation, idempotency key handling, signing, submission, retry classification, and error decoding.
- Keep all mutating calls routed through Overgate and verified by Overkey-managed credentials.
- Make workload, manifest, policy dry-run, usage, receipt, and result access ergonomic enough for native apps, adapters, and mobile services.
- Preserve reason codes, trace ids, request ids, and audit references end to end for debugging and dispute handling.
- Keep client behavior deterministic under retries, duplicate submissions, partial network failures, and service denials.

## Non-Goals

- Do not make the SDK a source of policy truth; policy decisions belong to Overguard and the runtime services.
- Do not make the SDK a secret store; it may reference local credential material but must not persist private keys or long-lived secrets.
- Do not let the SDK call private service storage or internal worker endpoints.
- Do not hide failed denials behind automatic fallback behavior that changes user intent.
- Do not include economic forecasts, customer-count assumptions, or market projections.
- Do not promote local development bypasses into production defaults.

## Primary Actors And Clients

- Native service developers integrating Overrid APIs.
- Native app teams building social, messaging, directory, workspace, search, maps, wallet, and personal AI assistant clients.
- Adapter builders connecting external services to Overrid.
- Node-agent and worker developers needing typed control-plane calls.
- Operators using CLI or admin tooling built on top of the same client primitives.
- Mobile app developers using Overrid as backend infrastructure.

## Dependencies

- Shared schema package for command envelopes, API objects, events, errors, manifests, usage records, and receipts.
- Overgate API conventions for ingress paths, response shapes, rate-limit headers, and denial semantics.
- Overkey credentials for signing keys, API keys, service-account credentials, rotation state, and revocation checks.
- Overpass and Overtenant identifiers for actor and tenant references.
- Overwatch-compatible audit references surfaced in responses.
- Overmeter, ORU, and Seal Ledger response objects once usage and settlement flows are available.

## Owned Responsibilities

SDK owns client-side correctness only:

- Generated API clients and model types for supported languages.
- Request-envelope builders that require actor, tenant, trace, idempotency, timestamp, and command type fields for mutating calls.
- Signing helpers that use credential references and delegate private-key storage to the host environment or Overvault-backed integrations.
- Retry helpers that understand safe retry classes, idempotency replay responses, and terminal denials.
- Error objects that preserve stable reason codes, trace ids, audit refs, retryability, and user-correctable fields.
- Manifest and workload builders that validate locally before submission without claiming runtime acceptance.
- Test fixtures and contract helpers for service teams.

SDK does not own platform records, authorization state, usage accounting, or audit finality. Those remain runtime service responsibilities.

## Data Model

The SDK should define local, typed structures rather than durable platform records:

- `sdk_config`: base URL, client identity reference, tenant default, timeout budget, retry policy, and feature flags.
- `credential_ref`: reference to API key, signing key, service-account credential, hardware-backed key, or host secret slot; never raw private material.
- `request_context`: actor id, tenant id, trace id, idempotency key, command type, schema version, caller app id, and timestamp.
- `signed_request`: canonicalized method, path, body hash, signature metadata, credential id, and replay window metadata.
- `idempotency_entry`: local cache entry for in-flight or recently completed mutating calls, including request hash and terminal response digest.
- `overrid_error`: reason code, message, trace id, audit refs, retryable flag, correction fields, dependency name, and policy refs where provided.
- `workload_manifest_builder`: typed builder for workload, resource, data, policy, and secret-reference declarations.
- `usage_receipt_view`: typed reader for usage, ORU charge, Seal Ledger reference, and dispute refs once accounting APIs exist.

All objects must carry schema version metadata. Local caches must be bounded, user-clearable, and safe to discard.

## API Surface

The SDK should expose library functions, not a network API:

- `configureClient(config)`: create a typed client with base URL, environment, retry policy, and credential provider.
- `buildCommand(input)`: create a schema-checked command envelope with actor, tenant, trace, idempotency, and command type.
- `signRequest(command, credentialRef)`: canonicalize and sign a request without exposing private key material to SDK storage.
- `submitCommand(command)`: send mutating commands through Overgate and return typed accepted, denied, duplicate, or failed responses.
- `getCommandStatus(commandId | traceId)`: read accepted command progress through public control-plane APIs.
- `buildWorkloadManifest(input)`: validate workload, resources, data refs, secret refs, and policy refs before submission.
- `submitWorkload(manifest)`: send signed workload requests through Overgate.
- `readUsageReceipt(ref)`: decode usage, ORU, Seal Ledger, and dispute references when the accounting layer is present.
- `dryRunPolicy(input)`: call the approved policy dry-run API and return matched policy refs without mutating runtime state.

The first thin client may support one language. Later language bindings must be generated from the same schemas and pass the same contract tests.

## Event Surface

The SDK must not emit platform-authoritative events directly. It may produce local diagnostics and must preserve runtime event references returned by services.

- Local diagnostic categories: `request_built`, `request_signed`, `request_sent`, `response_received`, `retry_scheduled`, `request_denied`, `request_failed`, and `duplicate_resolved`.
- Runtime event references: Overgate and downstream services return audit refs that the SDK surfaces unchanged.
- Privacy rule: diagnostics may include ids, reason codes, timings, and schema versions, but must not log secrets, private payloads, or raw user content by default.
- Correlation rule: every local diagnostic should include trace id and idempotency key where applicable.

## Core Workflow

1. Load generated types and validators from the shared schema package.
2. Build a command envelope from caller input, filling or validating actor, tenant, trace id, idempotency key, command type, and schema version.
3. Validate the envelope and body locally.
4. Resolve a credential reference and sign the canonical request.
5. Submit the request to Overgate.
6. Decode accepted, denied, duplicate, retryable, and terminal failure responses into typed results.
7. Preserve trace ids, reason codes, policy refs, usage refs, and audit refs for caller workflows.
8. Apply bounded retries only for safe transport or explicitly retryable service responses.

## State Machine

SDK state is local request lifecycle state, not platform truth:

1. `prepared`: caller input has been accepted into a builder with no network side effects.
2. `validated`: local schema and required-field checks passed.
3. `signed`: canonical request has a credential id and signature metadata.
4. `sent`: request has been sent to Overgate.
5. `accepted`: Overgate accepted the command and returned a command or workload reference.
6. `duplicate_resolved`: idempotency replay returned the prior compatible result.
7. `retry_wait`: SDK is waiting before a safe retry.
8. `denied`: runtime service rejected the request with a stable reason code.
9. `failed`: request cannot continue because transport, schema, credential, or dependency state is terminal.
10. `completed`: read/query workflow returned a terminal successful result.

The SDK must never infer `accepted` or `completed` without a service response.

## Policy And Security

- Require explicit production configuration before using live endpoints.
- Deny unsigned mutating calls unless a test harness explicitly installs a local-only fake signer.
- Do not persist private keys, bearer tokens, seed phrases, or raw secret values.
- Redact request bodies, signatures, secrets, and private content from default logs.
- Treat expired, revoked, mismatched, or unknown credential ids as terminal client errors unless Overgate provides a retryable correction path.
- Validate tenant and actor fields before signing to avoid signing malformed commands.
- Keep local test fixtures visually and structurally separate from production config.
- Preserve central AI and operator auditability by surfacing evidence refs instead of hiding runtime decisions behind client abstractions.

## Metering And Accounting

The SDK is not an accounting authority. It should:

- Surface Overmeter usage objects, ORU charge previews, Seal Ledger refs, and dispute refs returned by runtime services.
- Attach caller app id, actor id, tenant id, and trace id so runtime services can meter accurately.
- Avoid direct payment calls for platform operations.
- Provide typed readers that make usage and receipt evidence understandable to native apps and mobile clients.
- Keep usage helpers structural and testable, without embedding charge tables or economic assumptions.

## Observability And Operations

The SDK should provide:

- Configurable structured logging with redaction defaults.
- Trace id propagation across retries and follow-up status calls.
- Client-side metrics for request count, latency, retry count, denial reason codes, duplicate resolutions, and validation failures.
- Debug mode that can print canonicalization inputs without exposing secrets.
- Contract-test harnesses that run against local development stack and synthetic control-plane flow.
- Version reporting so services can detect unsupported clients and return stable upgrade guidance.

## Failure Modes And Recovery

- Missing credential reference: fail before network submission with a local configuration reason code.
- Invalid command schema: fail before signing.
- Signing failure: fail before network submission and do not retry automatically.
- Network timeout before response: retry only when an idempotency key and matching request hash exist.
- Duplicate idempotency key with different request hash: surface terminal conflict.
- Runtime denial: return the service reason code, trace id, policy refs, and correction fields.
- Revoked credential response: stop retries and require credential refresh.
- Unsupported schema version: surface upgrade guidance and do not silently downgrade unless an explicit compatibility map exists.

## Validation Plan

The service implementation plan lists these specific validation requirements:

- SDK can submit a signed workload without manual internal API calls.
- Duplicate idempotency behavior is handled predictably.
- SDK error objects preserve reason codes and trace ids.

Additional SDS-level validation:

- Contract tests prove generated clients match shared schemas.
- Golden tests verify canonical request signing input is stable.
- Idempotency tests cover successful replay, retry after timeout, and conflicting request hashes.
- Redaction tests prove secrets and private payloads do not appear in logs.
- Fixture tests show test signers cannot be enabled in production configuration.
- Cross-language tests must pass the same request-envelope, error-decoding, and manifest-validation fixtures before any second language binding is released.

## Build Breakdown

1. Generate the first typed client from shared schemas and approved API descriptions.
2. Add request context, command envelope, trace id, idempotency key, and stable error helpers.
3. Add signing support through credential-provider interfaces and Overkey-compatible metadata.
4. Add manifest validation and workload submission helpers.
5. Add status, result, policy dry-run, usage, receipt, and dispute-reference readers.
6. Add local development and integration-test fixtures.
7. Add native app, adapter, and mobile convenience modules after [Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md).

The first implementation should stay small enough to run against the seed hardware control plane while preserving the path to grid-resident services.

## Handoff And Downstream Use

This is the client foundation for Docdex, Mcoda, Codali, native apps, mobile services, adapters, CLI, and admin tooling.

Downstream teams should integrate through SDK primitives instead of building private shortcuts to control-plane services. If a caller needs a new helper, update this SDS, the implementation plan, and the shared schema/API contract together.

## Open Design Questions

Resolved decisions:

- The first SDK binding should be the generated Rust SDK because the accepted stack is Rust-first and Phase 1 needs Rust control-plane, CLI, test-harness, and service code to share the same request, signing, idempotency, error, and schema-validation behavior. TypeScript/web bindings should be the next generated target for UI, adapter, and browser-facing client surfaces once the shared schema package can generate stable models and validators. Swift/iOS, Kotlin/Android, Python, and any other bindings remain blocked until schema compatibility checks, golden request-envelope fixtures, generated error objects, and cross-language contract tests are stable. No binding may maintain handwritten public objects that drift from `packages/schemas`.
- The SDK should expose credential-provider interfaces, not key storage. Phase 1 requires interfaces for file-backed local/test credentials, host signing-agent sockets, platform keychains where available, hardware-backed signing devices or secure enclaves, and Overkey credential refs. Overvault-backed secret refs are reference-only in the general SDK until Phase 8 introduces Overvault; before then the SDK may pass declared secret refs and mount intents but must not fetch, decrypt, persist, or emulate vault material. All providers return credential ids, public metadata, signing capability, rotation/revocation hints, and redaction class; raw private keys, bearer tokens, seed phrases, and vault values never enter SDK durable state.
- Local idempotency cache retention should be bounded by command class and safe to discard. Read-only calls need no idempotency cache. Phase 1 mutating control-plane commands should keep successful terminal digests for 24 hours and in-flight retry records for the shorter of the command deadline or 2 hours. Long-running workload submissions should retain request-hash and accepted command refs for 7 days or until the workload reaches a terminal state plus 24 hours. Local/test fixture commands may be cleared on `dev reset`. Security-sensitive credential, tenant, and key-rotation commands should keep only request hash, trace id, command ref, and terminal digest for 24 hours by default, with no raw payload retention. Accounting, receipt, dispute, and offline/mobile classes may define longer service-returned replay windows later, but the SDK must treat service idempotency state as authoritative.
- Mobile-specific behavior should be packaged separately as the Phase 12 Mobile SDK, while reusing the foundation SDK's generated models, signing, error, trace, and idempotency primitives. The foundation SDK should not embed mobile session management, push registration, OS background behavior, secure local storage policy, media upload state, sync cursors, or offline command queues as default general-client behavior. It should provide extension points and shared fixtures so the Mobile SDK and Mobile Backend Gateway can implement bounded offline queues with expiry, request hashes, session refresh, conflict states, redacted diagnostics, and OS-specific secure-storage adapters.
- Services should maintain a small explicit compatibility window for SDK versions once external native apps depend on them: support the current stable SDK major version and one previous stable major version for non-security-breaking public API behavior, with deprecation warnings and upgrade guidance returned through Overgate or gateway capability profiles. Additive schema changes may be accepted within the same major version. Breaking command-envelope, signing, tenant, policy, secret-ref, privacy, or accounting changes require a new major version, compatibility metadata, migration guidance, and contract tests. Security-critical breaks may force a shorter emergency window, but services must return stable `unsupported_sdk_version` or `schema_version_unsupported` errors rather than silently downgrading or accepting unsafe payloads.
