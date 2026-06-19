# Phase 2: Seed Private Swarm

## Objective

Turn founder-provided servers and GPUs into the first controlled private Overrid swarm. This phase proves node registration, heartbeat, capability discovery, inventory, and private resource visibility.

The founder hardware is the bootstrap environment. The architecture must already assume that core services will later migrate into grid-resident system workloads.

## Depends On

- Phase 1 identity, tenant, key, registry, queue, and audit primitives.
- Seed hardware inventory.
- Local and private network access plan.

## Build Order

1. Define node roles and inventory schema.
2. Build Overcell node-agent install and registration.
3. Add heartbeat and lifecycle state.
4. Add hardware discovery.
5. Add benchmark runner.
6. Publish capability records into Overregistry.
7. Add operator visibility for node health.
8. Prove one server and one GPU node.

## Workstream 1: Node Inventory Model

Define node classes:

- Compute.
- GPU.
- Storage.
- Database.
- Cache.
- Gateway.
- Specialized accelerator.
- System-service eligible.

Each node record should include owner, tenant visibility, physical or logical location, trust class, resource class, maintenance state, and current eligibility.

## Workstream 2: Overcell Node Agent

Build the node agent with:

- Install command.
- Registration command.
- Node credential creation or enrollment.
- Heartbeat.
- Capability update.
- Drain mode.
- Shutdown signal.
- Version reporting.
- Upgrade placeholder.

The first agent can be narrow, but it must be built as a long-running supervised process, not a one-off script.

## Workstream 3: Heartbeat And Lifecycle

Track node states:

- Registered.
- Live.
- Stale.
- Expired.
- Draining.
- Maintenance.
- Disabled.
- Revoked.

Heartbeat records should include agent version, observed resources, load summary, active leases count, and last successful control-plane contact.

## Workstream 4: Hardware Discovery

Discover and normalize:

- CPU model and core count.
- Memory capacity.
- GPU model, memory, driver, runtime, and supported compute modes.
- Disk capacity and performance hints.
- Network interface and bandwidth hints.
- OS and kernel.
- Container or sandbox runtime availability.
- Region or locality tag.

Hardware names are not enough. The scheduler will need normalized capabilities.

## Workstream 5: Benchmarks

Create safe benchmark jobs for:

- CPU throughput.
- GPU inference or matrix throughput.
- Disk read/write profile.
- Network throughput within private swarm.
- Cold-start overhead.
- Sustained thermal or throttling behavior where feasible.

Benchmarks must produce signed capability records so later scheduler decisions can cite evidence.

## Workstream 6: Private Visibility

Add tenant-scoped resource visibility:

- Founder operator sees all seed nodes.
- Test tenants see only explicitly granted capacity.
- System-service eligibility is visible only to operators.
- Suspended or unverified nodes are excluded from placement candidates.

## Validation

- At least one seed server registers.
- At least one GPU node registers.
- Control plane shows live, stale, expired, draining, and disabled states in tests or fixtures.
- Benchmark records are persisted and linked to node capability records.
- Node-agent reconnect after restart preserves identity and state.

## Exit Gate

Phase 2 is complete when the first private swarm can be observed through Overrid records and its capacity can be trusted enough for controlled private workload placement.

## Handoff To Phase 3

Phase 3 uses registered nodes, capability records, and pending workloads to build the first private execution loop.
