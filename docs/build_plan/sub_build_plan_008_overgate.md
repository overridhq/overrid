# SUB BUILD PLAN #8 - Overgate

Attached SDS: [docs/sds/control_plane/overgate.md](../sds/control_plane/overgate.md)

## Purpose

This sub-build plan turns SDS #8 into an implementation sequence for Overgate. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Overgate is the required API ingress and admission boundary for external callers and service-to-service mutating commands. It authenticates callers, verifies signatures, resolves tenant context, validates command envelopes, reserves idempotency, applies rate-limit and quota prechecks, emits audit evidence, and forwards accepted commands without taking ownership of downstream service state.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #8: Overgate](../sds/control_plane/overgate.md) | Controls Overgate purpose, API surface, state machine, owned responsibilities, validation, failure behavior, and resolved open-question decisions. |
| [Overgate service plan](../service_catalog/control_plane/overgate.md) | Controls service-catalog objective, first build phase, dependencies, development order, contracts, validation, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical build order from master Phase 0 through master Phase 13. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Controls the first build point for Overgate as the signed, tenant-aware, auditable command ingress. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies later Overguard policy checks, policy dry-run semantics, sensitivity classes, and replayable denial evidence. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies later Overmeter, ORU, Seal Ledger, Overgrant, and accounting precheck inputs without moving settlement into Overgate. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Supplies SDK, CLI, admin UI, Docdex, Mcoda, Codali, adapter, and product-client hardening requirements. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies system-service workload, failover, degraded-mode, backup, restore, and grid-resident operation requirements. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #8 first build work aligned to master Phase 1, with later hardening through policy, metering, products, and grid-resident operation. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first control-plane services, Axum/Tower/Hyper-style HTTP, signed command envelopes, native Overqueue and Overwatch boundaries, and no conventional cloud product boundary drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phase 1 | Attach SDS #8 to the build-plan layer, freeze Overgate as ingress authority, and preserve resolved admission-boundary decisions. |
| 2 | Master Phase 1 | Build the Rust service skeleton, route surface, dependency readiness checks, and local-stack integration. |
| 3 | Master Phase 0 and Phase 1 | Implement shared command envelope parsing, schema validation, canonicalization inputs, stable API errors, and strict payload boundaries. |
| 4 | Master Phase 1 | Integrate Overkey-lite, Overpass, and Overtenant for credential, actor, tenant, role, and service-account admission. |
| 5 | Master Phase 1 | Implement idempotency reservation, replay, conflict detection, trace/status views, and classed retention windows. |
| 6 | Master Phases 1, 4, and 5 | Add rate limits, conservative quota prechecks, and Overguard policy dry-run handoff without settlement or policy ownership. |
| 7 | Master Phases 1 and 7 | Emit Overwatch-compatible audit, metrics, traces, degraded audit-buffer rules, and grid-ready operations evidence. |
| 8 | Master Phases 1, 3, and 6 | Forward accepted commands to internal handlers and Overqueue, then harden product-client flows without direct downstream state writes. |
| 9 | Master Phases 1, 6, and 7 | Add tenant-isolated admin/debug views, product visibility, operator controls, and grid-resident readiness behavior. |
| 10 | Master Phase 1 through Phase 13 | Validate structure, tech-stack alignment, documentation links, queue state, and downstream handoff rules. |

## Tech Stack Guardrails

- Overgate remains one of the Rust-first control-plane services. Overgate core is a Rust service using Axum/Tower/Hyper-style HTTP and Tokio, with rustls/mTLS where transport bootstrap requires it.
- Command envelopes, API errors, audit records, idempotency records, reason codes, and fixtures use canonical JSON plus JSON Schema from the shared schema package.
- Rust validation and service code are authoritative for the core ingress path; TypeScript/web code may consume generated bindings for SDK, UI, and browser-facing clients only after the Rust contract is stable.
- Overgate persists ingress, idempotency, forwarding, and admission records through Overrid-owned abstractions or Overrid-shaped local stubs during early phases. It must not make PostgreSQL, Redis, Kafka, NATS, S3, MinIO, Vault, or similar products the platform boundary.
- Overgate calls Overkey, Overpass, Overtenant, Overwatch, Overqueue, Overguard, Overmeter, ORU, Seal Ledger, and downstream services through explicit service contracts. It must not write their private records directly.
- Overgate may store request hashes, payload refs, audit refs, quota refs, and redacted diagnostics. It must not persist raw secrets, private keys, raw private payloads, or user content unless an explicit service contract and retention rule requires it.
- Overgate must keep ORU, Seal Ledger, Overgrant, and Overasset references structural. It must not implement blockchain, NFT mechanics, speculative token behavior, pricing tables, revenue projections, or customer-count assumptions.

## Phase 1: SDS Attachment, Ingress Boundary, And Authority Rules

### Work Items

- **1.1 Attach the build plan to SDS #8.**
  - Design: Link this document from the numbered Overgate SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/control_plane/overgate.md`, `docs/service_catalog/control_plane/overgate.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #8 returns both the Overgate SDS and this sub-build plan.

- **1.2 Freeze Overgate as the admission boundary.**
  - Design: Record that every SDK, CLI, admin UI, native app, mobile client, adapter, node agent, service account, and operator tool must pass through Overgate or an explicitly isolated local test shortcut before mutating control-plane state.
  - Output: Ingress-boundary checklist for implementation reviews.
  - Validation: Architecture review confirms no work item allows direct external writes to Overpass, Overtenant, Overkey, Overregistry, Overwatch, Overqueue, accounting, storage, or native-app state.

- **1.3 Preserve master Phase 1 as the first build point.**
  - Design: Keep first implementation in master Phase 1 because Overgate depends on Phase 0 schemas/local harness and is part of the minimum signed control-plane path.
  - Output: Phase-gate note that SDS #8 starts in Phase 1 and only expands later through policy, accounting, product, and grid-resident gates.
  - Validation: Review proves this plan does not move Overgate into Phase 0 or change the master Phase 0 through Phase 13 order.

- **1.4 Carry forward resolved SDS decisions.**
  - Design: Preserve the SDS decisions for synchronous Phase 1 commands, classed idempotency retention, unsigned low-risk reads, the Rust-owned emergency audit WAL, and conservative pre-ORU quota precheck inputs.
  - Output: Resolved-decision checklist tied to SDS #8 open-question answers.
  - Validation: Review proves the plan does not reopen settled decisions or replace them with weaker generic gateway behavior.

- **1.5 Define runtime authority boundaries.**
  - Design: Require Overgate to own admission records, idempotency records, forwarding records, rate-limit buckets, quota-precheck refs, and ingress audit events while leaving domain state with downstream services.
  - Output: Ownership matrix for Overgate-owned records versus downstream service-owned records.
  - Validation: Design review rejects direct downstream storage writes and rejects Overgate-owned settlement, policy finality, identity lifecycle, key lifecycle, or native-service business state.

### Phase 1 Gate Outputs

#### Link Attachment Matrix

| Artifact | Required link target | Phase 1 state |
| --- | --- | --- |
| `docs/sds/control_plane/overgate.md` | `docs/build_plan/sub_build_plan_008_overgate.md` | `attached` |
| `docs/service_catalog/control_plane/overgate.md` | `docs/build_plan/sub_build_plan_008_overgate.md` | `attached` |
| `docs/build_plan/master_plan.md` | SDS #8 per-SDS sub-build plan row | `attached` |
| `docs/build_plan/service_catalog_alignment.md` | SDS #8 crosswalk row | `attached` |
| `docs/overrid_tech_stack_choice.md` | Rust-first Overgate control-plane guardrails | `attached` |
| `docs/planning/overgate_phase_01_plan.md` | Phase 1 execution plan | `attached` |
| `docs/planning/overgate_phase_01_progress.md` | Phase 1 progress and evidence trail | `attached` |

#### Frozen Ingress Boundary

Overgate is frozen as the required API ingress and admission boundary for every mutating SDK, CLI, admin UI, native app, mobile client, adapter, node agent, service-account, and operator-tool path. The only exception is an explicitly isolated local test shortcut owned by the integration harness, with deterministic fixtures and no production credentials.

| Boundary rule | Phase 1 state |
| --- | --- |
| Mutating external and service-to-service commands enter through Overgate before downstream side effects. | `ingress_boundary_frozen` |
| Direct external writes to Overpass, Overtenant, Overkey, Overregistry, Overwatch, Overqueue, accounting, storage, or native-app state are forbidden. | `forbidden_in_overgate` |
| Local test shortcuts must remain isolated fixtures, not runtime service bypasses. | `queue_backed_later` |
| Overgate forwards accepted commands through explicit service contracts or native Overqueue handoff instead of owning downstream domain state. | `downstream_owned` |

#### Master Phase Gate Matrix

| Master phase | Overgate authority rule |
| --- | --- |
| Phase 0 | Supplies shared schemas, local stack, deterministic fixtures, API/event conventions, and integration harness prerequisites only. |
| Phase 1 | First build point: signed, tenant-aware, auditable command ingress and admission records. State: `master_phase_1_owned`. |
| Phase 4 | Adds Overguard policy dry-run and admission handoff without moving policy finality into Overgate. |
| Phase 5 | Adds Overmeter/ORU/Seal Ledger quota and accounting refs without moving settlement into Overgate. |
| Phase 6 | Hardens SDK, CLI, admin UI, native-app, mobile, and adapter client flows through Overgate. |
| Phase 7 | Prepares grid-resident operation, degraded audit buffering, failover, backup, and restore evidence. |

#### Resolved SDS Decision Checklist

These rows are `resolved_decision_carried` from SDS #8 and must not be weakened by generic gateway behavior.

| Resolved SDS decision | Phase 1 carry-forward rule |
| --- | --- |
| Synchronous Phase 1 command scope | Keep synchronous work limited to ingress admission and small control-plane mutations; queue runtime-service waits and workload-producing commands. |
| Classed idempotency retention | Preserve bodyless-read, low-risk metadata, tenant/identity/credential/manifest/admin, queue-producing, and later accounting/rights retention classes. |
| Unsigned low-risk bodyless reads | Permit only public health/readiness/schema/version/capability discovery and tenant-authenticated bodyless GETs that expose no private tenant data. |
| Rust-owned emergency audit WAL | Use only a bounded, local, append-only, hash-chained, Overwatch-compatible WAL in explicit degraded mode; do not substitute Redis, Kafka, NATS, or external log products. |
| Conservative pre-ORU quota precheck refs | Attach quota-precheck refs and stable reason codes without mutating balances, finalizing charges, or creating ledger entries. |

#### Runtime Authority Ownership Matrix

| Record or decision type | Owner | Phase 1 state |
| --- | --- | --- |
| Admission records, request hashes, dependency checks, and denial reason codes | Overgate | `overgate_owned` |
| Idempotency records, replay metadata, conflict reasons, and retention class refs | Overgate | `overgate_owned` |
| Forwarding records, downstream target refs, retry refs, and status refs | Overgate until handoff, then target service or Overqueue | `overgate_owned` |
| Rate-limit buckets, quota-precheck refs, and local allowance snapshots | Overgate precheck; accounting services own final settlement | `overgate_owned` |
| Ingress audit events | Overgate emits; Overwatch stores append-only evidence | `downstream_owned` |
| Tenant, identity, key, manifest, queue, policy, accounting, storage, native-app domain state, and other downstream domain state | Owning downstream services | `downstream_owned` |
| Settlement, policy finality, identity lifecycle, key lifecycle, private storage mutation, and native-service business state | Not Overgate | `forbidden_in_overgate` |

#### Documentation Update Rule

Any new Overgate command type, authority exception, idempotency class, quota-precheck input, forwarding target, admin route, degraded audit mode, or unsigned-read rule must update the shared schema package, the Overgate SDS, this sub-build plan, the owning downstream service SDS/service plan, and the build-plan crosswalk before implementation code treats it as available.

## Phase 2: Rust Service Skeleton, Routes, And Dependency Readiness

### Work Items

- **2.1 Create the Overgate Rust service crate.**
  - Design: Add an Overgate service crate under the control-plane workspace using Tokio and Axum/Tower/Hyper-style HTTP, shared config loading, tracing setup, and testable handler boundaries.
  - Output: Service crate, module layout, dependency injection shape, and local-stack service entrypoint.
  - Validation: Compile and service-start checks pass once implementation exists; crate layout review confirms Overgate stays separate from downstream service internals.

- **2.2 Implement public ingress route skeletons.**
  - Design: Add route handlers for `POST /v1/commands`, `GET /v1/commands/{command_id}`, `GET /v1/traces/{trace_id}`, `GET /v1/limits`, `POST /v1/policy/dry-run`, `GET /v1/healthz`, and `GET /v1/readyz`.
  - Output: Typed route map with placeholder handlers that return stable schema-shaped responses.
  - Validation: Route tests confirm method/path registration, content type behavior, trace propagation, and no accidental admin exposure.

- **2.3 Implement admin route skeletons.**
  - Design: Add signed-operator-only route shells for ingress request lookup, idempotency lookup, idempotency expiration, and rate-limit views.
  - Output: Admin route module with authorization guard placeholder and audit hook placeholders.
  - Validation: Tests prove admin routes deny unauthenticated, non-operator, cross-tenant, and unsigned requests before returning data.

- **2.4 Implement dependency clients and readiness model.**
  - Design: Define clients for schema validation, Overkey-lite, Overpass, Overtenant, Overwatch, Overqueue, forwarding targets, and later Overguard/Overmeter/ORU refs.
  - Output: Dependency trait interfaces, local-stack adapters, readiness dependency matrix, and degraded-state reason codes.
  - Validation: Readiness tests distinguish process liveness from dependency authority and fail readiness when required admission dependencies are unavailable.

- **2.5 Connect local development and integration harness fixtures.**
  - Design: Wire Overgate into the loopback-only local stack and SDS #3 integration harness using deterministic tenants, actors, credentials, command envelopes, and audit fixtures.
  - Output: Local service config, fixture references, and harness scenario names.
  - Validation: Local smoke tests can submit a fixture command through Overgate without production credentials, real secrets, or non-Overrid product dependencies.

## Phase 3: Command Envelope, Canonicalization, Schema Validation, And Errors

### Work Items

- **3.1 Implement command-envelope parsing.**
  - Design: Parse command type, tenant id, actor id, trace id, idempotency key, credential id, schema version, payload type, request hash, and payload hash before any downstream side effect.
  - Output: Rust parser that produces a minimal admission context or stable parse-denial error.
  - Validation: Tests reject malformed, missing-field, wrong-version, wrong-content-type, oversized, and unsupported command envelopes with stable reason codes.

- **3.2 Implement canonicalization inputs.**
  - Design: Define canonical request components for signature verification, replay-window checks, request hashing, idempotency comparison, and audit evidence.
  - Output: Canonicalization module with version metadata and golden fixtures.
  - Validation: Golden tests prove canonicalization is deterministic and sensitive to method, path, tenant, actor, command type, idempotency key, body hash, timestamp, and credential metadata.

- **3.3 Attach shared schema validation.**
  - Design: Validate command envelopes and payloads through the shared schema package before idempotency reservation returns or forwarding occurs.
  - Output: Schema-validation adapter that maps validation failures to typed API errors.
  - Validation: Negative fixtures prove sensitive command objects reject unknown fields, missing privacy class, invalid refs, stale reason codes, and unsupported schema versions.

- **3.4 Implement stable API error responses.**
  - Design: Return errors with trace id, request id when available, reason code, retryability, correction hint, dependency name where relevant, and safe redacted diagnostics.
  - Output: API error builder shared across parse, signature, tenant, schema, idempotency, rate-limit, quota, policy, and forwarding failures.
  - Validation: Contract tests reject free-form error-only responses and verify private payloads, secrets, and credential material do not appear in error bodies.

- **3.5 Define body retention and redaction rules.**
  - Design: Store hashes and refs by default, and permit private request body retention only when a downstream contract explicitly names retention, privacy, and audit rules.
  - Output: Retention policy table for command classes and redaction metadata for diagnostics.
  - Validation: Sentinel-secret tests fail if private payloads or raw secrets appear in stored ingress records, logs, audit events, generated docs, or fixture reports.

## Phase 4: Credentials, Signatures, Actor Resolution, And Tenant Authorization

### Work Items

- **4.1 Integrate Overkey-lite signature verification.**
  - Design: Verify credential id, public signing key, key version, algorithm, canonicalization version, timestamp/replay window, revocation state, and rotation metadata before protected data is returned.
  - Output: Overkey client integration, signature-check record, and credential-denial reason-code mapping.
  - Validation: Signature tests cover valid, malformed, expired, replayed, revoked, rotated, wrong-tenant, wrong-key-version, and unknown-credential requests.

- **4.2 Integrate Overpass actor resolution.**
  - Design: Resolve people, organizations, apps, native services, node agents, service accounts, system services, and operator identities from signed request context.
  - Output: Actor-resolution record with actor state, type, refs, and failure reason.
  - Validation: Tests deny disabled, suspended, deleted-marker, unknown, wrong-type, and environment-mismatched actors before idempotency replay or downstream forwarding.

- **4.3 Integrate Overtenant role and membership checks.**
  - Design: Validate tenant state, membership, app ownership, delegated access, role bindings, quota scope, suspension state, and service-account permission.
  - Output: Tenant authorization module and role/permission reason-code table.
  - Validation: Tenant isolation tests prove callers cannot submit, inspect, replay, or expire another tenant's commands, traces, limits, idempotency records, or admin records.

- **4.4 Implement service-account and node-agent admission.**
  - Design: Support service-to-service commands and node-agent callbacks through signed service accounts with narrow command classes, scoped credentials, and explicit trace/audit requirements.
  - Output: Service-account admission policy and node-agent callback guard.
  - Validation: Tests reject hardcoded development secrets, broad service-account permissions, unsigned callbacks, wrong command classes, and missing audit context.

- **4.5 Add operator and admin command admission.**
  - Design: Require signed operator or system-service credentials for admin routes and break-glass operations, with stricter audit and fail-closed behavior.
  - Output: Operator guard, admin action envelope, and break-glass reason-code requirements.
  - Validation: Admin tests prove operator actions emit audit evidence, deny cross-tenant data access, and fail closed when Overwatch is unavailable.

## Phase 5: Idempotency, Trace Propagation, Status Views, And Retention

### Work Items

- **5.1 Implement idempotency reservation.**
  - Design: Reserve idempotency keys scoped by tenant, actor or service account, command type, request hash, credential context, and retention class before mutating side effects.
  - Output: Idempotency record model and reservation API.
  - Validation: Tests prove duplicate keys with the same request hash replay deterministically while duplicate keys with different request hashes deny with conflict reason.

- **5.2 Implement idempotency replay and response digests.**
  - Design: Store response digest, first trace id, current state, command ref, and safe replay metadata so compatible duplicates can return prior outcome without exposing private data.
  - Output: Replay handler and response-digest record.
  - Validation: Replay tests cover completed success, denied-before-side-effect, failed-after-acceptance, pending-forwarding, timeout, and retention-expired states.

- **5.3 Implement classed retention windows.**
  - Design: Apply SDS-defined retention windows for bodyless reads, low-risk metadata writes, tenant/identity/credential/manifest/admin/control-plane mutations, queue-producing workload commands, and later accounting/rights/ledger/namespace commands.
  - Output: Retention policy implementation and expiration job contract.
  - Validation: Retention tests prove records expire only after the allowed window and do not expire while dispute, retry, incident, or finality refs extend retention.

- **5.4 Implement trace propagation.**
  - Design: Assign or validate trace ids at ingress and propagate trace ids through admission, audit, forwarding, queue, downstream command refs, and caller-visible status.
  - Output: Trace context module and trace-summary response.
  - Validation: Tests prove every accepted or denied mutating command has trace id, request id, command ref when accepted, audit refs, and reason code when denied.

- **5.5 Implement caller-visible status APIs.**
  - Design: Expose command status, trace summary, and limit views with tenant/role filtering, redaction, and no private cross-tenant leakage.
  - Output: `GET /v1/commands/{command_id}`, `GET /v1/traces/{trace_id}`, and `GET /v1/limits` implementations.
  - Validation: Contract tests cover status visibility for pending, accepted, forwarded, denied, failed-after-acceptance, completed, expired, and unauthorized lookup cases.

## Phase 6: Rate Limits, Quota Prechecks, And Policy Handoff

### Work Items

- **6.1 Implement local rate-limit buckets.**
  - Design: Track actor, tenant, service-account, source-app, command-class, and environment-scoped buckets with deterministic window and reset behavior.
  - Output: Rate-limit bucket model, counter updates, and caller-visible limit response.
  - Validation: Tests prove exhausted buckets deny before side effects, reset correctly, and produce stable `overgate.rate_limited` audit evidence.

- **6.2 Implement conservative quota prechecks.**
  - Design: Use tenant id, actor/service account, tenant state, role refs, command class, source app, request size class, rate-limit bucket, quota scope, grant placeholders, local counters, optional Overmeter snapshot refs, and stable reason codes.
  - Output: Quota-precheck record and accepted-command quota refs.
  - Validation: Tests prove Overgate attaches quota refs to accepted commands but never mutates balances, finalizes charges, or creates Seal Ledger entries.

- **6.3 Add Overguard policy dry-run handoff.**
  - Design: Route `POST /v1/policy/dry-run` and policy-required command checks to Overguard once available, preserving policy version, matched rules, decision refs, and missing-prerequisite reasons.
  - Output: Policy-check client adapter and dry-run response shape.
  - Validation: Policy tests prove Overgate honors deny decisions before forwarding and does not become the policy engine or store policy truth.

- **6.4 Define command-class admission matrix.**
  - Design: Classify commands as low-risk read, Phase 1 synchronous control-plane mutation, queue-producing workload, policy-heavy, accounting-affecting, storage/namespace, native-app side effect, admin, or break-glass.
  - Output: Admission matrix mapping command class to required signing, idempotency, policy, quota, audit, forwarding, and fail-closed behavior.
  - Validation: Matrix tests reject command classes that lack required signature, tenant, idempotency, audit, or policy settings.

- **6.5 Implement budget and denial reason surfaces for clients.**
  - Design: Surface quota, budget, grant, rate-limit, and policy denial refs in stable error and limit responses so SDK, CLI, UI, and native apps can react predictably.
  - Output: Client-facing denial and correction response fields.
  - Validation: SDK/CLI contract tests prove clients do not parse free-form text to detect quota, rate-limit, policy, or budget errors.

## Phase 7: Audit, Observability, Degraded Mode, And Grid-Ready Operations

### Work Items

- **7.1 Emit Overwatch-compatible ingress events.**
  - Design: Emit request received, signature verified or denied, schema denied, tenant denied, idempotency reserved/replayed/conflicted, rate limited, command accepted, command forwarded, and forwarding failed events.
  - Output: Event builder, Overwatch client, and event-to-state transition map.
  - Validation: Audit tests prove accepted and denied mutating commands produce ordered Overwatch-compatible events with refs/hashes and no raw private payload leakage.

- **7.2 Implement fail-closed audit behavior.**
  - Design: Fail closed for mutating commands when Overwatch is unavailable unless an explicitly configured emergency buffer allows a low-risk Phase 1 mutation.
  - Output: Audit dependency guard and fail-closed reason codes.
  - Validation: Dependency-failure tests prove high-risk credential, tenant suspension, ledger/accounting, rights transfer, secret access, policy override, and admin break-glass commands fail closed.

- **7.3 Implement emergency audit WAL rules.**
  - Design: Build a Rust-owned, local, bounded, append-only, hash-chained, Overwatch-compatible WAL disabled by default outside degraded mode and storing only refs/hashes/redacted fields.
  - Output: Emergency WAL design and implementation gate for low-risk allowlisted commands.
  - Validation: WAL tests prove fsync-before-side-effect, time/size bounds, hash-chain verification, replay to Overwatch, degraded readiness until replay succeeds, and no Redis/Kafka/NATS/external log dependency.

- **7.4 Add operational metrics and traces.**
  - Design: Record requests, denials by reason, accepted commands, idempotency replays/conflicts, rate-limit denials, quota denials, policy denials, forwarding latency, dependency failures, and downstream failures.
  - Output: Rust tracing and OpenTelemetry-compatible metric hooks with Overwatch as authoritative audit evidence.
  - Validation: Metrics tests prove labels avoid private data, tenant leakage, secrets, raw payloads, and high-cardinality unbounded fields.

- **7.5 Prepare grid-resident operations behavior.**
  - Design: Define system-service workload class needs for Overgate, including readiness, maintenance mode, rolling update, rollback, break-glass controls, state backup, restore, and failover drills.
  - Output: Phase 7 operations checklist for Overgate.
  - Validation: Grid-readiness review confirms founder seed hardware can later be removed from the normal path without changing Overgate's public contract.

## Phase 8: Forwarding, Overqueue Dispatch, And Downstream Contract Boundaries

### Work Items

- **8.1 Implement Phase 1 synchronous forwarding.**
  - Design: Allow only SDS-approved small Phase 1 control-plane mutations to complete synchronously before response while preserving idempotency, audit, and trace chains.
  - Output: Synchronous forwarding adapter for identity, tenant, credential, manifest, trace/status, limit, and synthetic command flows.
  - Validation: Tests prove synchronous commands are narrow, audited, idempotent, and do not wait on execution, storage, accounting, native-app side effects, or another runtime service.

- **8.2 Implement Overqueue-backed dispatch.**
  - Design: Represent workload submission, execution requests, callbacks, package/artifact handoffs, policy-heavy commands, accounting-affecting commands, storage operations, native-app side effects, retries, and wait-on-service commands as durable native Overqueue work.
  - Output: Forwarding record, queue item creation, retry metadata, terminal reason, and status projection.
  - Validation: Queue tests prove accepted asynchronous commands become durable pending work with complete audit refs and idempotency state.

- **8.3 Implement downstream target registry.**
  - Design: Map command classes to target services, queue routes, API routes, required schema versions, permission requirements, policy requirements, and failover behavior without hardcoded storage writes.
  - Output: Forwarding target registry and validation rules.
  - Validation: Tests reject commands without registered target, schema version, owner service, retry behavior, audit mapping, or tenant isolation rules.

- **8.4 Handle forwarding failure after acceptance.**
  - Design: Preserve forwarding records and expose failed-after-acceptance status when downstream dispatch fails after admission, with retry through Overqueue where safe.
  - Output: Forwarding failure state machine and retry/dead-letter mapping.
  - Validation: Failure tests cover downstream unavailable, timeout, retryable failure, final failure, cancellation, dead-letter, status lookup, and audit evidence.

- **8.5 Harden product-client command flows.**
  - Design: Ensure SDK, CLI, admin UI, Docdex, Mcoda, Codali, adapters, native apps, mobile clients, node agents, and service accounts submit commands through Overgate instead of internal APIs.
  - Output: Product integration command checklist and contract fixtures for Phase 6.
  - Validation: Product integration tests fail when clients bypass signing, idempotency, trace ids, stable errors, audit refs, or Overgate forwarding.

## Phase 9: Admin Views, Tenant-Isolated Operations, And Client Ergonomics

### Work Items

- **9.1 Implement ingress request lookup.**
  - Design: Provide signed admin lookup for ingress request records with tenant and role filtering, redaction, dependency refs, and audit refs.
  - Output: `GET /v1/admin/ingress/{request_id}` implementation.
  - Validation: Admin lookup tests prove authorized operators can diagnose requests while tenant users cannot see cross-tenant private metadata.

- **9.2 Implement idempotency administration.**
  - Design: Provide signed admin lookup and controlled expiration for idempotency records, with retention rules, conflict visibility, and audit events.
  - Output: `GET /v1/admin/idempotency/{tenant_id}/{idempotency_key}` and `POST /v1/admin/idempotency/{record_id}/expire`.
  - Validation: Tests prove expiration refuses active, disputed, incident-linked, or finality-protected records and always emits audit evidence.

- **9.3 Implement rate-limit and quota operations views.**
  - Design: Expose scoped admin views for rate-limit buckets, quota-precheck refs, local counter snapshots, configured allowance/grant placeholders, and denial reason distribution.
  - Output: `GET /v1/admin/rate-limits` plus quota diagnostics visible by role.
  - Validation: Tests prove sensitive budget, grant, and tenant information is redacted or filtered by role.

- **9.4 Improve SDK/CLI and UI ergonomics.**
  - Design: Provide stable status, trace, limit, replay, conflict, quota, policy, and forwarding-failure response shapes that clients can render without internal API knowledge.
  - Output: Client contract examples and generated docs for common Overgate responses.
  - Validation: SDK/CLI/UI tests prove common denial and status cases decode from typed fields rather than free-form messages.

- **9.5 Define operator runbooks and incident hooks.**
  - Design: Document how to investigate signature denials, tenant denials, idempotency conflicts, audit dependency outages, forwarding failures, quota denials, policy denials, and degraded WAL replay.
  - Output: Operator runbook checklist tied to Overwatch events and Overgate admin views.
  - Validation: Incident drills prove operators can trace each failure from request to dependency, audit ref, forwarding record, and resolution status.

## Phase 10: Validation, Documentation Alignment, And Downstream Handoff

### Work Items

- **10.1 Validate sub-build-plan structure.**
  - Design: Check title prefix, attached SDS link, phase headings 1 through 10, work-item structure, Design/Output/Validation fields, and exit gate.
  - Output: Focused validation result for `SUB BUILD PLAN #8`.
  - Validation: Scripted text checks pass for required headings, numbered phases, local links, and work-item sections.

- **10.2 Validate tech-stack alignment.**
  - Design: Audit this plan and linked docs for Rust-first control-plane implementation, signed command envelopes, canonical JSON plus JSON Schema, native Overqueue/Overwatch boundaries, and no conventional cloud product-boundary drift.
  - Output: Tech-stack alignment checklist for Overgate.
  - Validation: Scans find no PostgreSQL, Redis, S3, MinIO, NATS, Kafka, Vault, blockchain, NFT, pricing, revenue, or customer-count assumptions except lines that explicitly reject those assumptions.

- **10.3 Validate master-plan alignment.**
  - Design: Confirm the master Phase 0 through Phase 13 order remains unchanged and SDS #8 is represented as a Phase 1 control-plane service with later hardening through policy, metering, product, and grid-resident phases.
  - Output: Updated master-plan and crosswalk rows for SDS #8.
  - Validation: Review confirms only per-SDS sub-build indexing changed; no master phase-order dependency was altered.

- **10.4 Validate service-catalog and SDS alignment.**
  - Design: Ensure SDS #8 and the Overgate service plan link back to this sub-build plan and preserve Overgate's ingress-only authority boundary.
  - Output: Updated source-document and sub-build-plan references.
  - Validation: Local Markdown link validation across changed docs returns no missing local targets.

- **10.5 Prepare downstream handoff rules.**
  - Design: Document how Overpass, Overtenant, Overkey, Overregistry, Overwatch, Overqueue, Overguard, Overmeter, ORU, Seal Ledger, Overpack, Overrun, SDK, CLI, admin UI, adapters, native apps, mobile clients, and grid-resident system services consume Overgate admission outputs.
  - Output: Downstream handoff checklist and owner-service expansion rules.
  - Validation: Handoff review confirms later services can depend on Overgate admission records, trace ids, audit refs, idempotency refs, quota refs, and forwarding records without moving their runtime authority into Overgate.

### Phase 10 Gate Outputs

| Artifact | Phase 10 output | Authority state |
| --- | --- | --- |
| [Structure validation](../../packages/overgate/handoff/phase10/structure_validation.valid.json) | Script-readable result for title prefix, attached SDS link, phase headings, work-item fields, exit gate, and local Markdown links. | `non_runtime_validation_artifact` |
| [Alignment checklist](../../packages/overgate/handoff/phase10/alignment_checklist.valid.json) | Script-readable checklist for Rust-first Overgate service boundaries, signed command envelopes, canonical JSON plus JSON Schema, native Overqueue/Overwatch handoff, master-plan order, SDS, service-catalog, and crosswalk alignment. | `non_runtime_validation_artifact` |
| [Downstream handoff rules](../../packages/overgate/handoff/phase10/downstream_handoff_rules.valid.json) | Owner-service handoff rules for admission records, trace ids, request ids, command refs, audit refs, idempotency refs, quota refs, policy refs, forwarding records, and client denial refs. | `owner_services_retain_runtime_authority` |
| [Phase 10 plan](../planning/overgate_phase_10_plan.md) | Implementation and validation plan for the Phase 10 closure slice. | `planning_only` |
| [Phase 10 progress](../planning/overgate_phase_10_progress.md) | Progress and validation evidence trail for the Phase 10 closure slice. | `planning_only` |
| [Overgate README](../../packages/overgate/README.md) | Operator-facing summary of Phase 10 artifacts and downstream handoff expectations. | `documentation_only` |

Phase 10 confirms that Overpass, Overtenant, Overkey, Overregistry, Overwatch, Overqueue, Overguard, Overmeter, ORU, Seal Ledger, Overpack, Overrun, SDK, CLI, admin UI, adapters, native apps, mobile clients, and grid-resident system services can consume Overgate outputs without moving runtime authority into Overgate.

## Alignment Review

- The sub-build plan keeps Overgate's first build point in master Phase 1, matching SDS #8, the service catalog entry, Phase 1 plan, master build plan, and build-plan crosswalk.
- The plan treats master Phase 0 as prerequisite work for schemas, local stack, fixtures, test harness, and shared API/event discipline, not as the Overgate implementation phase.
- The plan treats later phases as hardening or consumer gates: trust/policy in Phase 4, metering/accounting in Phase 5, product clients in Phase 6, grid-resident operations in Phase 7, storage/namespace command classes in Phase 8, deployment flows in Phase 9, federation/public-provider controls in Phases 10 and 11, native/mobile clients in Phase 12, and governance/compliance hardening in Phase 13.
- The plan carries forward SDS #8 resolved decisions for synchronous Phase 1 command scope, classed idempotency retention, unsigned low-risk read limits, Rust-owned emergency audit WAL behavior, and conservative quota precheck inputs before ORU and Seal Ledger are online.
- The plan keeps Overgate narrow: no downstream domain-state ownership, no direct ledger mutation, no policy finality, no identity lifecycle ownership, no key custody, no native-service business logic, no private payload storage by default, and no conventional cloud product boundary assumptions.
- The plan does not require any change to the master Phase 0 through Phase 13 dependency order.

## Exit Gate

SUB BUILD PLAN #8 is complete when a builder can implement Overgate as the Phase 1 Rust control-plane ingress service with signed command admission, strict schema validation, Overkey signature verification, Overpass actor resolution, Overtenant authorization, idempotency reservation/replay/conflict handling, classed retention, trace propagation, conservative rate-limit and quota prechecks, Overguard policy handoff, Overwatch-compatible audit events, fail-closed audit behavior with a bounded Rust-owned emergency WAL for allowlisted degraded cases, Overqueue-backed forwarding for asynchronous commands, tenant-isolated admin/status/trace/limit views, and downstream handoff rules that preserve the owning authority of every service behind Overgate.
