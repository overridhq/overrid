SDS #60

# Central AI Service SDS

## Purpose

Coordinate ecosystem-level AI stewardship for fraud detection, grant recommendations, public-interest investment, policy evidence review, and governance reporting.

Central AI Service is the evidence-bounded AI coordination layer for ecosystem stewardship. It can analyze fraud evidence, abuse patterns, public-interest pool outcomes, grant candidates, native-service surplus-routing proposals, policy evidence, and governance reports. It produces recommendations, risk assessments, intervention proposals, and report drafts. It does not silently mutate ledger history, seize funds, read private data without permission, bypass disputes, or become an unreviewable authority.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [central_ai_service.md](../../service_catalog/ai_rag_model_routing/central_ai_service.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md), [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md) |

## Service Family

- Family: AI, RAG, and model routing
- Owning layer: Evidence-bounded ecosystem stewardship, recommendations, and governance analysis
- Primary data scope: evidence packages, analysis jobs, fraud/abuse assessments, grant recommendations, public-interest allocation recommendations, intervention proposals, stewardship report refs, appeal/dispute refs, model/run provenance, and governance audit records
- First build phase from service plan: [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md) interface; stronger governance in [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md)

## Problem Statement

Overrid needs a central AI mechanism powerful enough to help govern a large distributed ecosystem: detect fraud, identify abuse, recommend public-interest investments, review policy evidence, and surface system health. At the same time, a central AI with opaque power would recreate the centralized control patterns Overrid is trying to replace.

Central AI Service must therefore operate through evidence, policy thresholds, privacy rules, appeal paths, and public reporting. It can recommend holds, throttles, challenge tasks, grants, public-interest allocations, or interventions, but final mutation must occur through the owning service with auditable reason codes and correction paths.

## Goals

- Intake evidence packages from Overwatch, Fraud Control, Overclaim, Overgrant, Seal Ledger, Public-Interest Pool Service, native apps, and governance services.
- Analyze fraud, abuse, collusion, grant eligibility, public-interest outcomes, policy risks, and system health.
- Recommend interventions with evidence refs, confidence, proportionality, and appeal paths.
- Recommend public-interest pool allocations and project donations without directly spending resources.
- Support native-service surplus-routing recommendations structurally without speculative financial assumptions.
- Produce governance and stewardship report refs with privacy-preserving summaries.
- Preserve model/run provenance, prompt/context refs, route decisions, and replay evidence.

## Non-Goals

- Do not directly mutate Seal Ledger, ORU balances, payouts, grants, provider eligibility, or dispute finality.
- Do not read private user, tenant, workspace, Docdex, or vault content without explicit authorization and context refs.
- Do not replace Overguard policy decisions, Overclaim disputes, Fraud Control cases, Overgrant authorization, or stewardship legal duties.
- Do not run an opaque black-box enforcement system with no evidence, reason codes, or appeal path.
- Do not optimize native services for addiction, private extraction, ad traps, or monopoly control.
- Do not add pricing, revenue forecasts, customer counts, blockchain mechanics, NFT mechanics, or per-transaction fee economics.

## Primary Actors And Clients

- Fraud Control Service submitting risk cases and evidence packages.
- Overwatch, Oververify, Challenge Task Service, and Overclaim submitting audit, verification, challenge, and dispute evidence.
- Overgrant, Public-Interest Pool Service, Purpose Tag Registry, and Seal Ledger submitting grant, pool, purpose, usage, and accounting refs.
- Stewardship Reporting Service, PIP Registry, Compliance Boundary Service, Incident Response Service, and Threat Modeling Tracker requesting governance analysis.
- Native apps and Central AI Stewardship Interface requesting bounded stewardship recommendations.
- AI Gateway Router and Encrypted Docdex RAG Adapter providing model routing and authorized context refs.
- Human stewards, appeal bodies, and operators reviewing recommendations.

## Dependencies

- [AI Gateway Router](ai_gateway_router.md) for model/resource routing and route-decision replay.
- [Encrypted Docdex RAG Adapter](encrypted_docdex_rag_adapter.md) for authorized context retrieval where permitted.
- [Overwatch](../control_plane/overwatch.md) for audit events, traces, evidence bundles, incidents, and report refs.
- [Fraud Control Service](../federation_public/fraud_control_service.md), [Overclaim](../trust_policy_verification/overclaim.md), [Challenge Task Service](../trust_policy_verification/challenge_task_service.md), and [Oververify](../trust_policy_verification/oververify.md) for fraud, disputes, challenges, and verification evidence.
- [Overgrant](../accounting/overgrant.md), [Public-Interest Pool Service](../federation_public/public_interest_pool_service.md), [Purpose Tag Registry](../federation_public/purpose_tag_registry.md), [Seal Ledger](../accounting/seal_ledger.md), and [ORU Account Service](../accounting/oru_account_service.md) for grant, purpose, accounting, and usage refs.
- [Stewardship Reporting Service](../governance_ops/stewardship_reporting_service.md), [Protocol Improvement Proposal Registry](../governance_ops/pip_registry.md), [Compliance Boundary Service](../governance_ops/compliance_boundary_service.md), and [Threat Modeling and Security Review Tracker](../governance_ops/threat_modeling_security_review_tracker.md) for governance boundaries.

## Owned Responsibilities

Central AI Service owns:

- Evidence package intake and validation for AI analysis.
- Analysis job records, model route refs, prompt/context refs, and run provenance.
- Fraud/abuse risk assessments derived from evidence packages.
- Grant, public-interest pool, and surplus-routing recommendation records.
- Intervention proposal records with target service, proportionality, reason codes, confidence, and appeal path.
- Governance report analysis refs and privacy-preserving summary refs.
- Recommendation review, acceptance, rejection, correction, expiration, and supersession records.
- Replay bundles for analysis jobs and recommendation decisions.

## Data Model

- `central_ai_evidence_package`: source service, evidence refs, data classes, privacy constraints, purpose tag refs, policy refs, retention class, allowed analysis scope, and redaction requirements.
- `analysis_job`: job type, requester refs, route decision refs, model/run provenance, context plan refs, prompt template refs, tool refs, state, and audit refs.
- `risk_assessment`: target refs, signal refs, severity, confidence, matched patterns, missing evidence, recommended next actions, and correction path.
- `recommendation_record`: recommendation type, target service, target refs, evidence package refs, reasoning summary, confidence, proportionality, policy threshold refs, expiry, and review state.
- `intervention_proposal`: proposed hold/throttle/challenge/grant/revocation/report action, owning service, required human/steward review, appeal path, and downstream command refs.
- `public_interest_recommendation`: pool refs, purpose tag refs, candidate project refs, outcome refs, usage refs, fairness facts, and stewardship review refs.
- `governance_report_ref`: report type, time window, included evidence refs, redaction profile, publication state, and audit refs.
- `model_run_provenance`: route decision id, model/resource refs, prompt template version, context refs, tool calls, result hash, usage refs, and replay constraints.

Recommendations are append-only. Corrections, retractions, accepted actions, and rejected actions create new records linked to the original recommendation.

## API Surface

- `POST /central-ai/evidence-packages`: registers an evidence package for bounded AI analysis.
- `POST /central-ai/analysis-jobs`: starts an analysis job with scope, privacy constraints, and requested output type.
- `GET /central-ai/analysis-jobs/{job_id}`: returns state, redacted summary, route refs, and evidence refs.
- `POST /central-ai/recommendations`: creates a recommendation from an analysis job or evidence package.
- `GET /central-ai/recommendations/{recommendation_id}`: returns recommendation, evidence refs, review state, and allowed downstream actions.
- `POST /central-ai/recommendations/{recommendation_id}/review`: records steward/operator accept, reject, request-info, retract, or correct decision.
- `POST /central-ai/intervention-proposals`: creates a proposal for the owning service to evaluate.
- `POST /central-ai/reports`: creates a governance/stewardship report draft or publication ref.
- `GET /central-ai/replay/{analysis_or_recommendation_id}`: reconstructs the evidence, route decision, prompt/context refs, and recommendation chain permitted by privacy policy.

Mutating APIs require service/steward identity, tenant/system scope, trace id, idempotency key, policy refs, evidence refs, route refs where applicable, and Overwatch audit refs. Stable errors include `evidence_scope_missing`, `private_context_not_authorized`, `analysis_scope_too_broad`, `route_unavailable`, `policy_threshold_missing`, `recommendation_expired`, `owning_service_required`, `human_review_required`, and `appeal_path_missing`.

## Event Surface

- `central_ai_service.evidence_package_registered`: evidence package accepted for bounded analysis.
- `central_ai_service.analysis_requested`: analysis job requested.
- `central_ai_service.analysis_completed`: analysis job completed with result refs.
- `central_ai_service.analysis_denied`: analysis denied by privacy, policy, or missing evidence.
- `central_ai_service.risk_assessment_created`: fraud/abuse/system risk assessment produced.
- `central_ai_service.recommendation_created`: recommendation recorded.
- `central_ai_service.intervention_proposed`: hold/throttle/challenge/grant/report proposal emitted to owning service.
- `central_ai_service.recommendation_reviewed`: steward/operator review recorded.
- `central_ai_service.recommendation_retracted`: recommendation retracted or corrected.
- `central_ai_service.report_ref_created`: governance/stewardship report ref created.

Events include evidence package refs, route refs, analysis refs, recommendation refs, policy refs, review refs, and redacted summaries. They must not include private evidence content or encrypted RAG inputs.

## Core Workflow

1. Owning service submits an evidence package with data-class, privacy, scope, and allowed analysis rules.
2. Central AI validates scope, evidence refs, policy refs, and required appeal/review boundaries.
3. AI Gateway Router selects an allowed model/resource route and context plan.
4. Central AI runs bounded analysis using authorized evidence, optional encrypted RAG context, and approved prompt/tool templates.
5. Analysis produces risk assessments, grant recommendations, public-interest recommendations, intervention proposals, or report drafts.
6. Recommendations are routed to the owning service or steward review body; Central AI does not directly mutate downstream state.
7. Owning service accepts, rejects, requests more information, or records correction/retraction.
8. Stewardship reports publish redacted summaries, aggregate metrics, and evidence refs according to governance rules.

## State Machine

Analysis job lifecycle:

1. `submitted`
2. `scope_checked`
3. `route_selected`
4. `running`
5. `completed`
6. `denied`
7. `failed`
8. `cancelled`

Recommendation lifecycle:

1. `draft`
2. `evidence_bound`
3. `review_pending`
4. `accepted_by_owner`
5. `rejected_by_owner`
6. `needs_more_evidence`
7. `expired`
8. `retracted`
9. `corrected`

Report lifecycle:

1. `draft`
2. `redaction_pending`
3. `review_pending`
4. `published`
5. `withheld`
6. `corrected`

## Policy And Security

- Require explicit evidence scope and privacy policy before analysis.
- Do not ingest private content, encrypted RAG context, workspace data, or vault data without authorized refs.
- Recommendations must cite evidence refs, policy thresholds, route refs, model provenance, and appeal/review paths.
- Interventions must be proportional and routed to the owning service for final mutation.
- Human or stewardship review is required for severe sanctions, grant allocation changes, public reports, compliance-sensitive cases, and any action policy marks as review-required.
- Preserve correction and retraction paths when evidence changes or recommendations are wrong.
- Keep anti-fraud internals, private evidence, and sensitive reports redacted from public summaries.

## Metering And Accounting

- Emit usage refs for analysis jobs, model routes, context retrieval, tool calls, report generation, and review work.
- Link usage to requesting service, evidence package, analysis job, model/resource route, and recommendation/report refs.
- Native-service surplus-routing recommendations reference structural surplus records and public-interest pool refs; Central AI does not compute speculative revenue forecasts.
- Overgrant, Seal Ledger, ORU Account Service, Overbill, and Public-Interest Pool Service remain the accounting and allocation authorities.
- Do not encode financial projections, hardcoded prices, payouts, or per-transaction costs.

## Observability And Operations

- Expose analysis backlog, denied analyses, route failures, evidence-scope failures, recommendation volume, accepted/rejected recommendations, retractions, corrections, appeal outcomes, and report publication state.
- Alert on private-context denial spikes, severe fraud recommendations, missing appeal paths, stale policy thresholds, and high model failure rates.
- Provide replay for analysis jobs and recommendation chains within privacy limits.
- Provide stewardship dashboards for public-interest recommendations, fraud trend summaries, intervention outcomes, and governance report status.
- Provide model/run provenance views for audit without exposing raw private prompts where policy forbids it.

## Failure Modes And Recovery

- Evidence package missing scope: deny analysis until owner supplies allowed use and data-class refs.
- Private context authorization missing: run without that context if allowed; otherwise deny.
- AI Gateway route unavailable: queue, retry, or fail analysis with route reason codes.
- Model output lacks evidence refs: mark result invalid and do not create recommendation.
- Recommendation targets wrong owning service: reject and require reissue with correct owner.
- Appeal path missing for intervention: block proposal until appeal path is supplied.
- Evidence later corrected: create corrected or retracted recommendation records and notify downstream services.
- Report redaction fails: keep report withheld until redaction review passes.

## Validation Plan

- Central AI recommendations cite evidence refs, policy thresholds, route refs, and model/run provenance.
- Private data is not ingested without explicit authorization and context refs.
- Severe interventions require review and appeal path before downstream action.
- Recommendations never mutate Seal Ledger, ORU balances, provider payouts, grant authorization, or dispute finality directly.
- Fraud and abuse assessments can be corrected or retracted when evidence changes.
- Report generation redacts private evidence and exposes only allowed summaries.
- Replay reconstructs analysis inputs, route decision, prompt/template refs, result refs, recommendation state, and review outcome.

## Build Breakdown

1. Define evidence package, analysis job, risk assessment, recommendation, intervention proposal, public-interest recommendation, report ref, and model-run provenance schemas.
2. Implement evidence intake with privacy scope and policy checks.
3. Integrate AI Gateway Router for model/resource selection and route replay.
4. Implement bounded analysis jobs for fraud evidence and grant/public-interest recommendation prototypes.
5. Implement recommendation review, correction, retraction, expiration, and owning-service handoff.
6. Integrate Overwatch, Fraud Control, Overclaim, Overgrant, Public-Interest Pool Service, Seal Ledger, and Stewardship Reporting.
7. Add report refs, redaction review, dashboards, replay, and Phase 13 governance hardening tests.

## Handoff And Downstream Use

Central AI Service hands evidence-bound recommendations, risk assessments, intervention proposals, public-interest recommendations, report refs, model/run provenance, usage refs, and replay bundles to Fraud Control Service, Overclaim, Overgrant, Public-Interest Pool Service, Purpose Tag Registry, Stewardship Reporting Service, Incident Response Service, Central AI Stewardship Interface, Overwatch, AI Gateway Router, SDK, CLI, and admin UI.

Downstream services must treat Central AI output as evidence-backed recommendations unless their own policy explicitly allows automated action for a bounded low-risk case.

## Open Design Questions

Resolved decisions:

- Automated Central AI output is limited to non-mutating, evidence-bounded support work: evidence package validation, missing-evidence prompts, low-severity triage summaries, duplicate or stale recommendation expiry, public-safe report draft sections, risk or grant recommendation drafts, and routing recommendations to the owning service. Steward or owner-service review is always required before severe sanctions, provider suspension, payout holds after finality, grant or pool allocation, native-service surplus routing, public report publication, compliance-sensitive action, broad eligibility changes, policy changes, or any action that touches private, regulated, secret-bearing, fraud-sensitive, or appealable evidence. A downstream service may automate only a bounded low-risk action when its own policy explicitly allows it and the Central AI recommendation is only supporting evidence.
- Provider suspension and payout-hold recommendations require a source-trusted evidence package with policy threshold refs, Overguard approval, severity and confidence bands, owning-service target, release conditions, expiry, and Overclaim or Provider Payout refs. The first automatic payout-hold recommendation threshold is high-confidence, pre-finality evidence such as an active payout or settlement dispute, fabricated evidence, impossible benchmark or capacity claims, high-confidence challenge failure, duplicate-execution mismatch tied to payable work, repeated no-shows or result inconsistency inside the payout period, sandbox escape, unauthorized egress, attempted secret or private-data access, payout-destination/compliance blocker, or a severe multi-source Fraud Control case. Central-AI-only, heuristic-only, single-source, broad-cluster, post-finality, compliance-sensitive, or suspension-level findings must become challenge/manual-review recommendations rather than direct suspension or hold recommendations.
- Public-interest allocation recommendations are public before action only when the candidate, purpose, and evidence are already public or stewardship-cleared and the recommendation would direct shared public-interest resources to a public project. The pre-action public view may show pool refs, enabled purpose tag versions, candidate/project class, public evidence refs, fairness and quota summary, confidence/proportionality band, review state, expiry, conflict-of-interest markers, and correction/objection path. Recommendations involving private grantees, regulated work, sensitive research, abuse/fraud facts, donor or participant privacy, unresolved eligibility disputes, or unreviewed allegations stay participant/steward-only until redacted Stewardship Reporting artifacts can publish an aggregate or corrected summary.
- Model/run provenance is exposed by audience class. Public views get route decision id or hash, model/resource class, prompt template version, context-plan hash, evidence package refs, data-class and redaction profile, output hash, reason-code summary, review state, and correction/retraction refs. Affected-party, steward, auditor, legal, or compliance views may see stronger refs when policy allows, but raw prompts, decrypted RAG content, private evidence payloads, vault secrets, payment details, and fraud heuristics stay in owning services or Overvault-compatible refs. Retention follows the AI Gateway/Overwatch classed pattern: caller-visible summaries for 30 days, private or regulated replay details for 7 days unless pinned by claim, incident, legal, compliance, or audit policy, redacted mismatch/rollout evidence for 90 days, and long-term archives reduced to hashes, ids, version refs, reason codes, usage refs, and Overwatch refs.
- The first Phase 12 Central AI interface is a fraud and evidence review queue inside the Central AI Stewardship Interface, backed by Fraud Control, Overclaim, Overwatch, Provider Payout, and AI Gateway refs. It should be read-heavy first: show evidence packages, recommendation drafts, missing-evidence prompts, model/run provenance refs, proposed hold/throttle/challenge actions, appeal/correction paths, and signed steward review actions routed to owning services. Grant recommendation and public-interest allocation review should follow once the proof-pool and Overgrant flows are stable; Stewardship Reporting publication and native-app surplus-routing review come later because they depend on redaction/report workflows and real native-service surplus records.
