SDS #13

# Overrid Protocol Core SDS

## Purpose

Define the protocol rules that make all Overrid services behave like one coherent resource allocation ecosystem.

Overrid Protocol Core is a specification and conformance layer, not a runtime microservice. It defines the command envelope, event envelope, state-transition discipline, versioning rules, service ownership boundaries, compatibility rules, and conformance tests that every service must follow.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [overrid_protocol_core.md](../../service_catalog/control_plane/overrid_protocol_core.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 0: Foundation](../../build_plan/phase_00_foundation.md) |
| Sub-build plan | [SUB BUILD PLAN #13 - Overrid Protocol Core](../../build_plan/sub_build_plan_013_overrid_protocol_core.md) |

## Service Family

- Family: Control plane.
- Owning layer: Protocol specification, shared rules, and conformance tests.
- Runtime shape: versioned protocol docs, schema rules, helper packages, fixtures, and test suites.
- Primary data scope: protocol definitions, state-machine definitions, compatibility metadata, conformance fixtures, and PIP references.
- First build phase from service plan: [Phase 0: Foundation](../../build_plan/phase_00_foundation.md).

## Problem Statement

Overrid has many services, clients, native apps, and adapters that must act like one system. If each service invents its own command shape, state names, event rules, idempotency behavior, or compatibility policy, the platform will become impossible to audit or govern. Protocol Core makes the common rules explicit before service behavior drifts.

## Goals

- Define command, event, audit, error, trace, idempotency, versioning, and compatibility rules.
- Define workload, manifest, queue, lease, execution, metering, accounting, dispute, and governance state-machine conventions.
- Define ownership boundaries so services do not write each other's private state.
- Define conformance tests for command handling, event emission, policy evidence, idempotency, and replay.
- Provide the base rules for the later PIP process.
- Keep the protocol aligned with non-blockchain ORU and Seal Ledger accounting, Overasset rights, native-service public utility behavior, central AI evidence boundaries, and grid-resident backbone operation.

## Non-Goals

- Do not implement a deployed control-plane service under this name in early phases.
- Do not duplicate the shared schema package; Protocol Core defines rules while the schema package implements typed schemas and validators.
- Do not make runtime policy decisions; Overguard and domain services decide from facts.
- Do not store production records.
- Do not bypass service SDS ownership boundaries.
- Do not encode pricing, customer-count, or market assumptions.

## Primary Actors And Clients

- Service implementers using protocol rules to build APIs and workers.
- Shared schema package maintainers turning rules into schemas and validators.
- SDK, CLI, and admin UI builders relying on stable envelopes and errors.
- Integration-test harness and conformance-test authors.
- Governance and PIP maintainers evolving the protocol.
- Native app, adapter, and mobile teams building on the same service rails.

## Dependencies

- Whitepaper architecture and principles.
- Build-plan phase order.
- Shared schema package for typed implementation of protocol objects.
- Service catalog and SDS layer for concrete ownership boundaries.
- Integration test harness for conformance execution.
- PIP registry in Phase 13 for formal protocol evolution.

## Owned Responsibilities

Protocol Core owns:

- Canonical command envelope rules.
- Canonical event and audit envelope rules.
- Error shape and reason-code discipline.
- Idempotency, trace id, timestamp, signature, and replay-window rules.
- State-machine naming and transition evidence rules.
- Schema versioning, deprecation, compatibility, and migration requirements.
- Service boundary rules, including private storage boundaries and event refs.
- Conformance fixture definitions and acceptance criteria.
- PIP handoff rules for protocol changes.

Protocol Core does not own runtime data, credential verification, tenancy decisions, policy decisions, scheduling, accounting, or evidence storage.

## Data Model

Protocol Core should define specification artifacts:

- `protocol_spec`: spec id, version, status, scope, owner, source refs, compatibility class, and PIP refs.
- `command_envelope_rule`: required fields, idempotency rules, signature rules, trace rules, tenant and actor refs, payload type rules, and denial shape.
- `event_envelope_rule`: event id, source service, subject, tenant, actor, event type, sequence, occurred time, trace id, schema version, privacy classification, and evidence refs.
- `state_machine_definition`: domain, allowed states, allowed transitions, terminal states, correction model, required events, and replay expectations.
- `service_boundary_rule`: owner service, allowed writers, allowed readers, private storage rule, emitted events, consumed events, and forbidden shortcuts.
- `compatibility_rule`: additive, deprecated, breaking, migration-required, and retired change classes.
- `conformance_fixture`: fixture id, protocol version, valid payloads, invalid payloads, expected reason codes, and service applicability.
- `pip_reference`: proposal id, status, accepted version, migration notes, rollback notes, and security/privacy/economic impact refs.

These artifacts should live as docs, schemas, fixtures, and generated reports rather than production service rows.

## API Surface

Protocol Core exposes package and documentation interfaces:

- `protocol/specs`: versioned Markdown or structured spec files.
- `protocol/conformance`: fixture sets for command, event, state, idempotency, error, audit, and replay behavior.
- `protocol/check`: local validation command used by CI to test service APIs and events against protocol rules.
- `protocol/report`: generated conformance and compatibility reports.
- `protocol/reason-codes`: canonical reason-code registry grouped by service family and failure class.
- `protocol/state-machines`: canonical state-machine definitions and allowed transition maps.

Any HTTP API would be an implementation convenience around these artifacts, not the source of protocol truth.

## Event Surface

Protocol Core does not emit runtime platform events. It defines which events other services must emit.

Required event rules:

- Every accepted mutating command must emit an acceptance or state event.
- Every denial must include a stable reason code and trace id.
- Every service-owned state transition must emit an append-only event.
- Every event must cite schema version and privacy classification.
- Every correction must preserve prior evidence rather than overwriting history.

Build and CI tools may generate conformance reports, but those reports are artifacts rather than runtime governance events.

## Core Workflow

1. Write or update protocol rule text and structured definitions.
2. Update shared schemas and fixture sets.
3. Run compatibility classification against the prior protocol version.
4. Run conformance tests against services, SDK, CLI, and integration harness fixtures.
5. Publish a protocol version only when schemas, docs, and conformance reports align.
6. For later changes, route non-trivial changes through the PIP process with security, privacy, compatibility, migration, and rollback sections.

## State Machine

Protocol artifact lifecycle:

1. `draft`: proposed rule exists but is not binding.
2. `reviewed`: rule has owner, scope, compatibility notes, and test plan.
3. `implemented_in_schema`: shared schema package contains matching definitions.
4. `conformance_ready`: fixtures and tests exist.
5. `accepted`: protocol version is binding for new work.
6. `deprecated`: rule remains supported but has replacement and migration notes.
7. `retired`: rule is no longer valid for new work after migration.
8. `superseded`: newer protocol version replaces it.

Runtime services must use their domain-specific lifecycle states defined by the protocol and their SDS files.

## Policy And Security

- Protocol rules must enforce tenant, actor, trace, idempotency, signature, schema version, privacy classification, and reason-code requirements.
- Security-sensitive protocol changes require explicit migration and rollback guidance.
- Private data and secret-bearing payloads must use refs and privacy labels.
- No service may gain hidden write access to another service's private state through protocol ambiguity.
- PIP governance must handle breaking protocol changes after Phase 13 begins.
- Conformance tests must fail closed when required evidence is missing.

## Metering And Accounting

Protocol Core does not meter usage. It defines accounting-compatible rules:

- Usage events must include resource dimensions, tenant refs, actor refs, workload refs, and evidence refs.
- ORU and Seal Ledger objects must avoid blockchain and NFT mechanics and support low-friction internal settlement.
- Accounting state transitions must be replayable from signed usage, policy, and ledger evidence.
- HTTP 402-style machine-to-machine settlement flows must use signed events, policy limits, batching, rollups, and settlement refs rather than per-operation friction.
- Native service surplus handling belongs in stewardship mechanisms, not protocol-level charge assumptions.

## Observability And Operations

Protocol Core should provide:

- Conformance reports by service, SDK, CLI, and adapter.
- Compatibility reports by protocol version.
- Coverage reports showing which services implement required command and event rules.
- Reason-code coverage reports.
- State-machine coverage reports.
- Drift detection when service SDS, schemas, and implemented behavior disagree.
- PIP traceability once governance begins.

## Failure Modes And Recovery

- Service implements a private envelope: fail conformance and block release.
- Schema package diverges from protocol rule: block protocol publication or schema release.
- Breaking change lacks migration notes: mark as blocked.
- Runtime service emits unversioned or unclassified event: fail conformance.
- Protocol ambiguity creates ownership overlap: update SDS and protocol boundary rules before implementation proceeds.
- PIP accepted but schema not updated: keep protocol version pending until implementation artifacts match.

## Validation Plan

The service implementation plan lists these specific validation requirements:

- Services use the same command envelope and event rules.
- Lifecycle transitions are deterministic and testable.
- Protocol additions include migration and compatibility notes.

Additional SDS-level validation:

- Conformance fixtures cover valid and invalid command envelopes, events, state transitions, denials, idempotency, and audit refs.
- Shared schema package imports or generates from protocol definitions without manual drift.
- Integration tests prove one signed command can traverse Overgate, Overregistry, Overwatch, and Overqueue using the same trace id and event rules.
- Compatibility tests classify every protocol change.
- PIP simulation proves a later protocol change can be proposed, reviewed, accepted, migrated, and rolled back.

## Build Breakdown

1. Define the first protocol spec structure and status lifecycle.
2. Define command envelope, event envelope, audit, error, trace, idempotency, signature, and schema-version rules.
3. Define shared state-machine requirements for identity, tenant, key, registry, queue, lease, execution, usage, ledger, dispute, and governance flows.
4. Define service ownership boundary rules and forbidden direct-storage shortcuts.
5. Add conformance fixtures and reports to the integration test harness.
6. Connect shared schema package generation and validation to protocol definitions.
7. Move protocol changes into the PIP process in [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md).

## Handoff And Downstream Use

This governs all service implementation and becomes the base for the PIP registry.

Downstream services should treat Protocol Core as the common rulebook and use shared schemas plus conformance tests for implementation. If a service needs a protocol exception, update this SDS, the affected service SDS, shared schemas, and the PIP path when governance is active.

## Open Design Questions

Resolved decisions:

- Machine-readable Protocol Core definitions start with canonical JSON plus JSON Schema in the shared schema/protocol packages, not a deployed protocol service. Day-one machine-readable families are command envelopes, event and audit envelopes, API error shape, stable reason-code registry, trace/idempotency/signature/schema-version/privacy-label primitives, service-boundary metadata, compatibility classifications, conformance fixture manifests, golden trace schemas, and Phase 0/1 state-machine definitions for identity, tenant, key, registry, audit, and queue flows. Markdown remains the explanatory rulebook, while Rust validation, generated Rust types, fixtures, reports, and CI checks consume the machine-readable files. Protobuf may be generated or introduced later only for compact internal RPC or event contracts where binary compatibility matters; it must not replace JSON Schema as the docs-facing and fixture-facing protocol authority.
- Before Phase 13 creates the formal PIP workflow, only low-risk compatible changes can be accepted without a PIP: additive optional fields on non-authority objects, new reason codes that do not change existing semantics, documentation clarifications, new valid/invalid fixtures, conformance report improvements, compatibility metadata, and validator tightening that rejects payloads already invalid under the accepted rule text. Each such change still needs a protocol compatibility record, updated schemas/fixtures/docs, and passing conformance checks. Changes touching command or event envelopes, signatures, tenant/actor identity, service writer boundaries, privacy classifications, secret refs, accounting/ledger/rights, namespace ownership, policy enforcement, deletion/retention, stable SDK behavior, public report semantics, or accepted state transitions require an explicit SDS/schema/build-plan update before Phase 13 and must move through a PIP once the PIP registry exists.
- The minimum Phase 1 conformance suite applies to every service participating in the Phase 1 control-plane path and to every shared package used by that path. It must prove strict command envelope validation, generated-schema validator use, tenant and actor refs, signature checks where required, trace propagation, idempotency replay and conflict behavior, stable denial reason codes, Overwatch-compatible accepted/denied/state-transition events, append-only audit refs, privacy labels on emitted events, and deterministic state-machine transitions. The required end-to-end golden trace is a signed tenant-scoped synthetic workload command admitted through Overgate, backed by Overpass/Overtenant/Overkey refs, accepted into Overregistry, recorded in Overwatch, and placed into durable Overqueue pending state; invalid schema, invalid signature, duplicate idempotency key, conflicting duplicate, and missing-tenant paths must fail with expected reason codes before side effects.
- Service SDS ownership conflicts are release blockers, not implementation choices for individual services. Protocol Core may expose the conflict, but resolution must name one owning writer service, allowed readers, emitted and consumed event refs, private-storage boundaries, migration notes, and forbidden shortcuts. The affected service SDS files, service catalog files, shared schemas, conformance fixtures, and build-plan crosswalk must be updated together before implementation proceeds. Until the conflict is resolved, services must communicate by refs/events through their public contracts and must not add dual writers, direct private-table reads, or hidden bypass APIs. Before Phase 13, unresolved conflicts are settled by explicit SDS/build-plan decision evidence; after Phase 13, non-trivial boundary changes require a PIP with compatibility, migration, rollback, security, and privacy sections.
- Public governance reporting should expose protocol trust signals without exposing private platform evidence. Once Phase 13 reporting begins, public reports may include accepted protocol versions, compatibility classes, deprecation and migration windows, PIP refs and redacted summaries, service conformance status by service family and build phase, reason-code coverage, state-machine coverage, public incident or rollback refs, and known public exceptions with remediation status. Public reports must not include tenant ids, actor ids, raw command or event payloads, trace bodies, private audit refs, secret refs, fraud heuristics, sensitive topology, embargoed security details, payment/compliance evidence, or any private user data. Detailed evidence remains in Overwatch, PIP, security, compliance, and stewardship records with redacted public summaries and stable evidence refs.
