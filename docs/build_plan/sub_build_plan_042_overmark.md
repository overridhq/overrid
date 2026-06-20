# SUB BUILD PLAN #42 - Overmark

Attached SDS: [docs/sds/accounting/overmark.md](../sds/accounting/overmark.md)

## Purpose

This sub-build plan turns SDS #42 into an implementation sequence for Overmark. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Overmark is a Phase 5 accounting support service that publishes versioned resource cards, bounded reference bands, budget-preview facts, placement signal refs, rate-change audit records, and replay bundles. It helps scheduling, policy dry-runs, grants, billing previews, native services, SDK, CLI, and admin UI compare resource use without hidden constants or speculative market behavior. It must not own usage truth, ORU balance projections, Seal Ledger entries, billing documents, provider payout amounts, final scheduling, policy admission, auctions, order books, token pricing, NFTs, revenue projections, or customer-count assumptions.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #42: Overmark](../sds/accounting/overmark.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, API/event surfaces, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Overmark plan](../service_catalog/accounting/overmark.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical schemas, signed envelopes, idempotency, trace ids, deterministic fixtures, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identity refs, Overtenant tenant scope, Overkey signing/service refs, Overgate request discipline, Overwatch audit, Overregistry refs, and Overqueue-safe command context. |
| [Phase 2: Seed Private Swarm](phase_02_seed_private_swarm.md) | Supplies Hardware Discovery, Benchmark Runner, Overcell node facts, resource capability evidence, seed private-swarm inventory, and measured capacity inputs. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies Overpack workload refs, Oversched placement consumers, Overlease reservation refs, Overrun execution refs, and Overmeter raw usage dimensions. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy decisions, Policy Dry-Run consumers, Workload Classifier facts, Oververify trust evidence, Overclaim dispute refs, and challenge/trust evidence. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Controls the first build point: resource cards, explicit ORU dimensions, bounded reference bands, budget-preview facts, placement signals, immutable version refs, and accounting handoffs. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies protected system-service operation, failover, restore, maintenance, and grid-resident hardening for Overmark publication/resolution workers. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies native Overbase persistence, Overstore replay/export artifacts, Overvault private evidence refs, namespace refs, retention, backup/restore, and migration handoffs. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Controls federation participant resource-card expansion, federation templates, cross-tenant review, public-interest allocation refs, and canonical dimension mapping. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public-provider constraints, low-sensitivity workload eligibility, fraud/reputation/payout-hold constraints, redaction, and public-node resource-card limits. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Supplies Wallet and Usage Center, admin UI, SDK, CLI, native apps, central AI stewardship interfaces, and user-facing budget preview views. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies compliance boundaries, incident handoffs, threat-model reviews, stewardship reports, audit exports, retention controls, migration governance, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #42 first build work aligned to master Phase 5, with Phase 10 federation expansion and later public-provider, native-app, and governance gates. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, speculative-market, pricing, revenue, customer-count, or external-provider-as-core-boundary drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 2, 3, 4, 5, 10, 11, 12, and 13 | Attach SDS #42, freeze Overmark authority, preserve Phase 5 as first build point, and record federation/public/native/governance gates. |
| 2 | Master Phases 0, 1, 2, 3, 4, and 5 | Build Rust contracts, schemas, dimensions, catalogs, reason codes, signed envelopes, and deterministic fixtures. |
| 3 | Master Phases 1, 2, 4, and 5 | Implement resource-card proposal, validation, review, publication, query, deprecation, and immutable version behavior. |
| 4 | Master Phases 2, 3, 4, and 5 | Implement reference bands, review classes, effective windows, deterministic resolution, supersession, and mixed-resource decomposition. |
| 5 | Master Phases 3, 4, 5, 6, and 12 | Implement budget previews, placement signals, policy dry-run facts, consumer refs, and client examples without owning downstream decisions. |
| 6 | Master Phases 4, 5, 8, and 13 | Implement version replay, audit history, retractions, corrections, stale-version detection, and dispute evidence handoffs. |
| 7 | Master Phases 4, 5, 11, 12, and 13 | Implement redaction, trust/locality guardrails, public-safe views, provider-sensitive evidence controls, and deny-by-default access. |
| 8 | Master Phase 10, with Phase 11 constraints | Expand to trusted federation participant cards, canonical dimension mapping, public-interest templates, and public low-sensitivity limits. |
| 9 | Master Phases 7, 8, 12, and 13 | Harden operations, replay/recompute, native persistence, grid-resident runtime, governance, compliance, and migration controls. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, authority boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Overmark core is a Rust service/module using shared contract types, Tokio for bounded async workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Resource cards, dimension catalogs, capability tier markers, trust tier markers, locality markers, availability snapshot refs, reference bands, budget previews, placement signals, rate-change records, API objects, events, fixtures, reason-code catalogs, redaction profiles, and replay bundles use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be added only through the shared contract layer where needed.
- Mutating endpoints require signed actor or service envelopes, tenant or system scope, trace id, idempotency key, schema version, source evidence refs, policy refs, stable reason codes, and append-only Overwatch events.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for evidence fingerprints, version bundles, replay/export bundles, schema fixtures, and deterministic comparison tests.
- Overmark may later persist resource-card and reference-band records through Overbase, replay/export artifacts through Overstore, and private provider/evidence refs through Overvault. It must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, exchanges, auctions, spot markets, or external payment providers the platform boundary.
- Phase 5 allows coarse, reusable, private/local reference bands with explicit resource dimensions, source evidence, effective windows, and review classes. Phase 10 owns trusted federation expansion. Phase 11 owns unknown/public provider low-sensitivity constraints.
- Overmark supplies reference facts and version refs. It never settles usage, mutates ORU balances, appends Seal Ledger entries, calculates invoices, executes payouts, replaces Overguard admission, replaces Oversched placement, or owns raw provider capability truth.
- Planning and implementation must avoid speculative token pricing, NFT valuation, auction/order-book mechanics, hidden per-operation fee logic, revenue projections, customer-count assumptions, direct provider self-publication into scheduler/billing behavior, and provider-sensitive evidence leaks.

## Phase 1: SDS Attachment, Overmark Authority, And Phase Gates

### Work Items

- **1.1 Attach the build plan to SDS #42.**
  - Design: Link this document from the Overmark SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/accounting/overmark.md`, `docs/service_catalog/accounting/overmark.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #42 returns both the Overmark SDS and this sub-build plan.

- **1.2 Freeze Overmark authority boundaries.**
  - Design: Record that Overmark owns versioned resource cards, dimension mapping, capability/trust/locality markers, availability snapshot refs, bounded reference bands, budget-preview facts, placement signal refs, rate-change records, replay bundles, redaction profiles, and version-use audit history.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms Overmark does not own usage truth, ORU balances, Seal Ledger entries, billing documents, provider payouts, final scheduling, policy admission, raw provider capability truth, or speculative market behavior.

- **1.3 Preserve master Phase 5 as the first build point.**
  - Design: Keep first implementation in Phase 5 after Phase 0 contracts, Phase 1 identity/audit, Phase 2 capability evidence, Phase 3 execution dimensions, and Phase 4 policy/trust inputs exist.
  - Output: Phase-gate note that Phase 5 builds private/local card and reference-band primitives, Phase 10 adds trusted federation card expansion, Phase 11 limits public providers, Phase 12 exposes native/client views, and Phase 13 hardens governance.
  - Validation: Review proves this plan does not move federation/public-provider behavior into Phase 5 and does not defer the core private/local Overmark primitive behind later phases.

- **1.4 Carry forward resolved SDS #42 decisions.**
  - Design: Preserve review-class rules, explicit mixed GPU/model dimensions, public-card redaction boundaries, coarse Phase 5 band granularity, and Phase 10 federation participant card publication through templates and proposal APIs.
  - Output: Resolved-decision checklist tied to implementation reviews.
  - Validation: Review rejects one global review rule, one vague AI cost number, public leakage of provider-sensitive fields, per-provider/per-node/per-minute Phase 5 bands, demand-reactive bands, and federation participants injecting local prices directly into scheduler or billing.

- **1.5 Define owner-service and consumer boundaries.**
  - Design: Create a dependency matrix for Overregistry, Hardware Discovery, Benchmark Runner, Overmeter, Overguard, Policy Dry-Run API, Oversched, Overgrant, Overbill, ORU Account Service, Seal Ledger, Provider Payout Service, Overclaim, Oververify, Overwatch, Wallet and Usage Center, central AI stewardship, SDK, CLI, admin UI, and native services.
  - Output: Boundary matrix listing consumed refs, emitted version refs, final authority owner, redaction class, replay evidence, blocking behavior, and later phase gate.
  - Validation: Review confirms every handoff uses explicit APIs, versioned refs, signed evidence, reason codes, trace ids, policy refs, and Overwatch audit rather than direct privileged state reads.

## Phase 2: Rust Contracts, Schemas, Dimensions, Catalogs, And Fixtures

### Work Items

- **2.1 Create the Overmark Rust contract module.**
  - Design: Add contract types for resource cards, resource dimensions, capability tier markers, trust tier markers, locality markers, availability snapshot refs, reference bands, budget previews, placement signals, rate-change records, replay bundles, API errors, events, and redaction profiles.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, marker-kind enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from Overmeter, ORU Account Service, Seal Ledger, Overbill, Provider Payout Service, Overguard, and Oversched internals.

- **2.2 Define explicit resource dimensions and marker catalogs.**
  - Design: Encode CPU-ORU, GPU-ORU, STOR-ORU, NET-ORU, MEM-ORU, DATA-ORU, Service-ORU, resource class, capability tier, trust tier, locality class, availability class, workload class, sensitivity class, and future extension rules.
  - Output: Catalog files, schema enums, docs-facing tables, compatibility notes, and fixture references for every dimension/marker.
  - Validation: Tests prove catalog entries name their source authority, allowed phase, allowed consumers, redaction profile, replay inputs, and stable reason codes.

- **2.3 Define canonical JSON Schemas and examples.**
  - Design: Add versioned schemas for resource-card proposal/publication/query, reference-band proposal/publication/resolve, budget preview, placement signal, version replay, deprecation/supersession, rate-change records, events, and export bundles.
  - Output: Schema files, valid examples, invalid examples, generated validators, compatibility notes, and docs-facing examples.
  - Validation: Schema tests reject missing actor/service identity, tenant or system scope, trace id, idempotency key, schema version, source evidence refs, policy refs, effective windows, dimensions, state, reason codes, and audit refs where required.

- **2.4 Model card, band, and preview state machines.**
  - Design: Encode card and band states from draft through submitted, validated, review_required, published, deprecated, superseded, retracted, and corrected; encode budget preview states from requested through created, expired, superseded, and replayed.
  - Output: Transition tables, Rust enums, JSON Schema enums, event emission rules, invalid transition fixtures, and lifecycle review notes.
  - Validation: State tests reject silent edits to resource dimensions, source evidence, reference bands, effective windows, policy refs, public visibility, or historical version history.

- **2.5 Create deterministic Overmark fixtures.**
  - Design: Build fixtures for card proposal, validation, publication, reference-band proposal, reference-band resolution, budget preview, placement signal, supersession, retraction, replay, redaction, stale-version detection, duplicate handling, and invalid source evidence.
  - Output: Fixture directory, expected projections, reason codes, Overwatch events, replay examples, redacted views, and invalid examples.
  - Validation: Fixture tests produce stable version ids, denial reason codes, band resolution, preview outputs, signal refs, redaction behavior, replay hashes, and idempotency outcomes across repeated runs.

## Phase 3: Resource Card Lifecycle, Evidence Validation, Publication, And Queries

### Work Items

- **3.1 Implement evidence intake and source-ref validation.**
  - Design: Validate resource capability and measurement evidence from Overregistry, Hardware Discovery, Benchmark Runner, Overmeter, operator-reviewed sources, and later federation templates without copying raw provider truth into Overmark.
  - Output: Evidence resolver interfaces, source-service allowlist, freshness rules, conflict behavior, missing-evidence reason codes, and evidence validation audit events.
  - Validation: Tests reject missing, stale, conflicting, unsigned, wrong-scope, provider-only, wrong-dimension, or policy-blocked evidence refs with stable reason codes.

- **3.2 Implement resource-card proposal.**
  - Design: Create draft/submitted resource card versions with resource class, dimensions, capability traits, measurement unit, normalization notes, source evidence refs, schema version, redaction class, and proposed effective scope.
  - Output: `POST /resource-cards`, idempotent create behavior, draft/submitted state projection, command audit, and `overmark.resource_card_proposed` events.
  - Validation: API tests reject proposal without signed actor/service identity, tenant/system scope, idempotency key, trace id, source evidence refs, policy refs, dimensions, and audit refs.

- **3.3 Implement card validation and review routing.**
  - Design: Validate schema, dimension compatibility, evidence freshness, workload-class compatibility, trust/locality markers, redaction class, review class, and policy requirements before publication.
  - Output: Validation worker, review-required records, operator/stewardship review hooks, denial records, and `overmark.validation_denied` events.
  - Validation: Tests prove ambiguous evidence, new dimensions, material GPU/model budget behavior, regulated/safety-sensitive use, public/native-facing card scope, provider evidence conflicts, and exception requests enter review or denial rather than auto-publication.

- **3.4 Implement card publication and queries.**
  - Design: Publish reviewed resource card versions and support queries by resource class, dimension, capability tier, trust tier, locality, state, schema version, effective window, and redaction class.
  - Output: `POST /resource-cards/{id}/publish`, `GET /resource-cards`, published projections, indexes, pagination fields, redacted reads, and `overmark.resource_card_published` events.
  - Validation: Tests prove published versions are immutable, query results are audience-redacted, and downstream consumers receive card refs rather than private raw evidence.

- **3.5 Implement card deprecation, supersession, and correction metadata.**
  - Design: Allow published cards to be deprecated, superseded, retracted, or corrected for new use while preserving historical refs for replay.
  - Output: `POST /versions/{id}/deprecate`, replacement refs, correction refs, reason codes, downstream warning fields, and `overmark.resource_card_deprecated` events.
  - Validation: Tests prove old card refs remain replayable, new decisions resolve to allowed replacement versions, and retracted cards cannot be used for new decisions unless an explicit correction policy allows it.

## Phase 4: Reference Bands, Review Classes, Resolution, And Mixed-Resource Decomposition

### Work Items

- **4.1 Implement reference-band proposal.**
  - Design: Create bounded reference bands by dimension, resource card/class, capability tier, trust tier, locality class, workload class, effective window, source evidence refs, review class, and variance threshold.
  - Output: `POST /reference-bands`, idempotent proposal behavior, submitted state projection, conflict detection, and `overmark.reference_band_proposed` events.
  - Validation: API tests reject unbounded bands, missing evidence refs, missing dimensions, invalid scope, unsupported workload classes, hidden provider terms, and speculative or demand-reactive fields.

- **4.2 Implement review-class policy and variance checks.**
  - Design: Allow automated evidence validation for Phase 5 private/local bands only when dimensions are approved, evidence is current, effective windows are bounded, no new trust/locality semantics exist, and movement stays inside configured variance.
  - Output: Review-class catalog, variance threshold rules, operator/stewardship review routing, source-evidence conflict handling, and reason-code fixtures.
  - Validation: Tests prove human or stewardship review is required for new dimensions, material GPU/model classes, public/native-facing bands, cross-tenant/public-interest scope, regulated workloads, provider conflicts, large movement, retractions, corrections, and grant/payout/public-trust impact.

- **4.3 Implement deterministic band resolution.**
  - Design: Resolve the active band for card, dimension, workload class, tenant/system scope, trust tier, locality class, and time while returning exact version id, effective window, source refs, and reason codes.
  - Output: `GET /reference-bands/resolve`, deterministic resolver, no-match response, stale-version warnings, and `overmark.reference_band_published`/resolution audit refs as appropriate.
  - Validation: Tests prove the same input/time returns the same version id, overlapping windows resolve deterministically or deny, missing facts return reason codes, and consumers store version refs for replay.

- **4.4 Implement overlap, supersession, and rollback-by-supersession.**
  - Design: Reject unsafe overlaps or require explicit supersession records; use new versions for rollback rather than mutating old bands.
  - Output: Supersession graph, effective-window checks, deprecation records, replacement refs, version-diff fields, and `overmark.reference_band_superseded` events.
  - Validation: Tests prove historical decisions keep old refs, new decisions do not use deprecated/superseded bands by default, and bad bands are retracted for new use without rewriting history.

- **4.5 Implement mixed GPU/model workload decomposition.**
  - Design: Decompose budget inputs into GPU-ORU, CPU-ORU, MEM-ORU, STOR-ORU, DATA-ORU, NET-ORU, and Service-ORU only where a bounded model/native-service unit cannot be cleanly represented by lower-level counters.
  - Output: Decomposition schema, model-card fields, runtime adapter refs, cache/retrieval profile refs, token/request unit refs, and examples for mixed AI/RAG workloads.
  - Validation: Tests reject one vague AI cost number, hidden model-specific fee fields, collapsed resource accounting, and any preview or band that prevents Overmeter, Overbill, or Overgrant from citing exact Overmark version refs.

## Phase 5: Budget Previews, Placement Signals, Policy Dry-Runs, And Consumer Contracts

### Work Items

- **5.1 Implement deterministic budget previews.**
  - Design: Create estimate input and output bundles for workload, app, native-service, grant, and dry-run requests using explicit card/band refs, dimensions, source evidence refs, assumptions, and expiry.
  - Output: `POST /budget-previews`, preview records, missing-input reports, expiry behavior, estimate labels, and `overmark.budget_preview_created` events.
  - Validation: Tests prove previews are labeled estimates, cite all cards/bands/evidence refs, return stable reason codes for missing inputs, and never create ledger, invoice, payout, or scheduling truth.

- **5.2 Implement scheduler-facing placement signals.**
  - Design: Produce placement signal refs from card, band, availability snapshot, trust/locality markers, workload class, grant/budget context, and constraint inputs without ranking or placing workloads.
  - Output: `POST /placement-signals`, signal refs, reason codes, redacted signal views, and `overmark.placement_signal_created` events.
  - Validation: Integration tests prove Oversched remains final placement authority and Overmark signals are one input among policy, trust, availability, quota, lease, locality, cache, and grant facts.

- **5.3 Feed Overguard and Policy Dry-Run API.**
  - Design: Provide budget-preview facts, missing-prerequisite reports, version refs, policy refs, and redaction-safe summaries for side-effect-free admission and policy previews.
  - Output: Overguard/Policy Dry-Run adapter contract, policy input bundle schema, preview refs, denial/review state handling, and examples.
  - Validation: Tests prove policy dry-runs cannot treat missing Overmark facts as allow, cannot bypass Overguard, and cannot mutate cards, bands, budgets, ledger records, or scheduler decisions.

- **5.4 Feed Overgrant, Overbill, ORU Account Service, and Seal Ledger consumers.**
  - Design: Provide resource dimensions, reference-band refs, budget preview refs, receipt/statement refs, grant quota/report refs, ORU dimension refs, and ledger-citation refs while preserving owner-service authority.
  - Output: Consumer read contracts, fixture traces, downstream citation fields, and missing-version errors.
  - Validation: Tests prove consumers cite Overmark refs without letting Overmark settle usage, mutate balances, append ledger entries, calculate final invoices, execute payouts, or adjudicate disputes.

- **5.5 Build SDK, CLI, admin UI, native-service, and wallet examples.**
  - Design: Provide client examples for resolving card/band versions, creating budget previews, requesting placement signals, replaying versions, and viewing redacted resource metadata.
  - Output: SDK/CLI examples, admin UI field list, Wallet and Usage Center view model, native-service examples, and central AI stewardship summary fields.
  - Validation: Tests prove clients cannot infer private provider evidence, override review state, broaden visibility, mutate reference versions, or turn previews into payable/billable documents.

## Phase 6: Version Replay, Rate-Change Audit, Retraction, Correction, And Stale-Version Handling

### Work Items

- **6.1 Implement version replay.**
  - Design: Reconstruct the card, band, preview, or placement-signal decision bundle from version refs, source evidence refs, policy refs, effective windows, state transitions, redaction profile, and Overwatch events.
  - Output: `GET /versions/{id}/replay`, replay bundle writer, hash comparison, missing-evidence reason codes, and export fields.
  - Validation: Replay tests reconstruct current and historical states, detect missing evidence, preserve old decisions, and flag mismatches without silently changing downstream behavior.

- **6.2 Implement rate-change records and version diffs.**
  - Design: Record proposal, review, publication, supersession, rollback-by-supersession, retraction, correction, and downstream-use history for cards and bands.
  - Output: `rate_change_record` model, version-diff view, review comments, source-evidence refs, audit refs, and query fields.
  - Validation: Tests prove every material card/band change has stable reason codes, actor/service identity, source evidence, policy refs, effective windows, and Overwatch audit.

- **6.3 Implement retraction and correction workflows.**
  - Design: Retract bad published versions for new use, publish corrected replacements, attach remediation records, and keep historical users replayable.
  - Output: Retraction API behavior, correction metadata, replacement refs, downstream warning events, repair queue records, and operator review fields.
  - Validation: Tests prove retracted versions cannot be resolved for new decisions, old decisions remain replayable, and correction records never rewrite historical card/band/previews.

- **6.4 Detect downstream stale-version use.**
  - Design: Observe downstream refs from Oversched, Overguard, Overgrant, Overbill, Seal Ledger, Wallet and Usage Center, SDK/CLI, and native services to identify deprecated, superseded, retracted, or missing versions.
  - Output: Stale-version scanner, affected-consumer report, alert rules, replay/export links, and migration hints.
  - Validation: Tests prove stale caches are traceable by version id and do not produce hidden behavior changes; affected consumers receive warnings or blockers based on version state.

- **6.5 Integrate dispute, verification, and evidence handoffs.**
  - Design: Link provider evidence disputes, validation conflicts, trust changes, retraction reasons, correction requests, and appeal refs to Overclaim, Oververify, Overguard, and Overwatch.
  - Output: Handoff schema, dispute refs, verification refs, policy refs, review queues, and redacted explanation fields.
  - Validation: Tests prove disputes do not mutate old decisions directly, disputed versions can be review-blocked for new use, and replay shows original facts plus correction/remediation evidence.

## Phase 7: Redaction, Policy, Trust, Locality, And Public-Safe Views

### Work Items

- **7.1 Implement role-aware redaction profiles.**
  - Design: Separate public/native, tenant/provider-restricted, steward, auditor, operator, and owning-service views for cards, bands, evidence, budget previews, placement signals, and replay bundles.
  - Output: Redaction policy map, view schemas, fixtures for each audience, and stable denial reason codes.
  - Validation: Security tests prove exact node/provider ids, private-swarm membership, precise capacity/topology, raw benchmark evidence, tenant-specific bands, grant/budget terms, payout/payment refs, workload/customer demand signals, fraud/risk flags, private locality routes, secret-bearing capability details, and raw evidence bundles stay hidden from unauthorized views.

- **7.2 Enforce deny-by-default policy and publication safety.**
  - Design: Block publication/resolution when cards, evidence refs, effective windows, trust/locality markers, policy facts, review state, or redaction class are ambiguous.
  - Output: Policy guardrail evaluator, missing-fact records, publication blockers, reason-code catalog, and review queue fields.
  - Validation: Tests prove ambiguous evidence, missing policy refs, missing redaction class, conflicting trust/locality facts, and public low-sensitivity overclaiming deny rather than publish or resolve.

- **7.3 Constrain trust-tier and locality markers.**
  - Design: Model trust and locality as budget/placement markers that downstream services may consume, not as automatic eligibility for private, regulated, secret-bearing, or system-service workloads.
  - Output: Marker schemas, allowed-use matrix, public/provider/federation visibility rules, and Phase 11 guardrails.
  - Validation: Tests prove public low-sensitivity resource cards do not imply private, regulated, secret-bearing, or system-service eligibility and cannot bypass Workload Classifier, Overguard, Oververify, or public-provider constraints.

- **7.4 Publish public/native-safe card fields.**
  - Design: Restrict public/native card views to catalog-safe resource class, coarse capability tier, supported workload classes, allowed data-sensitivity class, coarse region/locality, public/federation eligibility class, state, schema/version ids, effective window, redacted band class, and public-safe reason codes.
  - Output: Public/native view schema, Wallet and Usage Center view, native-service view, SDK/CLI example output, and redaction fixtures.
  - Validation: Tests prove public/native views cannot infer private provider evidence, exact capacity, tenant-specific terms, grant/budget internals, payment refs, fraud flags, or secret-bearing details.

- **7.5 Validate evidence-leak and consumer-abuse controls.**
  - Design: Add negative-control tests for evidence leakage, provider self-publication, hidden fee fields, unauthorized replay access, unreviewed public views, and consumer attempts to broaden card/band scope.
  - Output: Security test suite, negative fixtures, failure reason codes, and incident handoff refs.
  - Validation: Tests prove unauthorized consumers can only access refs or redacted summaries and cannot use Overmark APIs to gain privileged evidence, mutate published versions, or influence payouts/scheduling/policy directly.

## Phase 8: Phase 10 Federation Card Expansion And Phase 11 Public Limits

### Work Items

- **8.1 Add trusted federation participant card templates.**
  - Design: Support Phase 10 participants publishing local resource cards through federation templates, Overregistry refs, Overmark proposal APIs, participant refs, allowed tenant/workload/data/purpose scopes, and review policies.
  - Output: Federation card template schema, participant refs, review-class rules, proposal API extensions, and `overmark.resource_card_proposed` examples for federation scope.
  - Validation: Tests prove federation participants cannot inject local prices directly into scheduler or billing and cannot bypass canonical Overmark proposal/review/publication behavior.

- **8.2 Map local participant terms into canonical dimensions.**
  - Design: Normalize local participant resource descriptions into CPU-ORU, GPU-ORU, STOR-ORU, NET-ORU, MEM-ORU, DATA-ORU, Service-ORU, trust tier, locality class, workload class, and public-interest purpose refs where applicable.
  - Output: Mapping contract, canonical fallback dimensions, namespaced metadata rules, validation errors, and example mappings.
  - Validation: Tests prove namespaced local extensions cannot replace canonical dimensions and accepted cards remain versioned, reviewed, replayable, and consumable by Overgrant, Overbill, Seal Ledger, Oversched, and Overguard via common refs.

- **8.3 Gate cross-tenant and public-interest reference bands.**
  - Design: Require federation participant evidence, Purpose Tag Registry refs where public-interest claims exist, Overguard policy refs, Oververify trust refs, Overclaim dispute refs, and redaction class before cross-tenant bands resolve.
  - Output: Cross-tenant band policy, public-interest review rules, missing-evidence reason codes, and federation report fields.
  - Validation: Tests prove cross-tenant/public-interest bands are denied before Phase 10 refs exist and cannot be used by Phase 5 local/private programs.

- **8.4 Apply Phase 11 public-provider low-sensitivity limits.**
  - Design: Keep unknown or public providers in public-low-sensitivity scope with explicit workload/data restrictions, fraud/reputation refs, payout-hold constraints, evidence review, and redacted public card fields.
  - Output: Phase 11 public-provider card policy, public sandbox profile links, reputation/fraud handoff refs, and denial reason fixtures.
  - Validation: Tests prove public or unknown providers cannot publish cards that imply private, regulated, secret-bearing, high-trust, system-service, or unrestricted federation eligibility.

- **8.5 Build public-safe federation reporting.**
  - Design: Publish aggregate fields for accepted cards, active purpose refs, dimensions, public-safe capability classes, consumed/authorized refs, correction notices, review state, and report template/version refs.
  - Output: Federation/public report schema, redaction thresholds, export hashes, stewardship-report handoff fields, and public/native examples.
  - Validation: Tests prove public reports exclude private workload contents, raw evidence, raw account details, payment data, participant-private terms, fraud heuristics, private central-AI reasoning, and fields below aggregation thresholds.

## Phase 9: Operations, Replay, Native Persistence, Grid Residency, And Governance

### Work Items

- **9.1 Build Overmark dashboards and alerts.**
  - Design: Track card counts by state, reference bands by dimension/trust/locality/workload class, budget preview latency, placement signal counts, validation denials, source-evidence failures, supersession history, stale-version use, redaction failures, and downstream consumer refs.
  - Output: Metrics schema, dashboard definitions, alert rules, Overwatch aggregation, and runbook links.
  - Validation: Operations tests prove alerts fire for stale evidence, ambiguous overlaps, deprecated version use, retraction impact, preview failures, redaction leaks, missing source refs, and downstream use of invalid versions.

- **9.2 Prepare native Overbase, Overstore, and Overvault persistence handoffs.**
  - Design: Move card/band/projection records to native Overbase when available, replay/export artifacts to Overstore where appropriate, and private provider/evidence refs to Overvault without changing API semantics.
  - Output: Persistence adapter interfaces, migration plan, artifact refs, private-ref handoffs, retention hooks, and export format.
  - Validation: Migration tests prove no conventional database/object/vault boundary becomes canonical, records remain replayable, and private provider/evidence refs stay behind owning service access controls.

- **9.3 Prepare grid-resident protected operation.**
  - Design: Package Overmark as a protected grid-resident system workload with service identity, config contracts, secret/private refs, health checks, failover behavior, restore drills, maintenance mode, replay pause/resume, and signed operator actions.
  - Output: System-service packaging notes, runtime config contract, backup/restore hooks, health endpoints, maintenance controls, and break-glass audit rules.
  - Validation: Grid tests prove restart, failover, restore, replay pause/resume, and maintenance mode preserve immutable versions and do not publish or resolve unsafe stale refs after recovery.

- **9.4 Harden replay, recompute, and backfill.**
  - Design: Support scoped recompute by card, band, tenant/system scope, dimension, capability tier, trust tier, locality, policy version, source-evidence checkpoint, review class, and downstream consumer refs.
  - Output: Recompute worker, backfill run records, replay comparison model, operator controls, repair audit refs, and bounded backpressure rules.
  - Validation: Tests prove recompute is idempotent, bounded, resumable, replayable, preserves old projections during review, and never silently changes published historical decisions.

- **9.5 Add governance, compliance, threat-model, and incident handoffs.**
  - Design: Integrate Compliance Boundary policy refs, incident response refs, threat-model findings, stewardship reporting, migration controls, retention/export policy, region-specific restrictions, and audit exports.
  - Output: Governance checklist, compliance export schema, threat-model test list, incident handoff refs, stewardship report fields, and retention policy.
  - Validation: Governance tests prove high-impact band changes, public/federation card approvals, retractions, corrections, replay/export changes, and redaction repairs require signed action, evidence refs, Overwatch audit, and retention-compliant exports.

## Phase 10: Validation, Documentation Alignment, Queue State, And Handoff

### Work Items

- **10.1 Validate sub-build-plan structure.**
  - Design: Check title prefix, attached SDS link, phase headings 1 through 10, work-item structure, Design/Output/Validation fields, alignment review, and exit gate.
  - Output: Focused validation result for `SUB BUILD PLAN #42`.
  - Validation: Scripted text checks pass for required headings, numbered phases, local links, work-item sections, final newline, and tab-free formatting.

- **10.2 Validate tech-stack alignment.**
  - Design: Scan for accidental conventional cloud, SaaS-admin, blockchain/NFT, speculative-market, external-payment, pricing, revenue, customer-count, direct-ledger-mutation, ORU-balance-mutation, final-scheduler-authority, policy-admission-authority, and provider-payout-authority assumptions.
  - Output: Stack-alignment scan result and any corrected wording.
  - Validation: Scan shows only approved negative-control references, native Overrid service names, or explicit non-choice guardrails from `docs/overrid_tech_stack_choice.md`.

- **10.3 Validate SDS, service catalog, master plan, and crosswalk links.**
  - Design: Confirm this plan is linked from the SDS source table, service catalog sub-build-plan section, master per-SDS table, and service-catalog alignment index.
  - Output: Updated docs and link-check evidence.
  - Validation: Local Markdown link validation returns no missing local targets for changed docs.

- **10.4 Validate Overmark authority and phase gates.**
  - Design: Verify every planned behavior preserves Phase 5 as the first implementation point for local/private resource-card and reference-band primitives, Phase 10 as trusted federation expansion, Phase 11 as public-provider low-sensitivity scope, Phase 12 as client/native consumption, and Phase 13 as governance hardening.
  - Output: Authority-boundary checklist and implementation handoff notes.
  - Validation: Review confirms Overmark does not own usage truth, ORU balances, Seal Ledger entries, billing documents, provider payouts, final scheduling, policy admission, raw provider capability truth, auctions, order books, speculative pricing, NFTs, revenue projections, or customer-count assumptions.

- **10.5 Reindex and verify retrieval handoff.**
  - Design: Refresh Docdex for the new plan and linked docs, then verify search returns the SDS #42 plan and source docs in the aligned result set.
  - Output: Docdex index refresh evidence, retrieval evidence, queue/progress notes, repo memory, and implementation handoff note.
  - Validation: Docdex index succeeds, Docdex search for `SUB BUILD PLAN #42 Overmark Phase 5 resource cards reference bands budget previews placement signals` returns the new sub-build plan in the result set, and `docdexd run-tests` blocker is recorded if no test runner is configured.

## Alignment Review

- SDS #42 already contains resolved open-question decisions for review classes, explicit mixed GPU/model dimensions, public-card field redaction, coarse Phase 5 reference-band granularity, and Phase 10 federation participant card publication. This pass adds the sub-build-plan backlink and does not require SDS content correction.
- The service catalog already matches the SDS and master plan: Overmark starts in Phase 5 for resource cards, bounded reference bands, budget previews, placement signals, and rate-change audit history. This pass adds the sub-build-plan backlink.
- The master Phase 0 through Phase 13 order remains valid. This pass adds SDS #42 to the per-SDS index and keeps Overmark in Phase 5 while preserving Phase 10 as the trusted federation expansion, Phase 11 as public-provider low-sensitivity scope, Phase 12 as client/native consumption, and Phase 13 as governance hardening.
- The build-plan crosswalk remains valid. This pass adds SDS #42 to the sub-build-plan index with Phase 5 first-build alignment and later federation/public/native/governance gates.
- The accepted Rust-first/native-Overrid stack remains authoritative. The plan uses Rust, Tokio, shared contracts, canonical JSON plus JSON Schema, signed envelopes, Ed25519, BLAKE3/content hashes, and native Overrid persistence/storage/private-ref boundaries.

## Exit Gate

SUB BUILD PLAN #42 is complete when the new plan is linked from the SDS, service catalog, master build plan, and service-catalog alignment docs; Phase 5 remains the first build point for local/private resource cards, reference bands, budget previews, placement signals, and replayable version refs; Phase 10 remains the trusted federation card-expansion gate; Phase 11 remains the public-provider low-sensitivity gate; the plan has 10 phases with well-designed Design/Output/Validation work items; queue/progress docs record the pass; local link and structure validation pass; stack guardrail scans show no accidental conventional-cloud, NFT, speculative-market, external-payment, pricing, revenue, customer-count, ORU-balance-mutation, direct-ledger-mutation, final-scheduler-authority, policy-admission-authority, provider-payout-authority, or unreviewed public-provider drift; and Docdex retrieval can find the new plan with SDS #42 context.
