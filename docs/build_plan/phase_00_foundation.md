# Phase 0: Foundation

## Objective

Establish the engineering base that every later Overrid subsystem depends on: repository structure, local development, shared schemas, API conventions, test harnesses, and documentation rules.

This phase is not product work. It is the platform skeleton that prevents the rest of the build from becoming a pile of disconnected services.

## Depends On

- Approved whitepaper direction.
- Master build order in `docs/build_plan/master_plan.md`.
- Decision that founder hardware is bootstrap infrastructure, not the permanent backbone.

## Build Order

1. Define repository layout.
2. Create shared schema and protocol packages.
3. Create local development stack.
4. Add API and event conventions.
5. Add integration test harness.
6. Add documentation and progress tracking conventions.

## Workstream 1: Repository Layout

Create first-class folders for:

- `services/control-plane`: initial modular API and worker process.
- `services/node-agent`: Overcell agent.
- `packages/schemas`: shared request, command, manifest, event, and audit schemas.
- `packages/sdk`: client SDK generated from stable API contracts.
- `packages/cli`: developer and operator command line tool.
- `docs/specs`: protocol, schema, and service contract documents.
- `docs/build_plan`: build sequencing and progress.
- `tests/integration`: local stack and cross-service tests.
- `infra/local`: Overrid-shaped local durable state, durable job table, object/artifact stubs, service definitions, and development profiles.

Keep the first implementation modular but not over-split. The v1 system should be easy to run locally, inspect, and change.

## Workstream 2: Shared Schema Package

Build schema definitions before service logic. The first schema package should cover:

- Identity references: person, organization, node, app, service account, system service.
- Tenant references: tenant id, role, policy scope, quota scope, suspension state.
- Command envelopes: command id, tenant id, actor id, idempotency key, trace id, signature, payload type.
- Workload manifest draft: name, version, workload class, resource card, input refs, output refs, egress policy.
- Resource manifest draft: provider id, node id, resource class, capability record, trust class, region.
- Event envelope: event id, timestamp, source, tenant id, actor id, subject id, type, payload, schema version.
- Audit record: command id, policy version, decision, reason codes, state transition, linked event ids.

Use strict validation. A service boundary is valid only when both request and response objects are schema checked.

## Workstream 3: Local Development Stack

Create a local stack that can run without external cloud services:

- API process.
- Worker process.
- Overrid-shaped local durable state.
- Durable job table.
- Object/artifact storage stub.
- One local node-agent simulator.
- Test identity and tenant fixtures.

The local stack must support deterministic reset so integration tests are repeatable.

## Workstream 4: API And Event Discipline

Define platform conventions once:

- Every mutating request uses an idempotency key.
- Every request has a trace id.
- Every command names tenant, actor, target, and command type.
- Every state transition emits an append-only event.
- Every denial has stable reason codes.
- Every externally visible object has a schema version.
- Every API response has a stable error shape.

These conventions must be enforced in helper libraries before individual services build their own shortcuts.

## Workstream 5: Test Harness

Create integration tests for:

- Starting the local stack.
- Creating a fixture tenant.
- Creating a fixture identity.
- Submitting a signed no-op command.
- Writing and reading audit events.
- Rejecting invalid schema payloads.
- Replaying fixture data from clean Overrid-shaped local durable state.

Add deterministic fixture generation so the same test data can be used in later phases.

## Workstream 6: Documentation Rules

Create a documentation rule for every service:

- Purpose.
- Owned data.
- Public API.
- Events emitted.
- Events consumed.
- Security boundary.
- Operational checks.
- Test expectations.

This is the minimum contract for adding a new subsystem.

## Validation

- Local stack starts from a clean checkout.
- Schema package can validate valid and invalid fixtures.
- Integration smoke test creates tenant, actor, command, and audit event.
- Documentation folder contains service-contract stubs.
- No phase-specific service is allowed to bypass shared schemas.

## Exit Gate

Phase 0 is complete when a developer can start the local system, run smoke tests, inspect emitted audit events, and add a new schema without touching unrelated services.

## Handoff To Phase 1

Phase 1 consumes the schema package, local stack, API conventions, and audit/event helpers to build the first control-plane skeleton.
