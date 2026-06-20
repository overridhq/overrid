# SUB BUILD PLAN #66 - Mcoda Adapter

Attached SDS: [SDS #66 - Mcoda Adapter](../sds/adapters/mcoda_adapter.md)

## Purpose

This sub-build plan turns SDS #66 into an implementation sequence for Mcoda Adapter. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Mcoda Adapter is the Phase 6 product-integration adapter that turns Mcoda agent tasks into ordinary Overrid workloads. It owns task intake normalization, `agent_task_manifest` records, capability snapshots, tool-boundary declarations, context-access plans, route refs, workload refs, phase records, result refs, failure refs, usage refs, audit refs, and replay bundles. It does not own Mcoda agent definitions, the agent marketplace, final model routing, scheduler placement, sandbox enforcement, final policy decisions, vault secret material, ledger truth, or dispute resolution.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #66: Mcoda Adapter](../sds/adapters/mcoda_adapter.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Mcoda Adapter service plan](../service_catalog/adapters/mcoda_adapter.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, signed envelopes, trace ids, idempotency keys, stable errors, local stack, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant scopes, Overkey signing refs, Overgate ingress, Overregistry service and capability records, Overwatch audit refs, and Overqueue command/event primitives. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies Overpack manifests, Overqueue jobs, Oversched placement, Overlease reservations, Overrun lease-bound execution, Overcell node supervision, Overmeter raw usage facts, retries, cancellation, timeouts, and dead-letter handling. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Workload Classifier facts, Overguard policy checks, Policy Dry-Run previews, deny-by-default behavior, data/tool/egress/secret eligibility, and operation review decisions. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies Overmeter usage refs, ORU/Seal Ledger/Overbill handoffs, dispute visibility, budget refs, receipt visibility, and the rule that the adapter emits usage but never maintains balances or prices. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Controls the first build point for Mcoda agent workload packaging, model/resource route refs, tool-boundary declarations, agent phases, structured results, failure reasons, usage, audit, and product proof. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase state refs, Overstore artifact/log/result refs, Overvault key/secret refs, namespace refs, retention classes, protected private records, and replay substrates. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Later consumes the safe-agent integration pattern through Personal AI Assistant, workspace, mobile, and native app tool-delegation flows without moving Mcoda Adapter's first build out of Phase 6. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies hardening for tool grants, side-effect confirmations, capability spoofing, route fallback, sandbox escape, repair abuse, redacted logs, usage reconciliation, replay gaps, incident response, security review, and compliance controls. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #66 first build work aligned to master Phase 6 with earlier contract/execution/policy/accounting/protected-storage prerequisites and Phase 13 governance/security/compliance hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, speculative-market, pricing, revenue, customer-count, hardcoded model/provider, privileged side-door, or hidden unsandboxed tool-execution drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 3, 4, 5, 6, 8, 12, and 13 | Attach SDS #66, preserve Phase 6 as first build, record prerequisites, and freeze the Mcoda Adapter authority boundary. |
| 2 | Master Phases 0, 1, 3, 4, 5, 6, and 8 | Define Rust contracts, canonical schemas, state machines, fixtures, signed refs, hashes, stable errors, and events. |
| 3 | Master Phases 1, 4, 5, and 6 | Implement task intake, manifest validation, agent capability snapshots, policy prechecks, budget checks, and lifecycle state. |
| 4 | Master Phases 3, 4, 6, and 8 | Attach context refs, tool boundaries, route requests, Overpack workload refs, and queue readiness without direct privileged execution. |
| 5 | Master Phases 3, 4, 5, 6, and 8 | Execute phases through normal queue/scheduler/lease/runner paths and capture tool decisions, logs, artifacts, results, failures, and usage. |
| 6 | Master Phase 6, with prerequisites from Phases 0, 1, 3, 4, 5, and 8 | Prove a bounded private Mcoda task through Overrid with structured result refs, tool denials/confirmations, and usage/replay evidence. |
| 7 | Master Phases 4, 5, 6, 8, and 13 | Implement grants, confirmations, retries, repair, cancellation, output validation, and bounded side-effect handling. |
| 8 | Master Phases 1, 5, 6, 8, 12, and 13 | Implement usage, audit, replay, SDK/CLI/admin diagnostics, downstream projections, and role-scoped visibility. |
| 9 | Master Phases 7, 9, and 13, with prerequisites from Phases 0 through 8 | Prepare package/grid-resident readiness and harden security, privacy, incident, retention, and compliance behavior. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Mcoda Adapter core is a Rust service/module using shared contract crates, Tokio for bounded task/route/phase/tool/replay workers, and Axum/Tower/Hyper-style HTTP only where a service boundary is needed.
- Agent manifests, capability snapshots, tool boundaries, context-access plans, route refs, workload refs, phase records, result refs, failure refs, usage refs, audit records, replay bundles, events, fixtures, and diagnostics use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be generated later only through the shared contract layer.
- Mutating APIs require signed actor/service envelopes, tenant scope, trace id, idempotency key, manifest version, policy refs, tool/data boundary refs, route refs where needed, budget refs where material, schema version, stable reason codes, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for task manifests, capability snapshots, route requests, workload refs, tool-boundary snapshots, context plans, result refs, artifact/log refs, failure refs, usage refs, approval/confirmation records, replay bundles, and deterministic fixtures.
- Storage, queueing, artifact refs, private records, secret refs, route decisions, usage, policy, audit, execution, accounting handoffs, and diagnostics must use native Overrid service boundaries such as Overbase, Overqueue, Overstore, Overvault, Overpass, Overtenant, Overkey, Overwatch, Overmeter, Overguard, Workload Classifier, Policy Dry-Run API, Overpack, Oversched, Overlease, Overrun, Overcell, AI Gateway Router, Docdex Adapter, Encrypted Docdex RAG Adapter, SDK, CLI, and Admin and Developer UI.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, speculative assets, pricing, revenue projections, customer-count assumptions, raw private prompts in broad logs/events/metrics, raw decrypted context, raw vault secrets, direct model/provider hardcoding, direct scheduler bypass, or unsandboxed tool execution the adapter boundary.

## Phase 1: SDS Attachment, Phase 6 Scope, And Adapter Boundary

### Work Items

- **1.1 Attach the build plan to SDS #66.**
  - Design: Link this document from the Mcoda Adapter SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/adapters/mcoda_adapter.md`, `docs/service_catalog/adapters/mcoda_adapter.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #66 returns both the Mcoda Adapter SDS and this sub-build plan.

- **1.2 Preserve master Phase 6 as the first build point.**
  - Design: Keep first implementation in Phase 6 because Mcoda proves real product agent execution after contracts, identity, private execution, policy, metering, and protected refs exist.
  - Output: Phase-gate note that Phases 0, 1, 3, 4, 5, and 8 supply prerequisites; Phase 6 builds the adapter proof; Phase 12 later consumes the pattern; Phase 13 hardens it.
  - Validation: Review proves the plan does not move Mcoda Adapter into Phase 3 execution primitives, Phase 8 storage, Phase 12 native apps, or Phase 13-only hardening, and does not reorder master Phase 0 through Phase 13.

- **1.3 Freeze the adapter ownership boundary.**
  - Design: Record that Mcoda Adapter owns task intake normalization, `agent_task_manifest`, capability snapshots, tool boundaries, context plans, route refs, workload refs, phase/result/failure/usage refs, audit refs, and replay bundles.
  - Output: Ownership checklist for architecture, API, implementation, and review gates.
  - Validation: Review confirms the adapter does not own Mcoda agent marketplace/runtime truth, final model selection, scheduler placement, sandbox enforcement, final policy authority, vault secrets, ORU balances, Seal Ledger truth, or final dispute resolution.

- **1.4 Carry forward resolved SDS #66 decisions.**
  - Design: Preserve the resolved decisions for authoritative fresh signed routing facts, narrow unattended grants, stable Phase 6 result envelopes, repair boundaries, and audience-classed redacted logs.
  - Output: Resolved-decision checklist covering authoritative capability facts, advisory-only ranking data, per-call confirmation classes, `agent_result_ref` base payload shapes, same-or-stricter repairs, new signed task attempts for widened scope, and audience visibility classes.
  - Validation: Review rejects stale/unsigned capability authority, broad unattended tool grants, base result envelope drift, permission-widening repair, hidden billable task attempts, and raw private prompt/context/log exposure.

- **1.5 Define upstream and downstream boundaries.**
  - Design: Record how Mcoda clients/runtimes, SDK, CLI, Admin UI, AI Gateway Router, Overpack, Overqueue, Oversched, Overlease, Overrun, Overcell, Overguard, Workload Classifier, Policy Dry-Run API, Docdex Adapter, Encrypted Docdex RAG Adapter, Overvault, Overmeter, ORU Account Service, Seal Ledger, Overbill, Overwatch, and Overclaim interact through refs.
  - Output: Consumer-boundary matrix naming allowed inputs, owned outputs, denied direct authority, usage refs, audit refs, and replay requirements.
  - Validation: Review confirms downstream services keep final routing, context authorization, scheduling, sandbox enforcement, policy, usage/accounting, protected storage, secret, audit, and dispute boundaries.

## Phase 2: Contracts, Schemas, State Machines, And Fixtures

### Work Items

- **2.1 Create the Mcoda Adapter Rust contract module.**
  - Design: Add contract types for agent task manifests, agent capability snapshots, tool boundaries, context plans, route requests, workload refs, phase records, result refs, failure refs, usage refs, events, and replay bundles.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, tool-class enums, side-effect-class enums, route-state enums, repair-policy enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from Mcoda runtime internals, AI Gateway route authority, policy enforcement, scheduler placement, sandbox enforcement, vault secret material, and accounting internals.

- **2.2 Define task, capability, tool-boundary, and context schemas.**
  - Design: Model actor/tenant refs, Mcoda task refs, requested agent refs, task type, prompt/input refs, output schema, data/tool classes, route hints, resource hints, timeouts, retry limits, capability snapshots, allowed/denied tools, confirmation rules, and context access plans.
  - Output: JSON Schema files, valid examples, invalid examples, generated validators, docs-facing examples, route-needed fixtures, stale-capability fixtures, missing-boundary fixtures, and policy-denied fixtures.
  - Validation: Schema tests reject missing actor/tenant refs, missing tool boundaries, stale capability snapshots, raw secret requests, broad undeclared context, model/provider hardcoding, missing budgets where material, and unscoped side-effect authority.

- **2.3 Define route, workload, phase, result, failure, and usage schemas.**
  - Design: Model AI Gateway route requests, selected route refs, fallback rules, Overpack manifest refs, queue/lease/runner refs, sandbox profile refs, phase input/output refs, tool-call refs, structured results, artifact/log refs, failure reasons, and usage dimensions.
  - Output: Schema set, lifecycle state machines, redacted examples, BLAKE3 hash examples, stable error catalog, usage fixtures, failure fixtures, and replay fixtures.
  - Validation: Tests reject route refs without policy/capability facts, workload refs without Overpack/queue linkage, phases without status/reason codes, results without schema validation, logs without redaction status, and usage refs without tenant/task/phase dimensions.

- **2.4 Define events, stable errors, and replay bundles.**
  - Design: Model task validation, rejection, creation, boundary attachment, route requests, workload submission, phase start/completion, tool-call decisions, task completion/failure/cancellation, usage emission, and replay assembly.
  - Output: Event schemas, stable error enums, role-scoped replay schema, deterministic event-order rules, and redacted projection examples.
  - Validation: Tests prove events carry refs and reason codes, omit raw private prompts/decrypted context/vault secrets/private repo contents/unredacted logs, and can reconstruct effective boundaries from versioned refs.

- **2.5 Create deterministic fixtures and golden traces.**
  - Design: Build fixtures for valid task, invalid manifest, missing tool boundary, stale capability, policy denial, route unavailable, tool grant required, tool denied, successful result, failed result validation, retry, repair, cancellation, usage reconciliation, and replay.
  - Output: Fixture directory, canonical inputs, expected states, expected events, stable errors, hashes, redacted projections, usage refs, audit refs, and replay bundles.
  - Validation: Fixture tests produce stable ids, hashes, lifecycle states, reason codes, usage refs, audit refs, redacted outputs, and replay output across repeated runs.

## Phase 3: Task Intake, Manifest Validation, Capability Facts, And Policy Prechecks

### Work Items

- **3.1 Implement side-effect-free validation.**
  - Design: Add `POST /adapters/mcoda/tasks/validate` for manifest, output schema, capability snapshot, tool-boundary, context-plan, budget, route-precheck, and data-class validation without submitting work.
  - Output: Validation handler, validation report refs, missing-prerequisite refs, stable `manifest_invalid`, `tool_boundary_missing`, `context_not_authorized`, `agent_capability_stale`, `policy_denied`, and `budget_precheck_failed` behavior.
  - Validation: Tests prove validation cannot queue work, reserve leases, call tools, mutate grants, materialize private context, create artifacts, or emit execution usage beyond dry-run records.

- **3.2 Implement task creation.**
  - Design: Add `POST /adapters/mcoda/tasks` with signed actor/service envelope, tenant scope, Mcoda task ref, requested agent ref, expected output schema, trace id, idempotency key, capability refs, tool/data boundaries, and policy refs.
  - Output: Task creation handler, `agent_task_manifest` records, idempotent creation, initial lifecycle state, queue-readiness refs, and `mcoda_adapter.task_created` events.
  - Validation: API tests cover valid creation, duplicate idempotency, missing tenant/actor refs, stale capability, missing output schema, missing boundary refs, broad tool request, and role-safe error projections.

- **3.3 Integrate Workload Classifier, Overguard, and Policy Dry-Run.**
  - Design: Attach workload class, data-class facts, tool classes, side-effect classes, trust/locality facts, budget prechecks, and dry-run policy explanations before route selection or queueing.
  - Output: Classification adapter, policy input bundle, policy refs, dry-run refs, denied/review-required states, and stable reason-code mapping.
  - Validation: Tests prove policy denials happen before execution, review-required tasks cannot start without approval, classifier facts cannot be weakened by caller metadata, and dry-run remains side-effect-free.

- **3.4 Implement budget, quota, and confirmation prechecks.**
  - Design: Validate budget refs, route limits, phase limits, context limits, tool-call limits, retry/repair limits, unattended-grant eligibility, confirmation requirements, and tenant quota before a task becomes runnable.
  - Output: Budget precheck records, confirmation-required states, grant-required states, limit summaries, Overwatch refs, and stable `approval_required` or `budget_precheck_failed` behavior.
  - Validation: Tests cover missing budget, exceeded budget, broad shell/network/file/repo/vault/wallet/native-app side-effect requests, unattended grant denial, and confirmation-required task holds.

- **3.5 Implement task read, lifecycle, and early cancellation foundations.**
  - Design: Add `GET /adapters/mcoda/tasks/{task_id}` and early `POST /adapters/mcoda/tasks/{task_id}/cancel` with lifecycle transitions, phase placeholders, partial refs, terminal reason codes, and audit refs.
  - Output: Task read handler, cancellation handler, lifecycle state machine, terminal reason codes, role-scoped projections, and `mcoda_adapter.task_cancelled` events.
  - Validation: Tests prove invalid transitions are rejected, cancellation preserves refs and usage where material, cancelled tasks cannot start new phases, and unauthorized callers cannot inspect private task/context/log data.

## Phase 4: Context Access, Tool Boundaries, Route Requests, And Workload Packaging

### Work Items

- **4.1 Integrate context-access plans.**
  - Design: Request repo, Docdex, encrypted RAG, workspace/app, vault, and external-resource refs only through owning services with scope, grant, redaction profile, leakage profile, expiry, and result caps.
  - Output: Context-plan validator, context-request adapter, authorized context refs, denied-context records, leakage/redaction refs, and stable `context_not_authorized` behavior.
  - Validation: Tests prove the adapter cannot read private stores directly, bypass context grants, widen scopes, suppress denied context, retain unauthorized snippets, or expose raw decrypted context in events/logs.

- **4.2 Implement tool-boundary declarations and grant attachment.**
  - Design: Add `POST /adapters/mcoda/tasks/{task_id}/tool-grants` to attach, narrow, revoke, or require confirmation for tool classes with side-effect scope, tenant/actor scope, resource caps, policy refs, and replay refs.
  - Output: Tool-boundary records, grant records, denied-tool records, confirmation-required states, revocation behavior, and `mcoda_adapter.tool_boundary_attached` events.
  - Validation: Tests prove side-effecting tools require explicit confirmation or narrow durable grants, revoked grants block calls, broad grants are denied, and tool boundaries cannot widen during fallback or repair without new authority.

- **4.3 Integrate AI Gateway Router route requests.**
  - Design: Request model/resource routes with fresh signed capability facts, workload/data/tool classes, context refs, budget/latency class, trust/locality class, compatible sandbox profile, fallback policy, and replay refs.
  - Output: Router client adapter, route request refs, route decision refs, route unavailable behavior, fallback records, and route refs attached to task/phase records.
  - Validation: Tests prove the adapter does not hardcode model/provider names, treat advisory ranking as authority, run model work without route evidence, or widen fallback privacy/tool/context constraints.

- **4.4 Generate Overpack-compatible agent workload refs.**
  - Design: Convert validated tasks, context refs, tool boundaries, route decisions, sandbox profile, expected outputs, resource hints, retry policy, and cleanup requirements into Overpack-ready manifests.
  - Output: Manifest builder, package-validation handoff, Overpack manifest refs, Overqueue-ready records, BLAKE3 manifest hashes, and packaged lifecycle state.
  - Validation: Tests prove generated manifests are deterministic, schema-valid, package-validator-ready, and deny undeclared write paths, network classes, tools, secrets, context scopes, side effects, and outputs.

- **4.5 Validate readiness before start.**
  - Design: Require manifest validation, authorized context refs, tool boundaries, confirmation state, route decisions where needed, Overguard allow/review outcomes, budget refs, and audit refs before `POST /adapters/mcoda/tasks/{task_id}/start` queues execution.
  - Output: Start handler, readiness checklist, queued state, stable `route_unavailable`, `policy_denied`, `sandbox_profile_missing`, `tool_boundary_missing`, and `context_not_authorized` errors.
  - Validation: Tests prove incomplete tasks cannot queue, stale context/route/capability refs are rejected, review-required tasks wait for approval, and readiness checks are replayable.

## Phase 5: Agent Phase Execution, Tool Calls, Results, Failures, And Usage

### Work Items

- **5.1 Queue and schedule agent phases.**
  - Design: Submit phase work through Overqueue and Oversched with manifest refs, context refs, route refs, sandbox profile, priority, budget limits, cancellation tokens, tool boundaries, and replay refs.
  - Output: Phase queue records, placement reason refs, queued/leased/running transitions, and `mcoda_adapter.phase_started` events.
  - Validation: Integration tests prove phases cannot bypass queue/scheduler, placement respects private/trusted eligibility and policy facts, and cancellation/timeout/dead-letter records preserve reason codes.

- **5.2 Execute phases through Overlease, Overrun, and Overcell.**
  - Design: Run phases with lease-bound execution, manifest verification, sandbox setup, allowed tools, controlled network, result capture, cleanup, timeout enforcement, and safe termination.
  - Output: Lease/run refs, runner input/output refs, terminal state records, cleanup evidence, and `mcoda_adapter.phase_completed` or `mcoda_adapter.task_failed` events.
  - Validation: Tests cover lease failure, sandbox setup failure, route failure, tool denial, timeout, cancelled run, cleanup failure, and terminal failure with preserved logs/artifacts/usage refs.

- **5.3 Supervise tool-call lifecycle.**
  - Design: Track requested, policy-checked, grant-required, approved, executing, succeeded, denied, and failed tool-call states with policy refs, confirmation refs, grant refs, side-effect class, output refs, and usage refs.
  - Output: Tool-call records, denial records, confirmation records, execution refs, stable failure reasons, and `mcoda_adapter.tool_call_requested` / `mcoda_adapter.tool_call_denied` events.
  - Validation: Tests prove tool calls cannot execute before policy/confirmation/grant checks, denied calls preserve evidence, and tool outputs stay bounded by declared result/artifact refs.

- **5.4 Capture results, logs, artifacts, and failures.**
  - Design: Capture `agent_result_ref`, artifact refs, log refs, validation status, selected route/capability refs, redaction profile, checksum/hash refs, failure refs, retryability, and user-visible summaries.
  - Output: Result writer, log/artifact refs, failure reason catalog, result validation handler, result/failure APIs, and terminal task projections.
  - Validation: Tests reject results without schema version, task/phase ids, route/capability refs, validation status, redaction profile, hashes, usage refs, or replay refs; logs omit raw private prompts and decrypted context.

- **5.5 Emit phase and tool usage refs.**
  - Design: Emit usage refs for validation, classification, route selection, queue wait, lease/runtime, model/resource execution, tool calls, retries, repair loops, storage, bandwidth, result validation, cleanup, denials, and cancellations.
  - Output: Overmeter integration, `mcoda_adapter.usage_emitted` events, usage reconciliation records, and downstream Wallet/Usage Center, ORU Account Service, Seal Ledger, and Overbill handoff refs.
  - Validation: Tests prove usage links tenant, actor, Mcoda task ref, agent ref, route ref, workload ref, phase id, tool-call id, result ref, retry/repair/cancellation refs, and receipts without maintaining balances in the adapter.

## Phase 6: Bounded Phase 6 Mcoda Product Proof

### Work Items

- **6.1 Implement the first bounded Mcoda proof task template.**
  - Design: Define a Phase 6 proof task that uses a signed task manifest, accepted capability snapshot, declared tool boundaries, bounded context refs, AI Gateway route refs, Overpack workload refs, and structured output schema.
  - Output: `mcoda_agent_task_v0` proof template, manifest fixture, capability fixture, policy fixture, route fixture, usage fixture, and replay fixture.
  - Validation: End-to-end test proves a Mcoda task can run on private/seed rails and return a structured result ref without direct internal API calls or privileged side-door execution.

- **6.2 Prove route selection without model/provider hardcoding.**
  - Design: Use AI Gateway Router and accepted capability facts to select a model/resource route while treating rating, historical success, cost hints, latency observations, and free-form metadata as advisory only.
  - Output: Route proof record, route decision refs, fallback policy refs, stale-capability denial fixture, and replay bundle.
  - Validation: Tests prove stale/missing capability facts deny or hold routing, advisory facts cannot override policy/privacy/budget/trust constraints, and fallback cannot widen tool/data/model/network/filesystem permissions.

- **6.3 Prove tool-denial and confirmation behavior.**
  - Design: Run proof cases for allowed read-only inspection, missing grant, per-call confirmation, denied side-effecting tool, revoked grant, and deterministic adapter-owned cleanup.
  - Output: Tool decision proof records, confirmation refs, denied-tool refs, cleanup refs, usage refs, audit refs, and replay bundle.
  - Validation: Tests prove shell/code execution outside an accepted Overpack manifest, network widening, repo/file writes, vault access, wallet/grant changes, messaging/native-app side effects, external publication, and policy overrides require explicit authority.

- **6.4 Prove result envelope and payload shapes.**
  - Design: Return stable `agent_result_ref` envelopes for `text_summary`, `json_object`, `artifact_set`, `tool_proposal`, and `failure_report` payloads without changing the base envelope.
  - Output: Result proof records, payload fixtures, validation reports, redacted logs, artifact refs, usage refs, and replay bundle.
  - Validation: Tests prove richer multimodal, streaming, multi-agent, or native-app outputs attach as typed extension refs and cannot bypass result validation or redaction rules.

- **6.5 Prove SDK, CLI, and admin experience.**
  - Design: Exercise SDK/CLI/admin flows for validate, create, start, status, cancel, tool grants, retry, results, usage, replay, degraded capability, route unavailable, policy denied, and result validation failed states.
  - Output: Product proof checklist, CLI JSON examples, admin projection examples, degraded-state examples, and support-safe diagnostics.
  - Validation: Review confirms product users can submit tasks, inspect state, retrieve refs, see usage, debug failures, and replay decisions without manually calling internal APIs or violating stack guardrails.

## Phase 7: Grants, Confirmations, Retry, Repair, Cancellation, And Output Validation

### Work Items

- **7.1 Implement result and artifact validation gates.**
  - Design: Validate result envelope fields, payload schema, artifact hashes, log redaction, failure reason refs, usage refs, route/capability refs, and replay refs before a task can complete.
  - Output: Output validator, `result_validation_failed` behavior, failure state, repair eligibility record, and validation report refs.
  - Validation: Tests prove invalid results block completion, missing redaction quarantines logs, missing usage refs prevent terminal completion, and validation failure is replayable.

- **7.2 Implement narrow unattended grant handling.**
  - Design: Allow durable unattended grants only for narrow, reversible, non-destructive tool classes that are fully declared, time-bounded, actor/tenant scoped, resource capped, and replayable.
  - Output: Grant eligibility policy, grant records, expiration/revocation behavior, denied broad-grant records, and Overwatch refs.
  - Validation: Tests prove unattended grants cannot cover shell/code execution outside accepted manifests, network egress widening, repo/file writes, vault access, wallet/accounting/grant changes, messaging/native-app side effects, deletion outside adapter-owned temporary state, policy override, or other-service state changes.

- **7.3 Implement confirmation gates for side effects.**
  - Design: Require per-call confirmation for side-effecting tool classes, permission widening, operator overrides, external publication, and actions that change another service's authoritative state.
  - Output: Confirmation-required state, signed confirmation handler, confirmation refs, denial behavior, duplicate confirmation idempotency, and replay refs.
  - Validation: Tests prove missing, stale, duplicate, wrong-actor, or wrong-scope confirmations block execution and preserve evidence without executing the tool call.

- **7.4 Implement retry and repair boundaries.**
  - Design: Allow repair inside the same task only for retryable output-shape, validation, route-attempt, or transient execution failures under the same or stricter actor, tenant, context, data-class, tool, sandbox, route/fallback, budget, timeout, and output scopes.
  - Output: `POST /adapters/mcoda/tasks/{task_id}/retry`, repair-loop records, route/attempt refs, incremental usage refs, and new-task-required reason codes for widened scope.
  - Validation: Tests prove retries that widen permissions, add context/tools, change side-effect class, use less-local/higher-risk route, exceed automatic repair count, follow terminal policy denial, or change objective require a new signed billable task attempt.

- **7.5 Implement cancellation and partial-evidence preservation.**
  - Design: Allow cancellation across queued, leased, running, waiting-for-tool, waiting-for-confirmation, result-validating, retryable-failed, and repair-pending states while preserving refs and usage.
  - Output: Cancellation state transitions, partial result projections, cleanup behavior, usage reconciliation refs, and `mcoda_adapter.task_cancelled` events.
  - Validation: Tests prove cancellation stops new phases where possible, records whether queue/lease/runner/completed state was reached, preserves evidence, reconciles usage, redacts logs, and cannot delete audit evidence.

## Phase 8: Usage, Audit, Replay, SDK, CLI, Admin Diagnostics, And Downstream Handoffs

### Work Items

- **8.1 Implement replay API.**
  - Design: Add `GET /adapters/mcoda/tasks/{task_id}/replay` to reconstruct validation, policy, capability, route, tool, execution, result, failure, cancellation, usage, and audit decisions from versioned refs.
  - Output: Replay assembler, replay bundle schema, role-scoped projections, pagination for linked records, and deterministic replay fixtures.
  - Validation: Tests prove replay is deterministic, redacted, authorization-aware, complete enough for dispute/security/support review, and does not expose private content outside allowed scopes.

- **8.2 Emit Overwatch audit evidence.**
  - Design: Record signed audit refs for validation, creation, grant attachment, route requests, workload submission, phase starts, tool decisions, result capture, retries, repairs, cancellation, usage, and replay reads.
  - Output: Audit event mapping, Overwatch refs, redacted projections, and immutable decision evidence.
  - Validation: Tests prove every mutating API and terminal state has an audit ref and audit events omit raw private prompts, decrypted context, vault secrets, private repo contents, provider secrets, other-tenant data, and unredacted native-app payloads.

- **8.3 Implement role-scoped logs and diagnostics.**
  - Design: Provide separate projections for users, operators, Mcoda developers, support staff, and dispute reviewers with audience-appropriate reason codes, redactions, hashes, usage refs, and replay links.
  - Output: Projection matrix, admin/support diagnostics, redacted log readers, route/capability mismatch summaries, and alert inputs.
  - Validation: Tests prove no audience gets raw private prompts, decrypted RAG context, private repo contents, vault secrets, exact hidden paths beyond authorized classes, other-tenant data, provider secrets, or unredacted native-app payloads through the adapter.

- **8.4 Implement SDK and CLI commands.**
  - Design: Add SDK/CLI flows for validate, create, start, inspect status, attach/narrow/revoke grants, approve confirmations, retry, cancel, fetch results, show usage, and render replay summaries.
  - Output: Generated Rust SDK bindings, TypeScript/web bindings where required for clients, CLI commands, stable JSON output, error decoding, and docs-facing examples.
  - Validation: CLI/SDK tests prove signing, tenant scope, idempotency, trace ids, retries, stable errors, redaction, output schemas, and no direct internal API calls.

- **8.5 Implement downstream handoff projections.**
  - Design: Provide safe refs for Personal AI Assistant, Codali Adapter, Docdex Adapter, Encrypted Docdex RAG Adapter, Wallet/Usage Center, native app tool-delegation flows, Overclaim disputes, and central AI stewardship review queues.
  - Output: Handoff contract, role-scoped projections, denied/degraded refs, usage refs, audit refs, and replay refs.
  - Validation: Tests prove downstream consumers cannot bypass final context authorization, final model/resource routing, policy, tool confirmations, usage/accounting, replay, or native-app permission boundaries.

## Phase 9: Grid-Resident Readiness, Security, Privacy, Incident, And Compliance Hardening

### Work Items

- **9.1 Prepare grid-resident package readiness.**
  - Design: Define later Phase 7/9 readiness for managed or grid-resident Mcoda Adapter packages after system-service workload class, package validation, backup/restore, release strategy, and failover are proven.
  - Output: Package readiness checklist, Overpack manifest requirements, backup/restore refs, migration requirements, health/readiness commands, rollout/rollback requirements, and blocked-until gates.
  - Validation: Review proves Phase 6 does not pretend Mcoda Adapter is already a grid-resident system service and later package promotion uses Phase 7/9 gates without changing adapter APIs.

- **9.2 Harden capability, route, and fallback integrity.**
  - Design: Threat-model stale capability snapshots, spoofed agent/provider identity, quota/health drift, advisory ranking abuse, cost/latency hint manipulation, route fallback widening, and less-local route escalation.
  - Output: Threat model records, mitigation checklist, route policy gates, fallback tests, monitoring hooks, and accepted-risk records.
  - Validation: Security tests prove stale or missing facts fail closed, advisory metadata cannot override hard constraints, and fallback cannot widen privacy/tool/context/budget/trust/locality constraints.

- **9.3 Harden tool, sandbox, and side-effect controls.**
  - Design: Threat-model tool-boundary bypass, side-effect confirmation bypass, shell/network/file/repo/vault/wallet/native-app escalation, sandbox escape, undeclared egress, cleanup failure, and adapter-owned temporary-state deletion abuse.
  - Output: Tool hardening checklist, sandbox policy fixtures, confirmation fixtures, cleanup evidence, alert inputs, and incident hooks.
  - Validation: Security tests cover sandbox escape attempts, undeclared egress, raw secret access, host writes, unapproved side effects, tool grant tampering, cleanup failures, and policy/operator overrides without signed action and Overwatch evidence.

- **9.4 Harden privacy, logs, retention, and replay.**
  - Design: Enforce redaction, role-scoped diagnostics, retention classes, tombstones where applicable, log quarantine, usage reconciliation, audit completeness, and replay availability without reopening private content.
  - Output: Redaction profiles, retention rules, log quarantine flows, support-safe exports, compliance projections, and replay gap alerts.
  - Validation: Tests prove raw private prompts, decrypted RAG context, private repo contents, vault secrets, provider secrets, exact hidden paths, other-tenant data, and unredacted native-app payloads are absent from broad logs/events/metrics/exports.

- **9.5 Harden incident and compliance response.**
  - Design: Add incident playbooks for capability spoofing, route abuse, tool-boundary bypass, side-effect misuse, sandbox escape, private context exposure, unredacted log exposure, usage emission failure, replay gaps, and policy bypass.
  - Output: Incident playbooks, alert thresholds, freeze/quarantine behavior, affected-tenant/user notification refs, correction/retraction workflows, compliance export projections, and remediation tracking refs.
  - Validation: Drills prove incidents preserve evidence, stop unsafe jobs, quarantine logs/artifacts, reconcile usage, notify owning services/users where policy requires, and produce compliance-safe reports.

## Phase 10: Validation, Documentation Alignment, Queue State, And Handoff

### Work Items

- **10.1 Validate SDS #66 build-breakdown coverage.**
  - Design: Map each SDS build-breakdown item to sub-build phases covering manifest/tool/result schemas, validate/create/start/status/cancel/results/usage/replay APIs, policy prechecks, route selection, Overpack/Overqueue submission, phase/tool/result/failure/usage capture, UI/CLI visibility, retries, cancellation, redaction, migration, and hardening.
  - Output: Coverage checklist in review notes and implementation handoff records.
  - Validation: Review proves no SDS #66 build-breakdown item is missing and the plan preserves Mcoda Adapter as a Phase 6 product-integration service.

- **10.2 Validate structure and work-item quality.**
  - Design: Check title prefix, attached SDS link, phase headings 1 through 10, five work items per phase, Design/Output/Validation bullets, final newline, and no tab/format drift.
  - Output: Focused validation script evidence for this file.
  - Validation: Script passes for `SUB BUILD PLAN #66`, attached SDS link, phase headings 1 through 10, 50 work items, and complete work-item structure.

- **10.3 Validate links and source alignment.**
  - Design: Check local Markdown links across this plan, the SDS, service catalog entry, master plan, crosswalk, Phase 6, Phase 13, progress docs, and queue docs.
  - Output: Link-check evidence and corrected backlinks where needed.
  - Validation: Link checker reports no missing local targets and Docdex search returns aligned SDS/service/sub-build-plan/crosswalk results.

- **10.4 Validate tech-stack guardrails.**
  - Design: Scan the changed docs for accepted Rust-first, canonical JSON/JSON Schema, signed envelope, Ed25519, BLAKE3, native Overrid boundary, no conventional database/queue/object-store/vault/KMS, no Kubernetes-first, no blockchain/NFT, no pricing/revenue/customer-count, no hardcoded model/provider, no privileged side-door, and no hidden unsandboxed tool-execution drift.
  - Output: Guardrail scan evidence and any required wording fixes.
  - Validation: Scan output contains only accepted tech-stack language, native Overrid service names, or explicit non-goal/authority-boundary statements.

- **10.5 Update queue, progress, index, and handoff evidence.**
  - Design: Mark `066-build-plan` complete, update progress docs, run targeted Docdex index refresh, run retrieval checks, record the `docdexd run-tests` blocker if still present, and save repo memory.
  - Output: Updated `.codex55_sds_queue/state.json`, `.codex55_sds_queue/progress.md`, `docs/build_plan/progress.md`, Docdex index/search evidence, and implementation handoff note.
  - Validation: Queue JSON validates, next incomplete build-plan task is `067-build-plan`, Docdex search finds the new sub-build plan with SDS/service backlinks, and repo-wide test execution status is recorded.

## Alignment Review

- SDS #66 already aligns with master Phase 6 as the first build point for Mcoda agent workload translation, route refs, tool boundaries, phase records, structured results, failure reasons, usage refs, audit refs, and replay bundles.
- The SDS correctly separates Mcoda Adapter from Mcoda agent marketplace/runtime ownership, AI Gateway Router route authority, Overguard policy authority, Overpack/Oversched/Overrun execution authority, Overvault secret authority, and accounting/dispute truth.
- Phase 6 wording needed expansion so Mcoda integration names capability snapshots, context plans, route refs, tool grants, phase records, result refs, repair/retry, usage, and replay rather than only generic agent execution.
- Phase 13 wording needed expansion so Mcoda Adapter receives explicit threat-model and security-review coverage alongside Docdex, Codali, classifier, and Personal AI hardening.
- The master Phase 0 through Phase 13 order remains unchanged. Required alignment updates are backlinks/index rows for SDS #66, richer Phase 6 and Phase 13 wording, service catalog sub-build-plan linkage, queue/progress evidence, and Docdex index/search refresh.
