# SUB BUILD PLAN #37 - Workload Classifier

Attached SDS: [docs/sds/trust_policy_verification/workload_classifier.md](../sds/trust_policy_verification/workload_classifier.md)

## Purpose

This sub-build plan turns SDS #37 into an implementation sequence for Workload Classifier. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Workload Classifier is a Phase 4 deterministic classification-fact producer. It normalizes workload class, data class, secret-bearing status, regulated status, system-service status, public-provider eligibility, sandbox/cache/egress class, and allowed execution environments before Overguard policy evaluation, Policy Dry-Run API previews, scheduler placement, runner preflight, public-pool controls, and compliance review. It does not replace Overguard, schedule workloads, execute workloads, grant secrets, mutate package/storage/billing state, or let user-declared labels override evidence.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #37: Workload Classifier](../sds/trust_policy_verification/workload_classifier.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, API/event surfaces, workflows, state machine, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Workload Classifier service plan](../service_catalog/trust_policy_verification/workload_classifier.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared schemas, signed envelopes, idempotency, trace ids, stable reason codes, deterministic fixtures, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies identities, tenants, credentials, Overgate request discipline, Overregistry refs, Overwatch audit, and Overqueue-safe command context. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies Overpack manifests, package/runtime facts, scheduler/runner consumers, lease-bound execution context, and raw usage-event handoffs. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Controls the first build point: workload/data sensitivity classes, classification facts, Overguard policy inputs, Policy Dry-Run API previews, verification/challenge/dispute evidence, cache trust scopes, and replayable safety decisions. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies accounting precheck refs, usage facts, dispute/hold refs, and downstream evidence refs while preserving mutation authority in accounting services. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies protected system-service placement, grid-resident operation, failover, restore, maintenance, and operator-action hardening. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase data-class refs, Overstore artifact refs, Overvault secret/private refs, namespace refs, retention/export handoffs, and native persistence boundaries. |
| [Phase 9: Overpack Deployment Platform](phase_09_overpack_deployment_platform.md) | Supplies deployment-plan and application-intent classification batch consumers. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies strict public workload eligibility, no-secret/no-private/no-regulated/no-system-service controls, public sandbox profile, public-provider reputation/fraud/challenge refs, and public-pool downgrade/deny gates. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Supplies SDK, CLI, admin UI, native app, central AI, and stewardship consumers for safe explanations and pre-submit dry-run previews. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies compliance boundaries, threat-model checks, audit exports, stewardship reports, incident handoffs, retention rules, and class-definition governance. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #37 first build work aligned to master Phase 4, with later storage/private-ref, public-provider, native-app, and governance hardening gates. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, pricing, revenue, or customer-count drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 3, 4, 8, 11, 12, and 13 | Attach SDS #37, freeze classifier authority, preserve Phase 4 as the first build point, and record prerequisite plus hardening gates. |
| 2 | Master Phases 0, 1, and 4 | Build Rust contracts, schemas, class definitions, state enums, reason codes, compatibility fixtures, and deterministic golden cases. |
| 3 | Master Phases 1, 3, 4, and 8 | Validate classify requests and assemble manifest, package, data, secret, egress, tenant, native-service, system-service, and storage fact snapshots. |
| 4 | Master Phases 4, 8, 11, and 13 | Evaluate rule matches, apply strictest-class selection, downgrade or deny unsafe declarations, and emit replayable classification decisions. |
| 5 | Master Phases 1, 4, and 12 | Expose classify, batch, read, explain, replay, class-listing, data-class-listing, override, and event surfaces with role-aware redaction. |
| 6 | Master Phases 4, 5, and 11 | Export policy input facts to Overguard and Policy Dry-Run API, with accounting precheck refs and public-provider eligibility constraints. |
| 7 | Master Phases 3, 4, 8, and 11 | Provide scheduler, runner, mesh, cache, Overvault, and public sandbox handoffs without moving those services' authority into Workload Classifier. |
| 8 | Master Phases 4, 8, 12, and 13 | Add replay, replacement decisions, override flow, class-definition rollout, historical compatibility checks, and native storage/private-ref migration. |
| 9 | Master Phases 4, 7, 11, 12, and 13 | Add operations, dashboards, native-app previews, role-scoped evidence views, grid-resident hardening, and governance/compliance controls. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, behavior fixtures, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Workload Classifier core is a Rust service/module using shared contract types, Tokio for bounded async validation/replay workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Workload/data class definitions, classification requests, input snapshots, rule matches, decisions, reason codes, policy input facts, overrides, replay bundles, API objects, events, fixtures, and explanation profiles use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be added only through the shared contract layer where needed.
- Mutating endpoints require signed actor or service envelopes, tenant context, trace id, idempotency key, schema version, class-definition version, policy refs, evidence refs, stable reason codes, redaction class, and append-only Overwatch events.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for manifest refs, data/secret metadata refs, input snapshots, class definitions, replay bundles, fixtures, and deterministic comparison tests.
- The service may later persist native classification records through Overbase, replay/evidence artifacts through Overstore, and private/compliance refs through Overvault. It must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, external workflow products, or external policy SaaS the platform boundary.
- The classifier must never inspect raw private payloads or raw secrets to invent sensitivity. It consumes versioned metadata refs from owning services and fails closed when critical owner-service facts are missing, stale, or contradictory.
- Public low-sensitivity placement requires positive evidence of no secrets, no private tenant data, no regulated data, no system-service responsibility, compatible egress, compatible sandbox/cache rules, and compatible runtime/resource caps.
- Accounting services use classification refs as evidence. Workload Classifier must not create pricing, financial projections, ORU transitions, ledger entries, invoices, receipts, payouts, refunds, holds, or settlement mutations.
- Planning and implementation must avoid opaque global trust numbers, hidden privileged state reads, broad public-provider trust, tokenized/classification markets, NFTs, blockchain mechanics, pricing tables, revenue projections, customer-count assumptions, and conventional SaaS-admin framing.

## Phase 1: SDS Attachment, Classifier Authority, And Phase Gates

### Work Items

- **1.1 Attach the build plan to SDS #37.**
  - Design: Link this document from the numbered Workload Classifier SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/trust_policy_verification/workload_classifier.md`, `docs/service_catalog/trust_policy_verification/workload_classifier.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #37 returns both the Workload Classifier SDS and this sub-build plan.

- **1.2 Freeze classifier authority boundaries.**
  - Design: Record that Workload Classifier owns class definitions, input schema, input snapshots, rule matches, decisions, reason codes, exported policy facts, replay bundles, redacted explanations, overrides, and replacement-decision history.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms it does not own Overguard admission, policy authoring, scheduler placement, runner execution, Overvault secret grants, raw private data inspection, package mutation, storage mutation, billing, settlement, or payout holds.

- **1.3 Preserve master Phase 4 as the first build point.**
  - Design: Keep the first implementation in Phase 4 because workload/data classification is required before replayable Overguard decisions, dry-run previews, scheduler eligibility, cache scope decisions, and safe public-pool expansion.
  - Output: Phase-gate note that Phase 0 and Phase 1 are prerequisites, Phase 3 provides manifest/execution consumers, Phase 4 is first build, and Phases 8, 11, 12, and 13 are later expansion or hardening gates.
  - Validation: Review proves this plan does not move public-provider broadening into Phase 4 and does not defer core classification behind Phase 11.

- **1.4 Carry forward resolved SDS #37 open-question decisions.**
  - Design: Preserve deny-by-default behavior for incomplete refs, role-scoped visibility, full replay requirements for eligibility-broadening changes, manual-review versus automatic-denial boundaries, and native-app dry-run presentation rules.
  - Output: Resolved-decision checklist tied to implementation reviews.
  - Validation: Review rejects fallback-to-public behavior, raw secret/private-data exposure, hidden rule thresholds, mutation of old decisions, review-required provisional permission, and native-app submission without fresh dry-run confirmation for material broadening.

- **1.5 Define owner-service and consumer boundaries.**
  - Design: Create a dependency matrix for Overgate, Overpack, Package Validator, Overregistry, Overtenant, Overvault, Overbase, Overstore, Overcache, Overguard, Policy Dry-Run API, Oversched, Overrun, Overmesh, Compliance Boundary Service, Overclaim, SDK, CLI, admin UI, native apps, and central AI stewardship.
  - Output: Boundary matrix listing consumed refs, emitted decision refs, authority owner, freshness owner, visibility class, redaction profile, replay evidence, expiry behavior, and later phase gate.
  - Validation: Review confirms every handoff uses explicit APIs, versioned refs, signed evidence, reason codes, policy/class versions, trace ids, and Overwatch evidence rather than direct privileged state reads.

## Phase 2: Rust Contracts, Class Definitions, Schemas, And Fixtures

### Work Items

- **2.1 Create the Workload Classifier Rust contract module.**
  - Design: Add contract types for workload class definitions, data class definitions, classification requests, input snapshots, rule matches, classification decisions, reason codes, override requests, policy input facts, replay bundles, state enums, API errors, and events.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, state enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from Overguard policy, scheduling, execution, Overvault, storage, and accounting internals.

- **2.2 Define canonical workload and data classes.**
  - Design: Encode system service, private tenant, trusted federation, public low-sensitivity, research/public-interest, regulated, and secret-bearing workload classes plus public, tenant private, user private, organization private, secret-bearing, regulated, system-service, and grant-funded public-interest data classes.
  - Output: Versioned class definition files, allowed relationships, required trust class, secret policy, egress policy, sandbox requirements, cache scope, placement restrictions, and compatibility notes.
  - Validation: Schema and fixture tests prove incompatible data/workload combinations are rejected, downgraded, unknown, or review-required according to SDS policy rather than silently allowed.

- **2.3 Define classification schemas and examples.**
  - Design: Add JSON Schemas for requests, batches, input snapshots, rule matches, decisions, reason codes, policy facts, override requests, explain responses, replay bundles, class listings, data-class listings, and events.
  - Output: Schema files, valid examples, invalid examples, generated validators, compatibility notes, and docs-facing examples.
  - Validation: Schema tests reject missing tenant context, actor/service account, manifest id/version, class version, trace id, idempotency key, evidence refs, policy refs, redaction class, and state.

- **2.4 Model decision and override state machines.**
  - Design: Encode submitted, validating, collecting_facts, evaluating, classified, downgraded, denied, unknown, review_required, overridden, and expired decision states plus replacement-decision override semantics.
  - Output: Transition tables, Rust enums, JSON Schema enums, event emission rules, invalid transition fixtures, and replacement-link rules.
  - Validation: State tests reject mutation of prior decisions, override without signed action/evidence/expiry, classified state without replay bundle, and review-required state being treated as provisional permission.

- **2.5 Create deterministic classification fixtures.**
  - Design: Build fixtures for system-service, private tenant, trusted federation, public low-sensitivity, public-interest, regulated, secret-bearing, incomplete data refs, missing secret metadata, prohibited egress, sandbox mismatch, downgrade, denial, review-required, override, replay, and class-definition rollout.
  - Output: Fixture directory, expected decisions, reason codes, exported facts, redacted explanations, replay bundles, Overwatch events, and invalid examples.
  - Validation: Fixture tests produce stable final classes, reason codes, policy facts, redaction behavior, replay behavior, and replacement-decision history across repeated runs.

## Phase 3: Request Intake, Fact Normalization, And Snapshot Assembly

### Work Items

- **3.1 Implement classify and batch request validation.**
  - Design: Validate `POST /workload-classifications` and bounded batch requests for actor identity, tenant context, manifest refs, declared class hints, data refs, secret refs, egress refs, storage refs, package validation refs, trace id, idempotency key, and schema versions.
  - Output: Request validator, batch bounds, idempotency behavior, stable errors, and `workload_classifier.requested` or `workload_classifier.invalid` events.
  - Validation: API tests reject unsigned requests, missing identity/tenant/manifest facts, conflicting idempotency body, unbounded batches, stale schema versions, and user labels that try to override evidence.

- **3.2 Normalize manifest, package, and provenance facts.**
  - Design: Resolve Overpack manifest, application-intent manifest, runtime contract, permission refs, egress declarations, storage refs, package validation reports, package provenance, and Overregistry package/app/native-service records.
  - Output: Manifest/package fact normalizer, owner-service adapter interfaces, freshness policy, invalid-ref reason codes, and redacted snapshot fields.
  - Validation: Tests prove missing, stale, invalid, unsigned, or mismatched manifest/package facts produce denied, unknown, or review-required results rather than broad placement.

- **3.3 Normalize data, storage, cache, and secret refs.**
  - Design: Consume Overbase data-class refs, Overstore object/artifact class refs, Overcache scope refs, and Overvault secret metadata refs without reading raw private payloads or raw secret values.
  - Output: Data/secret/storage fact normalizer, redaction profile metadata, missing-owner-service behavior, strict fallback mapping, and snapshot commitments.
  - Validation: Tests prove secret-bearing refs force secret-bearing handling, regulated refs force regulated handling, incomplete data refs block public placement, and missing secret metadata blocks or classifies as secret-bearing.

- **3.4 Apply tenant, app, native-service, and system-service context.**
  - Design: Resolve Overtenant policy overlays, quota scope, suspension/compliance flags, app/native-service records, system-service records, service-account context, and region/jurisdiction constraints.
  - Output: Context resolver, tenant overlay fact set, system-service detection rules, compliance marker refs, and class restriction fields.
  - Validation: Tests prove suspended tenants, system-service workloads, compliance markers, private app state, and region constraints cannot be hidden by a public low-sensitivity declaration.

- **3.5 Produce immutable input snapshots.**
  - Design: Build `classification_input_snapshot` records with request refs, manifest summary, package permissions, data refs, secret metadata refs, tenant policy refs, native-service/system-service refs, collected-at timestamp, evaluator version, and source freshness map.
  - Output: Snapshot builder, BLAKE3/content hash, redacted read model, Overwatch evidence refs, and replay bundle inputs.
  - Validation: Replay tests prove unchanged snapshots reproduce decisions and missing critical refs produce stable `unknown`, `review_required`, or denial outcomes with `public_provider_allowed=false`.

## Phase 4: Rule Evaluation, Strictest-Class Selection, And Decisions

### Work Items

- **4.1 Build the deterministic rule evaluator.**
  - Design: Evaluate class definitions against manifest, package, data, secret, egress, tenant, native-service, system-service, cache, sandbox, and public-provider facts in a stable order.
  - Output: Rule evaluator, rule registry, matched-rule records, pass/fail/unknown states, class-definition version refs, evaluator version refs, and deterministic ordering.
  - Validation: Golden tests prove repeated evaluation yields identical rule matches, decision ids where deterministic inputs require them, reason code sets, and replay bundle hashes.

- **4.2 Implement strictest applicable class resolution.**
  - Design: Select the strictest evidenced workload/data class when declarations conflict with evidence, with system-service refs, secret refs, regulated markers, and tenant/private refs taking precedence over public or research claims.
  - Output: Class resolution module, class precedence table, downgrade mapping, required trust class, sandbox profile, cache scope, egress class, and public-provider eligibility fields.
  - Validation: Downgrade tests prove public declarations with secrets, private data, regulated data, system-service work, incompatible egress, or unsupported sandbox become stricter classes, denials, unknown, or review-required states.

- **4.3 Implement deny, unknown, review, and downgrade reason codes.**
  - Design: Create stable reason codes for incomplete data refs, missing secret metadata, conflicting manifest/storage facts, prohibited egress, unsupported sandbox, regulated marker, system-service placement, stale package validation, missing owner-service facts, and override-required cases.
  - Output: Reason-code catalog, user-safe messages, operator messages, remediation hints, redaction class, deprecation state, and compatibility policy.
  - Validation: Tests prove reason-code identifiers remain stable when wording changes and each reason has user-safe, developer/operator-redacted, and compliance/operator evidence renderings where allowed.

- **4.4 Compute execution-environment restrictions.**
  - Design: Derive required trust class, public-provider allowed flag, sandbox profile, cache scope, egress class, storage class, secret-bearing flag, regulated flag, system-service flag, and review/expiry behavior.
  - Output: Restriction projection, Overguard-ready policy facts, scheduler/runner/cache/Overvault handoff fields, and public-provider guard fields.
  - Validation: Tests prove public-provider allowed is false unless all required no-secret, no-private, no-regulated, no-system-service, compatible egress, compatible sandbox/cache, and capped runtime/resource facts are present.

- **4.5 Persist classification decisions and policy facts.**
  - Design: Write append-only classification decisions, matched rules, exported policy input facts, replay bundle refs, Overwatch event refs, and replacement links.
  - Output: Decision writer interface, policy fact records, replay bundle ref, `workload_classifier.decision_created`, downgraded, denied, and review-required events.
  - Validation: Tests prove old decisions are not mutated by class-definition changes, overrides create replacement decisions, and policy facts cite decision id, class version, reason code refs, and consumer service.

## Phase 5: API, Event, Listing, Explanation, And Redaction Surfaces

### Work Items

- **5.1 Implement core classification APIs.**
  - Design: Expose classify, batch classify, read decision, explain decision, replay decision, list workload classes, list data classes, and request override endpoints with signed envelopes, idempotency, pagination, stable errors, and redaction.
  - Output: API routes, request/response models, auth checks, pagination fields, error catalog, OpenAPI or schema docs where used, and endpoint fixtures.
  - Validation: Contract tests cover all endpoints, auth paths, idempotency conflicts, missing refs, stale refs, redaction classes, replay, and batch bounds.

- **5.2 Implement class and data-class listing surfaces.**
  - Design: Return current, canary, deprecated, blocked, and compatibility-visible class definitions for SDK, CLI, admin UI, documentation generation, and dry-run previews.
  - Output: Listing handlers, version filters, compatibility metadata, docs examples, and generated client fixtures.
  - Validation: Tests prove clients see only supported fields, deprecated classes remain readable for old decisions, and blocked classes cannot be used for new classification unless policy explicitly allows replay-only use.

- **5.3 Implement role-aware explanation surfaces.**
  - Design: Provide final class, state, class versions, public-provider eligibility, required trust class, sandbox/cache/egress labels, reason codes, messages, missing-prerequisite owners, remediation hints, trace id, and redacted evidence refs for ordinary callers.
  - Output: Explanation projection, provider/developer/operator/compliance view profiles, redaction fixtures, and remediation catalog.
  - Validation: Redaction tests prove normal callers cannot see raw private data refs, raw secret refs, provider-private facts, fraud heuristics, compliance marker internals, other-tenant facts, exact rule thresholds, or operator notes.

- **5.4 Emit classification events.**
  - Design: Emit requested, invalid, decision_created, downgraded, denied, review_required, override_requested, override_applied, and replay_completed events with decision id, class version, request refs, reason codes, evidence refs, and trace id.
  - Output: Event schemas, event publisher, Overwatch hooks, event fixtures, and consumer compatibility notes.
  - Validation: Event tests prove every state transition emits the expected append-only event and no event exposes raw secrets, private payloads, provider-private internals, or compliance-only details to unauthorized consumers.

- **5.5 Enforce API security and read redaction.**
  - Design: Require actor/service identity, tenant context, scope checks, trace ids, idempotency keys, redaction classes, policy refs, and audit refs on mutating paths and sensitive read paths.
  - Output: Authorization middleware, redaction middleware, audit hooks, safe error mapping, and security fixtures.
  - Validation: Security tests reject cross-tenant reads, unauthorized replay, unauthorized override, raw ref exposure, missing trace ids, and any read path that bypasses role-scoped projection rules.

## Phase 6: Overguard, Policy Dry-Run, And Policy-Fact Exports

### Work Items

- **6.1 Export policy input facts to Overguard.**
  - Design: Shape decision refs, final class, data class set, secret-bearing flag, regulated flag, system-service flag, public-provider allowed flag, required trust class, sandbox profile, cache scope, egress class, state, class version, and reason code refs for Overguard.
  - Output: `policy_input_fact` exporter, Overguard consumer contract, reason-code mapping, replay refs, and integration fixtures.
  - Validation: Integration tests prove Overguard consumes classification facts for admission while keeping final policy allow/deny/block/review authority in Overguard.

- **6.2 Integrate Policy Dry-Run API previews.**
  - Design: Provide side-effect-free classification results to Policy Dry-Run API before queueing, leasing, reserving, billing, Overvault mounting, route opening, or workload execution.
  - Output: Dry-run projection, missing prerequisite fields, expected class/restriction fields, remediation hints, and no-side-effect fixtures.
  - Validation: Dry-run tests prove no queue item, lease, reservation, Overvault mount, route, ORU transition, bill, payout, or workload record is created.

- **6.3 Wire SDK, CLI, admin UI, and native-app submission prechecks.**
  - Design: Expose stable client surfaces for submit-time classification previews, class changes, missing prerequisites, remediation hints, and explicit confirmation requirements for material broadening.
  - Output: Client response profiles, SDK/CLI examples, admin UI field list, native-app preview contract, and central AI review hooks.
  - Validation: Client tests prove material broadening such as private-data access, secret use, regulated handling, public-provider execution, external egress, system-service responsibility, or larger resource class requires manifest revision or explicit confirmation plus fresh dry run.

- **6.4 Integrate Overwatch evidence and audit refs.**
  - Design: Link classification requests, snapshots, decisions, overrides, replays, policy facts, dry-run previews, and consumer handoffs to Overwatch traces and audit events.
  - Output: Overwatch event refs, trace timeline fields, replay evidence refs, audit export fields, and operator timeline hooks.
  - Validation: Audit tests prove every decision and replacement decision is traceable from request through owner-service facts, rule matches, policy fact export, consumer use, and replay.

- **6.5 Emit usage facts without accounting mutation.**
  - Design: Emit classification count, batch count, class distribution, downgrade count, deny count, review-required count, override count, replay count, and decision refs for Overmeter, Overbill, Overclaim, and audit exports.
  - Output: Usage fact schema, Overmeter handoff refs, dashboard counters, and accounting evidence refs.
  - Validation: Tests prove usage facts support accountability without pricing assumptions, revenue projections, customer-count assumptions, per-operation external payment calls, ORU mutation, ledger mutation, invoices, payouts, refunds, or holds.

## Phase 7: Scheduler, Runner, Cache, Overvault, Mesh, And Public-Pool Handoffs

### Work Items

- **7.1 Provide Oversched placement class refs.**
  - Design: Feed scheduler reasoning with classification decision refs, required trust class, public-provider allowed flag, data class set, sandbox profile, egress class, cache scope, and reason codes.
  - Output: Oversched projection, placement explanation fields, stale-decision expiry behavior, and integration fixtures.
  - Validation: Scheduler tests prove placement explanations include classification refs and private, regulated, secret-bearing, system-service, or unresolved workloads cannot be placed on public providers.

- **7.2 Provide Overrun preflight restrictions.**
  - Design: Feed runner preflight with sandbox profile, secret-bearing flag, secret metadata prerequisites, data class restrictions, egress class, cache scope, public sandbox constraints, and expiry behavior.
  - Output: Overrun projection, preflight rejection codes, sandbox/cache/egress fields, and runner fixtures.
  - Validation: Runner tests prove execution blocks when classification is expired, unknown, review-required, denied, sandbox-incompatible, secret metadata is unavailable, or egress is prohibited.

- **7.3 Provide Overcache and Overmesh scope handoffs.**
  - Design: Feed cache and mesh consumers with cache trust scope, artifact quarantine hints, public low-sensitivity eligibility, private tenant boundaries, route class, egress class, and redacted refs.
  - Output: Overcache projection, Overmesh route-class projection, cache-scope fixtures, route fixtures, and reason-code mapping.
  - Validation: Tests prove cached artifacts and routes cannot cross into broader trust scopes without explicit policy approval and public cache/route paths require positive low-sensitivity evidence.

- **7.4 Provide Overvault and private-ref handoffs.**
  - Design: Feed Overvault consumers with secret-bearing prerequisites, mount-denied reason codes, redacted secret-ref metadata, private-ref evidence, and replacement-decision behavior without granting secrets locally.
  - Output: Overvault projection, secret prerequisite fields, redaction profile, and failure fixtures.
  - Validation: Tests prove Workload Classifier never grants secret access, never copies raw secret values, and blocks or marks review-required when secret metadata is missing, stale, or incompatible.

- **7.5 Harden Phase 11 public-pool constraints.**
  - Design: Attach no-secret, no-private-data, no-regulated-data, no-system-service, capped runtime, capped resources, compatible egress, compatible cache, public sandbox, public-provider reputation, and challenge/fraud refs to public-provider eligibility.
  - Output: Public-pool eligibility matrix, public sandbox projection, Phase 11 fixtures, and Overguard/Oversched handoff guards.
  - Validation: Public-provider tests prove the classifier cannot authorize broad public trust and public nodes receive only positively classified public low-sensitivity work.

## Phase 8: Replay, Overrides, Definition Rollout, And Compatibility

### Work Items

- **8.1 Build replay bundle storage and replay API.**
  - Design: Store request, input snapshot, class definitions, rule matches, decision, evaluator version, policy refs, Overwatch event refs, and redaction metadata needed to reconstruct a decision.
  - Output: Replay bundle writer, `POST /workload-classifications/{decision_id}/replay`, replay comparison model, and replay_completed events.
  - Validation: Replay tests prove stored inputs and class definitions reproduce decisions and mark minimized or unavailable refs explicitly rather than inventing evidence.

- **8.2 Gate class-definition rollout with replay.**
  - Design: Require full historical replay before activating changes that broaden eligibility, narrow denials, alter public-provider placement, secret-bearing/regulated/system-service handling, data-class allowlists, egress/cache/sandbox requirements, trust class, override semantics, compliance markers, or reason semantics.
  - Output: Rollout workflow, canary comparison, replay report, compatibility status, rollback path, and operator approval checklist.
  - Validation: Rollout tests reject activation when replay mismatches are unresolved or eligibility broadening lacks signed policy approval and Overwatch audit evidence.

- **8.3 Implement override and replacement-decision flow.**
  - Design: Support signed classification override requests with source decision, requested class, justification, evidence refs, approver refs, expiry, review state, and replacement decision ref.
  - Output: Override API handler, override state model, replacement-decision writer, override events, and Overclaim/Overwatch handoffs.
  - Validation: Tests prove overrides never mutate prior decisions, require signed authority, include expiry and evidence refs, preserve replay, and cannot override non-negotiable safety violations without policy support.

- **8.4 Add compatibility and migration reports.**
  - Design: Report class-definition changes, schema-version changes, reason-code deprecations, client compatibility, dry-run compatibility, native-app display effects, and stored-decision replay coverage.
  - Output: Compatibility report API, schema migration checklist, reason-code deprecation table, client impact notes, and validation fixtures.
  - Validation: Tests prove editorial wording, translations, UI labels, and documentation-only examples skip full replay only when stable codes and effects are unchanged.

- **8.5 Prepare native persistence and private-ref migration.**
  - Design: Move classification records to Overbase when available, replay/evidence artifacts to Overstore where appropriate, and private/compliance refs to Overvault without changing API semantics.
  - Output: Persistence adapter interfaces, migration plan, artifact refs, private-ref handoffs, retention hooks, and export format.
  - Validation: Migration tests prove no conventional database/object/vault boundary becomes canonical, records remain replayable, and raw secrets/private payloads never enter classification records.

## Phase 9: Operations, Client Profiles, Native Apps, And Governance Views

### Work Items

- **9.1 Build dashboards and alerts.**
  - Design: Track class distribution, downgrade reasons, denial reasons, unknown fact sources, review queues, class-definition rollout, replay health, public-low-sensitivity spikes, secret-bearing downgrades, and replay mismatches.
  - Output: Metrics schema, dashboard definitions, alert rules, Overwatch event aggregation, and operator runbook links.
  - Validation: Operations tests prove alerts fire for sudden public-low-sensitivity spikes, unexpected secret-bearing downgrades, unknown classification increases, class-definition replay mismatches, and owner-service fact outages.

- **9.2 Build role-scoped operator and compliance views.**
  - Design: Provide authorized views for matched rules, input snapshot structure, owner-service fact refs, redaction classes, policy/class rollout state, replay bundles, override history, compliance boundary markers, and audit exports.
  - Output: Operator timeline API, compliance projection, access audit events, export fixtures, and stewardship report fields.
  - Validation: Tests prove compliance/operator views require authorization and still do not expose raw secrets or raw private payloads outside Overvault/owner-service access controls.

- **9.3 Build native-app dry-run presentation contracts.**
  - Design: Support native app displays such as public execution changed to trusted private execution, secret access required, external network access denied, regulated review required, or public-provider capacity unavailable.
  - Output: Native-app preview schema, user-safe messages, manifest revision hints, explicit confirmation contract, and no-side-effect status fields.
  - Validation: Native-app tests prove previews state that no work has started, no secret has mounted, no route has opened, no lease/reservation exists, and no ORU/billing action occurred.

- **9.4 Add governance, threat-model, and incident handoffs.**
  - Design: Integrate Compliance Boundary policy refs, threat-model findings, incident escalation refs, stewardship reporting, PIP change controls, retention/export requirements, and region-specific review rules.
  - Output: Governance checklist, threat-model test list, incident handoff refs, compliance export schema, PIP change controls, and retention policy.
  - Validation: Governance tests prove broadening changes, compliance marker interpretation changes, and public-provider eligibility changes require signed action, evidence refs, expiry where applicable, and Overwatch audit.

- **9.5 Prepare grid-resident operation.**
  - Design: Package the service as a protected grid-resident system workload with service identity, config contracts, secret refs, health checks, failover behavior, restore drills, maintenance mode, and signed operator actions.
  - Output: System-service packaging notes, runtime config contract, backup/restore hooks, health endpoints, recompute pause/resume controls, and break-glass audit rules.
  - Validation: Grid tests prove restart, failover, restore, replay, maintenance mode, and class-definition rollback preserve append-only history and do not emit stale broad eligibility after recovery.

## Phase 10: Validation, Documentation Alignment, Queue State, And Handoff

### Work Items

- **10.1 Validate sub-build-plan structure.**
  - Design: Check title prefix, attached SDS link, phase headings 1 through 10, work-item structure, Design/Output/Validation fields, alignment review, and exit gate.
  - Output: Focused validation result for `SUB BUILD PLAN #37`.
  - Validation: Scripted text checks pass for required headings, numbered phases, local links, work-item sections, final newline, and tab-free formatting.

- **10.2 Validate tech-stack alignment.**
  - Design: Scan for accidental conventional cloud, SaaS-admin, blockchain/NFT, pricing, revenue, customer-count, external policy SaaS, broad public-provider, raw secret/private inspection, and direct accounting mutation assumptions.
  - Output: Stack-alignment scan result and any corrected wording.
  - Validation: Scan shows only approved negative-control references or explicit non-choice guardrails from `docs/overrid_tech_stack_choice.md`.

- **10.3 Validate SDS, service catalog, master plan, and crosswalk links.**
  - Design: Confirm this plan is linked from the SDS source table, service catalog detailed-SDS section, master per-SDS table, and service-catalog alignment index.
  - Output: Updated docs and link-check evidence.
  - Validation: Local Markdown link validation returns no missing local targets for changed docs.

- **10.4 Validate behavior fixtures and consumer handoffs.**
  - Design: Verify fixture coverage for public-low-sensitivity, secret-bearing, regulated, system-service, private tenant, incomplete refs, replay, override, dry-run, scheduler, runner, cache, Overvault, and native-app preview cases.
  - Output: Fixture coverage checklist and implementation handoff notes.
  - Validation: Review confirms each SDS validation requirement has at least one planned fixture and consumer-specific assertion.

- **10.5 Reindex and verify retrieval handoff.**
  - Design: Refresh Docdex for the new plan and linked docs, then verify search returns the SDS #37 plan and source docs in the aligned result set.
  - Output: Docdex index refresh evidence, retrieval evidence, queue/progress notes, repo memory, and implementation handoff note.
  - Validation: Docdex index succeeds, Docdex search for `SUB BUILD PLAN #37 Workload Classifier Phase 4` returns the new sub-build plan in the result set, and `docdexd run-tests` blocker is recorded if no test runner is configured.

## Alignment Review

- SDS #37 already has resolved open-question decisions for incomplete refs, role-scoped visibility, replay requirements, manual review versus denial, and native-app dry-run previews. No SDS correction is required for this pass beyond linking this build plan.
- The service catalog remains aligned to Phase 4 as the first build phase; this pass adds the sub-build-plan backlink.
- The master Phase 0 through Phase 13 order remains valid. This pass adds SDS #37 to the per-SDS index and tightens Phase 4 wording so Workload Classifier is explicit as the classification-fact producer before Overguard policy evaluation.
- The build-plan crosswalk remains valid. This pass adds SDS #37 to the sub-build-plan index and keeps Workload Classifier in Phase 4, with Phase 11 public-pool hardening as a later gate.
- The accepted Rust-first/native-Overrid stack remains authoritative. The plan uses Rust, Tokio, shared contracts, canonical JSON plus JSON Schema, signed envelopes, Ed25519, BLAKE3/content hashes, and native Overrid persistence/storage/secret boundaries.

## Exit Gate

SUB BUILD PLAN #37 is complete when the new plan is linked from the SDS, service catalog, master build plan, and service-catalog alignment docs; Phase 4 remains the first build point; the plan has 10 phases with well-designed Design/Output/Validation work items; queue/progress docs record the pass; local link and structure validation pass; stack guardrail scans show no accidental conventional-cloud or public-provider drift; and Docdex retrieval can find the new plan with SDS #37 context.
