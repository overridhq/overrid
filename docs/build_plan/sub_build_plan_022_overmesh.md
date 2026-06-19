# SUB BUILD PLAN #22 - Overmesh

Attached SDS: [docs/sds/execution_scheduling/overmesh.md](../sds/execution_scheduling/overmesh.md)

## Purpose

This sub-build plan turns SDS #22 into an implementation sequence for Overmesh. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Overmesh is the policy-bound private connectivity, service-discovery, endpoint-health, artifact-transfer-path, and namespace route-control layer for Overrid. It starts narrowly in master Phase 4 as trusted private endpoint discovery and tenant-scoped service routing. It expands in master Phase 8 into namespace route binding and route resolution for service endpoints, app routes, API routes, storage refs, identity refs, and asset refs. It is not a general-purpose public VPN, not the scheduler, not the policy engine, not canonical storage, not identity or namespace ownership authority, and not billing or settlement logic.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #22: Overmesh](../sds/execution_scheduling/overmesh.md) | Controls Overmesh purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering facts, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Overmesh service plan](../service_catalog/execution_scheduling/overmesh.md) | Controls the service-catalog objective, first build phases, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical build order from master Phase 0 through master Phase 13. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared schemas, fixture discipline, local stack stubs, signed envelopes, idempotency, trace ids, integration harnesses, and Rust workspace prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overgate, Overpass, Overtenant, Overkey, Overregistry, Overwatch, Overqueue, identity, tenant, key, registry, queue, and audit primitives that Overmesh consumes. |
| [Phase 2: Seed Private Swarm](phase_02_seed_private_swarm.md) | Supplies registered Overcell nodes, endpoint identity, heartbeat-adjacent reachability inputs, capability refs, and node lifecycle state. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies Overrun, Overpack, Overlease, Overmeter, and private execution consumers that need route decisions, transfer paths, and usage facts. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Controls Overmesh's first build point: trusted private node discovery, tenant-scoped service routing, deny-by-default policy checks, endpoint health, and private connectivity evidence. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Consumes Overmesh raw route, transfer, endpoint-locality, retry, and network usage facts through Overmeter and accounting rollups. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies stricter system-service route, health, failover, maintenance, and grid-resident backbone requirements. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase, Overstore, Overvault, Universal Namespace, Overasset, data-class, storage-ref, route-ownership, and namespace primitives required for broad route binding and resolution. |
| [Phase 9: Overpack Deployment Platform](phase_09_overpack_deployment_platform.md) | Consumes application route, service endpoint, health, deployment activation, rollback, and route-change behavior. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Supplies known-organization and purpose-scoped route constraints before federation routes can widen. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public-provider sandbox, anti-abuse, fraud, challenge, payout-hold, and low-sensitivity constraints for public route exposure. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Consumes app, API, identity, storage, asset, and later native-app page routes through normal Overrid APIs. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies retention, migration, incident, compliance, reporting, threat-model, and PIP governance for route history, endpoint evidence, and mesh safety. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #22 first build work aligned to master Phase 4, with Phase 8 namespace expansion and later product, federation, native-app, and governance hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, authenticated HTTP/2 with mTLS/rustls first, versioned transport fields for later QUIC/libp2p, canonical JSON plus JSON Schema, optional Protobuf for compact contracts, Ed25519 signatures, BLAKE3/content hashes, signed envelopes, and native Overrid service boundaries. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 2, 3, 4, and 8 | Attach SDS #22, freeze Overmesh scope, preserve Phase 4 as the first build point, and record Phase 8 namespace expansion. |
| 2 | Master Phases 0, 1, and 4 | Build Rust service contracts, schemas, APIs, state machines, fixtures, reason codes, and local harness scenarios. |
| 3 | Master Phases 2 and 4 | Implement trusted endpoint registration, identity verification, transport bootstrap, endpoint expiry, and redacted reads. |
| 4 | Master Phase 4 | Implement tenant-scoped service discovery with deny-by-default policy decisions and bounded cacheability. |
| 5 | Master Phase 4 | Implement endpoint health, route decisions, route binding foundations, ttl behavior, route history, and replayable audit events. |
| 6 | Master Phases 3, 4, and 5 | Add artifact transfer path metadata, transfer lifecycle reporting, retry behavior, and Overmeter raw usage facts. |
| 7 | Master Phases 3, 4, and 5 | Add scheduler-safe connectivity facts, bandwidth hints, optional lease-window refs, locality signals, and traffic-shaping hooks. |
| 8 | Master Phase 8 | Add namespace route binding and route resolution after Overbase, Overstore, Overvault, Universal Namespace, and Overasset primitives exist. |
| 9 | Master Phases 6, 7, 9, 10, 11, 12, and 13 | Harden product, grid-resident, deployment, federation, public-provider, native-app, incident, compliance, and governance handoffs. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, tech-stack alignment, documentation links, queue state, and implementation gates. |

## Tech Stack Guardrails

- Overmesh core is a Rust service module using shared contract types, Tokio where async service calls and health probes are required, and Axum/Tower/Hyper-style HTTP for mesh APIs where an HTTP boundary exists.
- Phase 4 uses authenticated HTTP/2 with mTLS/rustls as the first private control-plane-to-node and private service transport. Endpoint records keep versioned `transport_kind` and address-ref shapes so later QUIC and libp2p-style peer paths can be added for artifact transfer, node discovery, and federation without changing the public contract.
- Endpoint records, service instances, service discovery queries, route bindings, route decisions, connectivity health records, artifact transfer sessions, bandwidth hints, API errors, fixtures, and lifecycle events use canonical JSON plus JSON Schema. Compact Protobuf contracts may be added only where the shared contract layer requires them.
- Mutating calls require signed actor, service-account, or node envelopes, tenant scope, workload class, data class, trace id, idempotency key, schema version, stable reason codes, policy refs, audit refs, and append-only Overwatch events.
- Ed25519 is used for actor, service, node, route-binding, and command signatures where signatures are required. BLAKE3/content hashes are used for payload hashes, transfer refs, artifact refs, route-binding versions, endpoint evidence hashes, and replay checkpoints.
- Route and transfer credentials are short-lived, scoped to the route or transfer session, and never exposed as raw bearer material in route decisions, audit events, diagnostics, or scheduler-safe facts.
- Cross-tenant discovery, routing, and transfer are denied unless Overguard explicitly allows them. Private addresses, provider topology, node internals, and transport parameters are hidden from unauthorized reads.
- Overmesh owns endpoint metadata, health state, service discovery responses, route decisions, route-binding state, transfer-session metadata, bandwidth hints, and connectivity facts. Overpass and Universal Namespace own identity/name/route ownership refs; Overtenant owns tenant boundaries; Overguard owns policy finality; Oversched owns placement; Overrun moves bytes through owned execution/storage paths; Overstore owns object bytes; Overmeter owns usage rollups; accounting services own ORU, Seal Ledger, billing, payouts, and settlement.
- PostgreSQL, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Vault, cloud KMS, Kubernetes-first orchestration, generic VPN product boundaries, public internet replacement assumptions, blockchain, NFT, pricing, revenue, customer-count, or per-operation external payment mechanics must not become Overmesh's product boundary.

## Phase 1: SDS Attachment, Mesh Scope, And Boundary Rules

### Work Items

- **1.1 Attach the build plan to SDS #22.**
  - Design: Link this document from the numbered Overmesh SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/execution_scheduling/overmesh.md`, `docs/service_catalog/execution_scheduling/overmesh.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #22 returns both the Overmesh SDS and this sub-build plan.

- **1.2 Freeze Overmesh as policy-bound connectivity metadata and route control.**
  - Design: Record that Overmesh owns endpoint lifecycle metadata, service discovery responses, route decisions, route bindings, endpoint health, artifact transfer-session metadata, bandwidth hints, and connectivity facts.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms Overmesh does not become a generic VPN, public internet replacement, scheduler, policy engine, identity owner, namespace owner, storage owner, billing engine, or settlement service.

- **1.3 Preserve master Phase 4 as the first build point.**
  - Design: Keep first implementation in master Phase 4 because private endpoint discovery and tenant-scoped service routing require policy, verification, and audit controls before multi-tenant use.
  - Output: Phase-gate note that Phase 0 through Phase 3 are prerequisites, Phase 4 starts private discovery, Phase 5 consumes usage facts, and Phase 8 unlocks namespace route resolution.
  - Validation: Review proves this plan does not move broad namespace routing, public routing, or storage route refs into Phase 4 and does not change the master Phase 0 through Phase 13 order.

- **1.4 Carry forward resolved SDS decisions.**
  - Design: Preserve SDS #22 decisions for HTTP/2 with mTLS/rustls first, versioned transport fields, bounded cacheability, scheduling-safe endpoint facts, bandwidth hints as non-billing facts, and Phase 8 route kinds.
  - Output: Resolved-decision checklist tied to SDS #22 open-question answers.
  - Validation: Review rejects Phase 4 QUIC/libp2p as a required control transport, reusable endpoint details for secret-bearing routes without policy approval, raw addresses in scheduler views, bandwidth billing mutation inside Overmesh, and unsupported route kinds before their owning phases.

- **1.5 Define runtime authority boundaries.**
  - Design: Create a boundary matrix for Overcell, Overrun, Overstore, Oversched, Overlease, Overmeter, Overguard, Overwatch, Overtenant, Overpass, Universal Namespace, Overasset, Overkey, Overregistry, Overqueue, SDK, CLI, admin UI, deployment services, native apps, federation services, and public-provider services.
  - Output: Boundary matrix listing read/write authority, command authority, state ownership, evidence refs, policy refs, redaction level, route ownership refs, and ownership exclusions.
  - Validation: Design review rejects direct scheduler placement writes, direct policy override, raw private address leaks, namespace ownership mutation, storage-byte ownership, route hijack by endpoint registration, and accounting mutation inside Overmesh.

## Phase 2: Rust Service, Schemas, APIs, Fixtures, And State Machines

### Work Items

- **2.1 Create the Overmesh Rust service module.**
  - Design: Add a Rust service module with endpoint repositories, service-instance repositories, route-decision engine, route-binding repository, health probe worker, transfer-session coordinator, bandwidth-hint repository, Overguard client, Overwatch emitter, Overmeter event client, and integration-test hooks.
  - Output: Service crate or module skeleton, repository traits, client interfaces, health worker entry point, route resolver boundary, transfer coordinator boundary, error types, reason-code mapping, and test harness entry points.
  - Validation: Compile and service-start checks pass once implementation exists; crate layout review confirms Overmesh remains separate from Overcell, Oversched, Overlease, Overrun, Overstore, Overguard, Overmeter, and accounting services.

- **2.2 Define Overmesh contract schemas.**
  - Design: Add schemas for `endpoint_record`, `service_instance`, `service_discovery_query`, `route_binding`, `route_decision`, `connectivity_health`, `artifact_transfer_session`, `bandwidth_hint`, API errors, lifecycle events, redaction classes, and reason codes.
  - Output: JSON Schema files, Rust types, fixtures, lifecycle enums, reason-code enums, schema-version rules, redaction metadata, route-kind enums, and compatibility rules.
  - Validation: Schema tests reject missing endpoint id, node/service id, tenant scope, transport kind, address ref, trust class, health state, route id, target refs, policy refs, audit refs, trace id, idempotency key, data class, and state where required.

- **2.3 Define narrow API contracts.**
  - Design: Implement or specify `POST /mesh/endpoints`, `GET /mesh/endpoints/{endpoint_id}`, `POST /mesh/service-discovery`, `POST /mesh/routes/resolve`, `POST /mesh/routes/{route_id}/bindings`, `POST /mesh/transfers`, `POST /mesh/transfers/{transfer_id}/complete`, and `POST /mesh/endpoints/{endpoint_id}/disable`.
  - Output: API request/response schemas, signed-envelope rules, idempotency behavior, ttl semantics, pagination/read filters, redaction profiles, and Overwatch event payloads.
  - Validation: API tests cover valid calls, duplicate idempotency keys, wrong tenant, wrong node/service identity, missing workload class, missing data class, unauthorized route binding, missing ownership refs, expired commands, and restricted endpoint reads.

- **2.4 Implement endpoint, route, and transfer state machines.**
  - Design: Model endpoint states across registered, healthy, degraded, draining, unreachable, disabled, revoked, and retired; route binding states across draft, pending_policy, active, degraded, disabled, superseded, and revoked; transfer states across requested, authorized, transferring, completed, failed_retryable, failed_final, and cancelled.
  - Output: State transition engine, illegal-transition reasons, append-only transition records, terminal-state evidence requirements, and replay fixtures.
  - Validation: State tests reject active routes without policy, endpoint resurrection after revocation, health promotion from retired state, transfer completion without authorization, route binding mutation without version history, and cleanup that deletes evidence.

- **2.5 Build deterministic local harness fixtures.**
  - Design: Model healthy private endpoint, degraded endpoint, unreachable endpoint, disabled endpoint, wrong-tenant discovery, policy-denied discovery, valid private route, stale route target, route binding conflict, transfer success, retryable transfer failure, final transfer failure, route hijack attempt, and namespace-service unavailable.
  - Output: Valid and invalid fixtures with expected state transitions, route responses, ttl values, Overwatch events, Overmeter raw facts, redacted reads, and reason codes.
  - Validation: Local harness scenarios produce deterministic outputs and prove Overmesh behavior does not depend on conventional database, queue, object-store, VPN, cloud-networking, or external payment product boundaries.

## Phase 3: Trusted Endpoint Registration And Private Transport Bootstrap

### Work Items

- **3.1 Register and refresh trusted Overcell endpoints.**
  - Design: Accept endpoint registration only from authorized Overcell node agents or service accounts with tenant/provider scope, service identity, transport kind, address ref, locality, trust class, health metadata, expiry, trace id, and idempotency key.
  - Output: Endpoint registration path, duplicate refresh behavior, expiry update logic, initial health state, and `overmesh.endpoint_registered` event.
  - Validation: Registration tests reject missing node/service identity, wrong tenant, revoked node credential, missing transport kind, malformed address ref, stale command envelope, missing trace id, and unauthorized service account.

- **3.2 Verify endpoint identity, tenant scope, and route policy.**
  - Design: Validate endpoint registrations through Overtenant boundaries, Overkey credential refs, Overpass identity refs, Overregistry service refs, and Overguard route/egress policy before the endpoint can be used.
  - Output: Identity and policy validator, accepted/rejected endpoint states, policy refs, denial reason codes, and audit refs.
  - Validation: Security tests prove endpoint spoofing, cross-tenant endpoint claims, stale service refs, wrong provider scope, revoked credential refs, and missing policy refs block route eligibility.

- **3.3 Bootstrap HTTP/2 mTLS private control transport.**
  - Design: Support Phase 4 authenticated HTTP/2 with mTLS/rustls for control-plane-to-node callbacks, heartbeat-adjacent reachability checks, command/control paths, and trusted private service discovery.
  - Output: Transport contract, certificate/key-ref expectations, `transport_kind` versioning, address-ref shape, connection health fields, and future QUIC/libp2p compatibility hooks.
  - Validation: Transport tests prove HTTP/2 mTLS is sufficient for Phase 4, route decisions are not tied to raw host strings, transport fields can represent later QUIC/libp2p paths, and missing or revoked credentials block route eligibility.

- **3.4 Manage endpoint expiry, disable, revoke, drain, and retire behavior.**
  - Design: Expire stale endpoints, stop returning disabled/revoked/unreachable endpoints, support operator disable with reason refs, record drain state, and preserve retired endpoint history.
  - Output: Expiry worker, disable API path, revoke detector, drain marker, retired-state history, and endpoint lifecycle events.
  - Validation: Lifecycle tests prove stale endpoints become unreachable, disabled/revoked endpoints are not returned, drain avoids new route decisions except policy-approved existing flows, retired endpoints preserve history, and repeated disable calls are idempotent.

- **3.5 Redact endpoint reads and operator diagnostics.**
  - Design: Separate scheduler-safe, tenant-safe, provider-safe, and operator-restricted endpoint views so private addresses, ports, transport parameters, provider topology, and probe failure detail are exposed only to authorized readers.
  - Output: Redaction profiles, read filters, diagnostics field classes, restricted evidence refs, and endpoint read APIs.
  - Validation: Redaction tests prove tenants cannot inspect unrelated endpoint details, schedulers see only safe refs and coarse facts, operators need explicit authority for exact address refs, and secrets, private keys, bearer credentials, and tenant payload data never appear.

## Phase 4: Tenant-Scoped Service Discovery And Deny-By-Default Policy

### Work Items

- **4.1 Register service instances and readiness state.**
  - Design: Store service id, version, workload id, tenant id, route labels, health check config, node refs, readiness state, policy refs, and audit refs for service instances that may participate in discovery.
  - Output: Service-instance repository, readiness transition logic, service labels, endpoint linkage, and instance lifecycle events.
  - Validation: Instance tests reject missing tenant id, missing service identity, unlinked endpoint refs, stale health config, invalid readiness transition, wrong workload class, and route labels outside allowed scope.

- **4.2 Implement tenant-scoped service discovery queries.**
  - Design: Evaluate requester identity, tenant scope, workload class, service selector, data class, policy refs, trace id, endpoint health, readiness state, and ttl before returning endpoint refs.
  - Output: Discovery evaluator, allowed endpoint selection, empty result reasons, ttl calculation, endpoint choice metadata, and response schema.
  - Validation: Discovery tests prove same-tenant private discovery works, cross-tenant discovery denies by default, data-class mismatch denies, stale endpoints are omitted, degraded endpoints carry warnings, and no raw private address is returned without authority.

- **4.3 Integrate Overguard policy into discovery.**
  - Design: Require Overguard decisions for cross-scope discovery, sensitive data classes, broader service selectors, egress-sensitive routes, and degraded-route fallback behavior.
  - Output: Policy client integration, matched policy refs, policy version capture, reason-code mapping, denial cache behavior, and discovery audit events.
  - Validation: Policy tests prove Overmesh cannot bypass Overguard, policy-unavailable defaults deny broader routes, cached denials expire quickly, matched rules are recorded, and policy updates invalidate affected discovery responses.

- **4.4 Enforce bounded discovery cacheability.**
  - Design: Apply SDS #22 ttl rules: healthy private endpoint discovery defaults to at most 30 seconds, degraded endpoint discovery to at most 10 seconds, unreachable/disabled/revoked/unknown endpoints to no cache, and denials to at most 5 seconds.
  - Output: Discovery cache policy, ttl calculator, invalidation triggers, endpoint-health dependency, route-binding-version dependency, and policy-version dependency.
  - Validation: Cache tests prove ttl caps are enforced, health changes invalidate discovery, policy changes invalidate discovery, denials are not treated as future authorization, and retries that start new side effects re-resolve first.

- **4.5 Emit discovery events and denial evidence.**
  - Design: Emit `overmesh.discovery_resolved` and `overmesh.discovery_denied` events with tenant-safe refs, endpoint refs, policy refs, reason codes, ttl, trace id, and redaction metadata.
  - Output: Event payloads, Overwatch emitter, denied-attempt evidence, replay fields, and metrics labels for raw counts.
  - Validation: Event tests prove discovery successes and denials are replayable, denial events do not leak private topology, Overwatch receives append-only events, and Overmeter can count raw discovery attempts without billing decisions in Overmesh.

## Phase 5: Health-Aware Route Decisions And Route Binding Foundations

### Work Items

- **5.1 Implement connectivity health probes.**
  - Design: Track endpoint id, probe result, latency/throughput hints, last success, failure reason, severity, collected-at timestamp, ttl, and redaction class without exposing raw private topology to untrusted readers.
  - Output: Probe worker, health record schema, severity mapping, health-state transitions, stale-health detector, and `overmesh.endpoint_health_changed` events.
  - Validation: Health tests cover healthy, degraded, unreachable, disabled, revoked, retired, probe timeout, policy-path failure, Overwatch emission failure, and stale-health downgrade behavior.

- **5.2 Implement private route decisions.**
  - Design: Resolve private service/control paths by combining requester context, endpoint state, service readiness, route labels, data class, workload class, Overguard policy, tenant boundary, and ttl.
  - Output: Route decision engine, allowed/denied response schema, endpoint choice list, matched policy refs, reason codes, route ttl, and evidence refs.
  - Validation: Route tests prove allowed same-tenant routes return bounded refs, cross-tenant routes deny by default, policy-unavailable denies broader routes, stale targets return explicit no-target/stale-target reasons, and route decisions are reconstructable.

- **5.3 Implement route binding version foundations.**
  - Design: Model route binding draft, pending_policy, active, degraded, disabled, superseded, and revoked states with target type, target refs, owner refs, policy refs, state, version, audit refs, and conflict behavior.
  - Output: Route-binding repository, version records, conflict detector, ownership-ref validator, state transition logic, and `overmesh.route_bound` events.
  - Validation: Binding tests reject updates without ownership refs, unsigned actor/service authority, conflicting active versions, missing policy refs, stale target refs, and silent overwrite of previous route versions.

- **5.4 Enforce route ttl, retry, and invalidation behavior.**
  - Design: Bound route responses by endpoint health, endpoint expiry, route-binding version, policy version, lease or transfer window, data class, and explicit invalidation.
  - Output: Route ttl calculator, retry re-resolution rules, invalidation index, side-effect retry guard, and route decision cache policy.
  - Validation: Tests prove command/control route decisions default to at most 15 seconds, retries that start new side effects re-resolve, disabled endpoints invalidate routes, policy updates invalidate routes, and cached decisions cannot outlive lease or transfer windows.

- **5.5 Emit route events and replayable audit evidence.**
  - Design: Emit `overmesh.route_resolved`, `overmesh.route_denied`, `overmesh.route_bound`, and route-change events with redacted endpoint refs, policy refs, route-binding version, reason codes, ttl, and trace id.
  - Output: Event schemas, Overwatch emitter, replay inputs, route history read model, and route-change diagnostics.
  - Validation: Replay tests prove route decisions can be reconstructed from endpoint, tenant, policy, and route records; route-change events preserve previous versions; and unauthorized readers cannot inspect private addresses or provider topology.

## Phase 6: Artifact Transfer Paths And Usage-Fact Handoff

### Work Items

- **6.1 Define artifact transfer sessions for Overrun and Overstore.**
  - Design: Model transfer id, source ref, destination ref, tenant, data class, object refs, path decision, endpoint refs, bytes, state, policy refs, audit refs, and metering refs.
  - Output: Transfer-session schema, create API, path-decision refs, authorization state, and transfer lifecycle events.
  - Validation: Schema and API tests reject missing source/destination refs, missing data class, missing object refs where required, wrong tenant, missing policy refs, unauthorized caller, and transfer requests that expose object bytes to Overmesh.

- **6.2 Select authorized transfer paths and retries.**
  - Design: Choose transfer paths using endpoint health, data class, route policy, cache/storage policy, workload sensitivity, ttl, lease or transfer window, retryability, and route constraints.
  - Output: Path selector, retry rules, alternative path selection, failed-path reasons, ttl enforcement, and `overmesh.transfer_started` events.
  - Validation: Transfer tests prove failed paths retry only when policy and ttl allow, stale route targets block transfer, cross-tenant transfer denies by default, regulated/secret-bearing transfers avoid reusable endpoint details unless policy allows, and retries do not bypass policy.

- **6.3 Complete, fail, and cancel transfer sessions.**
  - Design: Report transfer result, bytes, duration, retry count, endpoint locality, failure reason, retryability, evidence refs, and final state while keeping byte movement and storage ownership outside Overmesh.
  - Output: Complete API, failure API behavior, cancellation behavior, final-state records, duplicate completion handling, and `overmesh.transfer_completed` or `overmesh.transfer_failed` events.
  - Validation: Completion tests cover success, retryable failure, final failure, cancellation, duplicate complete calls, wrong transfer id, wrong tenant, late completion after terminal state, and missing usage dimensions.

- **6.4 Emit raw network and transfer usage facts to Overmeter.**
  - Design: Emit discovery request count, route resolution count, transfer bytes, transfer duration, route egress class, endpoint locality, retry count, app/service refs, node/provider refs, route id, and data class as raw usage facts.
  - Output: Overmeter client contract, raw usage event schema, attribution fields, denial-count behavior, and usage handoff events.
  - Validation: Metering tests prove Overmesh sends raw facts but does not calculate rollups, prices, balances, holds, payouts, bills, or settlement; denied route attempts are preserved for abuse/fraud/incident analysis without charging users.

- **6.5 Quarantine suspicious transfer and route behavior.**
  - Design: Detect endpoint spoofing, route hijack attempts, transfer abuse, stale endpoint reuse, repeated policy-denied transfer attempts, and topology anomalies, then emit incident evidence and route/endpoint quarantine refs.
  - Output: Suspicion reason codes, quarantine state, incident event payloads, Oververify/Overwatch notification refs, and operator-visible diagnostics.
  - Validation: Security tests prove spoofed endpoints are revoked or disabled, route hijack attempts keep current active binding, repeated denied attempts are visible, quarantined paths stop being selected, and incident diagnostics remain redacted by authority.

## Phase 7: Scheduler-Safe Connectivity Facts, Bandwidth Hints, And Traffic Shaping

### Work Items

- **7.1 Publish scheduler-safe connectivity facts.**
  - Design: Expose only endpoint id or route ref, node/service refs, visibility class, workload/data class compatibility, coarse locality, health state/severity, probe age, drain state, coarse bandwidth and latency class, trust/policy refs, constraints, collected-at timestamp, and ttl.
  - Output: Scheduler-safe projection, Oversched read contract, field-level redaction, staleness markers, and compatibility labels.
  - Validation: Projection tests prove Oversched never receives raw addresses, ports, bearer credentials, private probe detail, exact topology, secrets, private keys, tenant payloads, or route internals beyond safe refs.

- **7.2 Implement bandwidth hints as non-billing facts.**
  - Design: Record expected bytes, coarse bandwidth class, priority, transfer window, route id, endpoint locality, policy refs, observed transfer dimensions, optional lease refs, and metering refs without pricing or balance mutation.
  - Output: `bandwidth_hint` schema, create/update behavior, hint lifecycle, scheduler and Overmeter projections, and reason codes.
  - Validation: Hint tests prove bandwidth hints affect eligibility/scoring/capacity planning only, do not create bills or holds, and keep actual usage as Overmeter raw facts.

- **7.3 Integrate optional Overlease reservation windows.**
  - Design: Use Overlease only when an Overpack manifest or Overguard policy requires an exclusive transfer window, route window, or priority reservation that could otherwise double-book constrained network capacity.
  - Output: Lease-window request contract, lease-ref attachment, transfer-window enforcement, expired-window behavior, and denial reasons.
  - Validation: Integration tests prove Overmesh can operate without leases for ordinary transfers, uses lease refs only for constrained reservations, never creates resource leases by itself, and blocks transfer starts after lease-window expiry.

- **7.4 Add traffic-shaping and backpressure hooks.**
  - Design: Provide route/endpoint-level pressure hints, priority class handling, drain-aware selection, retry backoff, overload reason codes, and policy-aware shaping hooks as scheduling and metering mature.
  - Output: Traffic-shaping policy hooks, pressure-state records, route backoff behavior, overload diagnostics, and endpoint drain integration.
  - Validation: Backpressure tests prove overloaded endpoints receive fewer new route decisions, policy-required priority is honored, drain state is respected, retries do not storm denied paths, and shaping never bypasses tenant policy.

- **7.5 Validate locality and pressure feedback loops.**
  - Design: Feed coarse locality, latency class, bandwidth class, health severity, endpoint age, and route constraints back to Oversched and operator views as hints rather than placement authority.
  - Output: Locality/pressure fixtures, Oversched integration test cases, operator dashboards inputs, and stale-fact handling.
  - Validation: Integration tests prove Oversched treats mesh facts as inputs, Overmesh does not choose workload placement, stale hints downgrade candidates, and operator views explain reason codes without leaking private topology.

## Phase 8: Namespace Route Binding And Resolution

### Work Items

- **8.1 Gate namespace routing on Phase 8 primitives.**
  - Design: Require Overbase, Overstore, Overvault, Universal Namespace, Overasset, route ownership refs, data classes, storage refs, identity refs, and rights refs before broad namespace route resolution is enabled.
  - Output: Phase-gate checklist, dependency probes, disabled-route-kind defaults, missing-prerequisite reason codes, and migration notes.
  - Validation: Review proves Phase 4 does not fabricate namespace ownership, storage refs, identity refs, or asset rights and that Phase 8 route kinds remain disabled until owning services exist.

- **8.2 Implement Phase 8 route kinds.**
  - Design: Support `service_endpoint`, `app_route`, `api_route`, `storage_ref`, `identity_ref`, and `asset_ref` route kinds as the initial Phase 8 namespace route set.
  - Output: Route-kind enum, target schemas, route-kind validation, policy requirements, read filters, and fixtures for each route kind.
  - Validation: Route-kind tests prove a simple app can resolve application/API routes, platform service endpoints, Overbase/Overstore/Overvault-backed storage refs, identity records, and Overasset-backed rights refs without bypassing policy.

- **8.3 Implement namespace route binding create and update.**
  - Design: Bind namespace or route ids to targets with owner refs, target refs, policy refs, state, version, audit refs, signed authority, conflict handling, and supersession history.
  - Output: Binding API, ownership validator, versioned binding records, conflict state, signed update envelope, and route-change events.
  - Validation: Binding tests reject missing namespace ownership refs, unsigned updates, wrong actor authority, stale target refs, policy-denied target types, silent overwrites, and route bindings that hide previous active versions.

- **8.4 Resolve namespace routes with policy and data-class checks.**
  - Design: Resolve app/service/storage/identity/asset namespace routes by evaluating caller context, tenant scope, route ownership, target availability, data class, policy, endpoint health, route-binding version, and ttl.
  - Output: Namespace route resolver, response schemas, endpoint/target refs, redaction rules, ttl behavior, and route decision events.
  - Validation: Resolver tests prove unauthorized cross-tenant routes deny, secret-bearing/private storage routes avoid reusable endpoint detail unless policy allows, stale bindings return explicit reasons, and route update emits audit events.

- **8.5 Preserve future route placeholders without premature exposure.**
  - Design: Reserve `native_app_page_ref` under the same route-binding shape for later native app surfaces and defer public/federated route kinds until their trust, abuse, metering, and public-pool constraints exist.
  - Output: Reserved route-kind registry, disabled-by-default behavior, future-compatibility notes, and validation fixtures.
  - Validation: Compatibility tests prove reserved kinds cannot be activated accidentally, public/federated route kinds remain gated by later phase constraints, and future native routes can reuse the Phase 8 binding shape without schema breakage.

## Phase 9: Product, Grid-Resident, Federation, Native, And Governance Handoffs

### Work Items

- **9.1 Expose SDK, CLI, and admin/operator surfaces safely.**
  - Design: Provide generated SDK bindings, Rust CLI commands, and TypeScript admin/developer UI views for endpoint health, route decisions, route bindings, transfer sessions, denial counts, and diagnostics through Overgate/admin APIs.
  - Output: Client contract list, CLI command inventory, admin view requirements, redaction profiles, pagination behavior, and stable reason codes.
  - Validation: Client tests prove SDK/CLI/admin views cannot bypass Overgate, cannot reveal unauthorized endpoint details, and can inspect route decisions, denials, and health states with correct authority.

- **9.2 Integrate product, adapter, and deployment consumers.**
  - Design: Connect Docdex/Mcoda/Codali adapters, AI gateway flows, deployment planner, release strategies, package validator, and Overpack app deployment flows to route decisions and transfer paths.
  - Output: Consumer integration matrix, route decision expectations, transfer path expectations, deployment activation hooks, rollback hooks, and health-check contracts.
  - Validation: Integration tests prove product workloads request route decisions through Overmesh, deployments activate and roll back routes through versioned bindings, and adapters do not embed static endpoint lists or bypass tenant policy.

- **9.3 Harden grid-resident system-service connectivity.**
  - Design: Add strict route, health, maintenance, failover, backup/restore, rolling update, and break-glass expectations for grid-resident system services without exposing founder-hardware topology.
  - Output: System-service route profile, failover route behavior, maintenance route behavior, backup/restore route checks, and incident diagnostics.
  - Validation: Grid-resident tests prove trusted system routes survive rolling updates and failover, maintenance drains routes predictably, backup/restore drills include route state, and founder hardware can leave the normal path without static route assumptions.

- **9.4 Gate federation and public-provider route widening.**
  - Design: Keep known-organization federation routes, public-interest pool routes, and public low-sensitivity provider routes behind explicit trust, abuse, challenge, fraud, sandbox, payout-hold, and workload-class constraints.
  - Output: Federation route profile placeholders, public-provider route profile placeholders, anti-abuse route constraints, policy refs, and denial reasons.
  - Validation: Review proves private, regulated, tenant-sensitive, and secret-bearing workloads cannot route through public providers; unknown public nodes cannot receive private endpoint details; and federation widening requires explicit policy and evidence.

- **9.5 Add governance, compliance, and incident hardening.**
  - Design: Define retention, migration, incident response, compliance export, threat-model, PIP governance, route-history replay, endpoint evidence retention, and proportional intervention requirements.
  - Output: Governance checklist, retention matrix, incident runbook inputs, compliance export fields, migration strategy, and threat-model review items.
  - Validation: Governance review proves operators can explain route denials, endpoint revocations, route changes, transfer quarantines, and policy decisions from stored evidence without exposing private tenant or provider data.

## Phase 10: Validation, Documentation Alignment, And Final Gates

### Work Items

- **10.1 Validate plan structure and documentation links.**
  - Design: Verify this file starts with `SUB BUILD PLAN #22`, has phases numbered 1 through 10, includes well-formed work items, links to SDS/service/master/crosswalk docs, and preserves the master Phase 0 through Phase 13 order.
  - Output: Link-check evidence, phase-heading evidence, work-item structure evidence, and no-order-change note.
  - Validation: Focused validation passes for title prefix, attached SDS link, 10 phase headings, work-item numbering, Design/Output/Validation structure, and local Markdown links.

- **10.2 Validate Overmesh contract and state behavior.**
  - Design: Run contract tests for endpoint registration, service discovery, route resolution, route binding, transfer start, transfer completion, health updates, bandwidth hints, and state transitions.
  - Output: Contract test suite, state-machine fixtures, golden route decisions, transfer lifecycle fixtures, and reason-code coverage.
  - Validation: Tests prove valid flows work and invalid flows reject missing refs, unauthorized scope, stale endpoint state, wrong route ownership, policy denial, expired ttl, and terminal-state mutation.

- **10.3 Validate policy, security, and redaction behavior.**
  - Design: Run tenant isolation, cross-tenant denial, route-hijack, endpoint-spoofing, credential revocation, private address redaction, scheduler-safe view, operator-restricted view, and secret-bearing route tests.
  - Output: Security test suite, redaction fixtures, policy replay cases, incident evidence cases, and denial evidence.
  - Validation: Tests prove deny-by-default behavior, no raw private topology leaks, no route policy bypass, no scheduler exposure of restricted fields, and no public/VPN-style widening before later phase gates.

- **10.4 Validate integration, replay, operations, and usage handoffs.**
  - Design: Run integration tests across Overcell, Overguard, Overwatch, Overmeter, Oversched, Overlease, Overrun, Overstore, Universal Namespace, Overasset, SDK, CLI, admin UI, deployment services, native apps, and governance consumers where phase-appropriate.
  - Output: Integration traces, replay traces, route-history reconstruction, usage-fact handoff evidence, operation runbook evidence, and failure-mode evidence.
  - Validation: Tests prove route decisions are replayable, route changes are auditable, scheduler uses mesh facts as hints, transfer usage reaches Overmeter as raw facts, namespace routes wait for Phase 8 primitives, and failures produce explicit retryability or terminal reasons.

- **10.5 Update progress, queue state, Docdex index, and memory.**
  - Design: Record SDS #22 plan completion in build-plan progress, update the Codex55 queue state/progress, refresh Docdex indexing for changed docs, and save concise repo memory.
  - Output: Updated `docs/build_plan/progress.md`, `.codex55_sds_queue/state.json`, `.codex55_sds_queue/progress.md`, Docdex index refresh, Docdex search evidence, and repo memory entry.
  - Validation: JSON validation passes, queue next incomplete starts at `023-build-plan`, Docdex search returns this sub-build plan for SDS #22 queries, and configured test-runner status is recorded.

## Alignment Review

- This plan preserves the master Phase 0 through Phase 13 order. No master phase renumbering or major order change is required.
- Overmesh's first build remains master Phase 4 because private connectivity must be policy-bound, tenant-scoped, auditable, and deny-by-default before multi-tenant routing is useful.
- Namespace route binding and route resolution remain gated by master Phase 8 because route ownership, storage refs, private-state refs, identity refs, Overasset refs, and broader app route semantics depend on Phase 8 primitives.
- The plan follows `docs/overrid_tech_stack_choice.md`: Rust-first core service, Tokio/Axum-style HTTP where needed, authenticated HTTP/2 with mTLS/rustls first, versioned transport fields for later QUIC/libp2p, canonical JSON plus JSON Schema, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, and native Overrid service boundaries.
- The SDS, service catalog, master build plan, and service-catalog crosswalk already align on Overmesh as Phase 4 plus Phase 8. The required update is link/index alignment, not a correction to the master build order.

## Exit Gate

SDS #22 is ready for implementation planning when this sub-build plan is linked from the Overmesh SDS, Overmesh service plan, master build plan, and build-plan crosswalk; all 10 phases have Design/Output/Validation work items; local Markdown links resolve; queue/progress docs are updated; and validation evidence records any configured test-runner blocker.
