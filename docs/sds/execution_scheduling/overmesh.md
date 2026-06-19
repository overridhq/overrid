SDS #22

# Overmesh SDS

## Purpose

Provide policy-bound private connectivity, service discovery, endpoint health, artifact transfer paths, and namespace route resolution for Overrid workloads and grid-resident services.

Overmesh is the connectivity metadata and route-control layer. In Phase 4 it starts as trusted private node discovery and tenant-scoped service routing. In Phase 8 it expands into namespace route resolution for apps, storage refs, service endpoints, and native app routes.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [overmesh.md](../../service_catalog/execution_scheduling/overmesh.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 4: Trust, Policy, and Verification](../../build_plan/phase_04_trust_policy_verification.md), [Phase 8: Data, Storage, and Namespace Platform](../../build_plan/phase_08_data_storage_namespace_platform.md) |

## Service Family

- Family: Execution and scheduling
- Owning layer: Private service discovery, route metadata, connectivity policy, and transfer coordination
- Primary data scope: endpoint records, service instance records, route bindings, route decisions, health probes, connectivity policy refs, transfer sessions, bandwidth hints, and audit events
- First build phase from service plan: private discovery in [Phase 4](../../build_plan/phase_04_trust_policy_verification.md); namespace route resolution in [Phase 8](../../build_plan/phase_08_data_storage_namespace_platform.md)

## Problem Statement

Overrid nodes and services need to find each other without falling back to ungoverned ad hoc networking. Control-plane-to-node calls, node-to-storage transfers, tenant app service routing, grid-resident backbone connectivity, and native app routes all need tenant boundaries, health awareness, policy decisions, and audit trails.

Overmesh supplies that controlled connectivity layer. It does not make every node globally reachable; it resolves allowed routes and records why they were allowed.

## Goals

- Register trusted private node endpoints with health, locality, tenant/provider scope, and service identity.
- Provide tenant-scoped service discovery with deny-by-default cross-tenant behavior.
- Resolve namespace routes to service endpoints, app routes, storage refs, identity records, or asset refs after Phase 8.
- Coordinate artifact transfer paths for Overrun, Overstore, deployment artifacts, and cache warming.
- Expose connectivity and locality facts that Oversched may use as hints, never as policy bypasses.
- Emit route, endpoint, health, transfer, and denial events for Overwatch and Overmeter.
- Preserve route-change history and policy refs for incident review and disputes.

## Non-Goals

- Do not become a public internet replacement or general-purpose VPN product in early phases.
- Do not bypass Overtenant boundaries or Overguard policy.
- Do not own identity, namespace ownership, or route ownership disputes. Overpass, Universal Namespace, and Overasset own those records.
- Do not store artifact bytes. Overstore and node-local stores own bytes; Overmesh coordinates paths and refs.
- Do not choose workload placement. Oversched consumes mesh facts as inputs.
- Do not hide cross-tenant route attempts; denials are part of the evidence trail.

## Primary Actors And Clients

- Overcell node agents registering reachable control endpoints.
- Overrun and Overstore requesting artifact transfer paths.
- Oversched reading locality/connectivity facts as placement hints.
- Overguard evaluating tenant, workload, data-class, egress, and route policy.
- Overpass and Universal Namespace supplying names and route ownership refs.
- Native apps, deployment services, and grid-resident backbone services resolving allowed routes.
- Operators inspecting endpoint health, route changes, denials, and transfer failures.

## Dependencies

- [Overpass](../control_plane/overpass.md) and Universal Namespace for identity, name, and route ownership refs.
- [Overtenant](../control_plane/overtenant.md) for tenant and private-swarm boundaries.
- [Overcell](overcell.md) for node endpoint registration and heartbeat.
- [Overguard](../trust_policy_verification/overguard.md) for route, egress, data-class, and workload policy decisions.
- [Overwatch](../control_plane/overwatch.md) for route and connectivity audit events.
- [Overstore](../data_storage_namespace/overstore.md) for object/artifact refs once storage is built.
- [Overmeter](overmeter.md) for network and transfer usage events.
- [Overlease](overlease.md) when bandwidth or transfer windows need bounded reservations.

Phase 4 can start with private endpoint and service discovery; Phase 8 adds namespace route resolution after storage and namespace primitives exist.

## Owned Responsibilities

Overmesh owns:

- Endpoint record lifecycle and health state for trusted private nodes and services.
- Tenant-scoped service discovery responses.
- Route resolution decisions with policy refs and reason codes.
- Route binding state for app/service/storage/namespace targets after Phase 8.
- Artifact transfer session metadata, path selection, health checks, retries, and usage dimensions.
- Connectivity health and locality facts for scheduling and operations.
- Traffic-shaping and bandwidth-hint records as the scheduler matures.

Overmesh must not directly mutate tenant ownership, identity records, storage objects, or scheduler placements.

## Data Model

The first implementation should define:

- `endpoint_record`: endpoint id, node id or service id, tenant/provider scope, service identity, transport kind, address ref, port/ref, locality, trust class, health state, and expiry.
- `service_instance`: service id, version, workload id, tenant id, route labels, health check config, node refs, and readiness state.
- `service_discovery_query`: requester, tenant, workload class, service selector, data class, policy refs, and trace id.
- `route_binding`: namespace or route id, target type, target refs, owner refs, policy refs, state, version, and audit refs.
- `route_decision`: query id, allowed/denied decision, endpoint choices, matched policy, reason codes, ttl, and evidence refs.
- `connectivity_health`: endpoint id, probe result, latency/throughput hints, last success, failure reason, and severity.
- `artifact_transfer_session`: transfer id, source ref, destination ref, tenant, data class, object refs, path decision, bytes, state, and metering refs.
- `bandwidth_hint`: node or route id, expected bytes, priority, time window, policy refs, and optional lease refs.

Common envelope fields:

- `id`, `tenant_id`, `actor_id` or service account.
- `trace_id`, `idempotency_key`, `state`, `created_at`, `updated_at`, `policy_refs`, and `audit_refs`.

## API Surface

The initial API should stay narrow:

- `POST /mesh/endpoints`: register or refresh an endpoint for a node or service.
- `GET /mesh/endpoints/{endpoint_id}`: read authorized endpoint metadata and health state.
- `POST /mesh/service-discovery`: resolve tenant-scoped service instances for a caller context.
- `POST /mesh/routes/resolve`: resolve an app/service/storage/namespace route after policy checks.
- `POST /mesh/routes/{route_id}/bindings`: create or update a route binding after ownership and policy checks.
- `POST /mesh/transfers`: request an artifact or object transfer path.
- `POST /mesh/transfers/{transfer_id}/complete`: report transfer result and usage dimensions.
- `POST /mesh/endpoints/{endpoint_id}/disable`: disable an endpoint with reason and audit refs.

API requirements:

- Every query includes requester identity, tenant scope, workload class, data class, and trace id.
- Cross-tenant discovery and routing default to deny.
- Route responses must include ttl, policy refs, endpoint refs, and reason codes.
- Route binding updates require ownership refs and signed actor/service authority.
- Reads must hide endpoint details that are not needed by the caller.

## Event Surface

- `overmesh.endpoint_registered`: endpoint accepted or refreshed.
- `overmesh.endpoint_health_changed`: endpoint health moved between healthy, degraded, unreachable, or disabled.
- `overmesh.discovery_resolved`: service discovery returned allowed endpoints.
- `overmesh.discovery_denied`: discovery denied by tenant or policy.
- `overmesh.route_bound`: route binding created or updated.
- `overmesh.route_resolved`: route resolved for a caller context.
- `overmesh.route_denied`: route request rejected with reason.
- `overmesh.transfer_started`: transfer session started.
- `overmesh.transfer_completed`: transfer finished with usage dimensions.
- `overmesh.transfer_failed`: transfer failed with retryability and evidence.

Events should avoid exposing private addresses or route internals beyond authorized refs.

## Core Workflow

1. Overcell registers a node endpoint with tenant/provider scope, service identity, and health metadata.
2. Overmesh verifies identity, tenant scope, and endpoint policy.
3. Health checks update endpoint state and locality/connectivity hints.
4. A workload or service asks for discovery or route resolution with caller context.
5. Overmesh consults Overguard, Overtenant, route ownership refs, endpoint state, and data class.
6. If allowed, Overmesh returns bounded endpoint or transfer path refs with ttl and policy refs.
7. Transfer sessions report bytes, duration, result, and metering refs.
8. Route changes emit Overwatch audit events and preserve previous versions.

## State Machine

Endpoint lifecycle:

1. `registered`: endpoint metadata accepted.
2. `healthy`: health checks pass and route eligibility is possible.
3. `degraded`: endpoint is reachable but has warning signals.
4. `draining`: endpoint should not receive new traffic except allowed existing flows.
5. `unreachable`: endpoint health failed or heartbeat expired.
6. `disabled`: operator or policy disabled endpoint routing.
7. `revoked`: endpoint identity or trust was revoked.
8. `retired`: endpoint removed while history remains.

Route binding lifecycle:

1. `draft`: binding is being prepared.
2. `pending_policy`: ownership and policy are being evaluated.
3. `active`: route may resolve for authorized callers.
4. `degraded`: route target is available only with warnings or reduced priority.
5. `disabled`: route is intentionally unavailable.
6. `superseded`: a newer route binding version replaced it.
7. `revoked`: route was removed by policy, dispute, or incident action.

Transfer lifecycle:

1. `requested`: transfer path requested.
2. `authorized`: policy allowed the transfer.
3. `transferring`: bytes are moving through the selected path.
4. `completed`: transfer finished and usage was emitted.
5. `failed_retryable`: transfer failed but can retry.
6. `failed_final`: transfer failed terminally.
7. `cancelled`: transfer cancelled by caller, policy, or timeout.

## Policy And Security

- Cross-tenant discovery, routing, and transfer are denied unless Overguard explicitly allows them.
- Endpoint registration requires service or node identity verification.
- Route binding updates require namespace/route ownership refs and signed authority.
- Private addresses, provider topology, and node internals must be redacted from unauthorized reads.
- Route decisions must include policy version, matched rules, and reason codes.
- Artifact transfers must respect data class, egress policy, cache scope, storage policy, and workload sensitivity.
- Transport credentials should be short-lived and scoped to route or transfer session.
- Route hijack, endpoint spoofing, and stale endpoint reuse must emit security events.

## Metering And Accounting

Overmesh emits raw network and route usage facts:

- Discovery request count, route resolution count, transfer bytes, transfer duration, route egress class, endpoint locality, and retry count.
- Attribute usage to tenant, workload, app/service, node/provider where applicable, route id, and data class.
- Distinguish control-plane traffic, artifact transfer, service-to-service traffic, and public/native-app traffic.
- Feed raw events into Overmeter; do not calculate billing or pricing in Overmesh.
- Preserve denied route attempts for abuse, fraud, and incident analysis without charging users for policy-denied requests.

## Observability And Operations

- Operators need endpoint health, route binding history, cross-tenant denial counts, transfer failures, stale endpoint counts, and topology pressure by locality.
- Health checks should cover endpoint registry, policy path, Overwatch event emission, and transfer path reporting.
- Route-change operations should support dry-run and rollback where possible.
- Incident tools need fast endpoint disable, route revoke, and transfer quarantine actions.
- Mesh diagnostics must show reason codes without leaking tenant-private addresses or topology.

## Failure Modes And Recovery

- Endpoint heartbeat lost: mark endpoint unreachable and stop returning it for new route decisions.
- Policy engine unavailable: deny new cross-tenant or broader-scope routes; optionally serve cached narrow private decisions only until ttl.
- Route target missing: return explicit no-target or stale-target reason.
- Transfer path fails: retry with another authorized path if policy and ttl allow.
- Endpoint spoofing detected: revoke endpoint, emit incident evidence, and notify Oververify/Overwatch.
- Route binding conflict: keep current active version and reject new binding until ownership/policy issue is resolved.
- Namespace service unavailable: avoid guessing route ownership and return blocked with retryability.

## Validation Plan

The service implementation plan lists these requirements:

- Cross-tenant routing is denied by default.
- Route changes are auditable.
- Scheduler can use connectivity and locality facts.

Additional SDS-level validation:

- Contract tests for endpoint registration, service discovery, route resolution, route binding, transfer request, and transfer completion.
- Tenant isolation tests proving unauthorized discovery and transfer are denied.
- Route version tests proving previous bindings remain auditable.
- Endpoint health tests for healthy, degraded, unreachable, disabled, revoked, and retired states.
- Transfer tests for success, retryable failure, final failure, cancellation, and usage emission.
- Redaction tests for private addresses and topology details.
- Replay tests proving route decisions can be reconstructed from endpoint, tenant, policy, and route records.

## Build Breakdown

1. Define endpoint, service discovery, route decision, route binding, health, and transfer-session schemas.
2. Add trusted private node endpoint registration from Overcell.
3. Add tenant-scoped service discovery with deny-by-default cross-tenant rules.
4. Add health-aware route decisions and Overwatch events.
5. Add artifact transfer path metadata for Overrun and later Overstore.
6. Add namespace route binding and resolution after Phase 8 namespace/storage primitives.
7. Add bandwidth hints and traffic shaping only after scheduling and metering need them.

The first Overmesh build should be narrow enough to secure private execution, not a broad public networking product.

## Handoff And Downstream Use

Overmesh supports private execution, grid-resident service connectivity, artifact transfer, application routes, native apps, and namespace resolution. Downstream services should request route decisions through Overmesh rather than embedding endpoint lists or bypassing tenant policy.

## Open Design Questions

Resolved decisions:

- Phase 4 uses authenticated HTTP/2 with mTLS/rustls as the first private control-plane-to-node transport for Overcell endpoint registration, heartbeat-adjacent reachability checks, command/control callbacks, and tenant-scoped private service discovery. Endpoint records must include a versioned `transport_kind` and address ref shape that can later support QUIC and libp2p-style peer paths for artifact transfer, node discovery, and federation, but those paths are not required to be the Phase 4 control transport. All requests still use signed command envelopes, tenant scope, trace ids, idempotency keys, stable reason codes, and Overwatch evidence refs.
- Cacheability is policy-profiled and bounded by endpoint health, endpoint expiry, route binding version, policy version, lease or transfer window, and explicit invalidation. Phase 4 private endpoint discovery responses default to a maximum 30-second ttl for healthy private endpoints, 10 seconds for degraded endpoints, and no cache for unreachable, disabled, revoked, or unknown endpoints. Phase 4 route decisions for command/control and private service paths default to a maximum 15-second ttl and must be re-resolved before retries that would start new side effects. Denials may be cached for at most 5 seconds to dampen retry storms, but never as proof of future authorization. Phase 8 namespace route bindings may declare their own ttl, capped by policy; app/API/identity/asset refs may use longer bounded route-binding ttls, while storage, private, regulated, or secret-bearing routes default to no reusable endpoint detail unless Overguard explicitly allows it.
- Schedulers may see only scheduling-safe endpoint facts: endpoint id or route ref, node/service id refs, tenant/provider visibility class, workload/data class compatibility, coarse locality, health state and severity, heartbeat/probe age, drain state, coarse bandwidth and latency class, trust/policy refs, route constraints, collected-at timestamp, and ttl. Operators with explicit authority may see exact address refs, port refs, transport parameters, private probe failure detail, provider topology needed for incident response, disable/revoke history, route-change history, and transfer/session diagnostics. Raw secrets, private keys, bearer credentials, and tenant payload data are not exposed through either view.
- Bandwidth hints are signed eligibility, scoring, capacity, and metering facts, not billing decisions. Overmesh may emit expected bytes, coarse bandwidth class, priority, transfer window, route id, endpoint locality, policy refs, and observed transfer dimensions for Oversched and Overmeter. Overlease is involved only when an Overpack manifest or Overguard policy requires an exclusive transfer window, route window, or priority reservation that could otherwise double-book constrained network capacity. The lease proves a bounded reservation window; actual usage remains an Overmeter raw fact, and pricing, balance mutation, holds, settlement, and billing stay with Overmeter, Seal Ledger, Overbill, and related accounting services.
- Phase 8 must first support namespace route kinds for `service_endpoint`, `app_route`, `api_route`, `storage_ref`, `identity_ref`, and `asset_ref`. These cover the Phase 8 exit gate: a simple app can resolve application/API routes, platform service endpoints, Overbase/Overstore/Overvault-backed storage refs, identity records, and Overasset-backed rights refs without bypassing policy. `native_app_page_ref` uses the same route-binding shape but remains reserved until native app surfaces need it; public/federated route kinds wait until the federation and public-pool phases define their trust, abuse, and metering constraints.
