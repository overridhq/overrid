SDS #8

# Overgate SDS

## Purpose

Build the API ingress layer for authentication, request signing, idempotency, rate limiting, quota prechecks, command validation, and ingress audit.

Overgate is the required control-plane entry point for external callers and service-to-service mutating commands. It accepts or denies requests before side effects happen, then forwards accepted commands to the correct downstream service or queue with a preserved trace and audit trail.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [overgate.md](../../service_catalog/control_plane/overgate.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md) |
| Sub-build plan | [SUB BUILD PLAN #8 - Overgate](../../build_plan/sub_build_plan_008_overgate.md) |

## Service Family

- Family: Control plane.
- Owning layer: API ingress, request admission, and command dispatch.
- Primary data scope: ingress request records, idempotency records, rate-limit counters, admission decisions, forwarding records, and audit refs.
- First build phase from service plan: [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md).

## Problem Statement

Overrid must prevent every native app, SDK, CLI, adapter, node agent, and service account from reaching internal services directly. Without a strict ingress service, signatures can be checked inconsistently, duplicate commands can create side effects, tenants can be confused, and audit evidence can be lost. Overgate provides the single admission path needed for seed-hardware development now and grid-resident operation later.

## Goals

- Authenticate every caller and resolve actor identity through Overpass and Overkey.
- Verify tenant context and role or service-account permission before forwarding.
- Validate the command envelope and payload against the shared schema package.
- Enforce idempotency for mutating commands before downstream side effects.
- Assign or validate trace ids and propagate them across services.
- Apply rate limits and quota prechecks using local counters and later Overmeter/ORU state.
- Emit Overwatch-compatible audit evidence for accepted and denied mutating commands.
- Forward accepted commands through explicit downstream APIs or queues, never by direct storage writes.

## Non-Goals

- Do not own business decisions for scheduling, storage, identity lifecycle, key lifecycle, accounting, or native-service behavior.
- Do not replace Overguard policy decisions; Overgate only enforces ingress and calls policy checks where required.
- Do not store private keys, user content, raw secrets, or full private payloads when references or hashes are enough.
- Do not silently retry non-idempotent requests.
- Do not bypass Overgate for admin, internal, or development flows unless the local integration harness explicitly isolates the shortcut.
- Do not encode charge tables, customer-count assumptions, or economic projections.

## Primary Actors And Clients

- SDK and CLI clients.
- Admin and developer UI.
- Native apps and mobile clients.
- External adapters and integration services.
- Node agents and workers submitting control-plane callbacks.
- Service accounts and system services.
- Operator tools using signed administrative commands.

## Dependencies

- Overpass for actor identity, identity state, and namespace refs.
- Overtenant for tenant state, membership, app ownership, and role bindings.
- Overkey-lite for API keys, public signing keys, service-account credentials, revocation state, and rotation metadata.
- Shared schema package for command envelope, API error, audit event, usage precheck, and downstream command schemas.
- Overwatch event log for ingress audit evidence.
- Overqueue for asynchronous forwarding once queue-backed flows exist.
- Overmeter, ORU, and Seal Ledger for later quota and accounting prechecks.
- Overguard for policy dry-run or admission decisions once policy services exist.

## Owned Responsibilities

Overgate owns the request admission boundary:

- Public and internal ingress endpoints.
- Request canonicalization and signature verification orchestration.
- Actor and tenant resolution for each command.
- Schema validation before side effects.
- Idempotency key reservation, replay, conflict detection, and retention.
- Rate-limit and quota-precheck records.
- Forwarding records for accepted commands.
- Stable denial responses with reason codes.
- Ingress audit events and trace propagation.

Downstream services own their domain state. Overgate must not write their private records directly.

## Data Model

The first implementation should define these records:

- `ingress_request`: request id, method, path, command type, body hash, schema version, actor id, tenant id, trace id, idempotency key, credential id, source app, client version, and received time.
- `signature_check`: credential id, key version, canonicalization version, signature algorithm, verified state, failure reason, and replay-window result.
- `actor_resolution`: actor id, actor type, identity state, tenant membership refs, service-account refs, and resolution reason.
- `admission_decision`: accepted or denied state, reason code, dependency checks, policy refs, quota refs, rate-limit bucket refs, and audit refs.
- `idempotency_record`: tenant id, actor id, command type, idempotency key, request hash, first trace id, current state, response digest, expiration time, and conflict reason.
- `rate_limit_bucket`: bucket id, actor or tenant scope, command class, window, capacity, consumed count, reset time, and denial reason.
- `forwarding_record`: downstream service, queue ref or API route, command ref, delivery attempt, delivery state, response ref, retry time, and terminal reason.

Private request bodies should be stored only when explicitly required by a service contract; otherwise Overgate should persist hashes and references.

## API Surface

Phase 1 should expose a minimal but real ingress surface:

- `POST /v1/commands`: submit a signed command envelope.
- `GET /v1/commands/{command_id}`: read admission and forwarding status visible to the caller.
- `GET /v1/traces/{trace_id}`: read caller-visible trace summary and audit refs.
- `GET /v1/limits`: read current caller-visible rate-limit and quota-precheck state.
- `POST /v1/policy/dry-run`: call policy checks without mutating runtime state once Overguard exists.
- `GET /v1/healthz`: liveness without dependency authority claims.
- `GET /v1/readyz`: readiness based on schema package, Overpass, Overtenant, Overkey, Overwatch, and forwarding targets.

Admin endpoints must require signed operator or service-account credentials and produce audit events:

- `GET /v1/admin/ingress/{request_id}`.
- `GET /v1/admin/idempotency/{tenant_id}/{idempotency_key}`.
- `POST /v1/admin/idempotency/{record_id}/expire`.
- `GET /v1/admin/rate-limits`.

## Event Surface

Overgate should emit these Overwatch-compatible events:

- `overgate.request_received`: request parsed far enough to assign or read trace id.
- `overgate.signature_verified`: credential and signature check passed.
- `overgate.signature_denied`: signature, credential, replay window, or revocation check failed.
- `overgate.schema_denied`: command envelope or payload failed strict validation.
- `overgate.tenant_denied`: tenant, membership, role, or app ownership check failed.
- `overgate.idempotency_reserved`: mutating command reserved an idempotency key.
- `overgate.idempotency_replayed`: duplicate compatible request returned prior result.
- `overgate.idempotency_conflict`: duplicate key had a different request hash.
- `overgate.rate_limited`: rate-limit or quota precheck denied the command.
- `overgate.command_accepted`: request passed ingress checks.
- `overgate.command_forwarded`: accepted command was delivered to downstream API or queue.
- `overgate.forwarding_failed`: downstream dispatch failed after admission.

Events must avoid raw secrets and private payloads. Use refs and hashes unless the audit contract explicitly requires more.

## Core Workflow

1. Receive request and extract or create trace id.
2. Parse command envelope enough to identify command type, tenant id, actor id, idempotency key, credential id, and schema version.
3. Canonicalize the request and ask Overkey to verify the credential and signature.
4. Resolve actor state in Overpass and tenant membership or service-account permission in Overtenant.
5. Validate the envelope and payload through the shared schema package.
6. Reserve or replay the idempotency key for mutating commands.
7. Apply rate-limit and quota prechecks.
8. Call Overguard policy checks where the command class requires it.
9. Persist admission decision and emit audit event.
10. Forward accepted commands to the downstream service or Overqueue.
11. Return a typed response with trace id, request id, command ref, audit refs, and reason code when denied.

## State Machine

Overgate request state:

1. `received`: request entered Overgate and has a request id.
2. `parsed`: minimal command envelope fields were read.
3. `signature_verified`: credential and signature are valid for the request.
4. `identity_resolved`: actor and tenant context are valid enough to continue.
5. `schema_validated`: command envelope and payload passed strict validation.
6. `idempotency_reserved`: mutating request reserved or matched an idempotency record.
7. `prechecked`: rate-limit, quota, and required policy prechecks passed.
8. `accepted`: Overgate admitted the command and emitted audit evidence.
9. `forwarded`: command was delivered to target service or queue.
10. `denied`: request ended before side effects with a stable reason code.
11. `failed_after_acceptance`: dispatch failed after admission and must be retried or surfaced by status.
12. `completed`: downstream service returned a terminal success visible through status.

No transition may skip audit evidence for mutating commands.

## Policy And Security

- Default deny when identity, tenant, credential, schema, quota, policy, or dependency state is missing.
- Signature verification must occur before idempotency replay returns sensitive data.
- Idempotency records must be scoped by tenant, actor or service account, command type, and request hash.
- Admin operations require operator or system-service identity and must be audited.
- Development fixtures must be isolated by environment and impossible to enable accidentally in production.
- Rate-limit and quota denials should use stable reason codes so SDKs and native apps can respond predictably.
- Overgate may store body hashes and refs, but private payload storage requires an explicit schema and retention rule.
- Service-to-service calls must use service accounts and signed commands; no hardcoded dev secrets.

## Metering And Accounting

Overgate does not settle usage. It should:

- Emit usage-relevant ingress events for request volume, command class, tenant, actor, app, and downstream target.
- Apply quota prechecks using current Overmeter/ORU information once available.
- Surface charge preview or quota denial refs returned by accounting services.
- Avoid direct payment or ledger mutation in ingress code.
- Preserve low-friction internal accounting by attaching the refs needed for ORU and Seal Ledger services to process accepted commands later.

## Observability And Operations

Overgate should expose:

- Health for local storage and process liveness.
- Readiness for schema package, Overpass, Overtenant, Overkey, Overwatch, and forwarding dependencies.
- Metrics for requests, denials by reason, accepted commands, idempotency replays, conflicts, rate-limit denials, forwarding latency, and downstream failures.
- Trace queries for operator debugging with tenant and role filtering.
- Dead-letter or retry views for accepted commands that failed dispatch.
- Audit search hooks through Overwatch.
- Safe migration behavior for idempotency record retention and admission-decision schema changes.

## Failure Modes And Recovery

- Malformed envelope: deny before signature-dependent or downstream work.
- Unknown credential: deny before idempotency replay.
- Revoked or rotated credential: deny and return credential reason code.
- Suspended actor or tenant: deny before side effects.
- Duplicate idempotency key with same request hash: return prior compatible response.
- Duplicate idempotency key with different request hash: deny with conflict reason.
- Overwatch unavailable for mutating command: fail closed unless a documented emergency buffer is active.
- Downstream unavailable after acceptance: preserve forwarding record and retry through Overqueue or expose blocked status.
- Rate-limit store unavailable: fail closed for high-risk command classes and use documented degraded rules for low-risk reads.

## Validation Plan

The service implementation plan lists these specific validation requirements:

- Invalid signatures are denied before side effects.
- Duplicate idempotency keys behave deterministically.
- Every accepted or denied mutating command emits an audit record.

Additional SDS-level validation:

- Contract tests cover command submission, status, trace, limits, and admin idempotency APIs.
- Signature tests cover valid, malformed, expired, revoked, rotated, and wrong-tenant credentials.
- Tenant isolation tests prove callers cannot read another tenant's trace or idempotency records.
- Idempotency tests cover replay, conflict, timeout, failed-after-acceptance, and retention expiration.
- Audit tests prove accepted and denied mutating commands create Overwatch-compatible events.
- Dependency-failure tests prove fail-closed behavior for identity, key, schema, and audit dependencies.

## Build Breakdown

1. Implement the command ingress endpoint, shared error shape, and request/trace/idempotency primitives.
2. Integrate Overkey-lite signature verification and credential revocation state.
3. Integrate Overpass actor resolution and Overtenant membership checks.
4. Attach shared schema validation for command envelopes and payloads.
5. Add idempotency reservation, replay, conflict detection, and retention.
6. Add local rate limits and quota-precheck placeholders.
7. Emit ingress audit events before forwarding accepted commands.
8. Forward accepted commands to the Phase 1 synthetic control-plane flow, then to Overqueue when available.
9. Add admin/debug endpoints after the public path is covered by tests.

The implementation should remain small enough for the initial seed servers while enforcing the same boundary the later grid will use.

## Handoff And Downstream Use

Overgate is the required entry point for SDK, CLI, admin UI, adapters, native apps, mobile services, node agents, and service accounts.

Downstream services should receive accepted commands from Overgate or Overqueue, not direct external calls. If a downstream service needs a new command type, update the shared schema, this SDS, that service SDS, and the build-plan crosswalk together.

## Open Design Questions

Resolved decisions:

- Phase 1 synchronous work is limited to ingress admission and small control-plane mutations that can complete before the response while preserving the audit chain: health and readiness checks, signed command admission, idempotency reserve or replay, caller-visible command status, trace summaries, limit views, tenant/identity/key/manifest creation or update, and the synthetic control-plane command path. Workload submission, execution requests, provider or node callbacks, package/artifact handoffs, policy-heavy commands, accounting-affecting commands, storage operations, native-app side effects, retries, and any command that waits on another runtime service should be admitted synchronously and then represented as durable native Overqueue work.
- Idempotency retention is classed by risk and side-effect horizon. Bodyless read requests do not need idempotency records beyond optional trace/status cache retention of up to 24 hours. Low-risk Phase 1 metadata writes keep records for at least 24 hours. Tenant, identity, credential, manifest, admin, and control-plane mutation commands keep records for at least 7 days. Queue-producing workload commands keep records until terminal queue state plus 7 days, with a 30-day cap unless a dispute, retry, or incident ref extends retention. Later accounting, rights, ledger, payout, namespace ownership, credential recovery, and policy-enforcement commands must keep idempotency refs for the owning service's audit/finality window and at least 90 days.
- Low-risk bodyless reads may avoid signed request bodies only when they still pass through Overgate and do not expose private tenant data. Public `/v1/healthz`, redacted `/v1/readyz`, public schema/version/capability metadata, and unauthenticated docs-facing discovery can be unsigned. Caller-specific command status, trace summaries, and limit views may omit a body signature because they are GET requests, but they still require authenticated tenant-scoped credentials, stable trace ids, and role filtering. Admin reads, private traces, idempotency lookups, tenant data, secret refs, audit evidence, quota state, and any mutating request require signed credentials and the normal Overgate admission checks.
- The acceptable emergency audit buffer is a Rust-owned, local, append-only, Overwatch-compatible WAL controlled by Overgate, not Redis, Kafka, NATS, or an external log product. It is disabled by default outside explicitly configured degraded mode, must be bounded by time and size, must fsync each event envelope before any admitted side effect, must hash-chain buffered events, must store only refs/hashes/redacted fields, and must keep readiness degraded until replay succeeds. High-risk commands such as credential revocation, tenant suspension, ledger/accounting, rights transfer, secret access, policy override, and admin break-glass operations fail closed while Overwatch is unavailable; only allowlisted low-risk Phase 1 mutations may use the buffer.
- Before ORU and Seal Ledger are online, quota prechecks require enough data to make a conservative admission decision without pretending to settle usage: tenant id, actor or service-account id, tenant state, role or permission refs, command class, app/source ref, request size class, rate-limit bucket, quota scope, configured allowance or grant refs from Overtenant/Overgrant placeholders, current local counter snapshots, optional Overmeter usage snapshot refs when available, and stable reason codes. Overgate should attach quota-precheck refs to accepted commands for later Overmeter, ORU, and Seal Ledger processing, but it must not mutate balances, finalize charges, or create ledger entries itself.
