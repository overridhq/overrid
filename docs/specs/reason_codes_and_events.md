# Reason Codes And Events

## Purpose

Define where stable reason codes, event envelopes, audit records, validation artifacts, and error shapes live before mutating service logic is accepted.

## Placement Rules

- Canonical reason-code and event schemas live under `packages/schemas`.
- Human-readable reason-code and event explanations live under `docs/specs` or owning SDS/service-contract docs.
- Service contracts must cite schema refs before emitting or consuming mutating events.
- Validation artifacts for layout checks remain CI/build artifacts, not Overwatch runtime events.
- Error shapes must keep stable reason-code families, trace refs, schema versions, retry classes, and safe remediation metadata.

## Required Contract Families

Mutating service contracts must cite the relevant source for:

- command admission reason codes;
- policy decision reason codes;
- validation failure reason codes;
- event envelopes;
- audit record envelopes;
- validation artifacts;
- API/CLI/SDK error shapes.

## Boundary Rules

Reason-code and event contracts must not contain raw secrets, private keys, bearer tokens, private payloads, decrypted RAG content, direct private storage paths, direct ledger mutation paths, pricing assumptions, revenue forecasts, blockchain assumptions, or NFT mechanics.

## Current Phase 4 Sources

- `packages/schemas/overrid_contracts/v0/cli_command.schema.json`
- `packages/schemas/overrid_contracts/v0/integration_harness.schema.json`
- `packages/schemas/overrid_contracts/v0/local_development_stack.schema.json`
- `packages/schemas/admin_ui/v0/admin_ui_contracts.schema.json`
- `packages/schemas/admin_ui/v0/admin_read_api_contracts.schema.json`

## Validation

`scripts/validate_repository_layout_phase4.py` checks this placement rule, schema package metadata, and service-contract template coverage.
