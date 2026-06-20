SDS #42

# Overmark SDS

## Purpose

Provide versioned resource cards, bounded reference rate bands, placement cost signals, budget-preview facts, and non-speculative resource metadata.

Overmark helps Overrid compare resource use in a predictable way. It is not a market exchange, auction engine, token-pricing oracle, or provider-payout authority. It publishes bounded, auditable signals that scheduling, policy dry-runs, budgeting, grants, billing previews, and native services can use without hardcoding rates or hiding GPU, storage, network, memory, data, and service-unit costs.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [overmark.md](../../service_catalog/accounting/overmark.md) |
| Sub-build plan | [SUB BUILD PLAN #42 - Overmark](../../build_plan/sub_build_plan_042_overmark.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 5: Metering, ORU, Seal Ledger, and Overbill](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md) |

## Service Family

- Family: Accounting, credits, billing, and rights
- Owning layer: resource cards, reference bands, capability/trust/locality markers, budget previews, placement signals, and rate-change audit history
- Primary data scope: resource card versions, resource-class dimensions, capability tiers, trust tiers, availability snapshots, locality metadata, reference bands, budget preview refs, placement signal refs, and audit refs
- First build phase from service plan: [Phase 5: Metering, ORU, Seal Ledger, and Overbill](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md).

## Problem Statement

Overrid cannot make scheduling, budgeting, grants, billing previews, and native-service metering depend on scattered constants or speculative market pricing. The system needs a shared way to describe resource classes and bounded reference bands while keeping actual accounting in ORU and Seal Ledger.

The ill-design to avoid is turning Overmark into a marketplace price oracle that decides who gets paid or what users owe. Overmark should publish versioned signals and references. Overmeter, Seal Ledger, Overbill, Overgrant, Provider Payout Service, and policy services consume those signals but remain responsible for their own decisions.

## Goals

- Define resource cards for CPU, GPU, storage, network, memory, data, model/service, and mixed workload resources.
- Publish capability tier, trust tier, locality, availability, and resource dimension markers.
- Maintain bounded reference bands with explicit version, effective window, source evidence, and review history.
- Produce budget-preview facts for policy dry-runs, grants, native apps, SDK, CLI, and admin UI.
- Produce placement cost hints for Oversched without replacing scheduling policy.
- Keep rate changes versioned, auditable, reversible by superseding records, and explainable.
- Prevent speculative token pricing, hidden per-operation fee logic, or provider payout manipulation.

## Non-Goals

- Do not calculate final invoices, receipts, payment obligations, or provider payouts.
- Do not append Seal Ledger entries or mutate ORU balances.
- Do not own raw usage events or signed rollups.
- Do not replace Overguard admission policy or Oversched placement decisions.
- Do not implement an exchange, auction, order book, spot market, speculative index, NFT valuation, or token pricing mechanism.
- Do not encode business forecasts, customer counts, revenue targets, or market-volume assumptions.
- Do not let providers self-publish rates directly into scheduler or billing behavior without review/versioning.

## Primary Actors And Clients

- Oversched, reading placement cost hints and resource-class facts.
- Overguard and Policy Dry-Run API, reading budget-preview facts and bounded reference bands.
- Overmeter, aligning usage rollups to explicit resource dimensions and resource cards.
- Overbill, producing billing previews and receipts that cite the reference version used.
- Overgrant, defining grant quotas and reporting in consistent resource dimensions.
- ORU Account Service and Wallet and Usage Center, showing projected usage cost by dimension.
- Native services, estimating near-cost service usage and resource budgets.
- Admin UI, CLI, SDK, and central AI stewardship, reading published cards and rate-change history.

## Dependencies

- [Overregistry](../control_plane/overregistry.md) for accepted resource capability manifests and package/runtime capability refs.
- [Overmeter](../execution_scheduling/overmeter.md) for usage dimensions and normalization requirements.
- [Oversched](../execution_scheduling/oversched.md) for placement decisions that consume cost hints.
- [ORU Account Service](oru_account_service.md) for account dimension vocabulary and budget precheck expectations.
- [Oververify](../trust_policy_verification/oververify.md) for trust-tier and eligibility evidence where relevant.
- [Hardware Discovery](../execution_scheduling/hardware_discovery.md) and [Benchmark Runner](../execution_scheduling/benchmark_runner.md) for observed capability and measured capacity evidence.
- [Overwatch](../control_plane/overwatch.md) for audit events and change-history evidence.

## Owned Responsibilities

Overmark owns:

- Resource card schemas and published resource card versions.
- Resource dimension mapping for CPU-ORU, GPU-ORU, STOR-ORU, NET-ORU, MEM-ORU, DATA-ORU, Service-ORU, and future explicit dimensions.
- Capability tier, trust tier, locality, availability, and workload-class markers used for budgeting and placement hints.
- Bounded reference bands with effective windows, review status, source evidence, and supersession history.
- Budget-preview facts for policy dry-runs and app/native-service planning.
- Placement signal refs that Oversched may use as one input among policy, trust, availability, quota, and lease constraints.
- Rate-change proposal, review, publication, deprecation, supersession, and rollback-by-supersession records.
- Replay bundles explaining which resource card and reference band version a downstream decision used.

Overmark does not own actual usage truth, ledger balances, billing documents, provider payout amounts, final scheduling, or policy admission.

## Data Model

The first implementation must define:

- `resource_card`: stable id, resource class, dimensions, capability traits, measurement unit, normalization notes, schema version, and status.
- `capability_tier_marker`: tier id, hardware/runtime evidence refs, minimum capability thresholds, and compatible workload classes.
- `trust_tier_marker`: trust/verification tier refs and allowed use contexts for budget/placement estimation.
- `locality_marker`: region, network locality, private swarm, trusted federation, or public low-sensitivity scope.
- `availability_snapshot_ref`: reference to capacity/availability evidence, not a real-time scheduler lock.
- `reference_band`: lower/upper bounded ORU range or budget hint by dimension, card, class, and effective window.
- `budget_preview`: deterministic estimate input bundle and output bundle for a workload/app/native-service plan.
- `placement_signal`: scheduler-facing signal with reason codes and the versions of cards/bands used.
- `rate_change_record`: proposal, review, publication, supersession, rollback-by-supersession, and audit refs.

Common envelope fields:

- `id`
- `tenant_id` where scoped, or `system_scope`
- `actor_id` or `service_account_id`
- `trace_id`
- `idempotency_key`
- `state`
- `effective_from`
- `effective_until`
- `schema_version`
- `source_evidence_refs`
- `policy_refs`
- `audit_refs`
- `created_at`
- `updated_at`

## API Surface

Phase 5 should expose:

- `POST /resource-cards`: propose a resource card version.
- `POST /resource-cards/{id}/publish`: publish a reviewed resource card version.
- `GET /resource-cards`: query cards by resource class, dimension, capability tier, trust tier, locality, and state.
- `POST /reference-bands`: propose a bounded reference band.
- `POST /reference-bands/{id}/publish`: publish a reviewed band version.
- `GET /reference-bands/resolve`: resolve the active band for a card, dimension, workload class, tenant scope, and time.
- `POST /budget-previews`: create a deterministic budget-preview fact bundle for a workload/app/native-service request.
- `POST /placement-signals`: create a scheduler-facing signal ref from card, band, availability, and constraint inputs.
- `GET /versions/{id}/replay`: return the card/band/preview decision bundle for audit and dispute review.
- `POST /versions/{id}/deprecate`: deprecate or supersede a published version with reason codes.

API rules:

- Published versions are immutable; changes create new versions.
- Reference band resolution must return the exact version and effective window used.
- Budget previews are estimates and must be labeled as such.
- Mutating commands require signed actor/service identity, idempotency key, trace id, and Overwatch audit refs.
- Reads used by public/native views must redact provider-sensitive evidence where a card or band ref is enough.

## Event Surface

- `overmark.resource_card_proposed`
- `overmark.resource_card_published`
- `overmark.resource_card_deprecated`
- `overmark.reference_band_proposed`
- `overmark.reference_band_published`
- `overmark.reference_band_superseded`
- `overmark.budget_preview_created`
- `overmark.placement_signal_created`
- `overmark.version_replay_requested`
- `overmark.validation_denied`

Events must include card/band version, resource dimensions, effective window, source evidence refs, actor/service account id, trace id, and stable reason codes. Events must not expose private capacity evidence to unauthorized consumers.

## Core Workflow

1. Resource capability and measurement evidence arrives from Overregistry, Hardware Discovery, Benchmark Runner, Overmeter, or operator-reviewed sources.
2. Authorized actor or service proposes a resource card or reference band version.
3. Overmark validates schema, source evidence, dimension compatibility, effective window, and policy requirements.
4. Reviewed versions are published and become resolvable for budget previews and placement signals.
5. Overguard, Oversched, Overgrant, Overbill, SDK, CLI, native apps, and admin UI resolve versions for their own decisions.
6. Downstream records store the resolved Overmark version refs.
7. New evidence or policy changes supersede old versions without rewriting historical decisions.

## State Machine

Resource card and reference band states:

1. `draft`: proposal is incomplete and not resolvable.
2. `submitted`: version has been submitted for validation.
3. `validated`: schema, dimension, and source refs are valid.
4. `review_required`: policy or operator review is required.
5. `published`: version can be resolved by downstream services.
6. `deprecated`: version remains valid for historical replay but should not be used for new decisions.
7. `superseded`: a newer version replaces it for new decisions.
8. `retracted`: version is blocked from new use due to error or policy issue; historical users must keep replay refs.
9. `corrected`: correction metadata points to a replacement or remediation record.

Budget preview states:

- `requested`
- `created`
- `expired`
- `superseded`
- `replayed`

## Policy And Security

- Overmark must be deny-by-default when resource cards, evidence refs, effective windows, or policy facts are ambiguous.
- Provider-submitted evidence cannot become a published reference band without validation and review rules.
- Trust-tier and locality markers must not leak sensitive provider details to unauthorized readers.
- Public low-sensitivity resource cards must not imply eligibility for private, regulated, secret-bearing, or system-service workloads.
- Published reference bands must include source-evidence refs and review refs.
- Operator overrides require signed action envelopes and Overwatch evidence.
- Historical decisions must be replayed with their original Overmark versions, even after newer bands are published.

## Metering And Accounting

- Overmark supplies reference versions; it does not settle usage.
- Seal Ledger entries, Overbill documents, Overgrant reports, and Provider Payout batches must cite Overmark version refs where relevant.
- Resource dimensions must remain explicit rather than collapsing into one vague unit.
- Reference bands should help keep budgets predictable and near-cost, not serve as speculative profit targets.
- Internal ORU accounting remains independent of external payment rails and per-operation transaction fees.

## Observability And Operations

Expose:

- Published card count by resource class and dimension.
- Active reference bands by dimension, trust tier, locality, and workload class.
- Budget preview latency and denial counts.
- Placement signal counts and downstream consumers.
- Version supersession and retraction history.
- Evidence-validation failure rates.
- Stale source-evidence warnings.
- Downstream use of deprecated versions.

Operators need a version-diff view showing exactly what changed in each card or band and which downstream records still reference older versions.

## Failure Modes And Recovery

- Missing resource evidence: reject publication with `source_evidence_missing`.
- Conflicting dimensions: reject with `dimension_mismatch`.
- Effective-window overlap: either reject or require explicit supersession record.
- Bad reference band discovered after publication: retract for new use and publish correction/supersession; never rewrite historical refs.
- Provider evidence dispute: keep current version review-blocked or deprecated until Overclaim/Oververify evidence resolves.
- Downstream service caches stale band: require version id in every downstream decision so stale usage is traceable.
- Budget preview estimate fails: return missing inputs and reason codes rather than inventing costs.

## Validation Plan

Required tests:

- Published resource cards are immutable and versioned.
- Reference band resolution returns deterministic version ids for the same input/time.
- Rate changes are versioned and auditable.
- Placement can consume Overmark signals without hardcoding rates.
- Budget preview output cites all source cards, bands, and evidence refs.
- No API exposes speculative token pricing, NFT valuation, or unbounded market mechanics.
- Downstream examples for Overguard, Oversched, Overgrant, Overbill, and native apps can consume refs without direct storage access.
- Deprecated/superseded versions remain replayable.

## Build Breakdown

1. Define resource dimensions, card schema, and published-version invariants.
2. Define reference band schema with effective windows and source evidence refs.
3. Implement resource-card and reference-band proposal/publication APIs.
4. Implement deterministic resolution for active card/band versions.
5. Implement budget preview and placement signal APIs.
6. Add Overwatch events and replay endpoints.
7. Wire example consumers: Oversched placement input, Overguard dry-run budget, Overgrant quota definition, and Overbill receipt preview.
8. Add retraction, supersession, and stale-version alerts.

## Handoff And Downstream Use

Overmark feeds:

- Oversched placement and candidate comparison.
- Overguard and Policy Dry-Run API budget checks.
- Overgrant resource dimensions, quota definitions, and reports.
- Overbill receipts/statements and native-service cost previews.
- Wallet and Usage Center budget views.
- Central AI stewardship resource-allocation planning.

Downstream records must store Overmark version refs so later audits can replay historical decisions.

## Open Design Questions

Resolved decisions:

- Resource-band review uses review classes, not one global rule. Automated evidence validation is allowed for Phase 5 private/local bands when the card uses existing approved dimensions, current Hardware Discovery/Benchmark Runner/Overmeter evidence, bounded effective windows, no cross-tenant or public scope, no public reporting claim, and no new trust/locality semantics. Human or stewardship review is required for new resource dimensions or capability tiers, GPU/model classes that materially alter budget behavior, public or native-service-facing bands, cross-tenant or public-interest scope, regulated or safety-sensitive workloads, provider-submitted evidence conflicts, large band movement outside configured variance limits, retractions/corrections, and any exception that could affect grants, payouts, or public trust.
- Mixed GPU/model workloads must stay decomposed into explicit dimensions rather than one "AI cost" number. A budget preview should include GPU-ORU for accelerator time and memory pressure, CPU-ORU and MEM-ORU for host overhead, STOR-ORU, DATA-ORU, and NET-ORU for artifacts, retrieval, and transfer, and Service-ORU only for the bounded model or native-service unit that cannot be represented cleanly by lower-level resource counters. Model-specific facts live on the resource card as model family, context/window class, quantization/runtime adapter, cache/retrieval profile, token/request units, and source evidence refs; Overmeter rollups keep those dimensions separate and Overbill/Overgrant cite the exact Overmark version refs used.
- Public card fields are limited to catalog-safe resource class, coarse capability tier, supported workload classes, allowed data-sensitivity class, coarse region/locality, public or federation eligibility class, active/deprecated status, schema/version ids, effective window, redacted reference-band class, and public-safe reason codes. Provider/tenant restricted fields include exact node/provider ids, private-swarm membership, precise capacity/topology, raw benchmark and availability evidence, tenant-specific negotiated bands, grant/budget terms, payout/payment refs, workload/customer demand signals, fraud/risk flags, private locality routes, secret-bearing capability details, and raw audit/evidence bundles. Audit, steward, and operator views can resolve stronger refs through role-scoped policy; public/native views use redacted summaries.
- Phase 5 reference-band granularity should be coarse, versioned, and reusable: dimension, resource card/resource class, capability tier, trust tier, locality class, workload class, and effective time window. The initial bands should cover CPU, GPU, memory, storage, network/data, and Service-ORU/native-service units at low/normal/high or bounded-range levels, with explicit private-swarm scope and variance thresholds for review. Phase 5 should not create per-provider, per-node, per-minute, per-model-variant, auction, spot, or demand-reactive bands; those would create rate-management overhead and market behavior before accounting, disputes, and federation are mature.
- Phase 10 federation participants publish local resource cards through federation templates and Overregistry/Overmark proposal APIs, not by injecting local prices directly into scheduler or billing. Each participant card must map local terms into canonical Overmark dimensions, carry participant and federation-template refs, allowed tenant/workload/data/purpose scopes, trust and verification refs, locality/federation boundary, effective windows, review policy, dispute/accounting refs, and redaction class. Local extensions may appear only as namespaced metadata with canonical fallback dimensions; accepted cards are versioned, reviewed, and replayable, and cross-tenant Overgrant/Overbill/Seal Ledger records cite the resolved common Overmark refs. Public or unknown providers remain Phase 11 public-low-sensitivity only and cannot publish cards that imply private, regulated, secret-bearing, or system-service eligibility.
