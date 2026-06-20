# SUB BUILD PLAN #60 - Central AI Service

Attached SDS: [docs/sds/ai_rag_model_routing/central_ai_service.md](../sds/ai_rag_model_routing/central_ai_service.md)

## Purpose

This sub-build plan turns SDS #60 into an implementation sequence for Central AI Service. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Central AI Service is the Phase 12 evidence-bounded stewardship and recommendation service behind the first Central AI Stewardship Interface fraud/evidence review queue, with stronger authority, reporting, retention, security, and compliance hardening in Phase 13. It owns evidence package intake, analysis jobs, risk assessments, grant/public-interest/surplus-routing recommendations, intervention proposals, governance report refs, recommendation review state, corrections, retractions, model/run provenance, usage refs, audit refs, and replay bundles. It does not directly mutate Seal Ledger, ORU balances, provider payouts, grants, provider eligibility, dispute finality, policy finality, or public report publication; those decisions remain with the owning services and steward review bodies.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #60: Central AI Service](../sds/ai_rag_model_routing/central_ai_service.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Central AI Service plan](../service_catalog/ai_rag_model_routing/central_ai_service.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, signed envelopes, trace ids, idempotency, stable errors, local stack, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant scopes, Overkey signing refs, Overgate ingress, Overregistry service records, Overwatch audit refs, and Overqueue command/event primitives. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy decisions, Policy Dry-Run previews, Overclaim dispute refs, Challenge Task evidence, Oververify facts, Workload Classifier facts, and deny-by-default semantics. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies Overmeter usage refs, Seal Ledger/ORU/Overgrant/Provider Payout handoff boundaries, and the rule that Central AI recommends but does not mutate accounting or allocation state. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Supplies AI Gateway Router product-routing contracts, route decisions, model/resource capability refs, fallback evidence, usage refs, and replay substrate. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase, Overstore, Overvault, namespace, private refs, protected content refs, encrypted context refs, redaction boundaries, and metadata-only replay substrates. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Supplies Purpose Tag Registry, Public-Interest Pool Service, Overgrant expansion, trusted federation refs, purpose scopes, quotas, fairness windows, and public-interest evidence. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies Fraud Control, Public Provider Onboarding, Public Sandbox Profile, public-provider fraud evidence, challenge refs, payout-risk refs, corrections, appeals, and low-sensitivity public proof boundaries. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Controls the first Central AI Service build point through a read-heavy Central AI Stewardship Interface fraud/evidence review queue and later grant/public-interest/native-surplus review surfaces. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies formal authority boundaries, stewardship reporting, compliance boundaries, incident escalation, threat review, retention, legal holds, redaction review, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #60 first build work aligned to master Phase 12, with Phase 13 as governance/security/compliance hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, speculative-market, pricing, revenue, customer-count, or external-provider-as-core-boundary drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 6, 8, 10, 11, 12, and 13 | Attach SDS #60, preserve Phase 12 as first build, record prerequisites, and freeze Central AI authority boundaries. |
| 2 | Master Phases 0, 1, 4, 5, 6, 8, 10, 11, and 12 | Define Rust contracts, canonical schemas, state machines, stable errors, signed refs, hashes, fixtures, and redaction-safe examples. |
| 3 | Master Phases 1, 4, 8, 11, and 12 | Implement evidence package intake, scope validation, privacy guards, lifecycle records, and missing-evidence prompts. |
| 4 | Master Phases 4, 5, 6, 8, and 12 | Integrate AI Gateway Router, authorized context planning, model/run provenance, usage refs, route failure behavior, and replay metadata. |
| 5 | Master Phases 4, 5, 10, 11, and 12 | Implement bounded fraud, abuse, grant, public-interest, policy evidence, and system-health analysis jobs that produce recommendations only. |
| 6 | Master Phases 4, 5, 10, 11, 12, and 13 | Implement recommendation review, owning-service handoff, intervention proposals, corrections, retractions, expiry, and audit coordination. |
| 7 | Master Phases 10, 11, and 12 | Prove Central AI Stewardship Interface queues for fraud/evidence, grants, public-interest recommendations, native surplus review, and steward actions. |
| 8 | Master Phases 5, 8, 12, and 13 | Implement report refs, redaction review, replay APIs, dashboards, observability, audit exports, and stewardship reporting handoffs. |
| 9 | Master Phase 13, with prerequisites from Phases 0 through 12 | Harden authority boundaries, compliance, retention, security, threat models, incident playbooks, and reliability behavior. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, authority boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Central AI Service core is a Rust service/module using shared contract crates, Tokio for bounded background analysis/replay workers, and Axum/Tower/Hyper-style HTTP only where a service boundary is needed.
- Evidence packages, analysis jobs, risk assessments, recommendation records, intervention proposals, public-interest recommendations, governance report refs, model/run provenance, review records, usage refs, events, fixtures, replay bundles, redaction profiles, and audit exports use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be generated later only through the shared contract layer.
- Mutating APIs require signed actor, service, steward, or operator envelopes, tenant/system scope, trace id, idempotency key, evidence refs, policy refs, route refs where applicable, privacy/data-class refs, appeal/review refs, schema version, stable reason codes, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for evidence package snapshots, context plans, prompt template refs, model/run results, recommendations, intervention proposals, report refs, replay bundles, audit exports, and deterministic fixtures.
- Storage, queueing, object persistence, vault, namespace, audit, usage, policy, and accounting handoffs must use native Overrid service boundaries such as Overbase, Overqueue, Overstore, Overvault, Overpass, Overwatch, Overmeter, Overguard, Seal Ledger, ORU Account Service, Overgrant, and Public-Interest Pool Service.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, speculative assets, pricing, revenue projections, customer-count assumptions, direct private-context reads, final enforcement authority, final grant authority, final fraud authority, final dispute authority, or direct accounting mutation the Central AI Service boundary.

## Phase 1: SDS Attachment, Phase 12 Scope, And Authority Boundaries

### Work Items

- **1.1 Attach the build plan to SDS #60.**
  - Design: Link this document from the Central AI Service SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/ai_rag_model_routing/central_ai_service.md`, `docs/service_catalog/ai_rag_model_routing/central_ai_service.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #60 returns both the Central AI Service SDS and this sub-build plan.

- **1.2 Preserve master Phase 12 as the first build point.**
  - Design: Keep the first implementation in Phase 12 because the first useful surface is a read-heavy fraud/evidence review queue inside Central AI Stewardship Interface, backed by already-built identity, policy, usage, protected refs, public-interest, fraud, and AI routing prerequisites.
  - Output: Phase-gate note that Phases 0, 1, 4, 5, 6, 8, 10, and 11 supply prerequisites, Phase 12 builds the service/interface proof, and Phase 13 hardens formal authority, compliance, security, reporting, and scale.
  - Validation: Review proves the plan does not move Central AI into Phase 6 routing, Phase 10 public-interest allocation authority, Phase 11 fraud finality, or Phase 13-only governance.

- **1.3 Define Phase 13 as hardening, not initial delivery.**
  - Design: Record that Phase 13 strengthens authority boundaries, legal/compliance rules, stewardship reporting, incident escalation, threat review, retention, and public reporting after the Phase 12 review queue proves evidence-bound operation.
  - Output: Hardening checklist covering compliance boundaries, public report publication rules, severe sanction review, legal hold retention, redaction review, and threat model evidence.
  - Validation: Review confirms the Phase 12 service does not claim final governance authority, and Phase 13 work does not hide the Phase 12 implementation need.

- **1.4 Freeze Central AI ownership boundaries.**
  - Design: Record that Central AI owns evidence intake, analysis jobs, recommendations, intervention proposals, report refs, review states, corrections, retractions, provenance, and replay evidence.
  - Output: Ownership checklist for architecture and implementation reviews.
  - Validation: Review confirms Central AI does not mutate ledgers, ORU balances, payouts, grants, provider eligibility, public-provider status, policy bundles, dispute finality, or report publication without owning-service/steward action.

- **1.5 Carry forward resolved SDS #60 decisions.**
  - Design: Preserve the resolved rules for non-mutating support work, payout-hold/suspension recommendation thresholds, public-interest pre-action visibility, provenance audience classes, and the first Phase 12 review queue.
  - Output: Resolved-decision checklist covering low-risk automation, severe sanction review, source-trusted evidence packages, public-interest redaction, model/run provenance exposure, retention classes, fraud/evidence queue scope, grant review ordering, public-interest ordering, report publication ordering, and native-surplus ordering.
  - Validation: Review rejects heuristic-only sanction proposals, Central-AI-only final actions, public exposure of private allegations, raw prompt/private evidence leakage, and grant/public-interest/native-surplus work before its prerequisites are stable.

## Phase 2: Rust Contracts, Schemas, State Machines, And Fixtures

### Work Items

- **2.1 Create the Central AI Rust contract module.**
  - Design: Add contract types for evidence package, analysis job, risk assessment, recommendation record, intervention proposal, public-interest recommendation, governance report ref, model/run provenance, review record, event payload, redaction profile, usage ref, replay bundle, and stable errors.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, review-action enums, recommendation-type enums, severity/confidence bands, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from model worker internals, AI Gateway internals, owning-service mutation logic, and governance finality.

- **2.2 Define evidence package and scope schemas.**
  - Design: Model `central_ai_evidence_package` with source service, evidence refs, data classes, privacy constraints, purpose tag refs, policy refs, retention class, allowed analysis scope, redaction requirements, appeal refs, owner refs, and hashable snapshots.
  - Output: JSON Schema files, valid examples, invalid examples, generated validators, docs-facing examples, and canonical evidence fixtures.
  - Validation: Schema tests reject missing source service, missing data class, missing allowed scope, missing privacy policy, missing retention class, missing redaction requirements, raw private evidence payloads, and unhashable evidence refs.

- **2.3 Define analysis job and provenance schemas.**
  - Design: Model `analysis_job` and `model_run_provenance` with job type, requester refs, route decision refs, prompt template refs, context plan refs, tool refs, output hash, usage refs, replay constraints, lifecycle state, denial/failure reason codes, and audit refs.
  - Output: Analysis job schema, model/run provenance schema, lifecycle state machine, denied-analysis examples, route-failure examples, invalid-output examples, and replay fixtures.
  - Validation: Tests reject jobs without evidence refs, jobs without privacy/scope checks, jobs without route refs where model work is required, jobs with raw private prompts in public views, and outputs lacking result hashes.

- **2.4 Define recommendation, intervention, report, and review schemas.**
  - Design: Model `risk_assessment`, `recommendation_record`, `intervention_proposal`, `public_interest_recommendation`, `governance_report_ref`, and `recommendation_review` with evidence refs, policy thresholds, owner service refs, proportionality, confidence, appeal path, expiry, review state, correction/retraction refs, redaction profile, and downstream command refs.
  - Output: Recommendation schema set, intervention schema, report-ref schema, review schema, stable error catalog, role-scoped redacted examples, and negative fixtures.
  - Validation: Tests reject recommendations without evidence refs, policy refs, owner refs, appeal path, expiry, review state, or proportionality; tests prove corrections/retractions create new linked records instead of mutating prior recommendations.

- **2.5 Create deterministic Central AI fixtures.**
  - Design: Build fixtures for fraud evidence intake, missing scope denial, unauthorized private context denial, valid route selection, route unavailable, invalid model output, risk assessment, grant recommendation, public-interest recommendation, surplus-routing recommendation, intervention proposal, owner review, correction, retraction, expired recommendation, report redaction, and replay.
  - Output: Fixture directory, canonical inputs, expected states, expected events, expected stable errors, BLAKE3 hashes, redacted views, policy refs, usage refs, and replay bundles.
  - Validation: Fixture tests produce stable ids, hashes, lifecycle states, reason codes, audit refs, redacted outputs, usage refs, review states, correction links, retraction links, and replay outputs across repeated runs.

## Phase 3: Evidence Intake, Scope Validation, And Privacy Guards

### Work Items

- **3.1 Implement evidence package registration.**
  - Design: Add `POST /central-ai/evidence-packages` with signed envelope checks, source-service validation, tenant/system scope, evidence refs, data-class refs, privacy constraints, purpose tags, policy refs, retention class, trace id, idempotency key, and Overwatch audit refs.
  - Output: Request handler, evidence package record, idempotency behavior, stable errors, and `central_ai_service.evidence_package_registered` events.
  - Validation: API tests cover valid evidence packages, duplicate idempotency keys, missing identity, missing source service, missing privacy constraints, missing data class, unknown evidence refs, malformed requests, and audience-safe errors.

- **3.2 Implement scope and policy precheck.**
  - Design: Validate allowed analysis scope, source-service authority, data classes, privacy constraints, purpose tags, Overguard/Policy Dry-Run refs, appeal/review requirements, and owner-service target before accepting analysis.
  - Output: Scope checker, policy precheck adapter, missing-prerequisite records, `analysis_scope_too_broad` behavior, and denial audit refs.
  - Validation: Tests prove broad scope, missing owner, missing policy refs, missing appeal path, private-context gaps, and policy denial stop analysis before route selection or model work.

- **3.3 Implement evidence package lifecycle.**
  - Design: Track evidence package states from registered through scope checked, usable, denied, superseded, expired, and withdrawn while preserving immutable package snapshots and correction links.
  - Output: Evidence lifecycle state machine, state transition rules, correction/supersession refs, expiry behavior, and event payloads.
  - Validation: Tests prove snapshots are immutable, corrections link to prior packages, expired packages cannot start new severe recommendations, and withdrawn packages invalidate pending analysis safely.

- **3.4 Implement missing-evidence prompts.**
  - Design: Produce bounded missing-evidence prompts when required evidence, policy thresholds, owner refs, appeal paths, route refs, or redaction rules are absent.
  - Output: Missing-evidence prompt records, owner-service request summaries, steward-facing summaries, redacted caller views, and no-analysis audit refs.
  - Validation: Tests prove missing-evidence prompts do not become recommendations, do not expose private evidence internals, and cite the exact missing prerequisite classes.

- **3.5 Implement role-scoped evidence reads.**
  - Design: Add read projections for requester, owning service, steward, auditor, affected party, and public report contexts using redaction profiles and protected refs instead of raw private content.
  - Output: Evidence read API/projections, redaction profile mapping, role-specific examples, and Overwatch audit refs.
  - Validation: Tests prove public views never include raw private evidence, affected-party views are policy scoped, steward/auditor views require explicit authority refs, and redaction failures block report publication.

## Phase 4: AI Gateway Routing, Authorized Context, And Provenance

### Work Items

- **4.1 Integrate AI Gateway Router decisions.**
  - Design: Route analysis jobs through AI Gateway Router using request classification, context access plans, model/resource capability snapshots, route decisions, fallback plans, usage refs, and replay refs.
  - Output: Gateway client adapter, route request mapping, route decision refs, route failure handling, fallback refs, and `central_ai_service.analysis_requested` events.
  - Validation: Tests prove Central AI does not hardcode model/provider names, bypass route decisions, weaken fallback constraints, or run model work without route evidence.

- **4.2 Integrate authorized context plans.**
  - Design: Use Encrypted Docdex RAG Adapter, Overvault, Overstore, and owning-service refs only through authorized context plans that specify retrieval scope, data class, redaction requirements, denied context, and replay constraints.
  - Output: Context plan adapter, allowed/denied context records, prompt/context refs, redaction requirements, and private-context denial behavior.
  - Validation: Tests prove Central AI does not decrypt Docdex indexes, read private storage directly, persist raw private context, widen retrieval scope, or hide denied context from replay.

- **4.3 Implement model/run provenance capture.**
  - Design: Capture route decision id, model/resource class, prompt template version, context-plan hash, tool refs, output hash, usage refs, result validation status, replay constraints, and audience-class redaction.
  - Output: Provenance writer, provenance read projections, public/steward/auditor redaction profiles, and replay fixture hooks.
  - Validation: Tests prove provenance is recorded for every model-backed analysis, public views expose only allowed hashes/classes/refs, and private prompts or decrypted RAG content do not leak.

- **4.4 Implement invalid-output guardrails.**
  - Design: Reject model outputs that lack evidence refs, policy threshold refs, owner-service refs, confidence/proportionality bands, appeal paths, review requirements, or schema-valid recommendation payloads.
  - Output: Output validator, invalid-output records, `model_output_invalid` behavior, correction prompts, and audit refs.
  - Validation: Tests prove invalid outputs cannot create recommendations, intervention proposals, public-interest records, or report refs.

- **4.5 Implement route degradation behavior.**
  - Design: Handle route unavailable, context unavailable, model failure, fallback denied, timeout, cancellation, and stale route snapshot states without broadening authority or hiding evidence gaps.
  - Output: Degradation state rules, retry/fallback records, denial summaries, stale-route replacement behavior, and operator diagnostics.
  - Validation: Tests prove degradation records are replayable, role-scoped, and never weaken privacy, locality, policy, review, or evidence requirements.

## Phase 5: Bounded Fraud, Grant, Public-Interest, And Policy Analysis

### Work Items

- **5.1 Implement fraud and abuse analysis jobs.**
  - Design: Analyze Fraud Control, Overclaim, Overwatch, Oververify, Challenge Task, Provider Payout, Public Sandbox Profile, and public-provider evidence packages to produce evidence-bound risk assessments and recommendation drafts.
  - Output: Fraud/abuse job type, risk assessment records, missing-evidence outputs, recommended next actions, correction paths, and owner-service handoff refs.
  - Validation: Tests prove Central AI does not suspend providers, hold payouts after finality, decide disputes, or bypass Fraud Control/Overclaim/Provider Payout review.

- **5.2 Implement grant recommendation jobs.**
  - Design: Analyze Overgrant, Purpose Tag Registry, Seal Ledger, ORU, usage, outcome, and eligibility refs to draft grant recommendations with evidence, purpose scope, fairness facts, confidence, proportionality, and review state.
  - Output: Grant recommendation job type, recommendation records, Overgrant handoff refs, review requirements, expiry rules, and redacted summaries.
  - Validation: Tests prove Central AI does not authorize grants, allocate resources, mutate ORU/Seal Ledger state, or bypass Overgrant eligibility and steward review.

- **5.3 Implement public-interest allocation recommendation jobs.**
  - Design: Analyze Public-Interest Pool Service, Purpose Tag Registry, usage/outcome refs, fairness windows, quotas, abuse throttles, and public evidence to draft pool allocation recommendations.
  - Output: Public-interest recommendation records, pool refs, purpose tag refs, candidate refs, fairness facts, objection/correction path, review refs, and publication redaction profiles.
  - Validation: Tests prove recommendations involving private candidates, regulated work, sensitive research, abuse facts, or unresolved eligibility disputes stay steward/participant scoped until redaction review permits publication.

- **5.4 Implement native surplus-routing recommendation jobs.**
  - Design: Analyze structural native-service surplus records and public-interest pool refs only after real native-service surplus records exist, producing recommendations without financial projections or direct spending.
  - Output: Surplus-routing recommendation type, source refs, pool refs, target candidate refs, confidence/proportionality bands, review state, and downstream owner-service handoff.
  - Validation: Tests prove Central AI does not compute speculative revenue forecasts, hardcoded prices, payout amounts, per-transaction costs, or direct allocation commands.

- **5.5 Implement policy evidence and system-health analysis jobs.**
  - Design: Analyze policy evidence, route failures, fraud trends, public-provider health, report refs, incidents, compliance boundaries, and system metrics to draft stewardship analysis and governance report inputs.
  - Output: Policy evidence job type, system-health assessment records, report draft refs, aggregate metric refs, privacy-preserving summaries, and incident/report handoff refs.
  - Validation: Tests prove report drafts remain unpublished until redaction/review passes and sensitive fraud, compliance, private tenant, or security internals remain role-scoped.

## Phase 6: Recommendations, Reviews, Handoffs, Corrections, And Retractions

### Work Items

- **6.1 Implement recommendation creation.**
  - Design: Add `POST /central-ai/recommendations` for schema-valid recommendations derived from an evidence package or analysis job, with evidence refs, policy refs, route refs, confidence, proportionality, owner service, appeal path, expiry, and review state.
  - Output: Recommendation writer, immutable records, stable errors, `central_ai_service.recommendation_created` events, and redacted read projections.
  - Validation: Tests prove recommendations without evidence refs, policy refs, owner refs, appeal path, expiry, or review requirements are rejected.

- **6.2 Implement owning-service intervention proposals.**
  - Design: Add `POST /central-ai/intervention-proposals` for proposed hold, throttle, challenge, grant, revocation, report, or review actions routed to the owning service for final evaluation.
  - Output: Intervention proposal records, owner-service command refs, required review flags, release conditions, downstream status refs, and `central_ai_service.intervention_proposed` events.
  - Validation: Tests prove proposals do not directly mutate owning-service state and severe sanctions require human/steward review plus appeal path before downstream action.

- **6.3 Implement recommendation review.**
  - Design: Add `POST /central-ai/recommendations/{recommendation_id}/review` for accept, reject, request-info, retract, correct, expire, and owner-service status actions with signed steward/operator/service envelopes.
  - Output: Review handler, review records, state transitions, role-scoped audit refs, and `central_ai_service.recommendation_reviewed` events.
  - Validation: Tests prove invalid transitions are rejected, review actors require authority refs, accepted actions still route through owning services, and review state is replayable.

- **6.4 Implement corrections, retractions, and supersession.**
  - Design: Create new linked records when evidence changes, analysis was wrong, policy thresholds changed, provenance was invalid, redaction changed, or owning-service review rejects a recommendation.
  - Output: Correction writer, retraction writer, supersession links, notification refs, old/new comparison views, and `central_ai_service.recommendation_retracted` events.
  - Validation: Tests prove prior recommendations remain immutable, downstream services receive correction/retraction notices, and public summaries expose corrected status without private evidence leakage.

- **6.5 Implement expiry and stale-policy handling.**
  - Design: Expire recommendations when policy thresholds, evidence freshness, appeal windows, route snapshots, capability classes, public-interest windows, or owner-service conditions become stale.
  - Output: Expiry worker, stale-policy detector, owner notification refs, replay records, and safe no-op behavior.
  - Validation: Tests prove expired recommendations cannot be accepted for severe actions, stale evidence triggers reanalysis or correction, and expiry does not delete audit evidence.

## Phase 7: Central AI Stewardship Interface And Review Workflows

### Work Items

- **7.1 Build the fraud and evidence review queue.**
  - Design: Expose Phase 12 read-heavy queue views for evidence packages, recommendation drafts, missing-evidence prompts, model/run provenance refs, proposed hold/throttle/challenge actions, appeal/correction paths, and signed review actions.
  - Output: Interface-facing query projections, queue filters, detail views, review actions, owner-service handoff refs, and role-scoped redactions.
  - Validation: UI/API tests prove the queue cannot directly suspend providers, hold finalized payouts, mutate disputes, or expose private evidence to unauthorized roles.

- **7.2 Build grant recommendation review.**
  - Design: Add steward review views for grant recommendations after proof-pool and Overgrant flows are stable, showing evidence refs, purpose tags, eligibility/fairness facts, confidence, proportionality, conflicts, and correction path.
  - Output: Grant review projections, review actions, Overgrant handoff refs, redacted summaries, and objection/correction records.
  - Validation: Tests prove grant review does not authorize allocations directly and private grantee or sensitive eligibility material stays scoped.

- **7.3 Build public-interest allocation review.**
  - Design: Add public-interest pool review views for candidate/project class, public evidence refs, purpose tags, fairness/quota facts, confidence/proportionality band, conflict markers, expiry, and objection path.
  - Output: Public-interest review projections, pool handoff refs, publication eligibility flags, redaction profile refs, and review audit refs.
  - Validation: Tests prove unreviewed allegations, private candidates, regulated work, donor privacy, and unresolved disputes are not exposed in public pre-action views.

- **7.4 Build native surplus-routing review.**
  - Design: Add native-service surplus-routing review only after native services produce structural surplus refs, showing source refs, pool refs, purpose alignment, recommendation confidence, owner-service handoff, and review state.
  - Output: Surplus-routing review projections, handoff refs, review actions, no-spending guardrails, and replay refs.
  - Validation: Tests prove the review surface does not introduce pricing, revenue forecasts, customer-count assumptions, direct spending commands, or allocation mutation.

- **7.5 Add SDK, CLI, and admin review commands.**
  - Design: Provide typed SDK/CLI/admin commands for listing queues, reading recommendations, submitting reviews, requesting missing evidence, correcting/retracting records, and exporting replay bundles.
  - Output: Command contracts, stable JSON output, generated bindings, signed envelope examples, idempotency behavior, and operator docs.
  - Validation: Contract tests prove commands require identity/tenant/scope refs, produce stable JSON, preserve idempotency, and match the same authority boundaries as the service APIs.

## Phase 8: Reports, Redaction, Replay, Observability, And Stewardship Handoffs

### Work Items

- **8.1 Implement governance report refs.**
  - Design: Add `POST /central-ai/reports` for report drafts and publication refs with time window, included evidence refs, aggregate metrics, redaction profile, publication state, review refs, and audit refs.
  - Output: Report-ref writer, report lifecycle, publication state projections, redaction-pending behavior, and `central_ai_service.report_ref_created` events.
  - Validation: Tests prove reports remain withheld until redaction/review passes and report refs do not include raw private evidence or sensitive fraud internals.

- **8.2 Implement redaction review workflow.**
  - Design: Require redaction review for public report sections, public-interest summaries, fraud trend summaries, compliance-sensitive findings, and affected-party summaries.
  - Output: Redaction review records, approval/rejection actions, withheld-state behavior, corrected-publication links, and audit refs.
  - Validation: Tests prove redaction failures block publication, corrections create linked records, and public views expose only allowed summaries, hashes, ids, version refs, reason codes, usage refs, and Overwatch refs.

- **8.3 Implement replay APIs.**
  - Design: Add `GET /central-ai/replay/{analysis_or_recommendation_id}` to reconstruct permitted evidence refs, route decisions, prompt/context refs, model/run provenance, output hash, recommendation chain, review outcome, corrections, retractions, and report refs.
  - Output: Replay API, role-scoped replay bundles, redaction profiles, pagination for review chains, and audit export hooks.
  - Validation: Tests prove replay is deterministic, privacy bounded, and able to reconstruct analysis and recommendation chains without fetching raw private payloads.

- **8.4 Implement observability and dashboards.**
  - Design: Expose analysis backlog, denied analyses, route failures, evidence-scope failures, recommendation volume, accepted/rejected recommendations, retractions, corrections, appeal outcomes, report state, severe recommendation alerts, and model failure rates.
  - Output: Metrics, tracing spans, alert rules, dashboard projections, operator views, and Overwatch event links.
  - Validation: Tests prove metrics are role-scoped, alerts fire for missing appeal paths/private-context denial spikes/severe recommendations/stale policy thresholds/high model failure rates, and dashboards avoid private payload leakage.

- **8.5 Integrate Stewardship Reporting and incident handoffs.**
  - Design: Hand report refs, aggregate metrics, compliance-sensitive findings, threat-review inputs, incident refs, correction/retraction summaries, and public-safe evidence refs to Stewardship Reporting, Compliance Boundary Service, Incident Response Service, and Threat Modeling Tracker.
  - Output: Handoff records, export schemas, redacted report bundles, incident/report correlation ids, and publication-ready summaries.
  - Validation: Tests prove handoffs cite evidence, policy refs, redaction profile, audit refs, and review state, and do not bypass report publication or incident response authority.

## Phase 9: Governance, Compliance, Security, Retention, And Scale Hardening

### Work Items

- **9.1 Harden formal authority boundaries.**
  - Design: Encode Phase 13 policies for which Central AI outputs can remain automated support and which require owner-service, steward, legal, compliance, or appeal-body review.
  - Output: Authority boundary policy matrix, severe-action gates, low-risk automation allowlist, review-required conditions, and denial reason codes.
  - Validation: Tests prove severe sanctions, provider suspension, post-finality payout holds, grant/pool allocation, native-surplus routing, public report publication, compliance-sensitive action, eligibility changes, and policy changes require the correct review path.

- **9.2 Harden retention, legal hold, and audit policy.**
  - Design: Apply audience-class retention for caller summaries, private/regulated replay details, mismatch/rollout evidence, long-term hashes/ids/version refs, legal holds, incidents, claims, compliance audits, and deletion/redaction rules.
  - Output: Retention policy config, legal hold refs, archive compaction rules, redaction tombstones, and audit export fixtures.
  - Validation: Tests prove retention follows SDS #60 provenance rules, legal holds override expiry, public archives reduce to safe refs, and deletion does not break audit chains.

- **9.3 Harden security and threat models.**
  - Design: Threat-model prompt/context injection, evidence poisoning, collusion, fraud heuristic leakage, privacy boundary bypass, model/provider compromise, route fallback abuse, report redaction failure, and steward action abuse.
  - Output: Threat model entries, mitigation checklist, security review records, abuse-case fixtures, and incident escalation paths.
  - Validation: Security tests prove poisoned evidence cannot silently create final actions, fallback cannot weaken constraints, report redaction blocks risky summaries, and sensitive anti-fraud internals remain protected.

- **9.4 Harden incident and abuse response.**
  - Design: Add incident playbooks for wrong recommendations, privacy leak suspicion, fraud-model failure, corrupted evidence packages, route compromise, public report correction, steward override abuse, and downstream action rollback coordination.
  - Output: Incident playbook refs, escalation records, correction/retraction workflows, owner-service notification refs, and post-incident report refs.
  - Validation: Drills prove each incident path creates audit evidence, freezes risky recommendations, notifies owning services, and preserves correction/retraction/public-summary paths.

- **9.5 Harden reliability and scale behavior.**
  - Design: Add bounded concurrency, queue depth controls, retry/dead-letter behavior, report batching, replay pagination, backpressure, load-shedding, degraded-mode reads, and stale-policy/stale-route refresh behavior.
  - Output: Reliability config, worker limits, retry/dead-letter records, backpressure metrics, load test fixtures, and scale dashboards.
  - Validation: Load tests prove Central AI remains read-heavy under review spikes, does not drop audit evidence, preserves idempotency, and degrades by denying or queuing analysis rather than bypassing policy.

## Phase 10: Validation, Documentation Alignment, Queue State, And Handoff

### Work Items

- **10.1 Validate sub-build-plan structure.**
  - Design: Check title prefix, attached SDS link, 10 phase headings numbered 1 through 10, five work items per phase, Design/Output/Validation blocks, local links, final newline, and tab-free Markdown.
  - Output: Targeted validation script output and recorded evidence in `docs/build_plan/progress.md`.
  - Validation: Script passes for this file and the linked SDS/service/build-plan docs.

- **10.2 Validate documentation alignment.**
  - Design: Confirm SDS #60, the service catalog plan, master build plan, service catalog alignment crosswalk, Phase 12, Phase 13, and tech-stack decision all preserve the same first build phase, prerequisites, authority boundaries, and non-goals.
  - Output: Alignment review notes and updated backlinks/index rows.
  - Validation: Review confirms there is no master Phase 0 through Phase 13 reorder, no Phase 6/10/11/13 first-build drift, and no missing sub-build-plan link.

- **10.3 Validate stack and authority guardrails.**
  - Design: Scan the changed docs for conventional database, queue, object store, vault/KMS, Kubernetes-first, blockchain, NFT, pricing, revenue, customer-count, final enforcement, final grant, final fraud, final dispute, and direct accounting mutation drift.
  - Output: Guardrail scan evidence with only expected negative-control references and native Overrid service-name references.
  - Validation: Review confirms Central AI Service remains Rust-first, native-Overrid-boundary-based, evidence-bounded, recommend-only, privacy-scoped, and replayable.

- **10.4 Refresh Docdex, queue, and progress state.**
  - Design: Update the Codex55 queue state/progress for `060-build-plan`, append build-plan progress evidence, index changed docs, and search for the new plan/backlinks.
  - Output: `.codex55_sds_queue/state.json`, `.codex55_sds_queue/progress.md`, `docs/build_plan/progress.md`, Docdex index refresh, Docdex search results, and DAG/session evidence where available.
  - Validation: JSON validation passes, queue counts advance from SDS #60 to SDS #61, Docdex search returns the new #60 plan with SDS/service backlinks, and no unrelated queue tasks are changed.

- **10.5 Prepare implementation handoff.**
  - Design: Summarize the ordered implementation entry points, required prerequisites, first usable Phase 12 proof, Phase 13 hardening gates, test expectations, and owner-service authority boundaries for builders.
  - Output: Implementation handoff checklist for contracts, evidence intake, routing/provenance, analysis jobs, review handoffs, stewardship interface queues, reports/replay, governance hardening, and validation.
  - Validation: Handoff review confirms a builder can start with contracts and evidence intake, prove the fraud/evidence review queue first, defer grant/public-interest/native-surplus expansions until prerequisites are stable, and avoid direct mutation or speculative business assumptions.

## Alignment Review

- SDS #60 remains attached to master Phase 12 as the first build point through the Central AI Stewardship Interface fraud/evidence review queue.
- Phase 13 remains governance, compliance, security, reporting, retention, incident, threat-model, and scale hardening rather than the first delivery point.
- Phases 0, 1, 4, 5, 6, 8, 10, and 11 remain prerequisites; this plan does not reorder master Phase 0 through Phase 13.
- The SDS/service docs only need backlink and first-build-prerequisite wording corrections; the SDS content itself already preserves the correct evidence-bounded, recommend-only boundary.
- The master plan and service catalog alignment crosswalk need a new SDS #60 row and a Phase 12 crosswalk entry so Central AI Service is explicitly tied to the stewardship interface it backs.
