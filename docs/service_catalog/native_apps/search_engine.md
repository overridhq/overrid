# Search Engine Implementation Plan

## Objective

Build permission-aware search across public Overrid content, native directory listings, public app pages, public-interest datasets, and user-authorized private content.

## First Build Phase

[Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md).

## Dependencies

- Overbase lexical, document, secondary, and vector indexes.
- Overstore chunk and artifact refs.
- Overvault private grant refs.
- Overpass namespace.
- Universal Namespace Service.
- Overguard.
- Overmeter.

## Development Order

1. Build public content and directory indexing.
2. Add permission-aware private/workspace indexing.
3. Add app page and public-interest dataset search.
4. Add spam and abuse controls.
5. Add transparent ranking controls that avoid marketplace ad extraction.

## Contracts And Interfaces

- Index record schema.
- Search query API.
- Permission filter contract.
- Ranking explanation fields.

## Validation

- Private content only appears to authorized users.
- Directory and public app search are usable without ad-dependent ranking.
- Search usage is metered where applicable.

## Handoff

Search supports directory, workspace, messaging, maps, personal AI, and native app discovery.

## Detailed SDS

The detailed design contract is [Search Engine SDS](../../sds/native_apps/search_engine.md).

## Sub-Build Plan

- [SUB BUILD PLAN #72 - Search Engine](../../build_plan/sub_build_plan_072_search_engine.md)

## Design Alignment

- Treat Search Engine as a permission-aware discovery utility, not as an ad auction, canonical data store, private-data bypass, or paid-ranking marketplace.
- Require source registrations, source policies, index jobs, index records, permission filter snapshots, query sessions, result sets, ranking explanations, removals/tombstones, abuse reports, and usage refs.
- Apply source-owned permission filters before snippets, previews, embeddings, ranking explanations, result handoffs, and Personal AI Assistant citations.
- Ranking must be inspectable and non-ad-dependent so directory listings, public app pages, public-interest datasets, and small organizations remain discoverable without paid placement.
- Build the first substrate as Search-owned Overbase indexes with Overstore refs for large chunks/artifacts and Overvault grants for private or sensitive material; external search/database products are not the product boundary.
