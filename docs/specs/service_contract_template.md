# Service Contract Template

## Purpose

Define the required content for service/module contract stubs before implemented service logic is accepted.

## Required Sections

Each implemented service/module contract must include:

- Purpose
- Owned Data
- Public API
- Events Emitted
- Events Consumed
- Security Boundary
- Operational Checks
- Test Expectations
- Schema Refs
- Owning Phase
- Downstream Dependencies

## Section Rules

### Purpose

State the bounded responsibility of the service or module and name the owning SDS/build-plan source.

### Owned Data

List state the service owns. Do not claim direct access to another service's storage, queue, ledger, vault, or object layer.

### Public API

Name public commands, API routes, SDK methods, CLI commands, or contract endpoints. Public API records must cite `packages/schemas` or a documented no-public-contract reason.

### Events Emitted

List emitted event families and schema refs. Runtime platform events belong to owning service contracts, not Repository Layout itself.

### Events Consumed

List consumed event families and schema refs. Consumers must not read private stores or test/local-only internals directly.

### Security Boundary

Describe tenant, actor, signing, key, policy, secret, redaction, and audit boundaries. Contracts must not expose raw secrets or private payloads.

### Operational Checks

Name health, readiness, audit, retention, recovery, and validation checks relevant to the service/module.

### Test Expectations

Name unit, integration, schema, docs, local-stack, fixture, or release checks required before acceptance.

### Schema Refs

List canonical JSON Schema files and optional internal Protobuf specs. Generated/projection code is not enough.

### Owning Phase

Name the master phase, sub-build plan, and lifecycle state that justify the service/module boundary.

### Downstream Dependencies

List downstream consumers, local-stack participation, integration harness scenarios, generated bindings, and service/module dependencies that must be updated when this contract changes.

## Usage Notes

- Use this template before service logic is accepted, not after implementation has already created an implicit boundary.
- Pair every accepted module contract with a module record in `overrid.workspace.toml` and the checklist in `new_module_checklist.md`.
- Keep the contract path under `docs/specs` unless an SDS or service catalog document is the documented equivalent.
- Do not treat generated Rust, TypeScript declarations, examples, or fixture output as the contract source of truth.

## Validation

Docs checks reject implemented modules without these sections unless the module record names a documented equivalent contract path and no-public-contract reason.
