# SUB BUILD PLAN #77 - Incident Response Service

Attached SDS: [SDS #77 - Incident Response Service](../sds/governance_ops/incident_response_service.md)

## Purpose

This sub-build plan turns SDS #77 into an implementation sequence for Incident Response Service. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Incident Response Service is the Phase 13 incident case-coordination layer for declared and suspected incidents, affected-scope snapshots, severity, role assignments, timelines, containment requests, recovery steps, communications, drills, post-incident reports, follow-up actions, and replay bundles. It does not replace Overwatch as the evidence log, directly execute containment, mutate owner-service state, adjudicate disputes or refunds, decide fraud or payout finality, expose raw private evidence, or become generic project management.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #77: Incident Response Service](../sds/governance_ops/incident_response_service.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, states, policy/security, metering, operations, failure modes, validation, build breakdown, downstream handoffs, and resolved open-question decisions. |
| [Incident Response Service plan](../service_catalog/governance_ops/incident_response_service.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, signed command envelopes, trace ids, idempotency keys, stable errors, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant scopes, Overkey signing refs, Overgate ingress, Overregistry service records, Overwatch audit/event refs, and Overqueue command/event primitives. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy decisions, Overclaim dispute refs, Fraud Control, Challenge Task, Oververify evidence, reason codes, deny-by-default behavior, and replayable decisions. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies Overmeter usage refs, ORU/Seal Ledger/Overbill truth, Overgrant grant refs, Provider Payout refs, Overclaim dispute refs, and the rule that Incident Response records accounting-impact refs without mutating accounting truth. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Supplies AI Gateway Router, Personal AI Assistant, encrypted RAG, adapter, SDK, CLI, admin, and mSwarm Runtime Bridge groundwork used by AI-related incidents and operator/client surfaces. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies failover, backup/restore, system-service packaging, recovery drills, route-shift evidence, backbone cutover evidence, and founder-hardware migration prerequisites. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase records, Overstore artifact/evidence refs, Overvault sensitive/private refs, Universal Namespace refs, retention/deletion substrates, and replay storage boundaries. |
| [Phase 9: Overpack Deployment Platform](phase_09_overpack_deployment_platform.md) | Supplies release, rollback, deployment plan, package validation, and migration evidence used by operational incidents and post-incident follow-up. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Supplies federation template, purpose tag, public-interest pool, stewardship, and grant/public-interest context for incident scope and reporting. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public-provider onboarding, public sandbox limits, fraud/reputation/challenge refs, payout holds, and public-provider abuse incident inputs. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Supplies native app, wallet, assistant, search, messaging, social, maps, workspace, mobile, and stewardship-interface incident consumers. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Controls the first build point for Incident Response Service, including governance, security review, threat modeling, incident hardening, drills, reporting, migration, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #77 first build work aligned to master Phase 13 with earlier phases as prerequisites and with simple incident seed records earlier in Overwatch. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services and contracts, Tokio where bounded workers are needed, Axum/Tower/Hyper-style HTTP where a service boundary exists, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, TypeScript for client surfaces only, native Overrid service boundaries, and no conventional database/object-store/vault/queue/compliance SaaS product boundary, Kubernetes-first architecture, blockchain, NFT, hidden enforcement service, hardcoded pricing, revenue, or customer-count assumptions. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 7, 8, 12, and 13 | Attach SDS #77, preserve Phase 13 as first build, record prerequisites, and freeze Incident Response authority. |
| 2 | Master Phases 0, 1, 4, 5, 7, 8, and 13 | Define Rust contracts, incident schemas, severity taxonomy, state machines, stable errors, and deterministic fixtures. |
| 3 | Master Phases 0, 1, 4, and 13 | Implement declaration, promotion, severity, scope, role, and timeline APIs with signed command envelopes and audit refs. |
| 4 | Master Phases 1, 4, 7, 8, 11, 12, and 13 | Integrate Overwatch evidence, auto-declaration policy, evidence refs, timeline correction, and replayable case history. |
| 5 | Master Phases 1, 4, 5, 7, 9, 11, and 13 | Implement containment requests through Overguard and owning services without direct route, tenant, payout, billing, vault, restore, or policy mutation. |
| 6 | Master Phases 5, 7, 8, 9, 12, and 13 | Implement recovery step tracking, verification refs, drill evidence, founder-hardware migration gates, and operational recovery handoffs. |
| 7 | Master Phases 4, 5, 8, 10, 11, 12, and 13 | Implement audience-classed communications, redaction review, correction/retraction, public reporting handoff, and affected-party status. |
| 8 | Master Phases 4, 5, 10, 12, and 13 | Implement post-incident reports, follow-up actions, stewardship, PIP, compliance, Central AI, and owner-service handoffs. |
| 9 | Master Phase 13 with evidence from Phases 0 through 12 | Implement operational metrics, threat/security review gates, reliability drills, scale hardening, and public summary checks. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, authority boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Incident Response Service uses Rust-first shared contracts and service-facing APIs for incident cases, affected scopes, timelines, roles, containment requests, recovery steps, communications, drills, reports, follow-up actions, and replay bundles. TypeScript is acceptable only for generated client/admin/stewardship surfaces and must call Overrid APIs without becoming an incident authority.
- Contracts, incident records, timeline entries, state machines, severity classes, communication projections, report manifests, stable errors, replay bundles, and deterministic fixtures use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf may be generated later only through the shared contract layer.
- Mutating APIs require signed actor/service envelopes, tenant/system scope, trace id, idempotency key, role/steward refs, evidence refs, policy refs, schema version, stable errors, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for evidence bundles, communication/report manifests, replay bundles, fixture inputs, and deterministic comparison artifacts. BLAKE3 must not be described as encryption.
- Structured incident state, evidence refs, sensitive records, private content refs, audit timelines, owner-service refs, usage refs, and replay must use native Overrid service boundaries such as Overwatch, Overguard, Overclaim, Fraud Control, Challenge Task, Oververify, Failover and Recovery Coordinator, Backup and Restore Service, Migration Tooling, Release Strategy Service, Overbill, ORU Account Service, Seal Ledger, Provider Payout Service, Overgrant, Compliance Boundary Service, Stewardship Reporting Service, PIP Registry, Central AI Service, Overbase, Overstore, Overvault, SDK, CLI, and Admin/Developer UI.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Elasticsearch, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, a compliance/incident SaaS, Kubernetes-first orchestration, blockchain, NFTs, external legal-advice systems, external payment custody, hardcoded pricing, revenue forecasts, customer-count assumptions, raw private-data exports, raw child-safety evidence, payment credentials, private user content, encrypted Docdex context, direct containment execution, accounting mutation, vault mutation, tenant mutation, policy finality, dispute finality, payout approval, or public-report publication the Incident Response authority.

## Phase 1: SDS Attachment, Phase 13 Scope, And Incident Authority Boundary

### Work Items

- **1.1 Attach the build plan to SDS #77.**
  - Design: Link this document from the Incident Response SDS, service plan, master build plan, Phase 13 plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/governance_ops/incident_response_service.md`, `docs/service_catalog/governance_ops/incident_response_service.md`, `docs/build_plan/master_plan.md`, `docs/build_plan/phase_13_governance_compliance_scale_hardening.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #77 returns both the Incident Response Service SDS and this sub-build plan.

- **1.2 Preserve master Phase 13 as the first build point.**
  - Design: Keep first implementation in Phase 13 because mature incident coordination depends on identity, tenancy, audit, policy, accounting refs, recovery tooling, storage/vault refs, native apps, public-provider controls, reporting, and compliance prerequisites from earlier phases.
  - Output: Phase-gate note that simple incident seed records may exist earlier in Overwatch, while managed cases, containment coordination, communications, reports, drills, and replay hardening are Phase 13 work.
  - Validation: Review proves the plan does not move Incident Response into Phase 1 Overwatch ownership, Phase 4 policy enforcement, Phase 5 accounting mutation, Phase 7 direct recovery execution, Phase 8 vault/storage ownership, Phase 11 public-provider authority, or Phase 12 native-app ownership.

- **1.3 Freeze the Incident Response ownership boundary.**
  - Design: Record that Incident Response owns incident cases, severity, affected-scope snapshots, role assignments, timelines, containment request records, recovery step records, communication records, drill reports, post-incident reports, follow-up actions, and replay bundles.
  - Output: Ownership checklist for architecture, API, implementation, operations, governance, and review gates.
  - Validation: Review confirms the service does not own Overwatch event storage, Overguard policy finality, owning-service containment execution, route truth, restore execution, billing/ledger/payout mutation, dispute finality, vault contents, tenant identity, or public-report publication authority.

- **1.4 Carry forward resolved SDS #77 decisions.**
  - Design: Preserve auto-declare rules, severity model, seed-hardware containment limits, public-reporting thresholds, and founder-hardware exit drill requirements as explicit planning constraints.
  - Output: Resolved-decision checklist covering Overwatch trusted auto-declarations, steward/human confirmation gates, `sev_0` through `sev_4`, reversible expiry-bound seed containment, public report redaction, and Phase 7 drill evidence for founder-hardware exit.
  - Validation: Review rejects ambiguous fraud/security/compliance/child-safety/public-report/sanction/finality cases without confirmation, irreversible automation on seed hardware, unredacted public artifacts, and founder-hardware exit without required drill evidence.

- **1.5 Define upstream and downstream service boundaries.**
  - Design: Record how Overwatch, Overguard, Overclaim, Fraud Control, Challenge Task, Oververify, Failover and Recovery Coordinator, Backup and Restore, Migration Tooling, Release Strategy, Overbill, Seal Ledger, ORU Account, Provider Payout, Overgrant, Compliance Boundary, Stewardship Reporting, PIP Registry, Central AI, native apps, SDK, CLI, and Admin/Developer UI interact through refs.
  - Output: Boundary matrix naming allowed reads, owned writes, required refs, denied direct mutations, redaction classes, evidence refs, usage refs, audit refs, replay refs, and owner-service finality.
  - Validation: Review confirms Incident Response exchanges signed refs/events/requests and never copies private internals or grants itself mutation authority owned by another service.

## Phase 2: Contracts, Incident Schemas, Severity Taxonomy, State Machines, And Fixtures

### Work Items

- **2.1 Create the Incident Response Rust contract module.**
  - Design: Add contract types for incident cases, affected-scope snapshots, timeline entries, role assignments, containment requests, recovery steps, communication records, incident drills, post-incident reports, follow-up actions, replay bundles, stable errors, and lifecycle states.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, severity/type/action/redaction enums, lifecycle enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from event-log storage, policy enforcement, recovery execution, accounting mutation, vault access, and dispute/payout finality.

- **2.2 Define incident case and affected-scope schemas.**
  - Design: Model `incident_case` and `affected_scope_snapshot` with incident id, type, severity, priority, state, originating refs, affected tenants/users/providers/apps/services, data classes, confidence, scope snapshots, commander refs, and audit refs.
  - Output: JSON Schema files, valid examples, invalid examples, stable error mapping, and deterministic fixtures for operational, fraud, public-provider, accounting, privacy, security, compliance, native-app, and AI-route incidents.
  - Validation: Schema tests reject missing evidence refs, missing affected scope, unbounded private details, unversioned schemas, missing severity, missing tenant/system scope, missing data class, missing trace id, and missing audit refs.

- **2.3 Define severity, incident type, and lifecycle taxonomies.**
  - Design: Model one shared severity class set: `sev_0_emergency`, `sev_1_severe`, `sev_2_significant`, `sev_3_minor`, and `sev_4_info_or_drill`, with classification inputs from impact, service criticality, data class, evidence confidence, scope size, reversibility, accounting/compliance exposure, public visibility, and recurrence risk.
  - Output: Severity taxonomy, incident type registry, lifecycle state schemas, downgrade/upgrade rules, state transition matrix, and examples for operational, security, privacy, compliance, ledger, vault, public-provider, central-AI, and public-report cases.
  - Validation: Tests prove severe security, privacy, compliance, ledger, vault, public-provider, central-AI, or public-report cases cannot be downgraded only because availability is healthy, and operational outages cannot be inflated by speculation without evidence refs.

- **2.4 Define timeline, role, containment, recovery, communication, report, follow-up, and replay schemas.**
  - Design: Model timeline entries, role assignments, containment requests, recovery steps, communication records, drill records, post-incident reports, follow-up actions, and replay bundles with source refs, actor refs, policy refs, redaction class, idempotency, correction refs, and replay refs.
  - Output: Schema set, stable error map, HTTP/API mapping, audience-classed projection schemas, deterministic examples, and golden replay fixtures.
  - Validation: Tests prove outputs are deterministic from stored refs and cannot include raw vault secrets, payment credentials, private user content, encrypted Docdex context, child-safety evidence, fraud heuristics, exploit details, or private topology.

- **2.5 Create deterministic fixtures and golden traces.**
  - Design: Build fixtures for declaration, promotion, severity change, scope correction, role handoff, timeline entry, Overwatch seed promotion, containment request, recovery step, communication publication/correction/retraction, drill completion, post-incident report, follow-up closure, and replay.
  - Output: Fixture directory, canonical inputs, expected states, expected events, expected stable errors, BLAKE3/content hashes, usage refs, audit refs, redacted projections, and replay bundles.
  - Validation: Fixture tests produce stable ids, hashes, lifecycle states, reason codes, redaction behavior, usage refs, audit refs, and replay output across repeated runs.

## Phase 3: Declaration, Promotion, Severity, Scope, Roles, And Timeline APIs

### Work Items

- **3.1 Implement incident declaration and seed promotion APIs.**
  - Design: Accept incident declarations from authorized operators, Overwatch, owning services, Central AI, or Fraud Control only with evidence refs, type, initial scope hints, severity hints, actor/service identity, trace id, idempotency key, and audit refs.
  - Output: `POST /incidents`, Overwatch seed promotion flow, `suspected`/`declared`/`triaging` states, stable errors, audit events, and usage refs.
  - Validation: Tests reject declarations without evidence, identity, scope hints, stable type, idempotency key, or audit refs; duplicate idempotency keys return the original result.

- **3.2 Implement severity and priority changes.**
  - Design: Require signer authority, evidence refs, classification inputs, current severity, proposed severity, reason codes, and redaction class for severity or priority updates.
  - Output: `POST /incidents/{incident_id}/severity`, severity-change records, timeline entries, downgrade/upgrade guards, and Overwatch audit refs.
  - Validation: Tests prove severity changes create timeline entries, cannot erase prior severity, require evidence, and enforce domain-specific downgrade/upgrade gates from the resolved SDS decisions.

- **3.3 Implement affected-scope snapshots and corrections.**
  - Design: Store each scope update as a time-bound snapshot with affected refs, confidence, data class, impacted services/apps/providers/tenants/users, source refs, correction refs, and superseded-by metadata.
  - Output: `POST /incidents/{incident_id}/scope`, snapshot records, corrected-scope links, public/private projections, and stable errors for missing or incompatible scope.
  - Validation: Tests prove wrong scope is corrected by appending a new snapshot rather than rewriting history, and public projections exclude private affected-party details.

- **3.4 Implement role assignment and handoff APIs.**
  - Design: Assign commander, technical lead, communications owner, evidence owner, recovery owner, security reviewer, compliance reviewer, and post-incident owner with authority boundary, backup owner, start/end, escalation path, and audit refs.
  - Output: `POST /incidents/{incident_id}/roles`, role assignment records, handoff events, missing-role alerts, and closure blockers for unowned required roles.
  - Validation: Tests prove severe incidents cannot proceed without required role assignments and closure fails while required owners or follow-up owners are missing.

- **3.5 Implement timeline append and correction APIs.**
  - Design: Append timeline entries for evidence, decisions, containment requests, recovery work, communications, owner-service responses, report steps, and corrections with source refs and redaction class.
  - Output: `POST /incidents/{incident_id}/timeline`, append-only timeline records, correction refs, audience-safe projections, and replay links.
  - Validation: Tests prove timeline history is append-only, corrections link to prior entries, redaction classes gate broad views, and replay reconstructs timeline order deterministically.

## Phase 4: Overwatch Evidence Intake, Auto-Declaration Rules, Timeline Integrity, And Replay

### Work Items

- **4.1 Implement Overwatch evidence intake.**
  - Design: Consume Overwatch incident seed records, traces, health events, audit refs, evidence bundles, append failures, reconciliation status, and export refs without copying raw private payloads.
  - Output: Evidence intake adapter, seed-link records, source-ref validation, evidence freshness checks, and degraded-state diagnostics.
  - Validation: Tests prove Incident Response stores refs, summaries, hashes, and redaction profiles only, and blocks closure when required Overwatch evidence reconciliation is pending.

- **4.2 Implement objective auto-declaration policy.**
  - Design: Allow Overwatch to auto-declare `suspected` or `triaging` cases only for objective, source-trusted signals such as service/route outage, queue backlog, failed backup/restore/failover drill, evidence-integrity failure, policy-denial spike, stale system-service health, redaction escape, public-provider abuse spike, payout-risk spike, challenge failure burst, or public report publication error.
  - Output: Auto-declaration rule registry, source-trust checks, severity hints, audit events, and steward/operator review requirements for ambiguous or sensitive signals.
  - Validation: Tests prove ambiguous fraud/abuse signals, security exploit findings, compliance or child-safety issues, public-report allegations, sanctions, finality actions, regulated-workload cases, or first external communications require human or steward confirmation.

- **4.3 Implement evidence integrity and freshness gates.**
  - Design: Validate source refs, bundle hashes, timestamp windows, actor/service refs, policy refs, and owner-service response refs before incident operations that depend on evidence.
  - Output: Evidence integrity worker, freshness diagnostics, stable errors, stale-evidence timeline entries, and replay input manifests.
  - Validation: Tests prove stale, missing, forged, mismatched, or unauthorized evidence refs block declaration promotion, containment requests, public communication, report publication, or closure where applicable.

- **4.4 Implement replay bundle generation.**
  - Design: Reconstruct incident case, scope snapshots, timeline entries, policy decisions, containment requests, owner-service responses, recovery refs, communications, reports, and follow-up actions from stored refs and versioned schemas.
  - Output: `GET /incidents/{incident_id}/replay`, replay bundle schema, redaction snapshots, source refs, generated hashes, and deterministic replay fixtures.
  - Validation: Tests prove replay reconstructs decisions deterministically and excludes unauthorized private payloads, raw exploit details, fraud heuristics, secret-bearing refs, or private AI context.

- **4.5 Implement local spool fallback for Overwatch degradation.**
  - Design: When Overwatch is degraded, store bounded local incident operation spools with strict expiry, integrity hashes, reconciliation status, and closure blockers.
  - Output: Local spool schema, reconciliation worker, degraded-mode alerts, replay reconciliation refs, and closure-blocking diagnostics.
  - Validation: Tests simulate Overwatch outage and prove incident work remains bounded, reconciles later, and cannot close until evidence is linked back to Overwatch.

## Phase 5: Containment Requests Through Overguard And Owning Services

### Work Items

- **5.1 Implement containment request records.**
  - Design: Accept containment requests only with incident id, requested action, target owning service, evidence refs, policy refs, reason codes, expiry, rollback refs, idempotency key, and downstream response expectations.
  - Output: `POST /incidents/{incident_id}/containment-requests`, draft/policy_pending/requested/accepted/denied/applied/failed/rolled_back/expired states, audit events, and usage refs.
  - Validation: Tests reject containment without owner service, evidence, policy refs, expiry where required, rollback refs where applicable, or idempotency key.

- **5.2 Implement Overguard authorization handoff.**
  - Design: Send route shift, admission pause, throttle, block, quarantine, challenge, payout hold recommendation, rollback, restore, or recovery requests through Overguard policy checks before owner-service dispatch.
  - Output: Overguard handoff envelope, policy decision refs, reason-code mapping, denied/allowed/review-required states, and replay links.
  - Validation: Tests prove Incident Response cannot directly allow/deny execution or bypass Overguard for policy-sensitive containment.

- **5.3 Implement owning-service dispatch and response refs.**
  - Design: Dispatch approved requests to owning services such as Oversched/Overmesh, Failover and Recovery, Backup/Restore, Migration Tooling, Release Strategy, Fraud Control, Overclaim, Overbill, Provider Payout, Overvault, Overtenant, native apps, or Central AI, then record accepted/denied/completed/failed refs.
  - Output: Owner-service dispatch refs, retry state, downstream response records, failure refs, rollback refs, and timeline entries.
  - Validation: Tests prove owner-service denial is preserved with reason codes and Incident Response never edits route, tenant, vault, billing, payout, ledger, dispute, restore, or policy truth directly.

- **5.4 Implement seed-hardware containment limits.**
  - Design: During seed hardware operation, allow automation to request only reversible, expiry-bound, policy-checked actions such as fresh evidence capture, health rechecks, audit spool reconciliation, temporary lane pause, route drain, stateless worker restart/replacement, artifact quarantine by hash, known-bad package block, challenge request, public-pool throttle, or pre-finality hold recommendation.
  - Output: Seed automation allowlist, human/steward review denylist, expiry defaults, escalation records, and audit refs.
  - Validation: Tests reject automation for tenant/provider suspension, permanent eligibility change, payout release/reversal, refund/correction finality, secret access, data deletion/retention waiver, stateful restore/promotion, broad policy change, public report publication, or sensitive evidence containment.

- **5.5 Implement containment rollback and expiry handling.**
  - Design: Track expiry, rollback request refs, owner-service rollback responses, unresolved containment alerts, and stale request escalation.
  - Output: Expiry worker, rollback timeline entries, owner-service reconciliation refs, stale-request alerts, and closure blockers.
  - Validation: Tests prove expired containment fails closed, rollback refs are recorded, unresolved severe containment blocks closure, and replay shows the exact request/response sequence.

## Phase 6: Recovery Steps, Verification, Operational Drills, And Founder-Hardware Exit Evidence

### Work Items

- **6.1 Implement recovery step tracking.**
  - Design: Record recovery action, owner service, expected result, verification refs, retry policy, rollback refs, final outcome, monitoring window, and state.
  - Output: `POST /incidents/{incident_id}/recovery-steps`, recovery records, verification gates, monitoring states, and usage/audit refs.
  - Validation: Tests prove recovery verification is required before resolution and failed verification reopens `recovering` or creates a linked incident.

- **6.2 Integrate Failover and Recovery, Backup/Restore, Migration, and Release Strategy refs.**
  - Design: Link operational recovery records to failover, backup restore, migration, release, rollback, package validation, route-shift, queue recovery, and Overwatch reconciliation evidence.
  - Output: Recovery integration refs, owner-service response mapping, drill evidence manifests, and replay fixtures.
  - Validation: Tests prove Incident Response records recovery refs and does not execute failover, restore, migration, rollback, route shift, or release actions directly.

- **6.3 Implement drill scheduling and drill records.**
  - Design: Support incident drills for node failure, provider abuse, payment outage, control-plane outage, queue backlog, restore failure, AI route failure, public report correction, redaction failure, owner-service outage, Overguard outage, Overwatch outage, and replay mismatch.
  - Output: `POST /incident-drills`, drill lifecycle, scenario templates, safety bounds, evidence refs, findings, follow-up refs, and report summaries.
  - Validation: Tests prove drills run in simulation or controlled mode without production side effects and always produce expected behavior, actual behavior, evidence, and follow-up work.

- **6.4 Implement founder-hardware exit evidence gates.**
  - Design: Track two consecutive full-backbone cutover drills, at least one planned and one failure-injection, plus per-critical-service backup restore, failover, rollback, queue recovery, Overwatch reconciliation, and route-shift drills with no unresolved `sev_0` or `sev_1` follow-up actions.
  - Output: Founder-hardware exit checklist, migration drill records, emergency-only transition refs, monthly restore sample records, quarterly full-backbone drill schedule, and blocker diagnostics.
  - Validation: Tests prove founder hardware cannot leave the normal production path until the required Phase 7 recovery evidence is complete and unresolved severe follow-up actions are closed.

- **6.5 Implement recovery observability and stale-step escalation.**
  - Design: Track time to contain, recover, verify, monitor, close, and complete follow-up; alert on stale recovery steps, missing owner responses, failed verification, and missing monitoring windows.
  - Output: Metrics schema, alert rules, dashboard refs, escalation records, and runbook notes.
  - Validation: Drills prove stale recovery and failed verification alerts fire and create incident timeline entries without exposing private evidence.

## Phase 7: Communications, Redaction, Affected-Party Status, And Public Reporting Handoff

### Work Items

- **7.1 Implement communication records and lifecycle.**
  - Design: Model draft, redaction_review, approved, published, corrected, retracted, and archived communications with audience, channel, template refs, approval refs, redaction profile, sent state, correction refs, and receipt refs.
  - Output: `POST /incidents/{incident_id}/communications`, communication records, approval workflow, audience projections, and audit refs.
  - Validation: Tests prove communication publication requires authorized role, evidence refs, redaction profile, and approval refs where required.

- **7.2 Implement public/private redaction profiles.**
  - Design: Separate internal, affected-party, steward, auditor, and public views using Compliance Boundary, security-review, and Stewardship Reporting redaction profiles.
  - Output: Redaction schema, projection APIs, public-safe fixtures, affected-party fixtures, and failed-redaction stable errors.
  - Validation: Tests prove public communications never expose exploit details, private user data, internal topology, fraud heuristics, sensitive compliance facts, vault secrets, payment credentials, encrypted Docdex context, or child-safety evidence.

- **7.3 Implement public reporting thresholds and handoff.**
  - Design: Require public reporting for `sev_0_emergency`, normally require it for `sev_1_severe` after containment and redaction, and require `sev_2_significant` public handling when ecosystem trust or broad affected-party visibility requires it.
  - Output: Public-report eligibility evaluator, Stewardship Reporting handoff refs, Compliance Boundary redaction refs, security-review refs, and publication blocker diagnostics.
  - Validation: Tests prove public artifacts are produced through Stewardship Reporting and redaction controls, never from raw incident timelines.

- **7.4 Implement affected-party and owner-service status views.**
  - Design: Provide authorized incident status, scope summary, next action, communication state, recovery state, and correction notices to affected users, tenants, providers, organizations, grantees, native app owners, and owner services.
  - Output: Audience-specific read projections, status refs, correction/retraction records, access-denied stable errors, and audit refs.
  - Validation: Tests prove affected-party views are scoped to that party and do not leak unrelated-party evidence, private topology, fraud heuristics, or sensitive compliance details.

- **7.5 Implement correction, retraction, and privacy-review triggers.**
  - Design: When communication redaction is wrong or public facts change, append correction/retraction records, trigger privacy/security review, update status projections, and preserve history.
  - Output: Correction/retraction APIs, review tasks, timeline entries, Stewardship Reporting handoff refs, and replay fixtures.
  - Validation: Tests prove corrections create new linked entries rather than rewriting history and privacy-impacting mistakes open follow-up actions.

## Phase 8: Post-Incident Reports, Follow-Up Actions, Stewardship, PIP, And Central AI Handoffs

### Work Items

- **8.1 Implement post-incident report records.**
  - Design: Create post-incident reports with incident refs, timeline summary, root-cause refs, contributing factors, impact refs, containment/recovery refs, corrective actions, redaction profile, publication refs, correction refs, and closure refs.
  - Output: `POST /incidents/{incident_id}/post-incident-reports`, report lifecycle, review records, publication handoff refs, and replay fixtures.
  - Validation: Tests reject reports missing root-cause refs where required, containment/recovery refs, corrective actions, redaction profile, review refs, or audit refs.

- **8.2 Implement follow-up action tracking.**
  - Design: Track action owner, target service, priority, due window, linked incident, verification requirement, state, escalation path, completion evidence, and recurrence-prevention refs.
  - Output: Follow-up action API, owner notifications, overdue alerts, Stewardship Reporting status refs, and PIP candidate refs.
  - Validation: Tests prove incident closure is denied while required follow-up ownership is missing or severe follow-up blockers remain unresolved.

- **8.3 Implement Stewardship Reporting and PIP Registry handoffs.**
  - Design: Provide redacted incident summaries, public/private report refs, correction/supersession notices, aggregate incident metrics, lessons learned, and protocol/process improvement candidates to governance consumers.
  - Output: Stewardship Reporting handoff contract, PIP Registry handoff refs, public summary fixtures, correction/withdrawal refs, and replay links.
  - Validation: Tests prove governance consumers receive redacted facts only and cannot mutate incident records, owner-service records, policy decisions, reports, or PIPs through the handoff.

- **8.4 Implement Central AI evidence-bounded analysis handoffs.**
  - Design: Allow Central AI and Central AI Stewardship Interface to analyze incident summaries, evidence refs, follow-up patterns, and recommendation refs without opaque enforcement or private evidence leakage.
  - Output: AI analysis request refs, recommendation refs, review action refs, appeal/correction refs, redaction profiles, and replay records.
  - Validation: Tests prove severe AI-related or fraud-related recommendations require evidence refs, human/steward review where required, and appeal/correction paths.

- **8.5 Implement report and follow-up usage/audit hooks.**
  - Design: Emit usage refs and audit refs for investigation work, evidence package generation, containment coordination, recovery work, drills, report generation, communications, replay, and follow-up verification.
  - Output: Usage class registry, Overmeter handoff events, Overwatch audit events, pending reconciliation markers, and diagnostics.
  - Validation: Tests prove every material incident operation emits or reconciles usage refs without hardcoded prices, balances, invoices, resource rates, revenue forecasts, or customer-count assumptions.

## Phase 9: Operations, Threat Review, Security Review, Reliability Drills, And Scale Hardening

### Work Items

- **9.1 Implement operational metrics and alerting.**
  - Design: Track open incidents by severity, service, type, age, owner, containment state, recovery state, communication state, report state, follow-up state, time to declare, time to assign, time to contain, time to recover, time to communicate, and time to close.
  - Output: Metrics schema, alert rules, dashboard refs, degraded-state summaries, runbook docs, and owner-service escalation refs.
  - Validation: Tests and drills prove alerts fire for incidents without commander, severe incidents without containment decision, stale severity, missing affected scope, missing communication owner, unresolved public-provider abuse spikes, and overdue follow-up actions.

- **9.2 Implement threat-model gates.**
  - Design: Add threat-model entries for forged incident declarations, evidence-ref spoofing, severity downgrade abuse, timeline rewrite attempts, owner-service handoff confusion, containment overreach, public communication leakage, report redaction failure, drill side effects, Central AI opaque enforcement, private evidence overexposure, and replay tampering.
  - Output: Threat model checklist, mitigation mapping, monitoring refs, accepted-risk records, remediation issue templates, and validation fixtures.
  - Validation: Review proves each threat has mitigation, tests, monitoring, or explicit accepted risk before broad release.

- **9.3 Implement security review gates.**
  - Design: Review incident declaration access, role authority, severity changes, containment request authorization, owner-service dispatch, communication/report publication, redaction policy, replay access, evidence-ref access, local spool reconciliation, and closure authority.
  - Output: Security review checklist, reviewer refs, remediation records, release blockers, and post-review evidence.
  - Validation: Release cannot pass if declaration, severity change, containment request, public communication, report publication, or closure bypasses signed role/steward controls.

- **9.4 Implement reliability and incident-response drills.**
  - Design: Run drills for node failure, provider abuse spike, payment outage, control-plane outage, queue backlog, restore failure, route failure, AI route failure, public-report correction, Overwatch degradation, Overguard outage, owner-service outage, redaction failure, replay mismatch, and follow-up overdue escalation.
  - Output: Drill scenarios, expected behavior, actual behavior, evidence bundles, incident hooks, remediation refs, and report summaries.
  - Validation: Drills prove safe failure modes: keep cases open, block unsafe containment, retain requests for retry, restrict views, open follow-up actions, and preserve evidence/version refs.

- **9.5 Implement governance reporting and public summaries.**
  - Design: Produce aggregate/redacted reports for incident volume, severity mix, response times, containment classes, recovery outcomes, communication corrections, drill outcomes, follow-up closure, public-provider abuse, fraud/accounting-impact trends, AI-related incidents, privacy/security/compliance incidents, usage reconciliation, and replay health.
  - Output: Reporting schema, redaction profiles, public-safe summaries, source refs, governance handoff refs, and report replay refs.
  - Validation: Tests prove reports are specific enough for trust while excluding raw private data, payment credentials, child-safety evidence, secret refs, encrypted Docdex context, fraud heuristics, exploit details, and unrelated-party data.

## Phase 10: Validation, Link Alignment, Queue, Index, And Handoff Readiness

### Work Items

- **10.1 Validate sub-build plan structure.**
  - Design: Check title prefix, attached SDS link, ten phase headings numbered 1 through 10, five work items per phase, and Design/Output/Validation fields.
  - Output: Structure validation evidence for `docs/build_plan/sub_build_plan_077_incident_response_service.md`.
  - Validation: Scripted checks pass for phase count, work-item count, numbering, and required fields.

- **10.2 Validate cross-document alignment.**
  - Design: Confirm SDS, service catalog entry, master plan, crosswalk, Phase 13, progress doc, and tech-stack guardrails all agree that Incident Response is Phase 13-first with simple incident seed records earlier in Overwatch and earlier phases as prerequisites.
  - Output: Alignment checklist and updated backlinks across changed docs.
  - Validation: Local Markdown link checks pass and review finds no mismatch with master Phase 0 through Phase 13 order.

- **10.3 Validate stack and authority guardrails.**
  - Design: Scan changed docs for prohibited external product boundaries, conventional database/object-store/vault/queue/compliance-SaaS authority drift, direct containment execution, public-report publication authority, accounting mutation, vault mutation, tenant mutation, policy finality, dispute finality, payout approval, blockchain/NFT language, pricing/revenue/customer-count assumptions, and BLAKE3-as-encryption wording.
  - Output: Guardrail scan evidence and corrected wording where needed.
  - Validation: Matches are either absent or explicit negative-control lines rejecting the prohibited assumptions.

- **10.4 Validate Docdex retrieval, impact, and index state.**
  - Design: Use Docdex impact, symbols, diagnostics, search, DAG export, and targeted index refresh for the new plan and linked docs.
  - Output: Impact evidence, symbols/Markdown structure evidence, search result evidence, DAG export evidence, and updated index stats.
  - Validation: Docdex search for SDS #77 returns the new sub-build plan and backlinks; impact diagnostics remain empty; targeted index refresh succeeds.

- **10.5 Validate implementation handoff readiness.**
  - Design: Update queue/progress evidence and confirm builders can start with contracts, case APIs, Overwatch intake, containment requests, recovery tracking, communications, reports, follow-up actions, operations, threat/security reviews, drills, and handoff gates.
  - Output: Queue/progress update, blocker notes, validation command notes, and handoff summary.
  - Validation: `docdexd hook pre-commit --repo /Users/bekirdag/Documents/apps/overrid` passes; `docdexd run-tests --repo /Users/bekirdag/Documents/apps/overrid` result is recorded, including the known missing test-runner blocker if unchanged.
