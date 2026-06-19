# Overrid Protocol Core Implementation Plan

## Objective

Define the protocol rules that make all Overrid services behave like one coherent resource allocation ecosystem.

## First Build Phase

[Phase 0: Foundation](../../build_plan/phase_00_foundation.md).

## Dependencies

- Whitepaper architecture.
- Build plan phases.
- Shared schema package.

## Development Order

1. Define command, event, state, versioning, and compatibility rules.
2. Define workload lifecycle states and allowed transitions.
3. Define service ownership boundaries.
4. Define conformance tests for command handling, audit, and policy evidence.
5. Move protocol changes into the PIP process in [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md).

## Contracts And Interfaces

- Protocol specs.
- Shared schemas.
- Conformance tests.
- State machine definitions.

## Detailed SDS

- [Overrid Protocol Core SDS](../../sds/control_plane/overrid_protocol_core.md)

## Design Alignment

- Treat Protocol Core as a specification and conformance layer, not a deployed runtime service.
- Define command envelope, event envelope, idempotency, trace, error, state-machine, versioning, compatibility, and ownership-boundary rules.
- Keep shared schemas as the typed implementation of protocol objects while Protocol Core remains the rulebook.
- Use conformance fixtures to detect services that drift from command, event, state, audit, or error rules.
- Route later non-trivial changes through the PIP process with migration and rollback notes.

## Validation

- Services use the same command envelope and event rules.
- Lifecycle transitions are deterministic and testable.
- Protocol additions include migration and compatibility notes.

## Handoff

This governs all service implementation and becomes the base for the PIP registry.
