# Overrid

Overrid is a distributed infrastructure and native application ecosystem built to replace the broken internet with public-interest rails for compute, storage, identity, AI, native apps, and resource accounting.

The premise is direct: the current internet concentrates infrastructure, identity, data, marketplaces, social feeds, AI compute, and app distribution into a few companies. That model turns people into locked-in users, extracts personal data, pushes addictive design, inflates infrastructure costs, and makes builders pay gatekeepers. Overrid is designed as a different system: owned by no corporation, run across participant resources, governed through evidence, and built around native services that pay for their actual resource usage.

## What Overrid Builds

- A resource grid for participant-owned compute, GPU, storage, network, and service capacity.
- A control plane for identity, tenant boundaries, manifests, policies, queues, leases, execution, metering, audit, and dispute evidence.
- Native data primitives: Overbase for structured state, Overstore for object and artifact persistence, Overvault for encrypted private state, and a universal namespace layer.
- Utility accounting through ORU credits, Seal Ledger, Overmark, Overbill, Overgrant, and Overasset, without blockchain, NFT, or per-transaction fee mechanics.
- AI infrastructure for personal AI assistants, central AI stewardship, encrypted Docdex RAG, ADES enrichment, lightweight routing, and model selection by job nature.
- Native public applications: wallet, workspace, directory listings, search, messaging, social photo/video sharing, maps/navigation, personal AI, and central AI stewardship tools.
- A mobile service layer so mobile apps can use Overrid identity, storage, messaging, AI, payments, and resource-backed services through native Overrid APIs.

## Repository Map

| Path | Purpose |
| --- | --- |
| [docs/whitepaper.md](docs/whitepaper.md) | Project narrative, principles, system model, native app vision, and diagrams. |
| [docs/overrid_tech_stack_choice.md](docs/overrid_tech_stack_choice.md) | Accepted technology stack decision. |
| [docs/build_plan/master_plan.md](docs/build_plan/master_plan.md) | Canonical phase order for building Overrid. |
| [docs/build_plan](docs/build_plan) | Phase-level build plans and service alignment. |
| [docs/service_catalog/master_services.md](docs/service_catalog/master_services.md) | Master catalog of tools, services, apps, adapters, and support modules. |
| [docs/service_catalog](docs/service_catalog) | Per-service implementation plans grouped by category. |
| [docs/sds/master_sds.md](docs/sds/master_sds.md) | High-level software design specification and document map. |
| [docs/sds/service_sds_index.md](docs/sds/service_sds_index.md) | Index of detailed SDS documents for every planned service. |

## Build Status

Overrid is currently in design and specification. The repository contains the whitepaper, build plan, service catalog, and service SDS layer that define the first implementation path.

The first hardware will bootstrap the private grid, but the backbone is designed to migrate into protected grid-resident system workloads. Founder machines are the start point, not the permanent dependency.

## Design Rules

- Build Overrid primitives as Overrid services, not as wrappers around conventional SaaS or cloud products.
- Keep ORU, Seal Ledger, and Overasset as utility/accounting/rights infrastructure, not speculative crypto mechanics.
- Treat public nodes as untrusted until verification, workload-sensitivity limits, challenge checks, abuse controls, and payout holds exist.
- Keep native apps non-profit oriented public utilities: revenue should cover resource usage and operating margin, with surplus directed toward projects approved through stewardship.
- Do not add revenue projections, pricing assumptions, user-count assumptions, or speculative fundraising claims to canonical docs.

## Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request. Keep public docs focused on the actual system, and do not commit local agent state, scratch planning trails, raw source imports, generated build output, or secrets.

## License

MIT License. Copyright (c) 2026 Bekir Dağ.
