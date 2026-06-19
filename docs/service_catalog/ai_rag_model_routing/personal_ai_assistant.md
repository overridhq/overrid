# Personal AI Assistant Implementation Plan

## Objective

Build the user's everyday native AI surface using central AI coordination, encrypted Docdex RAG, model/resource routing, permissions, privacy, and ORU metering.

## First Build Phase

[Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md), with integration groundwork in [Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md).

## Dependencies

- Encrypted Docdex RAG adapter.
- AI gateway router.
- Overpass.
- Overvault.
- ORU account service.
- Wallet app.

## Development Order

1. Build user chat/task interface and permission model.
2. Add context source selection with encrypted Docdex indexes.
3. Add gateway routing for model/tool/resource selection.
4. Add usage display and privacy audit.
5. Add delegated calls to other native apps and ecosystem tools.

## Contracts And Interfaces

- Assistant request schema.
- Context source refs.
- Tool call contract.
- Usage and receipt refs.
- Privacy/audit record.

## Validation

- Assistant can answer with authorized RAG context only.
- Simple tasks route to smaller resources when appropriate.
- Usage is metered and visible in wallet.

## Handoff

Personal AI becomes the user-facing AI layer for workspace, messaging, search, directory, and mobile clients.

## Detailed SDS

- [Personal AI Assistant SDS](../../sds/ai_rag_model_routing/personal_ai_assistant.md)

## Design Alignment

The SDS refines this implementation plan as a permission-first native AI application. It owns sessions, turns, permission manifests, context-source selections, tool-call proposals, delegated native-app calls, privacy audit, and usage receipt refs, while keeping raw app data, encrypted RAG authorization, policy enforcement, model routing, and accounting truth in their owning services.
