# Node Installer Implementation Plan

## Objective

Make seed and provider node onboarding repeatable and auditable.

## First Build Phase

[Phase 2: Seed Private Swarm](../../build_plan/phase_02_seed_private_swarm.md).

## Dependencies

- Overcell node agent.
- Overkey credential enrollment.
- Overgate registration endpoint.
- Host service supervisor.

## Development Order

1. Package the node-agent for supported operating systems.
2. Add enrollment and config validation.
3. Install supervised service files.
4. Add update, drain, uninstall, and diagnostics commands.
5. Record installer version and host facts in Overwatch.

## Contracts And Interfaces

- Install command.
- Enrollment token or credential flow.
- Node-agent config file.
- Diagnostics bundle format.

## Detailed SDS

The detailed design contract lives in [Node Installer SDS](../../sds/execution_scheduling/node_installer.md).

## Design Alignment

- Treat Node Installer as bootstrap and lifecycle tooling for Overcell, not remote shell access or scheduling infrastructure.
- Verify signed bundles before side effects, enroll scoped node credentials through Overkey, and write minimal protected local config.
- Make install, rerun, upgrade, drain, diagnostics, rollback, and uninstall flows idempotent and audit-backed.
- Preserve local host consent and privilege boundaries while recording install-session evidence for later provider onboarding.

## Validation

- Installer can enroll a clean seed node.
- Re-running installer is idempotent.
- Drain and uninstall preserve audit records.

## Handoff

This feeds Overcell registration and later public provider onboarding.
