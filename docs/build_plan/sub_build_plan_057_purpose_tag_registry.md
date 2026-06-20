# SUB BUILD PLAN #57 - Purpose Tag Registry

Attached SDS: [docs/sds/federation_public/purpose_tag_registry.md](../sds/federation_public/purpose_tag_registry.md)

## Purpose

This sub-build plan turns SDS #57 into an implementation sequence for Purpose Tag Registry. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Purpose Tag Registry is the Phase 10 stewarded purpose taxonomy and evidence-requirement authority for public-interest capacity. It owns purpose tag definitions, immutable tag versions, eligibility criteria, evidence requirement bundles, steward assignments, review proposals, activation/deprecation/supersession records, purpose-claim validation refs, policy export refs, redacted public documentation, and replay bundles. It does not authorize grants, allocate resources, decide project merit, set central AI priorities, adjudicate fraud or disputes, store raw sensitive evidence, schedule work, execute work, mutate accounting state, or turn purpose tags into speculative rights.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #57: Purpose Tag Registry](../sds/federation_public/purpose_tag_registry.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Purpose Tag Registry plan](../service_catalog/federation_public/purpose_tag_registry.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, signed envelopes, trace ids, idempotency, stable errors, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant scopes, Overkey signing refs, Overgate ingress, Overregistry service facts, Overwatch audit refs, and Overqueue command/event primitives. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard decisions, Policy Dry-Run previews, Workload Classifier facts, Overclaim correction paths, and policy input semantics for purpose-claim validation. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies Overgrant grant-authorization owner boundaries, Overmeter usage refs, ORU account projections, Seal Ledger refs, Overbill context, and Provider Payout boundaries without registry mutation. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overstore and Overvault refs for evidence storage, protected evidence, hash refs, retention classes, and redacted public summaries without raw-evidence ownership. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Controls the first build point: stewarded purpose tags, evidence-backed eligibility, policy exports, cross-tenant grant prerequisites, and public-interest pool proof. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies later public-provider hardening, public sandbox constraints, fraud controls, challenge refs, payout holds, and low-sensitivity public supply restrictions. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies PIP governance, compliance boundaries, stewardship reports, redaction review, incident response, threat review, audit export, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #57 first build work aligned to master Phase 10, with earlier phases as prerequisites, Phase 11 as public-provider hardening, and Phase 13 as governance hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, speculative-market, pricing, revenue, customer-count, or external-provider-as-core-boundary drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 8, 10, 11, and 13 | Attach SDS #57, preserve Phase 10 as first build, freeze authority boundaries, and identify owner-service gates. |
| 2 | Master Phases 0, 1, 4, 8, and 10 | Define Rust contracts, canonical schemas, lifecycle states, reason codes, signed refs, redaction profiles, and deterministic fixtures. |
| 3 | Master Phases 1 and 10 | Implement draft tag/version CRUD, canonical read/list APIs, publication refs, and immutable version snapshots. |
| 4 | Master Phases 4, 8, and 10 | Implement eligibility criteria and evidence requirement bundles with protected evidence refs and redacted public summaries. |
| 5 | Master Phases 1, 4, 10, and 13 | Implement steward assignment, review proposals, activation, deprecation, supersession, conflict checks, and governance escalation. |
| 6 | Master Phases 4, 5, 8, and 10 | Implement workload purpose-claim validation with evidence refs, stable reason codes, replay facts, and no grant allocation authority. |
| 7 | Master Phases 4, 5, 10, and 11 | Implement policy exports and integrations for Overguard, Policy Dry-Run, Overgrant, Public-Interest Pool Service, and later public-provider hardening. |
| 8 | Master Phases 8, 10, 12, and 13 | Implement replay, redacted public documentation, reporting hooks, retention refs, and native/client read surfaces. |
| 9 | Master Phases 10, 11, and 13 | Prove the first Phase 10 tag set and define later public-provider, compliance, medical/regulatory, public-infrastructure, and governance gates. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, authority boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Purpose Tag Registry core is a Rust service/module using shared contract crates, Tokio for bounded review/export/report workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Purpose tags, tag versions, eligibility criteria bundles, evidence requirement bundles, steward assignments, tag change proposals, purpose-claim validations, publication refs, policy exports, events, fixtures, replay bundles, redaction profiles, and audit exports use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be generated later only through the shared contract layer.
- Mutating endpoints require signed steward, service, or operator envelopes, tenant/stewardship scope, trace id, idempotency key, policy refs, evidence refs, governance refs, schema version, stable reason codes, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for tag snapshots, criteria bundles, evidence requirement bundles, proposal diffs, validation fact bundles, policy exports, public summaries, replay bundles, audit exports, and deterministic fixtures.
- Purpose Tag Registry may point to Overregistry, Overgrant, Public-Interest Pool Service, Overguard, Policy Dry-Run API, Overwatch, Overstore, Overvault, Central AI Service, Stewardship Reporting Service, PIP Registry, Fraud Control Service, Overclaim, SDK, CLI, admin UI, and native reporting surfaces, but it must not become the owner of those services' truth.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, speculative assets, pricing, revenue projections, customer-count assumptions, raw sensitive evidence storage, grant authorization, resource allocation, merit scoring, dispute adjudication, fraud adjudication, scheduling, workload execution, accounting mutation, or external-provider products the platform boundary.

## Phase 1: SDS Attachment, Phase 10 Scope, And Authority Boundaries

### Work Items

- **1.1 Attach the build plan to SDS #57.**
  - Design: Link this document from the Purpose Tag Registry SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/federation_public/purpose_tag_registry.md`, `docs/service_catalog/federation_public/purpose_tag_registry.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #57 returns both the Purpose Tag Registry SDS and this sub-build plan.

- **1.2 Preserve master Phase 10 as the first build point.**
  - Design: Keep first implementation in Phase 10 because stewarded purpose tags are required before accountable public-interest pools and cross-tenant grant eligibility can rely on stable purpose vocabulary.
  - Output: Phase-gate note that earlier phases are prerequisites, Phase 10 builds the registry core, Phase 11 adds public-provider hardening, and Phase 13 hardens governance/compliance.
  - Validation: Review proves the plan does not move purpose-tag authority into Phases 0 through 9, does not delay the core registry to Phase 11, and does not reorder the master Phase 0 through Phase 13 sequence.

- **1.3 Freeze registry ownership boundaries.**
  - Design: Record that the registry owns tag definitions, tag versions, criteria bundles, evidence requirement bundles, steward assignments, review proposals, activation/deprecation/supersession records, validation refs, policy export refs, public docs, and replay bundles.
  - Output: Ownership checklist for architecture and implementation reviews.
  - Validation: Review confirms the registry does not authorize grants, allocate resources, maintain balances, append ledger entries, schedule workloads, execute workloads, decide project merit, decide central AI priority, adjudicate disputes, adjudicate fraud, or store raw sensitive evidence.

- **1.4 Carry forward resolved SDS #57 decisions.**
  - Design: Preserve the initial six stewarded tag records, first allocation limit to active `science`, `education`, and `opensource`, inactive/review-only handling for `medical`, `climate`, and `public_infrastructure`, redacted evidence classes, strict governance for semantic widening, and indefinite redacted historical visibility.
  - Output: Resolved-decision checklist tied to proof tags, inactive tags, public versus steward-only evidence, medical/sensitive research handling, tag split/deprecation governance, and historical archive retention.
  - Validation: Review rejects informal purpose labels, immediate medical/climate/public-infrastructure allocation, raw public evidence payloads, silent tag widening, central-AI-only activation, and deletion of cited historical tag versions.

- **1.5 Define upstream and downstream dependency boundaries.**
  - Design: Create a dependency matrix for Overregistry, Overgrant, Public-Interest Pool Service, Overguard, Policy Dry-Run API, Overwatch, Overstore, Overvault, Central AI Service, Stewardship Reporting Service, PIP Registry, Fraud Control Service, Overclaim, SDK, CLI, admin UI, and native reporting surfaces.
  - Output: Boundary matrix listing owner, input refs, output refs, freshness rule, redaction class, policy refs, evidence refs, downstream consumer, and rejection behavior.
  - Validation: Review confirms every handoff uses explicit APIs, immutable refs, signed evidence, stable reason codes, trace ids, idempotency keys, policy refs, and Overwatch events rather than privileged shared records or hidden control paths.

## Phase 2: Rust Contracts, Schemas, Lifecycles, And Fixtures

### Work Items

- **2.1 Create the Purpose Tag Registry Rust contract module.**
  - Design: Add contract types for purpose tags, tag versions, eligibility criteria bundles, evidence requirement bundles, steward assignments, change proposals, claim validations, publication refs, policy exports, events, redaction profiles, and stable errors.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, tag-state enums, version-state enums, validation-state enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from Overgrant, pool allocation, Overguard policy internals, storage/vault internals, fraud, disputes, scheduling, execution, and accounting mutation.

- **2.2 Define tag and tag-version schemas.**
  - Design: Model `purpose_tag` and `purpose_tag_version` with stable tag id, slug, title, description, category, visibility, current version ref, owner/steward refs, lifecycle state, public docs refs, criteria hash, evidence requirement hash, activation window, signer refs, predecessor refs, and deprecation refs.
  - Output: JSON Schema files, valid examples, invalid examples, generated validators, docs-facing examples, and canonical tag/version fixtures.
  - Validation: Schema tests reject missing tag id, slug, version id, lifecycle state, criteria hash, evidence requirement hash, signer refs, activation window, public docs refs, trace id, idempotency key, or audit refs.

- **2.3 Define criteria, evidence, steward, and proposal schemas.**
  - Design: Model `eligibility_criteria_bundle`, `evidence_requirement_bundle`, `steward_assignment`, and `tag_change_proposal` with required predicates, disallowed predicates, evidence types, redaction rules, retention class, steward authority, conflict-of-interest refs, compatibility impact, privacy/compliance impact, affected pools/grants/policies, and review state.
  - Output: Criteria schema, evidence schema, steward schema, proposal schema, redaction profiles, compatibility examples, and negative fixtures.
  - Validation: Tests reject missing predicates, missing evidence requirement hashes, unsafe public evidence classes, missing retention class, missing steward scope, missing conflict metadata, missing affected-owner review refs, or missing migration/deprecation plan.

- **2.4 Define validation, publication, event, and replay schemas.**
  - Design: Model `purpose_claim_validation`, `tag_publication_ref`, events, and replay bundles with workload refs, claimed tag/version, submitted evidence refs, matched criteria, missing evidence, denial reason codes, policy refs, Overregistry refs, downstream subscriber refs, and redaction class.
  - Output: Validation schema, publication schema, event schema, replay schema, stable reason-code catalog, redacted examples, and replay fixtures.
  - Validation: Tests prove events never include private evidence content, replay reconstructs decisions from refs and hashes, and validation outputs cannot be interpreted as grant authorization.

- **2.5 Create deterministic Purpose Tag Registry fixtures.**
  - Design: Build fixtures for draft tags, active tags, inactive/review-only tags, deprecated tags, superseded tags, low-risk criteria update, semantic widening proposal, medical sensitive evidence refs, public evidence refs, missing evidence, stale evidence, policy export refresh, Overregistry publication, redacted public summary, and replay.
  - Output: Fixture directory, canonical inputs, expected states, expected events, expected reason codes, BLAKE3 hashes, redacted public views, steward-only views, policy exports, and replay bundles.
  - Validation: Fixture tests produce stable ids, hashes, lifecycle states, denial reason codes, audit refs, redacted views, and replay outputs across repeated runs.

## Phase 3: Tag Catalog CRUD, Versioning, And Publication Refs

### Work Items

- **3.1 Implement draft tag creation.**
  - Design: Add `POST /purpose-tags` for draft purpose tags with steward scope, category, public docs refs, visibility, initial lifecycle state, signed envelope, trace id, idempotency key, policy refs, and Overwatch audit refs.
  - Output: API handler, request/response schemas, signed envelope checks, idempotency behavior, stable errors, and `purpose_tag_registry.tag_created` events.
  - Validation: API tests cover valid draft creation, duplicate idempotency key, tag slug collision, missing steward scope, invalid visibility, unsafe category, missing audit refs, and audience-safe errors.

- **3.2 Implement draft tag-version creation.**
  - Design: Add `POST /purpose-tags/{tag_id}/versions` for draft versions with criteria refs, evidence requirement refs, compatibility notes, activation-window proposal, predecessor refs, signer refs, affected downstream refs, and validation preflight.
  - Output: Version API, version lifecycle state, version hash, predecessor/supersession refs, and `purpose_tag_registry.version_created` events.
  - Validation: Tests prove accepted versions are immutable, draft versions cannot overwrite active versions, and versions cannot proceed without criteria/evidence hashes and downstream compatibility notes.

- **3.3 Implement tag read and list APIs.**
  - Design: Add `GET /purpose-tags` and `GET /purpose-tags/{tag_id}` with active/public tag summaries, current version refs, public criteria summaries, public evidence requirement names, lifecycle status, deprecation/supersession refs, and audience-specific redaction.
  - Output: Read APIs, filters, pagination, public/steward/operator redaction profiles, and tag inventory projections.
  - Validation: Contract tests prove public reads do not expose raw submitted documents, private organization records, sensitive research details, fraud/abuse evidence, payment/payout details, private central-AI context, Overvault refs, or steward-only notes.

- **3.4 Implement Overregistry publication refs.**
  - Design: Publish approved active tag/version refs, criteria hashes, evidence requirement hashes, public docs refs, deprecation refs, and subscriber update refs to Overregistry without copying raw evidence.
  - Output: Overregistry adapter, publication ref builder, subscriber update refs, stale-publication detection, and `policy_export_updated` or publication events.
  - Validation: Tests prove publication fails closed when Overregistry rejects, refs are stale, hashes mismatch, or required active-version facts are missing.

- **3.5 Publish tag inventory diagnostics.**
  - Design: Provide diagnostics for active tags, draft tags, review-pending versions, deprecated versions, inactive/review-only tags, missing criteria, missing evidence requirements, stale policy exports, and affected pools/grants.
  - Output: Diagnostic APIs, operator projections, steward queues, stale-export alerts, public-safe summaries, and Overwatch timeline refs.
  - Validation: Tests prove diagnostics are explainable and correctable without exposing raw private evidence, restricted reviewer notes, anti-abuse internals, or account/payment facts.

## Phase 4: Eligibility Criteria And Evidence Requirement Bundles

### Work Items

- **4.1 Implement criteria bundle validation.**
  - Design: Validate required predicates, disallowed predicates, accepted organization/workload types, evidence thresholds, freshness rules, compatibility windows, and policy export facts as a single versioned criteria bundle.
  - Output: Criteria validator, matched predicate output, missing predicate output, incompatible predicate reason codes, and criteria fixtures.
  - Validation: Tests prove each criterion can independently allow, deny, request more evidence, or require review with deterministic ordered reason codes.

- **4.2 Implement evidence requirement bundle validation.**
  - Design: Validate required evidence types, accepted evidence refs, redaction rules, retention class, reviewer visibility, public explanation text, freshness, hash/integrity refs, and protected storage refs.
  - Output: Evidence requirement validator, evidence-ref checker, freshness checker, redaction-rule checker, and invalid evidence fixtures.
  - Validation: Tests prove missing, stale, unverifiable, wrong-class, unredactable, or privacy-unsafe evidence returns `needs_more_evidence`, `claim_evidence_missing`, or `claim_ineligible` without exposing raw evidence.

- **4.3 Implement public versus steward-only evidence classes.**
  - Design: Encode public evidence such as tag/version refs, public criteria summaries, public organization/project refs, repository/license refs, curriculum/public research refs, signed public attestations, criteria hashes, aggregate counts, safe denial totals, and redacted report refs separately from steward-only evidence.
  - Output: Evidence-class catalog, redaction profiles, public examples, steward-only examples, and disclosure-denial reason codes.
  - Validation: Tests prove raw submitted documents, private organization records, identity/contact refs, non-public grant letters, sensitive research protocols, private dataset/model refs, fraud/abuse evidence, Overvault refs, payment/payout facts, and private central-AI context never appear in public views.

- **4.4 Implement medical and sensitive research evidence handling.**
  - Design: Require Compliance Boundary facts, workload/data-class refs, signed steward-visible evidence refs, protected Overstore/Overvault refs, hash/integrity refs, authorized reviewer refs, redaction profiles, and Overwatch replay evidence before medical or sensitive research claims can validate.
  - Output: Sensitive-evidence validator, compliance fact adapter, reviewer-authorization checker, protected evidence ref schema, and denial/remediation summaries.
  - Validation: Tests prove missing compliance facts, private-data handling rules, authorized reviewer refs, protected storage refs, or redaction profiles return `needs_more_evidence` or `claim_ineligible`, and public-provider placement remains denied unless a later trusted policy explicitly allows a narrow low-sensitivity derivative.

- **4.5 Publish evidence and criteria diagnostics.**
  - Design: Provide diagnostics for missing criteria predicates, stale evidence, incompatible evidence classes, unsupported retention class, redaction failures, inactive tag evidence, deprecated version evidence, and affected downstream policy exports.
  - Output: Diagnostic API, steward remediation summaries, public-safe summaries, operator details, policy-export freshness refs, and Overwatch refs.
  - Validation: Tests prove diagnostics are role-scoped, stable, and useful for correction without leaking private evidence or turning diagnostics into allocation decisions.

## Phase 5: Stewardship Review, Activation, Deprecation, And Governance

### Work Items

- **5.1 Implement steward assignment and conflict checks.**
  - Design: Add steward assignment records with identity refs, scope, review authority, conflict-of-interest refs, start/end times, domain expertise refs, and audit refs.
  - Output: Steward assignment APIs or commands, conflict checker, scope checker, assignment lifecycle, and `steward_conflict` errors.
  - Validation: Tests prove tag changes cannot proceed when steward authority is missing, expired, outside scope, or conflicted without explicit governance escalation.

- **5.2 Implement review proposal workflow.**
  - Design: Add `POST /purpose-tags/{tag_id}/review` for opening or updating tag change proposals with motivation, criteria/evidence diffs, compatibility impact, privacy/compliance impact, affected pools/grants/policies, comments refs, and approval refs.
  - Output: Review API, review state machine, proposal diff records, reviewer comments refs, approval/rejection refs, and `purpose_tag_registry.review_opened` events.
  - Validation: Tests cover valid review opening, missing diff, missing affected owner refs, missing conflict metadata, unsupported semantic widening, rejected review, requested changes, and safe redacted review reads.

- **5.3 Implement activation gates.**
  - Design: Add `POST /purpose-tags/{tag_id}/versions/{version_id}/activate` that requires approved review refs, steward signatures, Overregistry publication readiness, policy export readiness, criteria/evidence bundle validation, downstream owner review for affected services, and activation window checks.
  - Output: Activation API, activation preflight bundle, active-version record, policy export refs, publication refs, and `purpose_tag_registry.version_activated` events.
  - Validation: Tests prove activation fails when review is missing, criteria invalid, evidence requirements missing, downstream owner review missing, Overregistry publication unavailable, policy export stale, or signer refs invalid.

- **5.4 Implement deprecation and supersession.**
  - Design: Add `POST /purpose-tags/{tag_id}/versions/{version_id}/deprecate` for planned deprecation, supersession, correction/retraction notices, replacement refs, compatibility windows, affected pool/grant/policy refs, and historical replay preservation.
  - Output: Deprecation API, supersession records, correction/retraction notices, stale-tag alerts, replacement refs, and `purpose_tag_registry.version_deprecated` events.
  - Validation: Tests prove deprecated versions cannot be used for new grants unless explicit historical-continuation policy allows it, while historical reports and replay remain visible with redaction.

- **5.5 Implement governance escalation paths.**
  - Design: Route new public tags, tag splits, semantic eligibility widening, medical/regulatory/public-infrastructure activation, and public-reporting/allocation-impacting changes through domain review, security/privacy/compliance/accounting checks, and PIP Registry refs when available.
  - Output: Governance escalation checker, PIP handoff refs, blocking-finding records, Central AI recommendation records, final steward decision refs, and public correction notices.
  - Validation: Tests prove Central AI can recommend changes but cannot activate, deprecate, split, or widen tags directly, and blocking security/privacy/compliance/accounting findings stop activation.

## Phase 6: Purpose Claim Validation And Evidence Ref Loading

### Work Items

- **6.1 Implement claim validation API.**
  - Design: Add `POST /purpose-tags/{tag_id}/validate-claim` for workload refs, claimed tag/version, evidence refs, criteria/evidence bundle refs, policy refs, trace id, idempotency key, and audience-specific explanations.
  - Output: Validation API, request/response schemas, signed envelope checks, idempotency behavior, stable errors, and `purpose_tag_registry.claim_validated` or `claim_denied` events.
  - Validation: API tests cover eligible claims, missing evidence, stale evidence, inactive version, deprecated version, privacy-unsafe evidence, unsupported workload/data class, duplicate idempotency key, and safe denials.

- **6.2 Implement evidence ref loading and hash checks.**
  - Design: Load evidence metadata from Overstore, Overvault, Overwatch, Compliance Boundary, public project refs, repository/license refs, curriculum refs, and signed attestations without copying raw restricted payloads into registry records.
  - Output: Evidence-loader adapter layer, ref freshness checks, hash/integrity checks, protected-ref guardrails, redaction inputs, and missing-evidence outputs.
  - Validation: Tests prove loader failures, stale refs, hash mismatch, revoked refs, inaccessible protected refs, or raw-private-payload attempts deny validation without leaking content.

- **6.3 Implement criteria matching and missing evidence output.**
  - Design: Evaluate active tag-version criteria against loaded evidence refs, produce matched criteria, missing criteria, missing evidence ids, denial reason codes, expiration refs, policy refs, and replay refs.
  - Output: Criteria matcher, ordered reason-code output, missing-evidence checklist, `needs_more_evidence` responses, validation lifecycle state, and replay snapshot.
  - Validation: Tests prove deterministic matching, strictest-denial precedence, stable reason codes, no permissive fallback for ambiguous evidence, and no allocation side effects.

- **6.4 Implement validation reads and role redaction.**
  - Design: Add validation status/read views for workload owners, pool/grant services, stewards, operators, and public reporting with role-specific evidence summaries and redaction.
  - Output: Validation read API, role-scoped projections, public-safe validation summaries, steward-only evidence summaries, and audit refs.
  - Validation: Contract tests prove public and downstream service views receive validation refs and redacted facts, not raw evidence, private contacts, payment facts, anti-abuse details, or protected Overvault payloads.

- **6.5 Link validation usage and accounting refs without owning accounting.**
  - Design: Emit usage-relevant refs for validation work, review work, policy export generation, report generation, and evidence ref loading to Overmeter/Overwatch where material while leaving ORU, Seal Ledger, Overbill, and Overgrant state untouched.
  - Output: Usage-ref handoff, validation usage events, accounting-friendly redacted summaries, and owner-service boundary notes.
  - Validation: Tests prove Purpose Tag Registry does not create ORU balances, grants, payouts, invoices, ledger entries, price records, fee records, or resource allocation decisions.

## Phase 7: Policy Exports And Owner-Service Integrations

### Work Items

- **7.1 Implement policy export API.**
  - Design: Add `GET /purpose-tags/{tag_id}/policy-export` returning active tag/version refs, criteria hashes, evidence requirement hashes, policy facts, deprecation status, compatibility windows, and downstream subscriber refs.
  - Output: Policy export API, export schema, export hash, stale export behavior, subscriber update refs, and `purpose_tag_registry.policy_export_updated` events.
  - Validation: Tests prove exports include active version ids and criteria hashes, reject stale/inactive/deprecated facts for new decisions, and never contain raw evidence.

- **7.2 Integrate Overguard and Policy Dry-Run.**
  - Design: Provide Overguard and Policy Dry-Run API with purpose-tag policy facts for admission, previews, missing evidence explanations, deprecated-version handling, public-provider low-sensitivity gates, and affected policy refs.
  - Output: Overguard adapter, dry-run fact bundle, policy decision refs, stale-policy behavior, and denial replay fixtures.
  - Validation: Tests prove Overguard denial, stale policy refs, missing export refs, deprecated versions, or unsupported data classes block admission rather than allowing unsafe defaults.

- **7.3 Integrate Overgrant and Public-Interest Pool Service.**
  - Design: Require active purpose-tag validation refs before Overgrant authorizes purpose-scoped resources and before Public-Interest Pool Service activates pools or allocation requests.
  - Output: Grant/pool handoff contract, validation-ref checker, missing-evidence reason codes, inactive/review-only tag behavior, and affected pool/grant alerts.
  - Validation: Tests prove the registry returns validation/policy refs only and never creates grant authorizations, pool allocations, resource reservations, quota decisions, or fairness decisions directly.

- **7.4 Integrate Fraud Control, Overclaim, and correction handoffs.**
  - Design: Provide evidence refs, validation refs, tag version refs, criteria hashes, denial reason codes, and replay refs to Fraud Control Service and Overclaim when claims are suspicious, disputed, corrected, or retracted.
  - Output: Fraud/dispute handoff contract, correction event refs, retraction refs, affected validation refs, and public-safe correction notices.
  - Validation: Tests prove the registry emits evidence and correction refs without adjudicating fraud, disputes, refunds, holds, reputation, payouts, or final claim outcomes outside tag-definition validation.

- **7.5 Integrate Central AI and stewardship reporting boundaries.**
  - Design: Provide Central AI Service and Stewardship Reporting Service with public tag metadata, aggregate validation refs, redacted denial totals, outcome-report refs, recommendation refs, and correction notices while preserving activation authority with stewards/governance.
  - Output: Reporting adapter, Central AI recommendation record, stewardship report refs, public-safe aggregates, and redaction threshold behavior.
  - Validation: Tests prove reporting excludes raw evidence, private workload facts, medical details, security/anti-abuse facts, payment facts, and private central-AI reasoning, and recommendations cannot mutate tag state.

## Phase 8: Replay, Public Documentation, Reporting, And Retention

### Work Items

- **8.1 Implement tag and validation replay APIs.**
  - Design: Add `GET /purpose-tags/replay/{validation_id}` and tag-change replay views to reconstruct request facts, tag version, criteria hash, evidence refs, matched predicates, missing predicates, denial reasons, policy refs, steward decisions, publication refs, and redacted evidence.
  - Output: Replay APIs, replay bundle schema, hash validation, role-scoped redactions, and replay fixtures.
  - Validation: Tests prove replay reconstructs historical decisions even after tags are deprecated, superseded, corrected, or public summaries are retracted.

- **8.2 Implement redacted public tag documentation generation.**
  - Design: Generate public documentation for active and historical tags including tag id, version, criteria summary, evidence requirement names, active/deprecated status, activation/deprecation window, supersession link, correction notices, and public-safe examples.
  - Output: Documentation generator, public summary schema, redacted examples, correction/retraction view, and public archive index.
  - Validation: Tests prove docs exclude raw sensitive evidence, steward-only notes, private data refs, payment facts, anti-abuse details, and misleading stale summaries after correction.

- **8.3 Implement retention and archive behavior.**
  - Design: Keep redacted public archive entries for historical tag versions as long as any report, pool, grant, policy export, deprecation notice, or validation replay cites them, normally indefinitely for public ids, hashes, windows, supersession links, and correction notices.
  - Output: Retention policy metadata, archive projections, correction/retraction records, protected evidence retention refs, and deletion-block reasons.
  - Validation: Tests prove cited historical versions cannot be hard-deleted from public history, while raw/steward-only evidence follows Overstore, Overvault, Overwatch, Compliance Boundary, and Overclaim retention policy.

- **8.4 Implement report and aggregate projections.**
  - Design: Provide aggregate validation counts, public-safe denial reason totals, active/deprecated tag status, affected pools/grants, policy export freshness, pending reviews, and claim evidence backlog for stewardship reporting.
  - Output: Report projection APIs, aggregate schemas, redaction thresholds, report hashes, and stewardship-report handoff fields.
  - Validation: Tests prove reports are role-scoped, thresholded where needed, and do not leak private workload evidence, medical facts, sensitive research details, fraud evidence, or private account data.

- **8.5 Publish operations dashboards and alerts.**
  - Design: Expose active tags, pending reviews, deprecated versions, claims needing evidence, denied claims by reason code, affected pools/grants, stale policy exports, stale downstream references, and public documentation freshness.
  - Output: Operator/steward query APIs, alert refs, Overwatch timeline views, status projections, and audit-export hooks.
  - Validation: Tests prove alerts fire when active pools or grants reference deprecated tag versions, exports go stale, evidence backlogs grow, or public docs are blocked by redaction review.

## Phase 9: Phase 10 Proof, Phase 11 Hardening, And Governance Expansion Gates

### Work Items

- **9.1 Configure the first stewarded tag proof set.**
  - Design: Create the six named tag records `science`, `education`, `medical`, `opensource`, `climate`, and `public_infrastructure` with evidence bundles, criteria bundles, public docs refs, steward refs, and lifecycle states.
  - Output: Proof tag fixtures, criteria fixtures, evidence requirement fixtures, steward fixtures, public summaries, and expected policy exports.
  - Validation: Scenario tests prove `science`, `education`, and `opensource` can become active for proof allocation while `medical`, `climate`, and `public_infrastructure` remain inactive or review-only until their compliance, outcome, redaction, steward, emergency/public-service, and governance gates are satisfied.

- **9.2 Prove public-interest pool and Overgrant integration.**
  - Design: Run scenarios where Public-Interest Pool Service and Overgrant consume active tag validation refs, criteria hashes, evidence requirement refs, policy exports, and denial reason codes for eligible and ineligible workloads.
  - Output: Integration scenario, active validation refs, missing-evidence refs, grant/pool handoff refs, Overguard decision refs, and replay bundle.
  - Validation: Tests prove eligible public proof claims pass with explicit refs, ineligible claims deny before grant authorization or pool allocation, and the registry never creates allocations itself.

- **9.3 Prove policy dry-run and Overguard preflight.**
  - Design: Run side-effect-free previews for draft tags, inactive tags, deprecated versions, stale policy exports, missing evidence, medical/sensitive evidence, and public-provider low-sensitivity restrictions.
  - Output: Dry-run fixtures, Overguard fact bundles, missing prerequisite lists, stale-export denial refs, and replay packs.
  - Validation: Tests prove dry-run cannot activate tags, validate final claims, authorize grants, allocate pools, schedule workloads, execute workloads, or mutate owner-service state.

- **9.4 Define Phase 11 public-provider hardening gates.**
  - Design: Specify how Purpose Tag Registry validation refs interact with Public Provider Onboarding, Public Sandbox Profile, Fraud Control Service, Challenge Task Service, Reputation and Anti-Sybil Service, Provider Payout Service, Workload Classifier, and Oververify for low-sensitivity public-provider expansion.
  - Output: Phase 11 handoff matrix, low-sensitivity public-provider restrictions, sandbox/public-tag interaction rules, fraud/challenge/payout/refutation refs, and public-safe explanations.
  - Validation: Review proves Phase 11 public nodes cannot inherit trusted federation privileges, private/regulated/secret/system-service workloads remain denied, and purpose tags do not weaken public sandbox controls.

- **9.5 Define Phase 13 governance and scale hardening gates.**
  - Design: Specify PIP governance, compliance boundary review, stewardship reporting, redaction review, incident response, threat modeling, audit export, retention review, domain expert review, public correction policy, and scale-readiness gates.
  - Output: Governance checklist, PIP trigger matrix, compliance handoff refs, reporting classes, retention classes, incident hooks, threat-review targets, and scale-readiness gate.
  - Validation: Review confirms tag addition, split, semantic widening, medical/regulatory/public-infrastructure activation, and public-reporting/allocation-impacting changes require appropriate governance before wider scale.

## Phase 10: Validation, Queue Closure, And Handoff Readiness

### Work Items

- **10.1 Validate plan structure and local links.**
  - Design: Check title prefix, attached SDS link, phases 1 through 10, five work items per phase, Design/Output/Validation structure, final newline, tabs, and local Markdown links.
  - Output: Validation command output and progress evidence.
  - Validation: Focused script passes for this file and every changed cross-document link.

- **10.2 Validate tech-stack and authority guardrails.**
  - Design: Scan this plan and changed docs for conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain/NFT, speculative-market, pricing, revenue, customer-count, raw sensitive evidence storage, grant authorization, resource allocation, merit scoring, dispute adjudication, fraud adjudication, scheduling, execution, accounting mutation, and external-provider-as-core-boundary drift.
  - Output: Guardrail scan output with only allowed negative-control, native Overrid service-name, authority-boundary, or explicit non-choice references.
  - Validation: Review confirms the plan keeps Rust-first/native Overrid service boundaries and does not turn comparison tools, local stubs, owner-service refs, or downstream APIs into Purpose Tag Registry's product boundary.

- **10.3 Validate SDS #57 build-breakdown coverage.**
  - Design: Map every SDS #57 build-breakdown item to this plan: schemas, tag/version CRUD, review/activation/deprecation/supersession, claim validation, policy exports, integrations, deprecation alerts, replay, public documentation, and Phase 10 validation proof.
  - Output: Coverage matrix, API checklist, workflow checklist, fixture checklist, integration-test targets, and authority-boundary checklist.
  - Validation: Review proves no SDS #57 build-breakdown item is missing and the plan preserves Purpose Tag Registry as a Phase 10 stewarded taxonomy and validation service.

- **10.4 Validate SDS, service catalog, master plan, crosswalk, and queue alignment.**
  - Design: Confirm this plan is linked from the SDS source table, service catalog sub-build-plan section, master per-SDS table, build-plan crosswalk, queue state, queue progress, and build-plan progress.
  - Output: Updated backlinks and index rows for SDS #57, queue state update, queue progress update, and build-plan progress evidence.
  - Validation: JSON validation passes; local link checks pass; queue validation confirms `057-build-plan` is complete, no materialized task is running, and `058-build-plan` is the next incomplete build-plan task.

- **10.5 Validate implementation handoff readiness.**
  - Design: Confirm the 10-phase plan gives builders source alignment, contracts, schemas, tag/version APIs, criteria/evidence bundles, stewardship workflow, claim validation, policy exports, integrations, replay/reporting, proof scenarios, governance gates, and validation work in dependency order without changing master Phase 0 through Phase 13 order.
  - Output: Handoff checklist, Docdex index refresh, search evidence, DAG export reference, and test-runner status.
  - Validation: Docdex targeted index refresh succeeds, Docdex retrieval finds the plan, `git diff --check` passes, and any `docdexd run-tests` blocker is recorded explicitly.

## Alignment Review

- The sub-build plan keeps Purpose Tag Registry first build work in master Phase 10 because trusted federation and public-interest pools need stewarded purpose tags, evidence requirements, policy export facts, and replayable validation refs before accountable public-interest allocation can operate.
- The plan treats earlier phases as prerequisites for identity, tenancy, signing, Overregistry publication, Overwatch audit, Overguard policy, Policy Dry-Run, Overgrant boundaries, accounting refs, and protected evidence refs.
- The plan treats Phase 11 as later public-provider hardening, not the first registry build point. Unknown public-provider capacity cannot inherit trusted federation privileges and cannot use purpose tags to bypass Public Sandbox Profile, Fraud Control, Workload Classifier, Oververify, Challenge Task, or payout-hold gates.
- The plan treats Phase 13 as governance/compliance/scale hardening for PIP-controlled changes, compliance boundaries, redaction review, stewardship reporting, threat review, incident response, audit export, and retention.
- The plan treats Overregistry as publication-ref owner; Purpose Tag Registry publishes tag/version refs and hashes without becoming the general declared-facts store.
- The plan treats Overgrant and Public-Interest Pool Service as allocation owners; Purpose Tag Registry returns validation and policy refs without authorizing grants or allocating resources.
- The plan treats Overguard and Policy Dry-Run API as policy-evaluation owners; Purpose Tag Registry exports purpose facts without becoming a general policy engine.
- The plan treats Overstore, Overvault, Overwatch, and Compliance Boundary services as raw/protected evidence and audit owners; Purpose Tag Registry stores refs, hashes, redacted summaries, and validation facts.
- The plan treats Fraud Control Service, Overclaim, Central AI Service, and Stewardship Reporting Service as consumers or recommenders, not owners of tag activation/deprecation authority.
- The plan aligns with `docs/overrid_tech_stack_choice.md`: Rust-first core service/contracts, native Overrid boundaries, canonical JSON/JSON Schema, optional Protobuf, Ed25519 signatures, BLAKE3/content hashes, signed envelopes, and no conventional cloud, blockchain, NFT, pricing, revenue, or customer-count assumptions.
