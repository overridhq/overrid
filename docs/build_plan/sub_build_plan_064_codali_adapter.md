# SUB BUILD PLAN #64 - Codali Adapter

Attached SDS: [docs/sds/adapters/codali_adapter.md](../sds/adapters/codali_adapter.md)

## Purpose

This sub-build plan turns SDS #64 into an implementation sequence for Codali Adapter. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Codali Adapter is the Phase 6 product-integration adapter that turns code-agent work into explicit Overrid workload packages. It owns code-agent job intake, package manifests, authorized repository context refs, sandbox/tool boundaries, phase logs, patch/artifact/result refs, bounded repair-loop records, usage refs, audit refs, and replay bundles. It does not own Git truth, branch protection, merge authority, final model selection, repository hosting, arbitrary repository writes, raw private repo reads, policy finality, or accounting truth.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #64: Codali Adapter](../sds/adapters/codali_adapter.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Codali Adapter service plan](../service_catalog/adapters/codali_adapter.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, signed envelopes, trace ids, idempotency, stable errors, local stack, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant scopes, Overkey signing refs, Overgate ingress, Overregistry package records, Overwatch audit refs, and Overqueue command/event primitives. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies Overpack manifests, Overqueue jobs, Oversched placement, Overlease reservations, Overrun lease-bound execution, Overmeter usage facts, retries, cancellation, timeouts, and dead-letter handling. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Workload Classifier facts, Overguard policy checks, Policy Dry-Run previews, Overmesh trusted routes, deny-by-default behavior, data sensitivity, egress, package trust, and sandbox eligibility. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies usage rollups, ORU account handoffs, dispute visibility, receipts, budget refs, and the rule that the adapter emits usage but does not maintain balances. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Controls the first build point for Codali/code-agent workloads, package execution, repository context refs, logs, artifacts, structured results, repair loops, phase usage, and developer-productivity proof. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase structured job/result state, Overstore patch/log/artifact refs, Overvault secret refs and mount leases, namespace refs, retention classes, and protected artifact handling. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies hardening for sandbox escapes, unauthorized repo scope, unredacted private logs, secret access, repair-loop abuse, approval bypass, model-route misuse, artifact tampering, replay gaps, incident response, security review, and compliance controls. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #64 first build work aligned to master Phase 6 with earlier contract/execution/policy/accounting/protected-storage prerequisites and Phase 13 governance/security/compliance hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, speculative-market, pricing, revenue, customer-count, direct raw repo read, arbitrary repo write, hardcoded model/provider, or hidden unsandboxed tool-execution drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 3, 4, 5, 6, 8, and 13 | Attach SDS #64, preserve Phase 6 as first build, record prerequisites, and freeze adapter authority boundaries. |
| 2 | Master Phases 0, 1, 3, 4, 5, 6, and 8 | Define Rust contracts, canonical schemas, state machines, fixtures, signed refs, hashes, and stable errors. |
| 3 | Master Phases 0, 1, 4, 5, and 6 | Implement job intake, manifest validation, policy/budget prechecks, idempotency, and lifecycle state. |
| 4 | Master Phases 3, 4, 6, and 8 | Attach authorized repo context refs, create Overpack manifests, define sandbox/tool boundaries, and request AI Gateway routes. |
| 5 | Master Phases 3, 4, 5, 6, and 8 | Execute phases through queue, scheduler, lease, runner, logs, artifacts, structured results, usage, and failure handling. |
| 6 | Master Phase 6, with prerequisites from Phases 0, 1, 3, 4, 5, and 8 | Prove the first bounded private code-agent test-generation job as a patch proposal rather than a repository write. |
| 7 | Master Phases 4, 5, 6, and 8 | Implement output validation, bounded repair, approval gates, cancellation, and host-neutral repo approval handoff. |
| 8 | Master Phases 1, 5, 6, 8, and 13 | Implement usage, audit, replay, SDK/CLI/admin diagnostics, redaction, and artifact/log visibility. |
| 9 | Master Phase 13, with prerequisites from Phases 0 through 8 | Harden sandboxing, network, secrets, logs, repair loops, approval, retention, incident, and compliance behavior. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Codali Adapter core is a Rust service/module using shared contract crates, Tokio for bounded job/context/package/execution/repair/replay workers, and Axum/Tower/Hyper-style HTTP only where a service boundary is needed.
- Code-agent tasks, manifests, repo context refs, phase records, log refs, artifact refs, structured results, repair-loop records, usage refs, approval handoffs, replay bundles, events, fixtures, and diagnostics use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be generated later only through the shared contract layer.
- Mutating APIs require signed actor/service envelopes, tenant/org scope, repo scope, trace id, idempotency key, manifest refs, context grant refs, policy refs, budget refs, route refs where needed, schema version, stable reason codes, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for task descriptions, manifest snapshots, authorized context refs, patch refs, artifact refs, validation reports, logs, structured results, approval bundles, replay bundles, and deterministic fixtures.
- Storage, queueing, artifact refs, private records, secret refs, identity, keys, usage, policy, audit, execution, context authorization, route decisions, accounting handoffs, and diagnostics must use native Overrid service boundaries such as Overbase, Overqueue, Overstore, Overvault, Overpass, Overtenant, Overkey, Overwatch, Overmeter, Overguard, Workload Classifier, Overpack, Oversched, Overlease, Overrun, AI Gateway Router, Encrypted Docdex RAG Adapter, Docdex Adapter, SDK, CLI, and Admin and Developer UI.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, speculative assets, pricing, revenue projections, customer-count assumptions, direct raw repository reads, raw private repo content in broad logs/events/metrics/default records, arbitrary repo writes, direct Git merge authority, hidden unsandboxed tool execution, or hardcoded model/provider names the adapter boundary.

## Phase 1: SDS Attachment, Phase 6 Scope, And Adapter Boundary

### Work Items

- **1.1 Attach the build plan to SDS #64.**
  - Design: Link this document from the Codali Adapter SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/adapters/codali_adapter.md`, `docs/service_catalog/adapters/codali_adapter.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #64 returns both the Codali Adapter SDS and this sub-build plan.

- **1.2 Preserve master Phase 6 as the first build point.**
  - Design: Keep first implementation in Phase 6 because the adapter proves real product integration after contracts, control plane, private execution, policy, usage, and protected artifact prerequisites exist.
  - Output: Phase-gate note that Phases 0, 1, 3, 4, 5, and 8 supply contracts, identity, execution, policy, accounting, and protected artifact prerequisites; Phase 6 builds the adapter; Phase 13 hardens it.
  - Validation: Review proves the plan does not move Codali Adapter into Phase 3 execution primitives, Phase 8 storage, Phase 12 native apps, or Phase 13-only hardening, and does not reorder master Phase 0 through Phase 13.

- **1.3 Freeze adapter ownership boundaries.**
  - Design: Record that Codali Adapter owns task intake, manifests, repo context refs, sandbox/tool profiles, phase records, log refs, artifact refs, structured results, repair-loop records, usage refs, audit refs, and replay bundles.
  - Output: Ownership checklist for architecture, API, implementation, and review gates.
  - Validation: Review confirms the adapter does not own Git truth, branch protection, merge decisions, repo hosting, arbitrary repo writes, direct raw repo reads, final model routing, final policy authority, ORU balances, or repository-owner approval authority.

- **1.4 Carry forward resolved SDS #64 decisions.**
  - Design: Preserve the resolved first proof as a bounded private test-generation job that returns patch proposals and validation artifacts, uses `private_code_agent_test_gen_v0`, and allows at most one automatic repair attempt inside the same or stricter scope.
  - Output: Resolved-decision checklist covering test-generation proof, sandbox profile, immutable approval artifacts, repair limits, and host-neutral `repo_approval_handoff`.
  - Validation: Review rejects direct repository writes, repo-host merge authority, third repair attempts, permission widening inside repair, new secret/context access during repair, and unchecked binary/generated asset output.

- **1.5 Define upstream and downstream boundaries.**
  - Design: Record how Codali, SDK, CLI, Admin UI, AI Gateway Router, Encrypted Docdex RAG Adapter, Docdex Adapter, Overpack, Overqueue, Oversched, Overlease, Overrun, Overstore, Overbase, Overvault, Overmeter, Overwatch, Overguard, Workload Classifier, and repository-owner approval flows interact with adapter refs.
  - Output: Consumer-boundary matrix naming allowed inputs, owned outputs, denied direct authority, usage refs, audit refs, and replay requirements.
  - Validation: Review confirms downstream services keep final routing, context authorization, execution, artifact persistence, policy, usage/accounting, audit, private storage, secret, and merge/approval boundaries.

## Phase 2: Contracts, Schemas, State Machines, And Fixtures

### Work Items

- **2.1 Create the Codali Adapter Rust contract module.**
  - Design: Add contract types for code-agent task, manifest, repo context ref, phase, log ref, artifact ref, structured result, repair-loop record, usage ref, approval handoff, event payload, replay bundle, and stable errors.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, tool-permission enums, sandbox-profile enums, repair-policy enums, approval-state enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from Codali reasoning internals, Git hosting state, model routing, policy enforcement, accounting, and raw repository storage.

- **2.2 Define task and manifest schemas.**
  - Design: Model actor/tenant/org refs, repo refs, task description hash/ref, requested phase set, expected outputs, privacy mode, data class, budget/limit refs, context needs, model capability needs, tool permissions, sandbox profile, validation commands, output schema, trace id, and idempotency key.
  - Output: JSON Schema files, valid examples, invalid examples, generated validators, docs-facing examples, route-needed fixtures, policy-denied fixtures, and manifest validation fixtures.
  - Validation: Schema tests reject missing actor/tenant/repo refs, unscoped tool permissions, missing sandbox profile, direct write paths, raw secret requests, missing budget refs, absent output schema, and model/provider hardcoding.

- **2.3 Define repo context, phase, log, and artifact schemas.**
  - Design: Model Docdex index refs, context bundle refs, allowed path scopes, redaction profile, grant refs, context expiry, phase input/output refs, route refs, lease/run refs, log redaction status, artifact hashes, storage refs, validation reports, and retention classes.
  - Output: Schema set, lifecycle state machines, redacted examples, BLAKE3 hash examples, stable error catalog, and replay fixtures.
  - Validation: Tests reject context refs without grant/scope/expiry, logs without redaction status, artifacts without hashes/storage refs, phases without route/lease refs where required, and validation reports without command ids or exit status.

- **2.4 Define structured result, repair, approval, usage, and replay schemas.**
  - Design: Model summary, changed files, validation status, risk notes, follow-up refs, confidence, reviewer/approval state, repair trigger, max attempts, policy/budget checks, terminal outcome, usage dimensions, and replay decision refs.
  - Output: Structured result schema, repair-loop schema, approval-handoff schema, usage schema, replay bundle schema, sample unified diff refs, validation report examples, redacted log examples, and failure examples.
  - Validation: Tests reject repair records without max attempts and prior artifact refs, approvals without immutable manifest/result refs, usage refs without job/phase/tenant/repo refs, and replay bundles missing context/package/route/execution/artifact/repair/usage decisions.

- **2.5 Create deterministic fixtures and golden traces.**
  - Design: Build fixtures for manifest validation, repo context denied, bounded test-generation success, route unavailable, lease failure, artifact validation failure, one automatic repair, approval-required repair, cancellation, log quarantine, usage reconciliation, and replay reconstruction.
  - Output: Fixture directory, canonical inputs, expected states, expected events, stable errors, hashes, redacted projections, usage refs, audit refs, approval refs, and replay bundles.
  - Validation: Fixture tests produce stable ids, hashes, lifecycle states, reason codes, usage refs, audit refs, redacted outputs, and replay output across repeated runs.

## Phase 3: Job Intake, Manifest Validation, And Policy Prechecks

### Work Items

- **3.1 Implement job submission.**
  - Design: Add `POST /codali/jobs` with signed actor/service envelope, tenant/org scope, repo scope, task metadata, expected outputs, budget refs, privacy/data class, trace id, and idempotency key.
  - Output: Job submission handler, `code_agent_task` records, idempotent creation, initial lifecycle state, stable errors, and `codali_adapter.job_submitted` events.
  - Validation: API tests cover valid submission, duplicate idempotency, missing actor/tenant/repo refs, unsupported data class, missing budget refs, missing expected outputs, and audience-safe error responses.

- **3.2 Implement manifest dry-run validation.**
  - Design: Add `POST /codali/jobs/validate` for side-effect-free manifest checks before execution, including schema, package shape, declared tools, sandbox profile, output paths, validation commands, data class, and budget refs.
  - Output: Validation handler, validation report refs, stable `manifest_invalid`, `sandbox_profile_missing`, `tool_not_allowed`, and `approval_required` errors, plus `codali_adapter.manifest_validated` events.
  - Validation: Tests prove validation cannot queue work, create artifacts, request raw repo context, reserve leases, or emit usage beyond dry-run records.

- **3.3 Implement Workload Classifier and Overguard prechecks.**
  - Design: Attach workload/data-class facts and Overguard admission policy decisions before context retrieval, manifest packaging, routing, queueing, or execution.
  - Output: Classification adapter, policy input bundle, policy refs, denied/review-required states, dry-run preview refs, and stable reason-code mapping.
  - Validation: Tests prove policy denials happen before execution, review-required jobs cannot start without approval, and classifier facts cannot be weakened by caller metadata.

- **3.4 Implement budget, quota, and approval prechecks.**
  - Design: Validate budget refs, phase limits, context limits, model-route limits, runtime limits, repair limits, validation-command limits, and approval gates before a job reaches runnable state.
  - Output: Budget precheck record, approval-required state, limit summaries, Overwatch refs, and `policy_denied` or `approval_required` behavior.
  - Validation: Tests cover missing budget, exceeded budget, risky tool classes, new context scopes, secret requests, generated binary assets, and repair attempts requiring approval.

- **3.5 Implement job read, lifecycle, and cancellation foundations.**
  - Design: Add `GET /codali/jobs/{job_id}` and early `POST /codali/jobs/{job_id}/cancel` with lifecycle transitions, visible state, phase placeholders, partial artifact handling, and audit refs.
  - Output: Job read handler, cancellation handler, lifecycle state machine, terminal reason codes, and `codali_adapter.job_cancelled` events.
  - Validation: Tests prove invalid transitions are rejected, cancellation preserves partial refs, cancelled jobs cannot start new phases, and unauthorized callers cannot inspect private repo job data.

## Phase 4: Repository Context, Overpack Packaging, Sandbox Profiles, And Route Requests

### Work Items

- **4.1 Integrate authorized repository context refs.**
  - Design: Request repository context only through Encrypted Docdex RAG Adapter or Docdex Adapter using repo/path/purpose/data-class scopes, grant refs, leakage/redaction profiles, result caps, and expiry.
  - Output: Context-request adapter, context bundle refs, denied-context records, redaction profile refs, stable `repo_context_required` and `repo_context_denied` errors, and `codali_adapter.context_attached` events.
  - Validation: Tests prove the adapter cannot read raw repo content directly, bypass context grants, widen path scopes, suppress denied context, or retain unauthorized snippets.

- **4.2 Generate Overpack-compatible code-agent manifests.**
  - Design: Convert validated jobs and context refs into Overpack manifests with runtime/tool permissions, sandbox profile, model capability needs, input refs, output/artifact refs, validation commands, and cleanup requirements.
  - Output: Manifest builder, Overregistry package refs, package-validation handoff, BLAKE3 manifest hashes, and packaged lifecycle state.
  - Validation: Tests prove generated manifests are deterministic, schema-valid, package-validator-ready, and deny undeclared write paths, network classes, tools, secrets, and outputs.

- **4.3 Implement the minimum private sandbox profile.**
  - Design: Define `private_code_agent_test_gen_v0` for private seed or trusted nodes only, read-only repo/context inputs, ephemeral workspace writes, declared output/artifact refs, deny-by-default network, no raw secret mounts by default, command allowlist, resource/time limits, redacted logs, BLAKE3-hashed artifacts, and cleanup evidence.
  - Output: Sandbox profile schema, policy hooks, network allowlist contract, cleanup evidence refs, and invalid profile fixtures.
  - Validation: Tests reject public-node placement, raw secret mounts, undeclared network egress, undeclared tools, non-root bypass, host writes, missing cleanup evidence, and missing log redaction.

- **4.4 Integrate AI Gateway Router route requests.**
  - Design: Request model/resource routes for planning, generation, review, and repair phases through AI Gateway Router with classifier facts, context refs, capability needs, budget/latency class, privacy mode, fallback rules, usage refs, and replay refs.
  - Output: Router client adapter, route request refs, route decision refs, route unavailable behavior, fallback records, and route refs attached to phases.
  - Validation: Tests prove the adapter does not hardcode model/provider names, choose final routes, run model work without route evidence, or widen fallback privacy/tool/context constraints.

- **4.5 Validate package, context, route, and policy readiness before queueing.**
  - Design: Require manifest validation, authorized context refs, sandbox profile approval, route decisions where needed, Overguard allow/review outcomes, budget refs, and audit refs before `POST /codali/jobs/{job_id}/start` queues execution.
  - Output: Start handler, readiness checklist, queued state, stable `route_unavailable`, `policy_denied`, `sandbox_profile_missing`, and `repo_context_denied` errors.
  - Validation: Tests prove incomplete jobs cannot queue, stale context or route refs are rejected, review-required jobs wait for approval, and readiness checks are replayable.

## Phase 5: Phase Execution, Logs, Artifacts, Structured Results, And Failure Handling

### Work Items

- **5.1 Queue and schedule code-agent phases.**
  - Design: Submit phase jobs through Overqueue and Oversched with manifest refs, context refs, route refs, sandbox profile, priority, budget limits, cancellation tokens, and replay refs.
  - Output: Phase queue records, placement reason refs, queued/leased/running state transitions, and `codali_adapter.phase_started` events.
  - Validation: Integration tests prove phases cannot bypass queue/scheduler, placement respects private/trusted eligibility, and cancellation/timeout/dead-letter records preserve reason codes.

- **5.2 Execute phases through Overlease and Overrun.**
  - Design: Run phases with lease-bound execution, manifest verification, sandbox setup, allowed tools, controlled network, validation commands, result capture, cleanup, and safe termination.
  - Output: Lease/run refs, runner input/output refs, terminal state records, cleanup evidence, and `codali_adapter.phase_completed` or `codali_adapter.phase_failed` events.
  - Validation: Tests cover lease failure, sandbox setup failure, tool denial, timeout, cancelled run, cleanup failure, and terminal failure with preserved logs/artifacts.

- **5.3 Capture and redact logs.**
  - Design: Capture stdout/stderr/tool logs as Overstore refs with redaction status, retention class, policy visibility, log quarantine, and Overwatch refs.
  - Output: Log stream writer, redaction processor, quarantine state, log read API behavior for `GET /codali/jobs/{job_id}/logs`, and stable visibility reason codes.
  - Validation: Tests prove private content, secrets, unauthorized paths, private prompts, raw context snippets, and operator-only diagnostics are not exposed to unauthorized callers.

- **5.4 Capture artifacts and structured results.**
  - Design: Capture patch/diff refs, generated file refs, validation reports, test results, structured result JSON, risk notes, follow-up refs, changed-file manifests, hashes, and storage refs.
  - Output: Artifact writer, `GET /codali/jobs/{job_id}/artifacts`, structured result records, artifact-created events, and result projections for Codali, SDK, CLI, and admin UI.
  - Validation: Tests reject artifacts without hashes, manifests without declared output paths, validation reports without command ids/exit codes, and structured results missing changed files, risk notes, validation status, or approval state.

- **5.5 Implement execution failure recovery records.**
  - Design: Preserve package refs, route refs, context refs, lease/run refs, logs, artifacts, usage refs, policy refs, and reason codes for denied, retryable, terminal, cancelled, and timed-out phases.
  - Output: Failure reason catalog, retry eligibility records, terminal state records, dead-letter refs, and user/operator projections.
  - Validation: Tests prove recovery records are complete enough for replay, dispute review, budget accounting, and safe repair decisions.

## Phase 6: Bounded Private Test-Generation Proof

### Work Items

- **6.1 Implement the first Codali proof job template.**
  - Design: Define a bounded test-generation job template that reads authorized repo context, writes only to an ephemeral workspace and declared patch/artifact refs, runs declared validation commands, and returns approval artifacts.
  - Output: `codali_test_generation_v0` template, manifest fixture, policy fixture, route fixture, validation fixture, and approval-handoff fixture.
  - Validation: End-to-end test proves the job can run on a private/trusted node and produce a patch proposal without direct repository write access.

- **6.2 Implement temporary sandbox checkout behavior.**
  - Design: Build a temporary workspace from authorized repo/context refs, not direct host repository access, with read-only input refs, declared output roots, cleanup evidence, and path-scope enforcement.
  - Output: Workspace setup contract, path-scope checks, cleanup refs, artifact refs, and denied write records.
  - Validation: Tests prove agents cannot write outside declared output refs, cannot mutate the source repo, cannot retain undeclared files, and must produce cleanup evidence.

- **6.3 Implement patch proposal output.**
  - Design: Emit unified diff or git-format patch refs plus changed-file manifest, hashes, generated asset metadata where needed, risk/follow-up notes, and structured result JSON.
  - Output: Patch artifact writer, changed-file manifest, hash list, result projection, and approval-ready bundle.
  - Validation: Tests prove patches are immutable refs, paths stay within declared scopes, binary/generated assets require Overstore refs and metadata, and no unchecked host writes occur.

- **6.4 Implement validation command capture.**
  - Design: Run declared validation commands inside the sandbox with command ids, exit codes, captured output refs, JUnit/TAP or equivalent test output where available, time/resource limits, and redaction.
  - Output: Validation report schema, command runner integration, test-output refs, failure reason codes, and validation summary projection.
  - Validation: Tests cover passing validation, failing validation, timeout, tool denial, redacted output, missing declared command, and retryable artifact-shape failure.

- **6.5 Implement approval-ready handoff artifacts.**
  - Design: Assemble immutable job manifest/replay bundle, authorized context summary, structured result JSON, patch refs, changed-file manifest, validation report, redacted logs, risk notes, usage refs, and Overwatch refs.
  - Output: Approval bundle contract and first `repo_approval_handoff` record.
  - Validation: Review confirms approval sees refs and summaries only, repo hosts remain merge authorities, and no branch protection or repository-owner approval state is bypassed.

## Phase 7: Output Validation, Repair Limits, Approval Gates, And Repo Handoff

### Work Items

- **7.1 Implement artifact and result validation.**
  - Design: Validate patch shape, changed-file scopes, hash coverage, validation report completeness, log redaction, structured result fields, usage refs, and replay refs before a job can complete.
  - Output: Output validator, `artifact_validation_failed` behavior, failure state, repair eligibility record, and validation report refs.
  - Validation: Tests prove invalid artifacts block completion, missing redaction quarantines logs, missing usage refs prevent terminal completion, and validation failure is replayable.

- **7.2 Implement one automatic bounded repair attempt.**
  - Design: Allow at most one automatic repair for retryable validation or artifact-shape failures inside the same or stricter context, sandbox, route, tool, network, budget, and output scopes.
  - Output: `POST /codali/jobs/{job_id}/repair`, automatic repair policy, repair-loop records, prior artifact refs, and terminal outcome records.
  - Validation: Tests prove repair cannot widen permissions, request new secrets, change context scope, change tool/network scope, bypass validation, or exceed one automatic attempt.

- **7.3 Implement approval-required repair.**
  - Design: Require explicit user/org approval for one additional repair, and require a new signed job for permission widening, secret access, new context scope, accounting/native-app side effects, or a third attempt.
  - Output: Approval-required state, signed approval handler, repair approval refs, and stable `repair_limit_exceeded` and `approval_required` errors.
  - Validation: Tests prove approval is required before second repair, rejected repair does not run, duplicate approvals are idempotent, and new authority requires a new signed job.

- **7.4 Implement cancellation and partial-artifact preservation.**
  - Design: Allow cancellation of queued/running/repair-pending jobs while preserving manifest refs, context refs, route refs, partial logs, partial artifacts, usage refs, cleanup evidence, and terminal reason codes.
  - Output: Cancellation state transitions, partial result projections, cleanup behavior, usage reconciliation refs, and `codali_adapter.job_cancelled` events.
  - Validation: Tests prove cancellation stops new phases, preserves evidence, reconciles usage, redacts logs, and cannot delete audit evidence.

- **7.5 Implement host-neutral repo approval handoff.**
  - Design: Create `repo_approval_handoff` for patch/result refs and approval state without embedding GitHub, Forgejo/Gitea, GitLab, Bitbucket, or Azure DevOps as the adapter's core boundary.
  - Output: Handoff contract, approval-state records, integration extension points, and first approval projection.
  - Validation: Review confirms the repo host remains merge and branch-protection authority and Codali Adapter only proposes handoff refs.

## Phase 8: Usage, Audit, Replay, SDK, CLI, And Admin Diagnostics

### Work Items

- **8.1 Emit phase-level usage refs.**
  - Design: Emit usage refs for context retrieval, model routes, queue time, lease/runtime, test execution, storage, artifact transfer, retries, repair loops, cancellation, and diagnostics.
  - Output: Overmeter integration, `codali_adapter.usage_emitted` events, usage reconciliation records, and downstream Wallet/Usage Center or ORU Account Service handoff refs.
  - Validation: Tests prove usage links to tenant/org, actor, repo, job, phase, route, artifact, result, repair, and cancellation refs without maintaining balances in the adapter.

- **8.2 Emit Overwatch audit evidence.**
  - Design: Record signed audit refs for submission, validation, context attachment, packaging, route decisions, queueing, lease/run execution, artifact creation, repair, approval, cancellation, usage, and replay.
  - Output: Audit event mapping, Overwatch refs, redacted projections, and immutable decision evidence.
  - Validation: Tests prove every mutating API and terminal state has an audit ref and audit events omit raw private repo content, secrets, unredacted logs, and unauthorized prompts.

- **8.3 Implement replay API.**
  - Design: Add `GET /codali/replay/{job_id}` to reconstruct task, manifest, context, policy, route, package, queue, lease, execution, logs, artifacts, repair attempts, approvals, usage, and audit decisions.
  - Output: Replay assembler, replay bundle schema, role-scoped projections, and deterministic replay fixtures.
  - Validation: Tests prove replay output is deterministic, redacted, authorization-aware, complete enough for dispute/security review, and does not expose private content outside allowed scopes.

- **8.4 Implement SDK and CLI diagnostics.**
  - Design: Add SDK/CLI flows for submit, validate, start, cancel, inspect status, fetch logs, fetch artifacts, approve artifacts, request repair, show usage, and render replay summaries.
  - Output: Generated Rust SDK bindings, TypeScript/web bindings where needed, CLI commands, stable JSON output, error decoding, and docs-facing examples.
  - Validation: CLI/SDK tests prove signing, idempotency, trace ids, retries, stable errors, redaction, and output schemas work without direct internal API calls.

- **8.5 Implement admin/support diagnostics.**
  - Design: Add admin/operator views for job volume, phase latency, queue/lease/runtime failures, context-denial rate, repair-loop rate, artifact validation failure rate, model-route failure rate, log quarantine, and usage reconciliation.
  - Output: Admin API projections, support-safe diagnostics, alert inputs, and operator action refs.
  - Validation: Tests prove diagnostics are role-scoped, redact private repo data, distinguish policy/user/operator/system failures, and never become a privileged write backdoor.

## Phase 9: Phase 13 Security, Privacy, Sandbox, And Compliance Hardening

### Work Items

- **9.1 Threat-model code-agent blast radius.**
  - Design: Add Phase 13 threat models for arbitrary repository writes, sandbox escape, path traversal, secret exfiltration, hidden network egress, unredacted private logs, unauthorized context widening, route misuse, repair-loop abuse, and approval bypass.
  - Output: Threat model records, mitigations, tests, monitoring hooks, accepted-risk records, and remediation backlog refs.
  - Validation: Review proves every high-risk threat has a mitigation, test, monitor, or explicit accepted-risk owner.

- **9.2 Harden sandbox and network policy.**
  - Design: Enforce non-root sandbox execution, read-only input refs, declared output refs, deny-by-default network, Overrid endpoint allowlists, resource/time limits, cleanup evidence, public-node denials, and route-bound capability limits.
  - Output: Hardened sandbox profile versions, policy bundles, Overguard fixtures, route fallback limits, and public/low-trust denial tests.
  - Validation: Security tests cover sandbox escape attempts, undeclared egress, public node placement for private code, secret mount requests, host writes, symlink/path traversal, and cleanup failures.

- **9.3 Harden secrets, logs, artifacts, and retention.**
  - Design: Keep raw secrets behind Overvault refs, quarantine unredacted logs, require artifact hashes, enforce retention classes, redact unauthorized paths/snippets, and preserve deletion/tombstone/audit behavior.
  - Output: Secret-ref checks, log quarantine flows, artifact integrity checks, retention rules, tombstone refs, and audit export hooks.
  - Validation: Tests prove secrets do not appear in logs/artifacts/events, unredacted logs cannot be broadly read, artifact hashes detect tampering, and retention/deletion states remain replayable.

- **9.4 Harden repair, approval, and repository handoff.**
  - Design: Prevent repair loops from widening permission, scope, model/resource, network, context, secret, output, accounting, or native-app side-effect authority without signed new jobs or explicit approvals.
  - Output: Repair-limit policy, approval-state checks, handoff-state checks, replay refs, and operator override rules.
  - Validation: Tests reject third attempts, changed scopes, missing approval, direct branch/merge actions, hidden repository writes, and operator overrides without signed action and Overwatch evidence.

- **9.5 Harden incident, compliance, and audit operations.**
  - Design: Add incident handling for sandbox escape, unauthorized context exposure, secret leak, unredacted log exposure, artifact tampering, usage emission failure, route abuse, and policy bypass.
  - Output: Incident playbooks, alert thresholds, forensic replay bundles, compliance export projections, and remediation tracking refs.
  - Validation: Tabletop drills prove incidents preserve evidence, stop unsafe jobs, quarantine logs/artifacts, reconcile usage, notify owning services, and produce compliance-safe exports.

## Phase 10: Validation, Documentation Alignment, Queue Evidence, And Handoff

### Work Items

- **10.1 Validate SDS #64 build-breakdown coverage.**
  - Design: Map each SDS build-breakdown item to sub-build phases covering schemas, job APIs, repo context refs, Overpack manifests, AI Gateway routes, execution, logs/artifacts/results, repair, approval, cancellation, replay, usage, diagnostics, and tests.
  - Output: Coverage checklist in review notes and implementation handoff records.
  - Validation: Review proves no SDS #64 build-breakdown item is missing and the plan preserves Codali Adapter as a Phase 6 product-integration service.

- **10.2 Validate structure and work-item quality.**
  - Design: Check title prefix, attached SDS link, phase headings 1 through 10, five work items per phase, Design/Output/Validation bullets, final newline, and no tab/format drift.
  - Output: Focused validation script evidence for this file.
  - Validation: Script passes for `SUB BUILD PLAN #64`, attached SDS link, phase headings 1 through 10, 50 work items, and complete work-item structure.

- **10.3 Validate links and source alignment.**
  - Design: Check local Markdown links across this plan, the SDS, service catalog entry, master plan, crosswalk, Phase 6, Phase 13, progress docs, and queue docs.
  - Output: Link-check evidence and corrected backlinks where needed.
  - Validation: Link checker reports no missing local targets and Docdex search returns aligned SDS/service/sub-build-plan/crosswalk results.

- **10.4 Validate tech-stack guardrails.**
  - Design: Scan the changed docs for accepted Rust-first, canonical JSON/JSON Schema, signed envelope, Ed25519, BLAKE3, native Overrid boundary, no conventional database/queue/object-store/vault/KMS, no Kubernetes-first, no blockchain/NFT, no pricing/revenue/customer-count, no arbitrary repo-write, no raw repo-read, and no hardcoded model/provider drift.
  - Output: Guardrail scan evidence and any required wording fixes.
  - Validation: Scan output contains only accepted tech-stack language, native Overrid service names, or explicit non-goal/authority-boundary statements.

- **10.5 Update queue, progress, index, and handoff evidence.**
  - Design: Mark `064-build-plan` complete, update progress docs, run targeted Docdex index refresh, run retrieval checks, record the `docdexd run-tests` blocker if still present, and save repo memory.
  - Output: Updated `.codex55_sds_queue/state.json`, `.codex55_sds_queue/progress.md`, `docs/build_plan/progress.md`, Docdex index/search evidence, and implementation handoff note.
  - Validation: Queue JSON validates, next incomplete build-plan task is `065-build-plan`, Docdex search finds the new sub-build plan, and repo-wide test execution status is recorded.
