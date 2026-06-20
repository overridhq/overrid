# SUB BUILD PLAN #36 - Reputation and Anti-Sybil Service

Attached SDS: [docs/sds/trust_policy_verification/reputation_anti_sybil_service.md](../sds/trust_policy_verification/reputation_anti_sybil_service.md)

## Purpose

This sub-build plan turns SDS #36 into an implementation sequence for the Reputation and Anti-Sybil Service. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Reputation and Anti-Sybil Service is Phase 11 public-supply hardening infrastructure. It validates public-provider risk signals, maintains append-only reputation and anti-Sybil records, computes risk windows, and emits explainable eligibility, throttle, duplicate-execution, challenge-cadence, sandbox, and payout-hold recommendations. It does not onboard providers, replace Oververify final trust records, mutate payouts or ledger state, run incident response, or widen public providers beyond public low-sensitivity workloads.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #36: Reputation and Anti-Sybil Service](../sds/trust_policy_verification/reputation_anti_sybil_service.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, API/event surfaces, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Reputation and Anti-Sybil Service plan](../service_catalog/trust_policy_verification/reputation_anti_sybil_service.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared schemas, signed envelopes, idempotency, trace ids, stable reason codes, deterministic fixtures, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies identities, tenants, credentials, Overgate request discipline, Overregistry refs, Overwatch audit, and Overqueue-safe command context. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies workload sensitivity classes, Overguard policy, Oververify evidence records, Challenge Task Service outcomes, Overclaim disputes/corrections, cache trust scopes, and replayable policy evidence. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies payout, hold, settlement, dispute-window, refund, correction, and accounting evidence refs while preserving mutation authority in owning accounting services. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies protected system-service placement, failover, restore, maintenance, operator action, and grid-resident hardening. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies native Overbase persistence, Overstore evidence artifacts, Overvault/private refs, namespace refs, and retention/export handoffs. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Supplies known external-provider and purpose-scoped capacity context that precedes open public supply. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Controls the first build point: public-provider onboarding refs, anti-Sybil controls, strict public workload eligibility, public sandbox profile, fraud/challenge controls, payout holds, and explainable reputation updates. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Supplies SDK, CLI, admin UI, wallet/usage, native app, central AI, and stewardship consumers of authorized reputation summaries and remediation flows. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies compliance boundary, incident, threat-model, stewardship, retention, PIP, audit-export, and scale hardening for risk and anti-Sybil behavior. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #36 first build work aligned to master Phase 11, with earlier prerequisite phases and later native-app/governance gates. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, pricing, revenue, or customer-count drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 7, 8, 10, 11, 12, and 13 | Attach SDS #36, freeze recommendation authority, preserve Phase 11 as first build point, and record prerequisite plus hardening gates. |
| 2 | Master Phases 0, 1, 4, and 11 | Build Rust contracts, schemas, signal catalogs, confidence bands, state enums, reason codes, fixtures, and redaction profiles. |
| 3 | Master Phases 1, 4, 10, 11, and 13 | Implement authenticated signal ingest with source validation, target validation, freshness, legal/region policy checks, and visibility classes. |
| 4 | Master Phases 4, 5, 8, 11, and 13 | Compute risk windows and layered anti-Sybil clusters from owner-service refs without raw secret, payout, identity, or private-tenant leakage. |
| 5 | Master Phases 4 and 11 | Publish eligibility, throttle, duplicate-execution, challenge-cadence, and sandbox recommendations for Oververify, Overguard, Oversched, and Challenge Task Service. |
| 6 | Master Phases 5, 11, and 13 | Emit payout-hold trigger recommendations with evidence refs and release conditions while leaving actual holds to Provider Payout Service, Overbill, Seal Ledger, and Overclaim. |
| 7 | Master Phases 4, 5, 8, 11, 12, and 13 | Provide provider-facing explanations, operator evidence timelines, Overclaim appeals/corrections, classed retention, and redaction-safe views. |
| 8 | Master Phases 6, 11, 12, and 13 | Expose APIs, SDK/CLI/admin UI/native app profiles, dashboards, stewardship views, and central AI read paths. |
| 9 | Master Phases 7, 8, 11, and 13 | Harden replay, recompute backfill, grid-resident operation, native persistence, storage refs, compliance boundaries, and governance controls. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, public low-sensitivity boundaries, owner-service handoffs, queue state, and implementation handoff gates. |

## Tech Stack Guardrails

- Reputation and Anti-Sybil Service core is a Rust service/module using shared contract types, Tokio for bounded async ingest/recompute workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Reputation records, anti-Sybil signals, risk windows, eligibility recommendations, throttles, payout-hold trigger recommendations, appeals/corrections, explanation bundles, replay bundles, API objects, events, fixtures, and reason-code catalogs use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be added only through the shared contract layer where needed.
- Mutating endpoints require signed actor or service envelopes, tenant or public-provider scope where applicable, idempotency keys, trace ids, schema versions, policy refs, evidence refs, stable reason codes, visibility class, expiry, and append-only Overwatch events.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for input refs, redacted signal commitments, replay bundles, recompute snapshots, and deterministic golden tests.
- The service may later persist native records through Overbase, evidence artifacts through Overstore, and private/compliance refs through Overvault. It must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, external fraud SaaS, or external workflow products the platform boundary.
- Anti-Sybil legality is a Compliance Boundary and Overguard policy fact. Region-restricted signals must be recorded as unavailable or review-required; the service must not replace unavailable signals with hidden collection.
- Public providers remain limited to public low-sensitivity workloads. No recommendation from this service can authorize private tenant data, regulated data, secrets, system-service workloads, broad egress, uncapped runtime, or uncapped resources.
- Payout-hold outputs are recommendation-only artifacts. Provider Payout Service, Overbill, Seal Ledger, and Overclaim own actual holds, releases, corrections, finality, balances, receipts, invoices, refunds, and payout mutation.
- Provider-facing explanations use stable reason codes, confidence bands, severity, time windows, remediation hints, policy/evaluator versions, redacted evidence refs, and Overclaim appeal refs. They must not expose exact cluster membership, IP/device/payout hashes, raw graph edges, model weights, secret/private evidence, fraud heuristics, operator notes, or incident details.
- Planning and implementation must avoid opaque global trust numbers, reputation markets, tokenized reputation, NFTs, blockchain mechanics, pricing tables, revenue projections, customer-count assumptions, per-operation external payment calls, and hidden privileged state reads.

## Phase 1: SDS Attachment, Recommendation Authority, And Phase Gates

### Work Items

- **1.1 Attach the build plan to SDS #36.**
  - Design: Link this document from the numbered Reputation and Anti-Sybil Service SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/trust_policy_verification/reputation_anti_sybil_service.md`, `docs/service_catalog/trust_policy_verification/reputation_anti_sybil_service.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #36 returns both the Reputation and Anti-Sybil Service SDS and this sub-build plan.

- **1.2 Freeze recommendation authority boundaries.**
  - Design: Record that the service owns public-provider reputation records, anti-Sybil signal refs, risk windows, eligibility recommendations, throttles, challenge cadence, duplicate-execution requirements, payout-hold trigger recommendations, explanations, replay, appeals/corrections, and correction history.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms the service does not own provider onboarding, Oververify final trust records, Overguard policy authoring or admission, Oversched placement, Challenge Task Service orchestration, Fraud Control incidents, payout holds, billing, ledger mutation, refunds, final settlement, or support-case finality.

- **1.3 Preserve master Phase 11 as the first build point.**
  - Design: Keep first implementation in Phase 11 because useful public-provider risk recommendations require Phase 0 contracts, Phase 1 identity/audit, Phase 4 policy/verification/dispute primitives, Phase 5 hold/accounting refs, Phase 10 trusted-federation lessons, and the Phase 11 public low-sensitivity boundary.
  - Output: Phase-gate note that earlier master phases are prerequisites, Phase 11 is first build, and Phase 12 plus Phase 13 are consumer/governance hardening gates.
  - Validation: Review proves this plan does not move public-provider broadening into Phase 4 or allow Phase 11 public providers to run private, regulated, secret-bearing, system-service, uncapped, or broad-egress workloads.

- **1.4 Carry forward resolved SDS #36 open-question decisions.**
  - Design: Preserve the SDS decisions for operating-region signal legality, confidence bands plus severity, automatic payout-hold recommendation limits, redacted provider-facing cluster explanations, and append-only corrected/expired signal history.
  - Output: Resolved-decision checklist tied to Phase 11 implementation reviews.
  - Validation: Review rejects hidden collection, one-signal final suspension, broad automatic cluster-wide payout holds, raw fraud heuristic exposure, provider-facing exact cluster membership, and deletion of corrected signal history.

- **1.5 Define owner-service and consumer boundaries.**
  - Design: Create a dependency matrix for Public Provider Onboarding, Oververify, Challenge Task Service, Overwatch, Overclaim, Overguard, Oversched, Fraud Control Service, Provider Payout Service, Overbill, Seal Ledger, admin UI, CLI, SDK, central AI stewardship, Compliance Boundary Service, Overbase, Overstore, and Overvault.
  - Output: Boundary matrix listing consumed refs, emitted recommendation refs, final authority owner, freshness owner, visibility class, redaction profile, replay evidence, expiry behavior, and later phase gate.
  - Validation: Review confirms every handoff uses explicit APIs, versioned refs, signed evidence, reason codes, policy versions, trace ids, and Overwatch evidence rather than direct privileged state reads.

## Phase 2: Rust Contracts, Schemas, Signal Catalogs, And Fixtures

### Work Items

- **2.1 Create the Reputation and Anti-Sybil Rust contract module.**
  - Design: Add contract types for public-provider reputation record, anti-Sybil signal, node uniqueness signal, payout uniqueness signal, network behavior signal, reputation window, eligibility recommendation, public-provider throttle, payout-hold trigger, appeal/correction record, explanation bundle, replay bundle, state enums, API errors, and events.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, state enums, reason-code enums, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from onboarding, Oververify final trust, scheduler placement, payout, billing, ledger, fraud incident, and support-case internals.

- **2.2 Define canonical JSON Schemas and examples.**
  - Design: Add versioned schemas for signal ingest, signal rejection, recompute request, reputation read, eligibility read, explanation read, appeal/correction request, throttle recommendation, hold-trigger recommendation, replay request, events, and consumer projections.
  - Output: Schema files, valid examples, invalid examples, schema index, generated validators, and compatibility notes.
  - Validation: Schema tests reject missing source service, target provider/node/payout ref, trace id, idempotency key, evidence refs, policy refs, visibility class, confidence, severity, expiry, reason codes, and correction state.

- **2.3 Model signal and recommendation state machines.**
  - Design: Encode reputation states from `new_public_provider` through `expired`, signal states from `received` through `revoked`, legal transitions, replacement links, recompute triggers, and append-only history semantics.
  - Output: Transition tables, Rust enums, JSON Schema enums, event emission rules, invalid transition fixtures, and recompute trigger catalog.
  - Validation: State tests reject active use of revoked signals, destructive deletion of corrected history, recommendation completion without evidence refs, suspension from a single non-critical signal, and exact idempotent retry conflicts.

- **2.4 Define signal catalogs, confidence bands, and reason codes.**
  - Design: Model allowed signal domains, source-service allowlists, confidence bands, severity, legal/region availability, visibility classes, appealability, redaction class, expiry, deprecation state, and public/developer/operator reason-code projections.
  - Output: Signal catalog files, reason-code catalog, confidence-band policy defaults, generated client fixtures, docs snippets, and migration notes for wording changes.
  - Validation: Tests prove code identifiers remain stable when text changes, region-restricted signals produce `signal_unavailable_by_region`, and operator-only categories still expose safe wrapper codes plus Overclaim appeal refs where allowed.

- **2.5 Create deterministic reputation fixtures.**
  - Design: Build fixtures for new provider restricted default, identity-tier signal, node uniqueness overlap, payout uniqueness overlap, network behavior risk, challenge failure, result consistency cluster, abuse report, public sandbox violation, payout hold trigger, appeal correction, expired signal, replay, and recompute backfill.
  - Output: Fixture directory, expected recommendations, Overwatch events, redacted provider explanations, operator evidence projections, replay bundles, and invalid examples.
  - Validation: Fixture tests produce stable states, reason codes, confidence bands, severity, recommendation refs, redaction behavior, replay behavior, and correction history across repeated runs.

## Phase 3: Signal Ingest, Source Authentication, And Region Policy Gating

### Work Items

- **3.1 Implement authenticated signal ingest.**
  - Design: Support `POST /reputation/public-providers/{provider_id}/signals` for onboarding, verification, challenge, behavior, payout, dispute, fraud, and operator-authorized signal refs with service identity, trace id, idempotency key, source service, evidence refs, visibility class, and source-specific schema version.
  - Output: Signal ingest handler, source allowlist, request validator, idempotency store interface, signal id generation, and `reputation.signal_ingested` events.
  - Validation: API tests reject unauthenticated sources, unsupported signal domains, wrong target refs, missing evidence, missing trace id, conflicting idempotency body, stale schema version, and direct raw sensitive payload submission.

- **3.2 Validate target identity, freshness, and ownership scope.**
  - Design: Resolve provider, node, payout, workload, challenge, dispute, and abuse refs through owning services without copying raw private state; verify freshness, target match, source authority, and replay scope.
  - Output: Target resolver, freshness policy, owner-service adapter registry, invalid-source reason codes, and blocked-state mapping.
  - Validation: Tests prove signals cannot be attached to the wrong provider, unknown node, unrelated payout ref, private tenant evidence, expired challenge ref, or withdrawn dispute ref.

- **3.3 Apply operating-region and purpose policy gates.**
  - Design: Evaluate Compliance Boundary and Overguard refs before accepting device/hardware uniqueness, payout-account uniqueness, network/IP/ASN/geography correlation, behavioral timing clusters, shared artifact/result patterns, or cross-provider graph links.
  - Output: Region policy adapter, allowed/review-required/unavailable state, purpose metadata, visibility class assignment, and `signal_unavailable_by_region` behavior.
  - Validation: Tests prove disallowed signals are not collected or substituted, review-required signals cannot auto-suspend or auto-hold, and unavailable signals keep providers in stricter public-limited classes.

- **3.4 Normalize and minimize sensitive inputs.**
  - Design: Store payout-account refs, identity refs, node fingerprint refs, network refs, fraud refs, and cluster refs as hashes, redacted summaries, or owner-service refs with purpose, expiry, correction state, and visibility class.
  - Output: Sensitive-ref normalizer, hashing/commitment helper, raw-payload rejection path, redaction profile metadata, and minimization evidence.
  - Validation: Tests prove raw identity documents, raw bank/card/tax data, exact fingerprints, exact IP histories, biometrics, private payloads, and other-provider identities are rejected or remain in owning services.

- **3.5 Rate-limit and protect signal sources.**
  - Design: Enforce source-specific ingest limits, burst controls, abuse-report flood handling, manual-review thresholds, invalid-signal quarantine, and dependency health behavior.
  - Output: Source rate-limit metadata, quarantine state, signal-source health events, retry behavior, and operator alert hooks.
  - Validation: Tests prove abuse-report floods are bounded, repeated invalid sources are throttled, source outages preserve prior recommendation until expiry, and missing critical signals restrict or require review rather than allowing broader placement.

## Phase 4: Risk Windows, Signal Clustering, And Recommendation Computation

### Work Items

- **4.1 Build reputation window assembly.**
  - Design: Assemble completed workloads, failed workloads, challenge outcomes, disputes, reversals, responsiveness, payout events, abuse markers, onboarding refs, verification refs, and public sandbox compatibility into bounded time windows.
  - Output: Reputation window assembler, window policy, snapshot hash, source freshness map, recompute trigger model, and `reputation.recomputed` prerequisites.
  - Validation: Tests prove windows are deterministic, cite source refs, exclude expired non-contributing signals, preserve corrected history, and fail closed when critical owner-service refs are missing.

- **4.2 Compute layered anti-Sybil signal clusters.**
  - Design: Combine legally allowed identity-tier, node uniqueness, payout uniqueness, network behavior, challenge, result consistency, dispute, abuse, and sandbox violation signals without relying on any single opaque score.
  - Output: Cluster builder, evidence-weight summary, confidence band assignment, severity assignment, visibility-class projection, and operator evidence timeline refs.
  - Validation: Tests prove layered signals increase confidence, single non-critical signals restrict or require review rather than suspend, and cluster details stay redacted from provider-facing outputs.

- **4.3 Implement policy-versioned confidence and severity evaluation.**
  - Design: Apply SDS #36 default bands below `0.50`, `0.50` through `0.75`, `0.75` through `0.90`, and `0.90` plus severity, critical-signal, and review rules.
  - Output: Evaluator module, policy-version refs, confidence-band docs, critical direct signal catalog, and review-required state mapping.
  - Validation: Tests prove missing, stale, inconclusive, or single non-critical signals do not auto-suspend and critical direct signals can recommend severe restriction or suspension only with evidence refs and policy support.

- **4.4 Add recompute, expiry, and backfill mechanics.**
  - Design: Support provider-level recompute, targeted signal recompute, policy-change backfills, expiry sweeps, stale recommendation marking, and dependency-outage behavior.
  - Output: Recompute worker, scoped backfill queue contract, expiry worker, stale-state events, replay refs, and operator controls.
  - Validation: Tests prove recompute is replayable, expiry removes active contribution where policy says so, stale recommendations alert operators, and failed recompute preserves prior recommendation with a stale marker rather than deleting state.

- **4.5 Produce explainable recommendation refs.**
  - Design: Convert risk windows and clusters into eligibility, throttle, duplicate-execution, challenge-cadence, sandbox, and payout-hold trigger recommendation refs with reason codes, evidence refs, confidence band, severity, expiry, and replay bundle refs.
  - Output: Recommendation builder, recommendation id/version model, replay bundle refs, Overwatch event refs, and downstream projection stubs.
  - Validation: Tests prove recommendations cite layered evidence, include expiry and policy/evaluator version, are deterministic for unchanged inputs, and never grant public providers private, regulated, secret-bearing, system-service, uncapped, or broad-egress eligibility.

## Phase 5: Eligibility, Throttle, Challenge Cadence, And Sandbox Recommendations

### Work Items

- **5.1 Publish public-provider eligibility recommendations.**
  - Design: Create Oververify, Overguard, and Oversched-ready recommendations for restricted, eligible public low-sensitivity, throttled, duplicate-execution-required, probation, suspended, appeal-open, corrected, and expired states.
  - Output: Eligibility API projection, consumer contract, expiry fields, allowed workload classes, required sandbox profile, reason codes, and `reputation.eligibility_changed` events.
  - Validation: Consumer tests prove Overguard and Oversched only receive public low-sensitivity eligibility and cannot infer broader private, regulated, secret-bearing, system-service, or uncapped placement rights.

- **5.2 Implement new-provider restricted defaults.**
  - Design: Keep new public providers restricted until onboarding refs, Oververify refs, challenge outcomes, behavior windows, and payout/dispute evidence justify broader public low-sensitivity eligibility.
  - Output: Default recommendation policy, new-provider state fixtures, public-limited sandbox defaults, and remediation hints.
  - Validation: Tests prove unknown or incomplete providers remain restricted, missing onboarding refs keep provider restricted, and no bootstrap path yields broad public-provider trust.

- **5.3 Emit throttle and earning-velocity recommendations.**
  - Design: Recommend rate limits, cooldowns, public workload volume caps, earning-velocity reductions, and review windows based on risk band, severity, challenge history, abuse markers, and public-provider maturity.
  - Output: `public_provider_throttle` records, consumer refs, cooldown policy, expiry behavior, and `reputation.throttle_recommended` events.
  - Validation: Tests prove throttles are evidence-backed, expire or recompute predictably, do not mutate provider earnings directly, and expose provider-safe reason codes plus appeal paths.

- **5.4 Emit duplicate-execution and challenge-cadence recommendations.**
  - Design: Recommend duplicate execution, result consistency checks, benchmark revalidation, challenge cadence changes, and manual review for selected public-provider work.
  - Output: Challenge cadence contract, Challenge Task Service handoff refs, duplicate-execution recommendation refs, and `reputation.challenge_cadence_changed` events.
  - Validation: Tests prove challenge and duplicate-execution recommendations do not orchestrate challenges directly and Challenge Task Service remains the challenge-work owner.

- **5.5 Enforce public sandbox and workload-class limits.**
  - Design: Attach public sandbox profile, no-secret, no-private-data, no-regulated-data, no-system-service, capped runtime, capped resource, output validation, artifact quarantine, and deny-by-default egress requirements to recommendations.
  - Output: Sandbox recommendation fields, Overguard/Oversched projection, failure reason codes, and low-sensitivity allow fixtures.
  - Validation: Public-provider placement tests prove policy fails closed for secrets, private tenant data, regulated data, system services, broad egress, uncapped runtime, uncapped resources, and sandbox mismatch.

## Phase 6: Payout-Hold Trigger Recommendations And Accounting Handoffs

### Work Items

- **6.1 Emit payout-hold trigger recommendations.**
  - Design: Create hold trigger records for high-confidence, policy-allowed, pre-finality cases with provider id, payout refs, severity, evidence refs, proposed hold scope, dispute window, release conditions, and downstream response refs.
  - Output: `payout_hold_trigger` contract, `reputation.payout_hold_recommended` events, accounting consumer projection, and hold-release condition schema.
  - Validation: Tests prove the service emits recommendations only and never creates Seal Ledger entries, ORU balance changes, Overbill records, payout batches, receipts, invoices, refunds, or actual holds.

- **6.2 Gate automatic hold recommendations by confidence and review policy.**
  - Design: Limit automatic hold recommendations to active Overclaim payout disputes, Phase 11 dispute windows, fabricated evidence, impossible benchmarks, high-confidence challenge/duplicate fraud, repeated no-shows, sandbox escape, unauthorized egress, attempted secret/private-data access, legally usable duplicate payout clusters, compliance blockers, and severe Fraud Control cases accepted by policy.
  - Output: Automatic-hold policy catalog, review-required mapping, manual-review checklist, and critical-trigger fixtures.
  - Validation: Tests prove low-confidence clusters, single-source suspicion, region-restricted signals, central-AI-only suspicion, first low-severity challenge failures, broad cluster-wide holds, post-finality reversals, sensitive evidence, cross-region correlation, and high-false-positive cases require stewardship review.

- **6.3 Integrate accounting-owner acknowledgements.**
  - Design: Record downstream acknowledgements, denials, release refs, correction refs, payout finality refs, Overclaim dispute refs, Overbill refs, Provider Payout Service refs, and Seal Ledger refs.
  - Output: Downstream response model, accounting handoff state, denial reason codes, release-condition tracking, and audit refs.
  - Validation: Tests prove downstream denial is recorded without local mutation and actual holds, releases, corrections, and finality remain owned by accounting/dispute services.

- **6.4 Add payout-risk explanation redaction.**
  - Design: Build provider-facing payout risk summaries that show safe reason codes, affected period, coarse category, confidence band, severity, current hold recommendation state, remediation steps, and appeal refs without raw payout or fraud details.
  - Output: Payout redaction profile, provider explanation fields, operator evidence fields, and support/stewardship projection.
  - Validation: Redaction tests prove exact payout hashes, bank/card/tax data, other-provider identities, cluster membership, fraud heuristics, operator notes, incident details, and raw compliance material are never exposed to providers.

- **6.5 Track earning-velocity and payout impact telemetry.**
  - Design: Record internal usage facts for signal ingest, recompute, explanation export, replay, throttle creation, hold trigger creation, downstream acknowledgement, and correction processing.
  - Output: Usage fact schemas, Overmeter handoff refs, Overwatch metrics, dashboard counters, and audit export fields.
  - Validation: Tests prove telemetry supports accountability without pricing assumptions, revenue projections, customer-count assumptions, per-operation external payment calls, or direct accounting mutation.

## Phase 7: Explanation, Redaction, Appeal, Correction, And Retention

### Work Items

- **7.1 Build provider-facing explanation bundles.**
  - Design: Show affected provider/node/payout period or public workload class, high-level cluster category, confidence band, severity, time window, stable reason codes, policy/evaluator versions, redacted evidence refs, current restriction or hold recommendation state, remediation steps, recheck options, and Overclaim appeal refs.
  - Output: `reputation_explanation_bundle` projection, provider redaction profile, API response fixtures, and remediation hint catalog.
  - Validation: Redaction tests prove explanations do not reveal other-provider identities, exact cluster membership, exact IP/device/fingerprint/payout hashes, raw graph edges, model weights, thresholds beyond coarse bands, challenge internals, fraud heuristics, private tenant evidence, raw payout/identity material, operator notes, or incident-response details.

- **7.2 Build operator and stewardship evidence timelines.**
  - Design: Provide authorized operator/stewardship views that join onboarding, verification, challenge, workload, payout, dispute, abuse, correction, replay, and Overwatch refs through signed access decisions and redaction profiles.
  - Output: Operator timeline API, stewardship projection, evidence dereference metadata, access audit events, and export fixtures.
  - Validation: Tests prove operator views require authorization, cite source refs, preserve redaction classes, and cannot be reached through provider-facing endpoints.

- **7.3 Implement Overclaim appeal and correction flow.**
  - Design: Open correction requests through Overclaim refs, mark signals disputed, accept correction outcomes, append replacement refs, recompute recommendations, and expose provider-safe correction state.
  - Output: Appeal/correction API handler, correction record contract, recompute trigger, old/new state links, and `reputation.correction_applied` events.
  - Validation: Tests prove accepted corrections immediately stop active provider-facing penalty where policy allows, append replacement records, preserve audit history, and avoid destructive deletion.

- **7.4 Implement classed retention and minimization.**
  - Design: Apply SDS #36 classed retention: 180-day redacted history for non-payout low-severity operational signals, two years for eligibility/throttle/challenge/duplicate/correction signals, and seven years or stricter requirement for payout-hold, confirmed fraud, dispute, compliance, suspension, and finality refs.
  - Output: Retention policy, minimization worker, owner-service raw-input minimization hooks, audit refs, and legal/compliance override model.
  - Validation: Tests prove raw correlation inputs are minimized sooner when allowed while hashes, reason codes, policy versions, correction refs, replay bundle refs, and Overwatch audit refs remain long enough for explanations, appeals, and accounting finality.

- **7.5 Add replayable explanation and correction evidence.**
  - Design: Bind explanations and corrections to signal replay bundles, input refs, policy version, evaluator version, risk calculation, recommendation refs, and Overwatch event refs.
  - Output: Explanation replay helper, correction replay helper, replay hash commitments, and audit export fixtures.
  - Validation: Replay tests prove stored refs reproduce recommendation/explanation state for unchanged retained inputs and clearly mark unavailable minimized raw inputs without inventing evidence.

## Phase 8: APIs, Consumer Profiles, Dashboards, And Stewardship Views

### Work Items

- **8.1 Expose internal and operator-facing APIs.**
  - Design: Implement signal ingest, recompute, reputation read, eligibility read, explain, appeal, throttle, hold-trigger, replay, and consumer projection endpoints with signed envelopes, trace ids, idempotency keys, pagination, redaction, and stable errors.
  - Output: API routes, request/response models, auth checks, OpenAPI or schema docs where used, and endpoint fixtures.
  - Validation: Contract tests cover all endpoints, authorization paths, redaction classes, idempotency conflicts, missing refs, stale refs, replay, and provider/operator projection differences.

- **8.2 Build Oververify, Overguard, Oversched, and Challenge Task Service projections.**
  - Design: Shape consumer-specific projections for final verification/eligibility updates, admission and workload-class restrictions, placement throttles, duplicate-execution requirements, sandbox profile, and challenge cadence.
  - Output: Consumer contracts, projection adapters, expiry behavior, reason-code mapping, and integration fixtures.
  - Validation: Integration tests prove each consumer receives only its authorized fields and final authority stays with Oververify, Overguard, Oversched, and Challenge Task Service respectively.

- **8.3 Build accounting, dispute, and fraud-control projections.**
  - Design: Shape hold trigger, dispute, correction, abuse cluster, fraud escalation, denial, release, and finality projections for Provider Payout Service, Overbill, Seal Ledger, Overclaim, and Fraud Control Service.
  - Output: Accounting/fraud consumer contracts, downstream ack tracking, denial/release refs, and escalation fixtures.
  - Validation: Tests prove accounting projections never mutate balances directly and Fraud Control Service remains broader investigation and incident owner.

- **8.4 Build SDK, CLI, admin UI, native app, and central AI read profiles.**
  - Design: Provide role-aware read models, command hints, admin action refs, provider-safe messages, remediation links, central AI stewardship summaries, and native-app/wallet usage warning surfaces.
  - Output: Consumer response profiles, SDK/CLI examples, admin UI field list, central AI summary schema, and provider remediation fixtures.
  - Validation: Tests prove client profiles use stable reason codes, avoid operator internals, respect redaction, and cannot turn recommendations into direct payout, placement, or suspension authority.

- **8.5 Build dashboards, alerts, and operational counters.**
  - Design: Track new public providers, restricted providers, eligible providers, throttle counts, hold recommendations, appeal rates, corrected signals, cluster risk, recompute freshness, signal source health, and false-positive correction rate.
  - Output: Metrics schema, dashboard definitions, alert rules, Overwatch event aggregation, and operator runbook links.
  - Validation: Operations tests prove alerts fire for suspected Sybil clusters, sudden public-provider volume spikes, repeated challenge failures, payout-account reuse clusters, high false-positive correction rates, recompute failure, and signal-source flood behavior.

## Phase 9: Replay, Backfill, Grid, Storage, Public-Provider, And Governance Hardening

### Work Items

- **9.1 Harden replay and recompute backfills.**
  - Design: Support policy-change backfills, evaluator-version backfills, source-service correction backfills, expiry sweeps, replay comparison, minimized-input markers, and scoped recompute jobs.
  - Output: Backfill worker, replay comparison model, backfill run records, progress events, and operator controls.
  - Validation: Tests prove backfills are idempotent, bounded, resumable, replayable, and do not erase prior decisions or silently widen eligibility.

- **9.2 Prepare grid-resident operation.**
  - Design: Package the service as a protected grid-resident system workload with service identity, config contracts, secret refs, health checks, failover behavior, restore drills, maintenance mode, and signed operator actions.
  - Output: System-service packaging notes, runtime config contract, backup/restore hooks, health endpoints, and break-glass audit rules.
  - Validation: Grid tests prove service restart, failover, restore, recompute pause/resume, and maintenance mode preserve append-only history and do not emit stale broad eligibility after recovery.

- **9.3 Add native Overbase, Overstore, and Overvault handoffs.**
  - Design: Move records to native Overbase when available, store evidence/replay artifacts through Overstore where appropriate, and store private/compliance refs through Overvault without changing public API semantics.
  - Output: Persistence adapter interfaces, migration plan, artifact refs, private-ref handoffs, retention hooks, and export format.
  - Validation: Migration tests prove no conventional database/object/vault boundary becomes canonical, records remain replayable, and raw secrets/private payloads never enter reputation records.

- **9.4 Harden public-provider and federation boundary behavior.**
  - Design: Distinguish known federation capacity, public-interest pools, and unknown/semi-trusted public providers; enforce purpose tags, public sandbox labels, challenge cadence, duplicate execution, fraud refs, and payout hold policy per provider class.
  - Output: Provider-class policy matrix, Phase 10/Phase 11 transition rules, public-provider fixtures, and federation/public-interest projection guards.
  - Validation: Tests prove trusted federation does not automatically imply open public trust, public providers cannot receive sensitive workloads, and stricter public-provider policy overrides broad batch or placement assumptions.

- **9.5 Add governance, compliance, threat-model, and incident handoffs.**
  - Design: Integrate Compliance Boundary policy refs, stewardship reporting, threat-model findings, incident response escalation refs, audit export, PIP change tracking, and region-specific retention or appeal requirements.
  - Output: Governance checklist, compliance export schema, threat-model test list, incident handoff refs, stewardship report fields, and PIP change controls.
  - Validation: Governance tests prove risk overrides require signed action, evidence refs, expiry, Overwatch audit, appealability where required, and no hidden policy change can silently alter provider-facing outcomes.

## Phase 10: Validation, Documentation Alignment, Queue State, And Handoff

### Work Items

- **10.1 Validate sub-build-plan structure.**
  - Design: Check title prefix, attached SDS link, phase headings 1 through 10, work-item structure, Design/Output/Validation fields, alignment review, and exit gate.
  - Output: Focused validation result for `SUB BUILD PLAN #36`.
  - Validation: Scripted text checks pass for required headings, numbered phases, local links, work-item sections, final newline, and tab-free formatting.

- **10.2 Validate tech-stack alignment.**
  - Design: Scan for accidental conventional cloud, SaaS-admin, blockchain/NFT, pricing, revenue, customer-count, opaque trust-score, direct-payout-mutation, and broad-public-provider assumptions.
  - Output: Stack-alignment scan result and any corrected wording.
  - Validation: Scan shows only approved negative-control references or explicit non-choice guardrails from `docs/overrid_tech_stack_choice.md`.

- **10.3 Validate SDS, service catalog, master plan, and crosswalk links.**
  - Design: Confirm this plan is linked from the SDS source table, service catalog detailed-SDS section, master per-SDS table, and service-catalog alignment index.
  - Output: Updated docs and link-check evidence.
  - Validation: Local Markdown link validation returns no missing local targets for changed docs.

- **10.4 Validate queue and progress state.**
  - Design: Mark `036-build-plan` complete, update `.codex55_sds_queue/progress.md`, append build-plan progress evidence, and record the next incomplete build-plan task.
  - Output: Queue JSON state, queue progress summary, and build-plan progress notes.
  - Validation: JSON validation passes, queue counts match total tasks, no running task remains for SDS #36, and next incomplete build-plan task starts at `037-build-plan`.

- **10.5 Reindex and verify retrieval handoff.**
  - Design: Refresh Docdex for the new plan and linked docs, then verify search returns the SDS #36 plan and source docs in the aligned result set.
  - Output: Docdex index refresh evidence, retrieval evidence, repo memory, and implementation handoff note.
  - Validation: Docdex index succeeds, Docdex search for `SUB BUILD PLAN #36 Reputation Anti-Sybil Service Phase 11` returns the new sub-build plan in the result set, and `docdexd run-tests` blocker is recorded if no test runner is configured.
