# Overrid Overgate

Overgate is the Rust-first API ingress and admission boundary for Overrid control-plane commands. This crate provides the Axum route surface, dependency-readiness model, admin authorization guard, Phase 3 command-envelope validation, Phase 4 local-stack credential, actor, tenant, service-account, node-agent, and operator admission adapters, Phase 5 idempotency/status records, Phase 6 rate-limit/quota/policy precheck refs, and Phase 7 Overwatch-compatible audit, metrics, emergency WAL, and grid operations evidence.

## Local Entrypoint

- Service id: `service:overgate`
- Local-stack port owner: `service:api`
- Command: `cargo run -p overrid-overgate --bin overgate`
- Default bind: `127.0.0.1:18080`
- Local base path: `/overgate`
- Health: `GET /v1/healthz` and `GET /overgate/v1/healthz`
- Readiness: `GET /v1/readyz` and `GET /overgate/v1/readyz`

The existing Phase 0 local-stack `service:api` contract remains the reserved loopback API port owner. Overgate Phase 2 attaches the `service:overgate` route surface, `/overgate` base path, and fixture references to that loopback boundary without replacing earlier local-stack service ids.

## Public Routes

- `POST /v1/commands`
- `GET /v1/commands/{command_id}`
- `GET /v1/traces/{trace_id}`
- `GET /v1/limits`
- `POST /v1/policy/dry-run`
- `GET /v1/healthz`
- `GET /v1/readyz`

## Admin Routes

- `GET /v1/admin/ingress/{request_id}`
- `GET /v1/admin/idempotency/{tenant_id}/{idempotency_key}`
- `POST /v1/admin/idempotency/{record_id}/expire`
- `GET /v1/admin/rate-limits`

Admin route shells require placeholder signed-operator headers and deny unauthenticated, non-operator, and cross-tenant requests before returning data.

## Phase 3 Command Validation

`POST /v1/commands` now parses the body as a strict command envelope before any downstream side effect. The handler validates content type, size, required fields, supported shared-schema version, Overgate command type, payload hash refs, signature metadata, private command privacy class, and raw-secret sentinel markers.

Accepted command responses include:

- Shared schema adapter evidence: `overgate.phase3.shared_schema_adapter`
- Canonicalization version: `overgate.canonical.v0.1`
- Canonical request inputs including `request_hash`, `payload_hash`, `body_hash`, tenant, actor, idempotency, timestamp, and credential metadata
- Hash/ref-only retention policy: `overgate.phase3.hashes_and_refs_only`
- Forwarding state: `not_forwarded_phase3_validation_only`

Denied command responses use stable API error fields: `reason_code`, `trace_id`, `request_id` when the envelope parsed far enough to derive one, `retryability`, `correction_fields`, `correction_hint`, dependency name when relevant, and redacted diagnostics. Private payloads, raw secrets, and credential material must not appear in error bodies.

## Phase 4 Admission

`POST /v1/commands` now admits commands through an explicit local-stack adapter before idempotency reservation or downstream forwarding. The adapter is not a replacement for Overkey, Overpass, or Overtenant; it records the request Overgate would make to those services and preserves downstream ownership.

Accepted command responses use `overgate.phase4.response.v0`, `overgate.command_admitted_phase4`, and `not_forwarded_phase4_admission_only`. The response includes:

- Overkey-lite signature evidence with `credential_id`, public-key ref, key version, algorithm allowlisting, canonicalization version, replay-window state, revocation state, rotation state, and `auth.signature_verified_phase4`.
- Overpass actor-resolution evidence with actor type, active state, identity ref, local environment ref, and `auth.actor_resolved_phase4`.
- Overtenant authorization evidence with tenant state, membership, app ownership, delegated access, role binding, service-account permission, quota scope ref, and `auth.tenant_authorized_phase4`.
- Service-account and node-agent admission evidence with narrow command-class state, scoped credential state, callback signature state, trace/audit context state, and signed command requirements.

The local adapter denies unknown, expired, replayed, revoked, rotated, wrong-tenant, wrong-key-version, unsupported-algorithm, disabled, suspended, deleted-marker, wrong-type, environment-mismatched, cross-tenant, role-denied, broad service-account, wrong callback-class, and missing audit-context cases using stable `auth.*` reason codes. Operator/admin routes require typed signed operator or system-service credentials, remain tenant-scoped, emit audit-hook refs, and fail closed with `auth.operator_audit_unavailable` when Overwatch readiness is unavailable.

## Phase 5 Idempotency And Status

`POST /v1/commands` now reserves idempotency after Phase 4 admission and schema validation, before any rate-limit, policy, forwarding, or downstream side effect. The local Phase 5 store is Overgate-owned Rust state for deterministic local-stack behavior; it is not Redis, Kafka, PostgreSQL, or a downstream service boundary.

Accepted command responses use `overgate.phase5.response.v0`, `overgate.command_accepted_phase5`, and `pending_forwarding_phase5`. The response includes:

- Idempotency reservation records scoped by tenant, actor or service account, command type, idempotency key, request hash, and credential context.
- Stable response digest refs that start with `hash:overgate:` and disclose no private payload or credential material.
- Safe replay metadata with `overgate.idempotency_replayed` for duplicate compatible requests.
- Conflict denial with `overgate.idempotency_conflict` when the same scoped idempotency key is reused with a different request hash.
- Classed retention metadata for bodyless reads, low-risk metadata writes, control-plane mutations, queue-producing workload commands, and finality/rights commands.
- Retention extension refs for dispute, retry, incident, and finality holds.

`GET /v1/commands/{command_id}` returns `overgate.command_status_phase5` for tenant-visible records, `overgate.status_visibility_denied` for cross-tenant lookups, and `overgate.status_not_found` for missing records. `GET /v1/traces/{trace_id}` returns `overgate.trace_summary_phase5` with audit refs and redacted command summaries. `GET /v1/limits` returns `overgate.limits_phase5` with visible idempotency record counts and quota-precheck refs.

Admin idempotency lookup and expiration routes now expose tenant-scoped Phase 5 idempotency records to signed operators without bypassing Overwatch fail-closed behavior. Expiration is also constrained by the operator tenant, so a cross-tenant operator cannot mutate a record by knowing its record id.

## Phase 6 Rate Limits, Quota Prechecks, And Policy Handoff

`POST /v1/commands` now runs Phase 6 prechecks after Phase 4 admission, Phase 3 schema validation, and Phase 5 idempotency scoping, before any forwarding or downstream state write. Accepted command responses use `overgate.phase6.response.v0`, `overgate.command_accepted_phase6`, and `prechecked_before_forwarding_phase6`.

The Phase 6 response includes:

- Deterministic local rate-limit buckets scoped by tenant, actor, service-account ref, source-app ref, command class, environment, and timestamp window.
- `overgate.rate_limited` denials with reset refs and `overgate.rate_limited` audit evidence before forwarding.
- Conservative quota-precheck records with quota scope refs, budget refs, grant placeholder refs, local counter refs, optional Overmeter snapshot refs, accepted-command quota refs, `overgate.quota_precheck_denied` denials, and `not_settled_by_overgate`.
- `no_balance_mutation: true` and `no_seal_ledger_entry: true` on quota prechecks so Overgate does not mutate accounting balances or create Seal Ledger entries.
- Overguard policy dry-run handoff records with `overguard.policy.v0`, matched rule refs, decision refs, `overgate.policy_denied` denials, missing-prerequisite reasons, and `stored_policy_truth_in_overgate: false`.
- A command-class matrix covering `low_risk_read`, `phase1_control_plane_mutation`, `queue_producing_workload`, `policy_heavy`, `accounting_affecting`, `storage_namespace`, `native_app_side_effect`, `admin`, and `break_glass`.
- Client-safe rate-limit, quota, budget, grant, and policy refs in `client_denial_refs` so SDK, CLI, UI, and native apps do not parse free-form text.

`GET /v1/limits` returns `overgate.limits_phase6` with Phase 6 bucket and quota summaries. `POST /v1/policy/dry-run` returns `overgate.policy_dry_run_phase6` and records an Overguard handoff without storing policy truth. `GET /v1/admin/rate-limits` returns `overgate.admin_rate_limits_phase6` for signed operators while preserving existing Overwatch fail-closed behavior.

## Phase 7 Audit, Observability, Degraded Mode, And Grid-Ready Operations

`POST /v1/commands` now records Phase 7 audit evidence after Phase 6 prechecks and before any future forwarding side effect. The evidence is Overwatch-compatible and still Rust-owned in local-stack mode: it emits ordered ingress events for request receipt, signature verification, idempotency reservation or replay, and command acceptance, with transition metadata for accepted and denied paths.

The Phase 7 response includes:

- Overwatch audit client and event-transition refs: `overwatch.audit.v0` and `event_transition_map:overgate:phase7`.
- Fail-closed behavior with `overgate.audit_fail_closed` when Overwatch is unavailable for high-risk, admin, break-glass, accounting, ledger, rights, credential, secret, or policy-override command types.
- An explicit emergency audit WAL path for low-risk Phase 1 control-plane mutations only. The WAL is bounded, append-only, hash chained, redacted to refs and hashes, fsync-marked before side effects, and reports `degraded_until_replayed_to_overwatch` until replay is required.
- Operational metrics and trace summaries with safe low-cardinality labels only. Tenant, actor, command, request, trace, payload, credential, and secret values are excluded from labels.
- A grid-resident operations checklist covering readiness, maintenance mode, rolling updates, rollback, break-glass controls, backup/restore, failover drills, and founder-hardware removal from the normal path.

The emergency WAL is disabled by default; tests enable it explicitly to prove degraded-mode behavior without adding PostgreSQL, Redis, Kafka, NATS, or another external core boundary.

## Fixtures

- `fixtures/valid/phase2_local_command.valid.json` defines the deterministic local smoke command, service entrypoint, dependency refs, and harness scenario id.
- `fixtures/valid/phase3_command.valid.json` defines the strict command-envelope validation fixture.
- `fixtures/invalid/admin_unsigned.invalid.json` defines the unsigned admin denial case.
- `fixtures/invalid/phase3_missing_tenant.invalid.json` proves missing required command fields deny with a stable schema error.
- `fixtures/invalid/phase3_unknown_private_payload.invalid.json` proves sensitive unknown fields are rejected.
- `fixtures/invalid/phase3_raw_secret.invalid.json` proves raw secret markers are rejected before storage or diagnostics.
- `fixtures/invalid/phase3_wrong_canonicalization_version.invalid.json` proves signed command envelopes must use the current Overgate canonicalization version.
- `fixtures/valid/phase4_command.valid.json` defines a Phase 4 service-account admission fixture with signature, actor, tenant, and audit evidence expectations.
- `fixtures/invalid/phase4_revoked_credential.invalid.json` proves revoked credentials deny through Overkey-lite admission before side effects.
- `fixtures/invalid/phase4_unsupported_algorithm.invalid.json` proves unsupported signature algorithms deny through Overkey-lite admission before side effects.
- `fixtures/invalid/phase4_actor_unknown.invalid.json` proves unknown actors deny through Overpass admission before idempotency or forwarding.
- `fixtures/invalid/phase4_actor_disabled.invalid.json` proves disabled actors deny through Overpass admission before idempotency or forwarding.
- `fixtures/invalid/phase4_tenant_role_denied.invalid.json` proves role-denied tenant authorization fails through Overtenant admission.
- `fixtures/invalid/phase4_service_account_broad.invalid.json` proves broad service-account command classes are rejected.
- `fixtures/valid/phase5_command.valid.json` defines the Phase 5 idempotency reservation, response digest, retention class, and pending-forwarding fixture.
- `fixtures/invalid/phase5_idempotency_conflict.invalid.json` proves duplicate scoped idempotency keys with different request hashes deny before downstream side effects.
- `fixtures/valid/phase6_command.valid.json` defines the Phase 6 rate-limit, quota-precheck, policy-handoff, and command-class matrix accepted response fixture.
- `fixtures/invalid/phase6_precheck_denials.invalid.json` proves rate-limit, quota-precheck, and policy denials surface stable client-denial refs before forwarding.
- `fixtures/valid/phase7_command.valid.json` defines the Phase 7 Overwatch-compatible audit events, emergency-WAL default state, safe metric labels, and grid-operations checklist fixture.

Fixtures are local/test scoped and contain no raw secrets or private payloads.
