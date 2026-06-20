# SUB BUILD PLAN #33 - Overguard

Attached SDS: [docs/sds/trust_policy_verification/overguard.md](../sds/trust_policy_verification/overguard.md)

## Purpose

This sub-build plan turns SDS #33 into an implementation sequence for Overguard. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Overguard is the deterministic policy decision engine for Overrid admission. It evaluates versioned input facts and signed policy bundles before work reaches queueing, scheduling, vault access, public-provider placement, or accounting reservation paths. It does not enqueue work, schedule work, execute work, store secrets, score trust, or mutate accounting state.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #33: Overguard](../sds/trust_policy_verification/overguard.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, API/event surfaces, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Overguard service plan](../service_catalog/trust_policy_verification/overguard.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared schemas, signed command envelopes, idempotency, trace ids, stable reason codes, deterministic fixtures, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overgate, Overtenant, Overregistry, Overkey, Overwatch, Overqueue, identity, tenant, credential, audit, and command primitives used by admission requests. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies workload manifests, queue refs, placement candidates, leases, runner refs, raw metering refs, and execution paths that must carry Overguard decision refs. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Controls Overguard's first build point for policy bundles, input facts, evaluation, reason codes, replayable decisions, dry-run behavior, trust refs, and deny-by-default admission. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies quota, grant, cost-class, budget, reservation-precheck, dispute, settlement, and accounting refs that Overguard cites without mutating balances or ledger state. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies system-service workload rules, trusted placement, failover, restore, and grid-resident hardening for Overguard as a protected backbone service. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies native Overbase persistence, Overvault secret/private-state refs, Overstore evidence artifacts, namespace route refs, and storage-backed replay expansion. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Controls public-provider admission limits, public low-sensitivity workload classes, sandbox constraints, anti-Sybil refs, fraud refs, and public-provider eligibility volatility. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Supplies SDK, CLI, admin UI, wallet/usage, native app, and central AI consumers of reason codes, explanations, dry-run output, and redacted policy decisions. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies stewardship, compliance, incident, threat-model, PIP, retention, audit-export, migration, and scale hardening for policy lifecycle and override controls. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #33 first build work aligned to master Phase 4, with Phase 5 accounting refs and later grid, storage, public-provider, native-app, and governance gates. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid storage/evidence/accounting boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, pricing, revenue, or customer-count drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 3, 4, 5, 7, 8, 11, 12, and 13 | Attach SDS #33, freeze Overguard authority, preserve Phase 4 as first build point, and record later accounting, grid, storage, public, native-app, and governance gates. |
| 2 | Master Phases 0, 1, and 4 | Build Rust contracts, JSON Schemas, policy bundles, bounded predicate rules, reason codes, and deterministic fixtures before policy side effects. |
| 3 | Master Phases 1, 3, and 4 | Implement admission input validation and core evaluation for workload, data, package, tenant, quota, egress, and core deny-by-default decisions. |
| 4 | Master Phases 3, 4, 8, 11, and 13 | Add secret prerequisites, provider eligibility, trust class, cache scope, region, compliance, abuse, and public-provider sensitivity gates. |
| 5 | Master Phases 4 and 5 | Add quota, grant, resource-card, budget, and reservation-precheck refs while preserving accounting mutation in ORU, Seal Ledger, Overbill, Overgrant, Overmark, and related owners. |
| 6 | Master Phases 1, 4, 7, 12, and 13 | Persist immutable decisions, publish Overwatch events, expose redacted reads and explanations, and publish reason codes to clients. |
| 7 | Master Phases 4 and 13 | Implement replay, policy bundle registration, staged rollout, canary comparison, emergency blocks, rollback, overrides, and governance controls. |
| 8 | Master Phases 3, 4, 5, 8, 11, and 12 | Wire Overguard decisions into Policy Dry-Run API, Overgate, Overqueue, Oversched, Overrun, Overvault, Overclaim, SDK, CLI, admin UI, and native clients. |
| 9 | Master Phases 7, 8, 11, 12, and 13 | Harden grid-resident operation, native persistence, public-provider volatility, native-app consumers, compliance, reporting, retention, and migration. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, phase boundaries, negative controls, queue state, and implementation handoff gates. |

## Tech Stack Guardrails

- Overguard core is a Rust service/module using shared contract types, Tokio for async evaluation and background workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Policy bundles, policy rules, reason codes, admission contexts, input fact bundles, decisions, matched rules, quota/budget prechecks, rollout records, override requests, replay bundles, events, API errors, fixtures, and reason-code catalogs use canonical JSON plus JSON Schema for docs-facing and test fixtures. Compact Protobuf contracts may be added only through the shared contract layer where needed.
- Policy evaluation uses a bounded Overguard-owned predicate expression format, not arbitrary scripts. Rules may use typed input selectors, boolean composition, equality, set membership, numeric thresholds, freshness windows, explicit effect precedence, stable reason-code refs, and signed evidence refs.
- Mutating policy APIs require signed actor or service envelopes, tenant scope, idempotency keys, trace ids, schema versions, policy refs, evidence refs, stable reason codes, and append-only Overwatch events.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for policy bundle commitments, replay bundles, canary comparison records, audit exports, evidence refs, and deterministic golden fixtures.
- Overguard consumes versioned facts from owning services and may later persist decisions through native Overbase and private refs through Overvault. It must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, or external workflow products the platform boundary.
- Overguard emits quota, budget, grant, reservation-precheck, usage, and decision refs. It does not bill, settle, reserve, hold, release, refund, correct, price, or mutate accounting state.
- Public-provider decisions stay gated to strict public low-sensitivity workload classes with no secrets, capped runtime, capped resource allocation, volatile eligibility TTLs, deny-by-default egress, and anti-abuse/fraud refs.
- Reason-code output must separate user-remediable explanations from operator-only details. Fraud heuristics, provider topology, other-tenant evidence, raw private data, secret risk detail, compliance/legal hold detail, incident internals, and override/revocation rationale remain operator-only behind stable wrapper codes.
- Planning and implementation must avoid blockchain, NFT, speculative token mechanics, pricing tables, revenue projections, customer-count assumptions, per-decision external payment calls, and hidden dependency reads.

## Phase 1: SDS Attachment, Overguard Authority, And Policy Gates

### Work Items

- **1.1 Attach the build plan to SDS #33.**
  - Design: Link this document from the numbered Overguard SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/trust_policy_verification/overguard.md`, `docs/service_catalog/trust_policy_verification/overguard.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #33 returns both the Overguard SDS and this sub-build plan.

- **1.2 Freeze Overguard as policy decision authority only.**
  - Design: Record that Overguard owns policy bundles, input fact schemas, deterministic evaluation, immutable decisions, matched rules, reason codes, rollout metadata, overrides, emergency blocks, and replay bundles.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms Overguard does not own queue state, scheduler placement, lease reservation, runner execution, secret mounting, trust scoring, balance mutation, billing, settlement, or payout state.

- **1.3 Preserve master Phase 4 as the first build point.**
  - Design: Keep first implementation in Phase 4 because Overguard requires Phase 0 contracts, Phase 1 identities and audit, and Phase 3 workload/manifests before it can safely gate admission.
  - Output: Phase-gate note that Phase 5 supplies accounting refs, Phase 7 grid-resident operation, Phase 8 native persistence/private refs, Phase 11 public-provider gates, Phase 12 client consumers, and Phase 13 governance hardening.
  - Validation: Review proves this plan does not move budget reservation, ledger mutation, secret access, public-provider eligibility expansion, native-app policy, or governance override authority into Phase 4 prematurely.

- **1.4 Carry forward resolved SDS decisions.**
  - Design: Preserve SDS #33 decisions for bounded policy predicates, classed decision TTLs, service-identity limits, policy canary redaction, and user-remediable versus operator-only reason-code classes.
  - Output: Resolved-decision checklist tied to SDS #33 open-question answers.
  - Validation: Review rejects arbitrary scripting, unbounded allow TTLs, automated allow-over-deny widening, canary raw private fact exposure, and opaque operator-only denials without user-safe wrapper codes.

- **1.5 Define fact-owner and dependency boundaries.**
  - Design: Create a dependency matrix for Overgate, Overtenant, Overregistry, Overpack, Package Validator, Oververify, Reputation and Anti-Sybil Service, Overvault, Overmesh, Overcache, Overgrant, Overmark, ORU Account Service, Overbill, Overwatch, Policy Dry-Run API, SDK, CLI, admin UI, Overqueue, Oversched, Overrun, Overclaim, native apps, and public-provider services.
  - Output: Boundary matrix listing consumed fact refs, emitted decision refs, final authority owner, freshness owner, redaction profile, replay evidence, TTL class, and later phase gate for each dependency.
  - Validation: Review confirms every handoff uses explicit APIs, versioned facts, signed refs, reason codes, policy versions, trace ids, and Overwatch evidence rather than privileged direct state reads.

## Phase 2: Rust Contracts, Policy Schemas, And Deterministic Fixtures

### Work Items

- **2.1 Create the Overguard Rust contract module.**
  - Design: Add contract types for policy bundles, rules, reason codes, admission contexts, input fact bundles, decisions, matched rules, quota/budget prechecks, override requests, rollout records, replay bundles, state enums, API errors, and events.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, stable reason-code catalog, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from queue, scheduler, runner, vault, trust-score, and accounting internals.

- **2.2 Define canonical JSON Schemas and examples.**
  - Design: Add versioned schemas for policy bundle registration, rule definitions, predicate expressions, reason codes, admission evaluation, batch evaluation, decision read, explanation, replay, rollout, canary comparison, emergency block, and override requests.
  - Output: Schema files, valid examples, invalid examples, schema index, generated validators, and compatibility notes.
  - Validation: Schema tests reject missing tenant id, actor or service account, workload id, manifest id/version, trace id, idempotency key, input fact refs, policy target, stable reason codes, schema version, and evidence refs where required.

- **2.3 Implement the bounded predicate rule model.**
  - Design: Model policy rules using typed input selectors, boolean composition, equality, set membership, numeric thresholds, freshness windows, effect precedence, and reason-code refs while disallowing arbitrary scripts or host callbacks.
  - Output: Predicate AST, static validator, deterministic evaluator interface, canonical serialization, and fixture-compatible hash behavior.
  - Validation: Tests prove rules cannot read hidden dependency state, call host functions, access wall-clock time outside input facts, depend on nondeterministic ordering, or widen effects without explicit precedence.

- **2.4 Define reason-code and remediation contracts.**
  - Design: Model user-remediable categories, operator-only categories, wrapper codes, redaction classes, deprecation state, remediation hints, severity, and publication metadata for SDK, CLI, admin UI, native apps, and central AI views.
  - Output: Reason-code registry, compatibility rules, docs-facing catalog, generated client fixture, and migration notes for renamed wording.
  - Validation: Tests prove code identifiers remain stable when messages change and operator-only categories still expose trace id, wrapper code, appeal/support path where allowed, and redacted evidence refs.

- **2.5 Create deterministic policy fixture scenarios.**
  - Design: Build fixtures for private tenant allow, missing manifest deny, data-class deny, egress deny, stale fact block, secret prerequisite deny, provider eligibility block, quota/budget insufficiency, public low-sensitivity allow, public-provider secret denial, emergency block, override review, replay, and canary diff.
  - Output: Fixture directory, expected API responses, Overwatch events, decision hashes, replay bundles, redacted explanations, and invalid examples.
  - Validation: Fixture tests produce stable decisions, reason codes, matched rules, replay hashes, canary diffs, and denial states across repeated runs.

## Phase 3: Admission APIs, Input Facts, And Core Evaluation

### Work Items

- **3.1 Implement evaluate and batch-evaluate APIs.**
  - Design: Support signed `POST /policy/admission/evaluate` and bounded `POST /policy/admission/batch-evaluate` for authenticated command and workload submissions before queueing or target-service mutation.
  - Output: Evaluation handlers, batch handler, request validators, idempotent responses, decision refs, and `overguard.admission_evaluated` events.
  - Validation: API tests reject unauthenticated actors, missing tenant context, missing manifest refs, unsupported command type, conflicting idempotency keys, unbounded batches, stale policy targets, and malformed fact bundles.

- **3.2 Validate input fact bundles from owner services.**
  - Design: Validate actor, tenant, manifest, workload class, data class, secret refs, egress, package provenance, provider class, quota, grant, budget, trust, cache scope, region, and evidence refs without direct storage reads.
  - Output: Fact bundle validator, owner-service ref registry, freshness checker, signed snapshot verifier, missing-fact reason codes, and dependency error mapping.
  - Validation: Tests prove missing or stale critical facts deny or block by default and live dependency state cannot silently alter a stored decision after evaluation.

- **3.3 Implement core admission rules.**
  - Design: Evaluate workload class, data sensitivity, tenant quota, package trust, manifest validation, sandbox profile, egress class, and baseline provider class before work reaches Overqueue or target services.
  - Output: Core rule bundle, matched-rule records, allow/deny/block/review effects, user-safe remediation hints, and operator detail refs.
  - Validation: Policy tests prove invalid packages, undeclared data classes, denied egress, wrong tenant context, insufficient quota, and unsupported sandbox profiles are rejected before execution.

- **3.4 Implement decision state transitions.**
  - Design: Model received, facts_validated, evaluating, allowed, denied, blocked, review_required, overridden, and expired decision states with immutable records and replacement links.
  - Output: State transition table, legal transition rules, terminal-state behavior, replacement-decision refs, and event emission rules.
  - Validation: State tests reject mutable decision updates, allow-after-expiry, override without linked replacement, and queue/scheduler consumption of blocked or review-required decisions.

- **3.5 Add admission TTL and re-evaluation controls.**
  - Design: Apply classed decision TTLs from SDS #33 and require re-evaluation for retries, reschedules, secret mounts, provider-class changes, queue reactivation, package changes, trust/quota/budget changes, and policy rollback or emergency blocks.
  - Output: TTL evaluator, expiry reason codes, re-evaluation requirements, lease-validity note, and dry-run TTL semantics.
  - Validation: Tests prove regulated/secret-bearing, system-service, public-low-sensitivity, trusted-federation, private-tenant, and research/public-interest TTLs match SDS #33 and expire at the earliest valid boundary.

## Phase 4: Secret, Provider, Cache, Region, And Abuse Policy Checks

### Work Items

- **4.1 Implement secret and private-data prerequisites.**
  - Design: Evaluate secret ref class, Overvault prerequisites, mount-lease constraints, regulated/secret-bearing workload class, and redaction class without reading or storing raw secrets.
  - Output: Secret prerequisite rules, secret-bearing denial codes, Overvault access prerequisite refs, blocked-access explanations, and replay-safe fact refs.
  - Validation: Tests prove secret-bearing workloads require Overvault prerequisites, are ineligible for public-provider placement, and never expose raw secret values or private endpoint details in user-facing explanations.

- **4.2 Implement provider eligibility and trust-class checks.**
  - Design: Consume Oververify, Reputation and Anti-Sybil Service, public-provider onboarding, challenge, provider, node, capability, and dispute refs to decide allowed provider classes and required trust class.
  - Output: Provider eligibility rules, trust-class matrix, challenge-required outcomes, stale trust blocking, and public-provider volatility TTLs.
  - Validation: Tests prove unknown or suspicious nodes cannot receive private, regulated, secret-bearing, or system-service workloads and public-provider eligibility changes force re-evaluation.

- **4.3 Implement public low-sensitivity gates.**
  - Design: Limit public-provider decisions to strict public low-sensitivity classes with no secrets, capped runtime, capped resource allocation, deny-by-default egress, and anti-abuse refs.
  - Output: Public workload class rules, sandbox profile refs, cap fields, deny egress defaults, and reason-coded public-provider explanations.
  - Validation: Phase 11 tests prove public nodes can only receive approved low-sensitivity workloads and cannot infer private tenant, provider, fraud, or challenge internals.

- **4.4 Implement cache, egress, region, and compliance checks.**
  - Design: Evaluate cache scope, Overmesh route class, egress requirement, jurisdiction/region refs, compliance boundary refs, locality, and data movement constraints from signed input facts.
  - Output: Cache scope rules, egress rules, region/jurisdiction matrix, compliance reason codes, and Overmesh/Overcache handoff refs.
  - Validation: Tests prove private-tenant cache cannot leak to public scopes, denied egress fails closed, region mismatch is user-remediable when selectable, and compliance/legal hold details remain operator-only.

- **4.5 Implement abuse and conflicting-fact handling.**
  - Design: Evaluate abuse markers, duplicate submissions, conflict facts, fraud-sensitive markers, challenge-required outcomes, and stricter-rule precedence without hiding severe safety or compliance paths.
  - Output: Abuse rules, conflict resolution policy, review-required outcomes, user-safe wrapper codes, and Overwatch evidence refs.
  - Validation: Tests prove conflicting facts use stricter rules or review, abuse throttling does not expose heuristics, and serious secret, compliance, payout, account, or public-provider safety issues retain an authorized review path.

## Phase 5: Quota, Budget, Grant, And Accounting Precheck Refs

### Work Items

- **5.1 Implement quota and budget precheck records.**
  - Design: Model tenant quota state, resource dimensions, grant/budget refs, reservation-required flags, insufficient-resource reasons, and downstream precheck refs without reserving resources.
  - Output: `quota_budget_precheck` contract, validation rules, matched-rule refs, and budget-related reason codes.
  - Validation: Tests prove quota and budget failures deny or block admission without mutating ORU balances, Seal Ledger entries, Overbill records, or Overlease reservations.

- **5.2 Integrate Overgrant, Overmark, ORU, and Overbill refs.**
  - Design: Consume grant, cost-class, resource-card, budget, reserve, sponsorship, invoice, receipt, and precheck refs from owning services as signed input facts.
  - Output: Accounting fact adapter contracts, freshness rules, owner-service error mapping, and redacted budget explanations.
  - Validation: Tests prove missing budget prerequisites are user-remediable where safe, while settlement, payout, fraud, or operator-only accounting internals remain redacted.

- **5.3 Preserve accounting mutation boundaries.**
  - Design: Record that Overguard may require a downstream reservation or denial but never creates holds, releases, refunds, corrections, payouts, invoices, receipts, or ledger entries.
  - Output: No-mutation guardrail tests, service boundary checklist, and static review criteria.
  - Validation: Architecture tests and scans prove Overguard code has no direct balance, payout, billing, ledger, pricing, or external payment mutation path.

- **5.4 Coordinate with Overlease and execution admission.**
  - Design: Return required reservation, lease, or re-evaluation conditions that Overlease, Oversched, Overqueue, and Overrun must satisfy before execution continues.
  - Output: Decision condition contract, allowed provider class, resource cap refs, reservation-required flags, and lease validity notes.
  - Validation: Integration tests prove Overqueue, Oversched, Overlease, and Overrun reject work without a valid Overguard decision ref and re-evaluate when admission conditions change.

- **5.5 Emit policy usage facts for metering.**
  - Design: Emit evaluation count, matched-rule count, deny/block count, replay count, policy comparison count, and override request count as usage facts for Overmeter without external payment friction.
  - Output: Policy usage event schema, Overmeter handoff refs, Overwatch audit refs, and aggregation hints.
  - Validation: Tests prove usage facts are append-only, tenant-scoped, traceable, and do not include raw private payloads, secrets, fraud heuristics, pricing tables, or per-decision external payment calls.

## Phase 6: Immutable Decisions, Explanations, And Overwatch Events

### Work Items

- **6.1 Persist immutable policy decisions.**
  - Design: Store decision id, admission context ref, bundle version, state, matched rules, reason codes, evidence refs, required trust class, allowed provider classes, sandbox profile refs, secret prerequisites, egress decision, TTL, and replay bundle ref.
  - Output: Decision repository, append-only record model, replacement-link rules, lookup indexes, and migration hooks for native persistence later.
  - Validation: Tests prove decisions are immutable, replacement decisions link to originals, reads use authorized projections, and replay uses stored facts and policy versions.

- **6.2 Implement decision read and explain APIs.**
  - Design: Support `GET /policy/decisions/{decision_id}` and `GET /policy/decisions/{decision_id}/explain` with user, developer, operator, steward, auditor, provider, native-app, and central AI projections.
  - Output: Read handler, explain handler, redaction profiles, remediation hints, operator detail refs, and audit events.
  - Validation: Authorization tests prove user-safe views disclose stable reason codes and remediation while hiding fraud internals, provider topology, other-tenant evidence, raw secrets, private data risk detail, compliance/legal hold detail, and override rationale.

- **6.3 Publish Overwatch policy events.**
  - Design: Emit bundle registration, rollout change, admission evaluated, allowed, denied, blocked, override requested, override applied, replay completed, and emergency block events.
  - Output: Event schemas, append-only Overwatch client, trace propagation, evidence refs, and failure/retry behavior.
  - Validation: Event tests prove each decision path emits the required event family with decision id, bundle version, request refs, reason codes, evidence refs, and trace id without raw private payloads or secrets.

- **6.4 Publish reason codes to clients.**
  - Design: Expose `GET /policy/reason-codes` and generated catalogs for SDK, CLI, admin UI, native apps, Policy Dry-Run API, and central AI review with compatibility metadata.
  - Output: Reason-code listing API, generated fixture, compatibility report, docs snippets, and client binding test cases.
  - Validation: Client tests prove code identifiers remain stable, wording changes do not break clients, and TypeScript is limited to generated bindings and UI/native-app surfaces.

- **6.5 Add metrics, dashboards, and alerts.**
  - Design: Track decision volume, allow/deny/block/review rates, top reason codes, stale facts, TTL expiry, replay mismatch, policy rollout, emergency block, override, and unexpected public-provider allowance signals.
  - Output: Metrics contract, dashboard queries, alert rules, readiness/health checks, and operator runbook notes.
  - Validation: Observability tests prove dashboards derive from Overwatch/decision refs and broad views do not expose raw private payloads, secrets, exact fraud heuristics, or provider-private topology.

## Phase 7: Replay, Rollout, Canaries, Emergency Blocks, And Overrides

### Work Items

- **7.1 Implement decision replay.**
  - Design: Support `POST /policy/decisions/{decision_id}/replay` using stored policy bundle, input fact bundle, evaluator version, matched rules, and decision result rather than current live facts.
  - Output: Replay endpoint, replay bundle contract, evaluator-version refs, replay-completed events, and mismatch records.
  - Validation: Replay tests prove stored facts and policy versions reproduce decisions and mismatch marks a policy integrity incident with evidence refs.

- **7.2 Implement policy bundle registration and activation.**
  - Design: Support signed bundle registration, semantic versioning, compatibility date, activation window, owner service, signature, emergency block refs, and lifecycle states from draft through revoked.
  - Output: `POST /policy/bundles`, bundle repository, signature verifier, compatibility checks, and bundle lifecycle events.
  - Validation: Tests reject unsigned bundles, parse failures, invalid predicate expressions, incompatible reason-code refs, non-canonical serialization, and activation without required compatibility fixtures.

- **7.3 Implement staged rollout and canary comparison.**
  - Design: Support staged, canary, active, paused, retired, and revoked states with old/new decision comparison against stored input fact refs and redacted summaries.
  - Output: `POST /policy/bundles/{bundle_id}/rollout`, canary comparison records, aggregate impact counts, redacted diff views, and rollback refs.
  - Validation: Canary tests prove non-operator callers see only safe outcomes while operator and stewardship views dereference detailed diffs through Overwatch redaction profiles, access-decision refs, and audit exports.

- **7.4 Implement emergency blocks and rollback.**
  - Design: Allow narrowly scoped emergency blocks, stale-decision invalidation, rollback to previous active policy bundle, pause, retire, and revoked states with audit evidence and expiry where applicable.
  - Output: Emergency block rules, rollback command path, invalidation events, operator alerts, and replay-safe block refs.
  - Validation: Tests prove emergency blocks fail closed, are auditable, narrowly scoped, reversible only by authorized policy, and invalidate affected allow decisions without deleting history.

- **7.5 Implement override workflows.**
  - Design: Support override requests with source decision, requester, requested effect, justification, evidence refs, expiry, approver refs, and replacement-decision version while preserving human stewardship requirements for widening access.
  - Output: `POST /policy/overrides`, override request records, approver rules, signed replacement decision refs, and override events.
  - Validation: Tests prove automated service identity cannot widen access, extend TTLs beyond class maximums, waive secret/regulated/public-provider prerequisites, release emergency blocks, or affect settlement/payout/finality without required human or stricter multi-approver policy.

## Phase 8: Cross-Service Handoffs And Client Integration

### Work Items

- **8.1 Wire Policy Dry-Run API.**
  - Design: Route dry-run requests through the same evaluator while marking dry-run decisions side-effect-free, non-capability-token by default, and same-or-shorter TTL than real admission.
  - Output: Dry-run evaluator adapter, comparison output, safe remediation hints, cost/precheck refs, and redacted explanation profiles.
  - Validation: Tests prove dry-run never queues work, reserves resources, grants secret access, changes accounting state, or creates capability tokens unless policy explicitly accepts a still-valid comparison.

- **8.2 Gate Overgate, Overqueue, Oversched, and Overrun.**
  - Design: Require valid Overguard decision refs before queueing, placement, lease-bound execution, retry, reschedule, secret mount, public-provider placement, or queue reactivation.
  - Output: Admission decision client, decision-ref validator, downstream condition checker, re-evaluation triggers, and fail-closed integration tests.
  - Validation: Integration tests prove invalid packages, denied egress, wrong tenant context, insufficient trust, expired decisions, and changed provider class are rejected before execution.

- **8.3 Integrate Overvault and Overclaim consumers.**
  - Design: Hand secret access prerequisites to Overvault and policy decision/replay refs to Overclaim for disputes, holds, appeals, and policy-integrity review without raw secret exposure.
  - Output: Overvault prerequisite contract, Overclaim decision-ref contract, redacted evidence refs, and replay bundle links.
  - Validation: Tests prove Overvault remains the secret access owner and Overclaim can replay or cite decisions without exposing raw private payloads, raw secrets, or owner-service internals.

- **8.4 Integrate verification, reputation, fraud, and challenge signals.**
  - Design: Consume Oververify, Reputation and Anti-Sybil Service, Fraud Control Service, Challenge Task Service, public-provider onboarding, provider eligibility, dispute, and challenge refs as versioned input facts.
  - Output: Trust fact adapters, freshness checks, public-provider risk refs, challenge-required outcomes, and operator-only fraud reason codes.
  - Validation: Tests prove Overguard consumes trust facts but does not create trust scores, assign challenges, decide payout holds, or expose fraud heuristics to user-facing output.

- **8.5 Integrate SDK, CLI, admin UI, and native clients.**
  - Design: Expose decision evaluate, dry-run, read, explain, reason-code listing, replay status, rollout views, and override request flows through generated SDKs, Rust CLI, admin/developer UI, wallet/usage surfaces, native apps, and central AI stewardship views.
  - Output: Client contract map, generated bindings, CLI command examples, UI projection notes, native-app projection notes, and central AI redaction rules.
  - Validation: Client tests prove Rust remains the core runtime, TypeScript stays limited to generated bindings and UI/native-app surfaces, and clients cannot bypass Overgate/Overguard authorization or redaction.

## Phase 9: Grid, Storage, Public-Provider, Native-App, And Governance Hardening

### Work Items

- **9.1 Add Overbase-backed decision persistence when Phase 8 exists.**
  - Design: Move from Phase 4 local policy records to Overbase collections/indexes for policy bundles, decisions, matched rules, reason codes, rollout records, overrides, replay bundles, canary comparisons, and audit projections.
  - Output: Overbase schema, collection/index definitions, migration path, consistency policy, backup/restore metadata, and query projections.
  - Validation: Persistence tests prove decision reads, replay queries, rollout state, reason-code publication, and override records survive restart and migration without record rewriting.

- **9.2 Harden Overguard as a grid-resident system service.**
  - Design: Apply Phase 7 system-service workload class, trusted placement, replicated state, failover, restore drills, maintenance mode, rolling update, rollback, and break-glass controls to Overguard.
  - Output: System-service manifest, placement policy, health/readiness contract, failover runbook, restore drill, and rollback checklist.
  - Validation: Grid tests prove Overguard remains available under controlled node failure and never serves stale or conflicting active policy bundles without explicit blocked/review-required outcomes.

- **9.3 Add Phase 11 public-provider hardening.**
  - Design: Expand public low-sensitivity policy rules for public-provider onboarding, sandbox profile, fraud controls, reputation refs, duplicate execution, payout-protection refs, egress caps, runtime caps, and appeal/review paths.
  - Output: Public-provider policy bundle, stricter TTLs, sandbox refs, fraud/reputation fact adapters, and public-safe explanation variants.
  - Validation: Public-provider tests prove unknown, suspicious, or low-trust nodes cannot receive private, regulated, secret-bearing, or system-service workloads and public reasons do not reveal fraud internals.

- **9.4 Add Phase 12 native-app and central AI consumers.**
  - Design: Support wallet/usage, search, directory, messaging, social, maps, workspace, personal AI, admin UI, and central AI views of policy decisions only through role-aware projections and reason-code catalogs.
  - Output: Native-app projection map, central AI redaction rules, app-service reason-code categories, and user-safe remediation flows.
  - Validation: Native-app tests prove user-facing views remain useful without exposing other users, provider-private data, secret-bearing refs, exact fraud heuristics, compliance/legal internals, or unrelated tenant facts.

- **9.5 Add Phase 13 governance, compliance, and migration controls.**
  - Design: Add stewardship review, compliance boundary checks, threat-model evidence, PIP-governed policy changes, incident handoffs, migration tooling, retention policies, and audit exports for policy lifecycle.
  - Output: Governance workflow refs, compliance export schemas, threat-model checklist, migration plan, retention rules, and public-safe aggregate reports.
  - Validation: Governance tests prove policy changes, emergency blocks, overrides, replay mismatches, and operator-only denials have signer, policy version, reason codes, evidence refs, retention class, and audit-export paths.

## Phase 10: Validation, Documentation Alignment, And Build Handoff

### Work Items

- **10.1 Run contract and API validation.**
  - Design: Validate evaluate, batch-evaluate, decision read, explain, replay, bundle registration, rollout, reason-code listing, override, dry-run adapter, and downstream decision-ref APIs against schemas.
  - Output: Contract tests, API tests, invalid-fixture tests, compatibility reports, generated binding checks, and fixture snapshots.
  - Validation: Tests pass for required fields, signed envelopes, idempotency, trace ids, tenant scope, policy refs, input fact refs, evidence refs, stable reason codes, TTLs, redaction profiles, and schema-version compatibility.

- **10.2 Run policy, replay, rollout, and canary validation.**
  - Design: Exercise workload class, data sensitivity, tenant quota, package trust, egress, secret access, provider eligibility, cache scope, region, abuse, budget precheck, replay, rollout, canary, emergency block, and override scenarios.
  - Output: Policy fixture suite, golden replay suite, rollout suite, canary comparison suite, emergency block suite, and override suite.
  - Validation: Tests prove decisions are deterministic, replayable, deny-by-default for missing/stale critical facts, redaction-safe, and fail closed under parse, signature, dependency, timeout, conflict, and bad-rollout failures.

- **10.3 Run security, privacy, and boundary negative controls.**
  - Design: Prove unauthorized actors cannot view decision internals, raw secrets cannot enter Overguard storage, public-provider details stay redacted, accounting mutation is absent, and hidden dependency reads cannot influence stored decisions.
  - Output: Security test suite, redaction test suite, no-secret-storage scan, no-accounting-mutation scan, hidden-read scan, and operator-only reason-code scan.
  - Validation: Negative tests fail closed and explanations remain useful without exposing private provider data, fraud heuristics, secrets, tenant-private facts, compliance/legal detail, or override/revocation rationale.

- **10.4 Validate documentation and phase alignment.**
  - Design: Check this plan against SDS #33, the Overguard service catalog entry, master Phase 0 through Phase 13 order, service_catalog_alignment, Phase 4, Phase 5, Phase 7, Phase 8, Phase 11, Phase 12, Phase 13, and `docs/overrid_tech_stack_choice.md`.
  - Output: Link-check results, phase-table verification, work-item count verification, stale-note scan, stack-guardrail scan, Docdex search evidence, and queue/progress updates.
  - Validation: Validation proves the plan has 10 phases numbered 1 through 10, five work items per phase, Design/Output/Validation fields, no external product-boundary drift, and no required master phase reordering.

- **10.5 Hand off implementation gates.**
  - Design: Convert this documentation into build-entry criteria for contracts, policy schemas, fixtures, evaluator core, sensitive/provider checks, accounting refs, immutable decisions, replay/rollout/override, service handoffs, grid/storage/public/native/governance hardening, and validation.
  - Output: Implementation gate checklist, dependency readiness checklist, risk register, first coding-task candidate list, and owner-service handoff checklist.
  - Validation: Handoff review confirms Phase 4 work can start after Phase 0 through Phase 3 prerequisites exist, Phase 5 accounting actions remain owner-service effects, and Phase 7/8/11/12/13 expansions remain gated until their owning rails are ready.
