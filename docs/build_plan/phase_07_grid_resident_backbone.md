# Phase 7: Grid-Resident Backbone

## Objective

Move Overrid core services from founder-operated seed hardware into protected grid-resident system workloads.

The founder servers and GPUs can start the ecosystem, but the backbone must not depend on them permanently. This phase proves Overrid can operate itself.

## Depends On

- Phase 6 real product workloads.
- Trusted private nodes.
- Policy and verification controls.
- Accountable system-service usage.

## Build Order

1. Define system-service workload class.
2. Mark trusted system-eligible nodes.
3. Containerize or package core services for grid execution.
4. Replicate critical state.
5. Add failover and restore drills.
6. Add rolling update and rollback.
7. Migrate one non-critical service.
8. Migrate control-plane services.
9. Remove founder hardware from normal production path.

## Workstream 1: System-Service Workload Class

Define a privileged but constrained class for:

- Overgate.
- Overregistry.
- Overqueue.
- Oversched.
- Overmeter.
- Overwatch.
- Overguard.
- Overpass.
- Supporting stores.
- Internal observability.

System-service workloads must have stricter placement, logging, backup, and upgrade rules than ordinary workloads.

## Workstream 2: Trusted Placement Rules

System workloads may run only on nodes that satisfy:

- Verified operator identity.
- Strong uptime history.
- Stable network.
- Sufficient storage and backup capability.
- Security baseline.
- No active disputes or abuse markers.
- Explicit system-service eligibility.

Unknown public nodes must never host backbone services.

## Workstream 3: Service Packaging

Package core services for grid deployment:

- Runtime image or package artifact.
- Config contract.
- Secrets contract.
- Health check.
- Readiness check.
- Migration command.
- Backup command.
- Restore command.
- Rollback command.

The same deployment path later feeds Overpack application deployment.

## Workstream 4: State Replication

Plan state stores:

- Primary database or equivalent state engine.
- Event log.
- Queue state.
- Object/artifact storage.
- Seal Ledger records.
- Policy version store.
- Key metadata.

Every critical store needs backup, restore, and corruption detection.

## Workstream 5: Failover

Implement or choose mechanisms for:

- Leader election or equivalent active/passive failover.
- Health-based route shifting.
- Queue worker failover.
- State lock safety.
- Split-brain prevention.
- Recovery after partial outage.

Prefer simple, testable reliability over complex orchestration that cannot be operated yet.

## Workstream 6: Updates And Rollback

Add operational controls:

- Maintenance mode.
- Rolling update.
- Rollback.
- Version pinning.
- Break-glass access.
- Signed operator actions.
- Audit records for all system changes.

System-service changes must be traceable because they affect the whole ecosystem.

## Workstream 7: Migration Sequence

Migrate in this order:

1. Non-critical observability replica.
2. Read-only registry or API replica.
3. Worker process for low-risk jobs.
4. Queue worker.
5. Policy service.
6. Metering service.
7. API ingress replica.
8. Primary control-plane path.
9. Founder hardware removal from normal path.

Keep founder hardware as emergency fallback until restore and failover drills pass.

## Validation

- Core services run on trusted grid nodes.
- Founder hardware can be removed from the normal production path without stopping product workloads.
- Backup restore drill succeeds.
- Failover drill succeeds.
- Rollback drill succeeds.
- Operator actions are signed and auditable.

## Exit Gate

Phase 7 is complete when Overrid's backbone is grid-resident, recoverable, and no longer operationally dependent on founder machines for normal production traffic.

## Handoff To Phase 8

Phase 8 builds the durable data, storage, vault, namespace, and routing substrate required for real applications.
