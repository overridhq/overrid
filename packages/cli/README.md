# Overrid CLI

This package is the Phase 2 Rust CLI skeleton for `SUB BUILD PLAN #2 - CLI`.

It is a Cargo workspace member and depends on the Rust contract projection in
`packages/schemas/overrid_contracts` plus the SDK transport skeleton in
`packages/sdk`. The package must remain a Rust CLI runtime. TypeScript bindings
may exist elsewhere as generated client projections, but they are not this CLI's runtime.

The canonical CLI schema source is
`packages/schemas/overrid_contracts/v0/cli_command.schema.json`; the Rust
projection is consumed by this crate but is not the contract authority.

Phase 2 validates the crate boundary, generated-contract compatibility checks,
Overgate-only SDK target validation, parser conventions, and contract-shaped
fixtures before later phases add mutating command behavior.
