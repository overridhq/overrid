# Overmark Implementation Plan

## Objective

Provide resource valuation, reference rate bands, resource cards, placement cost signals, and bounded market metadata without speculative token mechanics.

## First Build Phase

[Phase 5: Metering, ORU, Seal Ledger, and Overbill](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md).

## Dependencies

- Overregistry capability records.
- Overmeter dimensions.
- Oversched placement.
- ORU account model.

## Development Order

1. Define resource card schema.
2. Add capability tier, trust tier, locality, availability, and resource dimension markers.
3. Add bounded reference rate bands.
4. Expose placement signals to Oversched and budget previews to Overguard.
5. Add audit history for rate changes.

## Contracts And Interfaces

- Resource card schema.
- Reference band records.
- Placement signal API.
- Budget preview refs.

## Validation

- Placement can use Overmark signals without hardcoding rates.
- Rate changes are versioned and auditable.
- No service depends on speculative token pricing.

## Handoff

Overmark informs scheduling, budgeting, billing previews, grants, and native service metering.

## Detailed SDS

The detailed design contract lives in [Overmark SDS](../../sds/accounting/overmark.md).

## Design Alignment

- Treat Overmark as versioned resource-card, reference-band, budget-preview, and placement-signal infrastructure, not a spot market, payout oracle, exchange, or speculative token-pricing service.
- Publish immutable card/band versions with source evidence, effective windows, supersession, and replay support.
- Feed Oversched, Overguard, Overgrant, Overbill, native apps, SDK, CLI, and Wallet and Usage Center with refs that downstream decisions can store and replay.
- Keep actual usage truth in Overmeter/Seal Ledger and final billing or payout behavior in Overbill/Provider Payout Service.
