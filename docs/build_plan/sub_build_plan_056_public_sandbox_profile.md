# SUB BUILD PLAN #56 - Public Sandbox Profile

Attached SDS: [docs/sds/federation_public/public_sandbox_profile.md](../sds/federation_public/public_sandbox_profile.md)

## Purpose

This sub-build plan turns SDS #56 into an implementation sequence for Public Sandbox Profile. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Public Sandbox Profile is the Phase 11 safety contract that keeps unknown or semi-trusted public nodes limited to public low-sensitivity work. It owns sandbox profile versions, restriction sets, workload/data-class bindings, secret and private mount denials, output validation rules, artifact quarantine, log redaction profiles, replayable evaluations, stable reason codes, deprecation, supersession, and emergency disablement. It does not onboard providers, schedule jobs, run workloads, verify trust, decide fraud, decide payouts, or weaken runner enforcement boundaries.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #56: Public Sandbox Profile](../sds/federation_public/public_sandbox_profile.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Public Sandbox Profile plan](../service_catalog/federation_public/public_sandbox_profile.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, signed envelopes, trace ids, idempotency, stable errors, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant scopes, Overkey signing refs, Overgate ingress, Overregistry service facts, Overwatch audit refs, and Overqueue command/event primitives. |
| [Phase 2: Seed Private Swarm](phase_02_seed_private_swarm.md) | Supplies Overcell node-agent facts, Node Installer version evidence, Hardware Discovery facts, and Benchmark Runner evidence used before public-node sandbox compatibility is trusted. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies Overpack, Oversched, Overlease, Overrun, and Overmeter execution facts that Public Sandbox Profile later constrains for public low-sensitivity work. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Workload Classifier facts, Overguard policy decisions, Policy Dry-Run previews, Oververify evidence, Challenge Task refs, Overclaim correction paths, Overmesh routes, and Overcache trust-scope refs. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies usage, accounting, payout-hold, provider-payout, and correction refs without Public Sandbox Profile mutating balances, invoices, payouts, or ledger entries. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Supplies trusted federation and public-interest context that remains separate from unknown public-provider sandbox enforcement. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Controls the first build point: public sandbox requirements, public-provider gating, anti-Sybil/fraud/challenge controls, payout holds, throttles, and bounded low-sensitivity public capacity. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies public reporting, compliance retention, threat review, incident response, audit export, redaction review, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #56 first build work aligned to master Phase 11, with earlier phases as prerequisites and Phase 13 as later hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, speculative-market, pricing, revenue, customer-count, or external-provider-as-core-boundary drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 2, 3, 4, 5, 10, 11, and 13 | Attach SDS #56, preserve Phase 11 as first build, freeze authority boundaries, and identify prerequisite owner-service gates. |
| 2 | Master Phases 0, 1, 4, and 11 | Define Rust contracts, canonical schemas, lifecycle states, reason codes, redaction profiles, signed refs, and deterministic fixtures. |
| 3 | Master Phases 1, 4, and 11 | Implement profile creation, validation, activation, read, deprecation, supersession, and emergency-disable lifecycle APIs. |
| 4 | Master Phases 3, 4, 8, and 11 | Implement restriction sets, workload/data-class bindings, secret/private/regulated/system-service denials, egress classes, and public low-sensitivity caps. |
| 5 | Master Phases 3, 4, 10, and 11 | Implement evaluation APIs and Overguard/Oversched prechecks using classifier, provider, profile, policy, and candidate-node refs. |
| 6 | Master Phases 2, 3, 4, 8, and 11 | Implement Overrun/Overcell enforcement handoff contracts and reject assignments when local enforcement cannot satisfy the selected profile. |
| 7 | Master Phases 3, 4, 5, 8, 11, and 13 | Implement output validation, artifact quarantine, log redaction, quarantine review, usage refs, and audit refs. |
| 8 | Master Phases 4, 7, 8, 11, and 13 | Implement replay, profile deprecation, emergency disablement, in-flight cancellation/quarantine, supersession, and rollout safety. |
| 9 | Master Phases 3, 4, 5, 10, 11, and 13 | Prove the first public sandbox profile and define fraud, challenge, payout-hold, public-interest, and governance expansion gates. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, authority boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Public Sandbox Profile core is a Rust service/module using shared contract crates, Tokio for bounded evaluation/rollout/quarantine workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Profile versions, restriction sets, workload/data-class bindings, denials, output validation rule sets, quarantine records, log redaction profiles, evaluation records, replay bundles, events, fixtures, and audit exports use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be generated later only through the shared contract layer.
- Mutating endpoints require signed service or operator envelopes, tenant/system scope, trace id, idempotency key, policy refs, classifier refs, provider refs, profile version, evidence refs, schema version, stable reason codes, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for profile snapshots, restriction-set snapshots, evaluation fact bundles, output validation bundles, quarantine records, redacted summaries, replay bundles, audit exports, and deterministic fixtures.
- Public Sandbox Profile may point to Public Provider Onboarding, Workload Classifier, Overguard, Oversched, Overlease, Overrun, Overcell, Overpack, Overstore, Overvault, Overwatch, Fraud Control Service, Challenge Task Service, Provider Payout Service, SDK, CLI, admin UI, and provider-facing status surfaces, but it must not become the owner of those services' truth.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, speculative assets, pricing, revenue projections, customer-count assumptions, raw secret storage, private workload hosting, regulated workload hosting, system-service hosting, provider onboarding, scheduling, execution, trust verification, fraud adjudication, payout mutation, or external-provider products the platform boundary.

## Phase 1: SDS Attachment, Phase 11 Scope, And Authority Boundaries

### Work Items

- **1.1 Attach the build plan to SDS #56.**
  - Design: Link this document from the Public Sandbox Profile SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/federation_public/public_sandbox_profile.md`, `docs/service_catalog/federation_public/public_sandbox_profile.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #56 returns both the Public Sandbox Profile SDS and this sub-build plan.

- **1.2 Preserve master Phase 11 as the first build point.**
  - Design: Keep first implementation in Phase 11 because unknown public nodes require strict no-secret, no-private-data, no-regulated-data, no-system-service, egress, output-validation, quarantine, and runner-enforcement guarantees.
  - Output: Phase-gate note that earlier phases are prerequisites, Phase 10 remains trusted federation/public-interest context, Phase 11 builds public sandbox controls, and Phase 13 hardens governance.
  - Validation: Review proves the plan does not move unknown public-node sandbox enforcement into Phase 10 or allow public-provider work before Phase 11 gates exist.

- **1.3 Freeze sandbox profile ownership boundaries.**
  - Design: Record that Public Sandbox Profile owns profile versions, restriction sets, workload/data-class bindings, denials, output validation rules, quarantine records, log redaction profiles, evaluation records, reason codes, deprecation, supersession, and emergency disablement.
  - Output: Ownership checklist for architecture and implementation reviews.
  - Validation: Review confirms the service does not onboard providers, schedule jobs, issue leases, run containers, own local enforcement primitives, verify trust, score reputation, adjudicate fraud, decide payouts, or create exceptions for private/regulated/secret/system workloads.

- **1.4 Carry forward resolved SDS #56 decisions.**
  - Design: Preserve the first egress classes, mandatory output validators, safe provider-facing denial explanations, emergency-disable behavior, and Overguard/Oversched versus Overrun/Overcell enforcement split.
  - Output: Resolved-decision checklist covering `offline_public`, `overrid_control_artifact_only`, `declared_public_fetch`, quarantine-before-delivery, redacted reason categories, in-flight cancellation/quarantine, and local runner rejection.
  - Validation: Review rejects arbitrary open-internet egress, user-delivery without required validation, raw private evidence leakage, silent profile migration, and precheck-only enforcement.

- **1.5 Define upstream and downstream dependency boundaries.**
  - Design: Create a dependency matrix for Public Provider Onboarding, Workload Classifier, Overguard, Oversched, Overlease, Overrun, Overcell, Overpack, Overstore, Overvault, Overwatch, Fraud Control Service, Challenge Task Service, Provider Payout Service, SDK, CLI, admin UI, and provider-facing surfaces.
  - Output: Boundary matrix listing owner, input refs, output refs, freshness rule, redaction class, policy refs, evidence refs, downstream consumer, and rejection behavior.
  - Validation: Review confirms every handoff uses explicit APIs, immutable refs, signed evidence, stable reason codes, trace ids, idempotency keys, policy refs, and Overwatch events rather than privileged shared records or hidden control paths.

## Phase 2: Rust Contracts, Schemas, Lifecycles, And Fixtures

### Work Items

- **2.1 Create the Public Sandbox Profile Rust contract module.**
  - Design: Add contract types for profile versions, restriction sets, public workload bindings, secret/mount denials, output validation rule sets, quarantine records, log redaction profiles, evaluations, events, redaction profiles, and stable errors.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, reason-code enums, egress-class enums, profile-status enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from provider onboarding, scheduling, runner, storage, vault, verification, fraud, and payout internals.

- **2.2 Define sandbox profile version and restriction schemas.**
  - Design: Model `sandbox_profile_version` and `restriction_set` with profile id, version, status, compatibility window, workload/data class allowlists, profile hash, signer refs, rollout refs, deprecation refs, filesystem/mount rules, secret policy, egress classes, runtime caps, memory caps, CPU/GPU caps, storage caps, syscall/container refs, and cleanup requirements.
  - Output: JSON Schema files, valid examples, invalid examples, generated validators, docs-facing examples, and canonical profile/restriction fixtures.
  - Validation: Schema tests reject missing profile id, version, status, compatibility window, workload class allowlist, data class allowlist, signer refs, policy refs, trace id, audit refs, or required restriction fields.

- **2.3 Define binding, denial, evaluation, and replay schemas.**
  - Design: Model `public_workload_binding`, `secret_mount_denial`, `sandbox_evaluation`, and replay bundles with classifier refs, provider refs, profile refs, policy refs, matched rules, denied rules, enforcement handoff refs, and audience-specific explanations.
  - Output: Binding schema, denial schema, evaluation schema, replay schema, stable reason-code catalog, redacted examples, and negative fixtures.
  - Validation: Tests prove private, regulated, secret-bearing, tenant-private, and system-service facts produce stable hard-deny records before queue, lease, or execution.

- **2.4 Define output validation, quarantine, and log redaction schemas.**
  - Design: Model `output_validation_rule_set`, `artifact_quarantine_record`, and `log_redaction_profile` with expected output schema, allowed artifact types, size caps, checksum requirements, content marker checks, quarantine triggers, retention policy, review status, allowed log fields, sampling, trace-only fields, provider-visible fields, and user-visible fields.
  - Output: Output-validator schema, quarantine schema, log-redaction schema, release/deny decision examples, supersession examples, and replay examples.
  - Validation: Tests prove required validators block user delivery when unavailable, inconclusive, mismatched, or sensitive-output-positive, and log redaction removes user content, secret-looking values, private refs, provider-sensitive internals, and anti-fraud internals.

- **2.5 Create deterministic public sandbox fixtures.**
  - Design: Build fixtures for draft profile, active profile, stale profile, missing classifier facts, private data ref, secret ref, regulated marker, system-service workload, offline public job, declared public fetch, egress denial, cap denial, output quarantine, log redaction, emergency disablement, deprecation, supersession, and runner enforcement failure.
  - Output: Fixture directory, canonical inputs, expected states, expected events, expected reason codes, redacted summaries, BLAKE3 hashes, and replay bundles.
  - Validation: Fixture tests produce stable ids, hashes, lifecycle states, denial reason codes, audit refs, redacted views, and replay outputs across repeated runs.

## Phase 3: Profile Lifecycle APIs And Rollout Control

### Work Items

- **3.1 Implement profile draft creation.**
  - Design: Add `POST /public-sandbox/profiles` for draft profile versions with restriction sets, workload/data class allowlists, egress class rules, output validation rule refs, log redaction refs, compatibility windows, signer refs, and rollout refs.
  - Output: API handler, request/response schemas, signed envelope checks, idempotency behavior, stable errors, and `public_sandbox_profile.profile_created` events.
  - Validation: API tests cover valid draft creation, duplicate idempotency key, missing restriction set, missing signer refs, unsafe workload/data classes, unsupported egress class, and audience-safe errors.

- **3.2 Implement profile validation and activation.**
  - Design: Add validation and `POST /public-sandbox/profiles/{profile_id}/activate` to require Overguard policy refs, complete restriction sets, output validation rules, log redaction profiles, rollback/deprecation refs, and deterministic fixture coverage.
  - Output: Validation module, activation API, activation preflight bundle, profile hash, policy refs, and `public_sandbox_profile.profile_activated` events.
  - Validation: Tests prove profiles cannot activate when no-secret/no-private/no-regulated/no-system-service rules, deny-by-default egress, runtime caps, output validators, quarantine rules, redaction rules, or replay fixtures are missing.

- **3.3 Implement redacted profile reads.**
  - Design: Add `GET /public-sandbox/profiles/{profile_id}` with audience-scoped metadata, compatibility windows, safe restriction summaries, profile status, replacement refs, and policy/evaluator version refs.
  - Output: Read API, provider/user/operator redaction profiles, pagination/filtering for profile lists, and profile status fixtures.
  - Validation: Contract tests prove provider-facing reads do not expose secret refs, private data refs, anti-fraud internals, exact node/IP/device fingerprints, raw operator notes, or topology.

- **3.4 Implement deprecation and supersession.**
  - Design: Add `POST /public-sandbox/profiles/{profile_id}/deprecate` for planned deprecation, supersession, replacement refs, compatibility cutoffs, signed operator refs, and historical replay preservation.
  - Output: Deprecation API, supersession records, replacement profile refs, stale-profile reason codes, and `public_sandbox_profile.profile_deprecated` events.
  - Validation: Tests prove deprecated profiles stop new placement according to policy while preserving replay of historical decisions and not silently editing past profile versions.

- **3.5 Publish rollout diagnostics and profile inventory.**
  - Design: Provide operator views for active profiles, deprecated profiles, compatibility windows, stale profile use, missing validators, emergency-disabled profiles, and rollout readiness.
  - Output: Query APIs, filters, inventory projections, stale-use alerts, Overwatch timeline refs, and audit-export hooks.
  - Validation: Tests prove every mutating profile action emits actor, scope, trace id, idempotency key, policy refs, profile hash, evidence refs, and stable reason codes.

## Phase 4: Restriction Sets, Bindings, Denials, And Egress Classes

### Work Items

- **4.1 Implement restriction-set evaluation.**
  - Design: Evaluate filesystem mode, mount allowlist, secret policy, environment policy, process limits, runtime caps, memory caps, CPU/GPU caps, storage caps, network egress, syscall/container refs, cleanup requirements, and artifact rules as a single versioned profile decision.
  - Output: Restriction evaluator, matched-rule output, denied-rule output, stable errors, and fixture-backed rule tables.
  - Validation: Tests prove each restriction can independently deny a request and the combined evaluator returns deterministic ordered reason codes.

- **4.2 Implement workload and data-class binding rules.**
  - Design: Bind Workload Classifier facts to public low-sensitivity profile allowlists with strict rejection for missing, stale, private, regulated, secret-bearing, tenant-private, unknown, or system-service facts.
  - Output: Binding evaluator, classifier-ref validator, data-class map, stale-classification behavior, and negative fixtures.
  - Validation: Tests prove missing classification or ambiguous data class denies public placement rather than falling back to permissive defaults.

- **4.3 Implement secret, private mount, regulated data, and system-service denials.**
  - Design: Add denial recording for rejected secret refs, private data refs, regulated data refs, system-service refs, private mounts, and unsupported non-secret public token exceptions.
  - Output: `POST /public-sandbox/denials/secret-mount`, denial record builder, redacted reason-code summaries, remediation summaries, and `secret_or_mount_denied` events.
  - Validation: Tests prove secret refs from Overvault and private/regulated refs from Overstore/Overvault are denied before queue, lease, or execution in the first build.

- **4.4 Implement first egress class rules.**
  - Design: Support only `offline_public`, `overrid_control_artifact_only`, and tightly scoped `declared_public_fetch` for manifest-declared public endpoints or approved Overstore/Overmesh artifact transfer refs.
  - Output: Egress-class schema, egress evaluator, denied destination class records, public-fetch approval refs, and replay fixtures.
  - Validation: Tests prove open internet crawling, arbitrary hostnames, webhooks, private-network access, credentialed third-party APIs, peer discovery, and provider-supplied egress rules are rejected in the first profile.

- **4.5 Publish restriction and denial diagnostics.**
  - Design: Provide diagnostics for public profile missing, data class disallowed, secret ref denied, private mount denied, system service denied, egress denied, runtime cap exceeded, memory cap exceeded, and profile deprecated.
  - Output: Diagnostic API, provider-safe summaries, operator detail refs, Overwatch refs, and remediation checklist fields.
  - Validation: Tests prove diagnostics are explainable and correctable without exposing raw private data, secrets, regulated markers, anti-fraud internals, or other-provider evidence.

## Phase 5: Evaluation API And Placement Prechecks

### Work Items

- **5.1 Implement sandbox evaluation API.**
  - Design: Add `POST /public-sandbox/evaluate` for workload facts, provider refs, node refs, requested resources, profile refs, policy refs, classifier refs, sandbox compatibility refs, and desired egress class.
  - Output: Evaluation API, request/response schemas, deny-by-default behavior, reason-code catalog, replay bundle refs, and `evaluation_requested` events.
  - Validation: API tests cover allowed public low-sensitivity evaluation, missing profile, stale profile, missing classifier, missing provider eligibility, disallowed data class, secret ref, private mount, egress denial, cap denial, and deprecated profile.

- **5.2 Integrate Overguard prechecks.**
  - Design: Ask Overguard to precheck identity, tenant, classification, data/secret/regulated/system-service denial facts, active profile status, provider eligibility, egress class, policy refs, and public-placement admission before queue or lease creation.
  - Output: Overguard adapter, policy fact bundle, stale-policy behavior, policy decision refs, and denial replay fixtures.
  - Validation: Tests prove Overguard denial blocks queueing and placement and Public Sandbox Profile does not create policy allow decisions independently.

- **5.3 Integrate Public Provider Onboarding eligibility refs.**
  - Design: Require current provider/node eligibility, public workload acceptance contracts, sandbox compatibility refs, policy acknowledgement freshness, and eligibility publication refs before public sandbox evaluation can allow placement.
  - Output: Onboarding adapter, eligibility-ref validator, stale provider behavior, missing compatibility reason codes, and provider remediation summaries.
  - Validation: Tests prove public providers without active onboarding/sandbox compatibility refs cannot receive public sandbox placement.

- **5.4 Integrate Oversched candidate compatibility prechecks.**
  - Design: Supply Oversched with candidate compatibility facts, active profile refs, resource/runtime caps, node software and Overcell capability facts, locality constraints, and no-candidate reason codes before scoring and lease requests.
  - Output: Scheduler handoff projection, candidate filter facts, no-candidate summaries, freshness semantics, and integration fixtures.
  - Validation: Tests prove Oversched cannot use expired, revoked, stale, or incompatible profile refs for public-node placement.

- **5.5 Publish side-effect-free evaluation simulations.**
  - Design: Provide dry-run previews for proposed workload facts, provider refs, egress class, output validation requirements, cap changes, and profile rollout before queue or placement mutation.
  - Output: Simulation API, missing-prerequisite summaries, expected denial reasons, policy refs, and replay packs.
  - Validation: Tests prove simulation cannot create profiles, activate profiles, enqueue jobs, request leases, publish capabilities, release artifacts, or mutate owner-service state.

## Phase 6: Runner And Node Enforcement Handoff

### Work Items

- **6.1 Define enforcement handoff contract for Overrun and Overcell.**
  - Design: Package selected profile refs, restriction-set hashes, lease refs, manifest refs, artifact hashes, no-secret/no-private/no-regulated/no-system-service decisions, egress class, cap settings, output validation refs, redaction refs, cleanup rules, and evidence refs into a signed enforcement bundle.
  - Output: Enforcement handoff schema, signed bundle builder, BLAKE3 hash, replay fixture, and runner-facing stable errors.
  - Validation: Tests prove the handoff is immutable, profile-versioned, traceable, and rejected when required refs or hashes are missing.

- **6.2 Implement runner pre-start rejection rules.**
  - Design: Require Overrun/Overcell to verify lease/profile compatibility, manifest and artifact hashes, mount restrictions, environment sanitization, process/syscall/container profile, CPU/GPU/memory/storage/wall-clock caps, egress firewall, log redaction, output capture, validation handoff, quarantine, cancellation, timeout, and cleanup before process start.
  - Output: Runner pre-start checklist, enforcement-failure reason codes, `enforcement_handoff_recorded` state, and failure evidence events.
  - Validation: Tests prove Overrun rejects before process start when it cannot enforce any selected restriction.

- **6.3 Implement node capability and software compatibility checks.**
  - Design: Validate Overcell node-agent version, Node Installer version evidence, Hardware Discovery facts, supported sandbox primitives, container/OCI/WASI support where applicable, egress firewall support, output capture support, and cleanup support.
  - Output: Capability validator, node compatibility projection, stale software errors, remediation summaries, and node-readiness fixtures.
  - Validation: Tests prove nodes without required local enforcement primitives are filtered before scheduling and rejected by the runner if assigned anyway.

- **6.4 Implement enforcement evidence ingestion.**
  - Design: Accept runner-enforcement evidence for applied restrictions, denied mounts, blocked egress, runtime/memory cap outcomes, timeout/cancellation, cleanup, output capture, log redaction, and validation handoff.
  - Output: Evidence intake API or event consumer, enforcement status projection, Overwatch refs, and replay bundles.
  - Validation: Tests prove evidence is append-only, redacted, trace-linked, and cannot be used to weaken profile rules after the fact.

- **6.5 Publish runner/operator diagnostics.**
  - Design: Provide diagnostics for missing enforcement primitives, failed pre-start checks, denied mounts, blocked egress, cap exceeded, cleanup failure, output capture failure, and validation handoff failure.
  - Output: Diagnostic API, operator queue, provider-safe explanation summary, runner-safe remediation, and alert refs.
  - Validation: Tests prove provider-facing diagnostics are actionable without leaking raw private data, secret refs, regulated markers, challenge internals, fraud thresholds, or topology.

## Phase 7: Output Validation, Artifact Quarantine, And Log Redaction

### Work Items

- **7.1 Implement output validation intake.**
  - Design: Add `POST /public-sandbox/output-validation` for manifest-output conformance, declared artifact count/type checks, size caps, BLAKE3/content-hash checks, schema/media-shape validation, unexpected executable/archive checks, sensitive-marker scans, private refs, regulated markers, system-service refs, and log-redaction checks.
  - Output: Validation API, validator-result schema, unavailable/inconclusive behavior, stable reason codes, and `output_validated` events.
  - Validation: Tests prove unavailable, inconclusive, mismatched, undeclared, executable, archive-risk, sensitive-output-positive, or log-redaction-failed results block normal delivery.

- **7.2 Implement artifact quarantine workflow.**
  - Design: Add quarantine creation for failed validation, suspicious markers, undeclared outputs, oversized artifacts, unsafe archives, malware/safety scan refs where applicable, and manual/operator review requirements.
  - Output: `artifact_quarantine_record`, quarantine storage refs, retention policy, review state, release/deny decision refs, and `artifact_quarantined` events.
  - Validation: Tests prove quarantined artifacts cannot be returned through normal user delivery, provider retrieval, payout finality, or public report paths until release is explicitly authorized.

- **7.3 Implement quarantine status and review APIs.**
  - Design: Add `GET /public-sandbox/quarantine/{quarantine_id}` and review commands for released, deleted, retained for dispute, or escalated artifacts with audience-specific evidence refs.
  - Output: Quarantine read API, review API/command, redacted status summaries, Overclaim/operator handoff refs, and `quarantine_resolved` events.
  - Validation: Tests prove requester, provider, operator, and stewardship views expose only permitted evidence and preserve unsafe content isolation.

- **7.4 Implement log redaction profiles.**
  - Design: Apply selected log redaction profile to user content, secret-looking values, private refs, provider-sensitive internals, anti-fraud internals, trace-only fields, provider-visible fields, and user-visible fields before logs leave enforcement boundaries.
  - Output: Redaction engine, profile validator, sampled logs, role-specific views, and redaction fixtures.
  - Validation: Tests prove logs remain useful for debugging while never exposing private input, secret refs, user content, raw fraud heuristics, or unauthorized provider internals.

- **7.5 Link usage, accounting, and dispute refs without owning them.**
  - Design: Emit usage-relevant refs for evaluation, output validation, quarantine storage, review work, failed enforcement, and release/deny decisions to Overmeter, Overbill, Seal Ledger, Provider Payout Service, Overclaim, and Overwatch.
  - Output: Usage-ref handoff, quarantine review usage refs, payout-hold refs, dispute refs, and accounting-friendly redacted summaries.
  - Validation: Tests prove Public Sandbox Profile does not create prices, payouts, invoices, balances, or ledger entries directly.

## Phase 8: Replay, Deprecation, Emergency Disablement, And Supersession

### Work Items

- **8.1 Implement evaluation replay API.**
  - Design: Add `GET /public-sandbox/replay/{evaluation_id}` to reconstruct request facts, classifier refs, provider refs, profile refs, matched rules, denied rules, policy decisions, enforcement handoff refs, output validation refs, quarantine refs, and redacted evidence.
  - Output: Replay API, replay bundle schema, hash validation, role-scoped redactions, and replay fixtures.
  - Validation: Tests prove replay reconstructs historical decisions even after profiles are deprecated, superseded, or emergency-disabled.

- **8.2 Implement emergency disablement.**
  - Design: Support signed emergency disablement that stops new evaluations and placement immediately, cancels/requeues pending work, rejects leased-but-not-started work, sends signed cancellation where safety-critical running work is affected, and quarantines partial outputs, logs, and usage refs.
  - Output: Emergency disable command/API, profile status transition, affected-work projection, cancellation refs, quarantine refs, and `profile_deprecated` or emergency disable events.
  - Validation: Tests prove no new placement uses an emergency-disabled profile and no in-flight job silently migrates profiles without fresh evaluation, enforcement handoff, and Overwatch evidence.

- **8.3 Implement planned deprecation behavior.**
  - Design: Distinguish safety emergency disablement from planned deprecation metadata where policy may allow already-running fully offline work to finish, while outputs still require validation under replacement or superseding profile before release.
  - Output: Planned deprecation policy rules, replacement refs, grace-window records, and affected-work reports.
  - Validation: Tests prove planned deprecation does not allow new placement and cannot skip output validation, replay, or replacement-profile evidence.

- **8.4 Implement rollout and rollback safety checks.**
  - Design: Preflight profile rollout with deterministic fixtures, Overguard activation refs, compatibility windows, canary refs, stale profile detection, rollback profile refs, and affected consumer notifications.
  - Output: Rollout checker, rollback command, consumer notification records, stale-use alerts, and rollout replay bundles.
  - Validation: Tests prove unsafe profile rollout fails closed and rollback creates new refs/status transitions rather than editing immutable active-profile history.

- **8.5 Publish lifecycle reports and alerts.**
  - Design: Provide reports for active/deprecated/disabled profiles, stale profile use, emergency disables, quarantined in-flight work, unresolved replacement refs, output validation backlog, and replay failures.
  - Output: Operator report APIs, alert refs, Overwatch timeline views, stewardship hooks, and audit-export classes.
  - Validation: Tests prove lifecycle reports are role-scoped, replayable, and preserve privacy while giving enough evidence for incident and governance review.

## Phase 9: Public Sandbox Proof And Expansion Gates

### Work Items

- **9.1 Configure the first public sandbox proof profile.**
  - Design: Build a proof profile with no secret injection, no private mounts, no regulated data, no system-service workloads, ephemeral filesystem, deny-by-default egress, `offline_public`, `overrid_control_artifact_only`, tightly scoped `declared_public_fetch`, runtime/memory/storage caps, output validators, quarantine rules, and log redaction.
  - Output: Proof profile fixture, restriction-set fixture, workload/data-class matrix, egress matrix, validator matrix, quarantine matrix, and expected denial paths.
  - Validation: Scenario tests prove only public low-sensitivity workloads can pass and all private, regulated, secret-bearing, system-service, unknown, or stale-fact workloads deny before queue, lease, or execution.

- **9.2 Prove Overguard, Oversched, Overrun, and Overcell split.**
  - Design: Run an end-to-end proof where Overguard admits only safe facts, Oversched filters only compatible nodes, Overrun/Overcell enforce the selected restrictions locally, and failed enforcement rejects before process start.
  - Output: Scenario fixture, Overguard decision ref, scheduler candidate refs, enforcement handoff bundle, runner evidence, and replay bundle.
  - Validation: Scenario tests prove prechecks are not a substitute for local enforcement and local enforcement cannot silently ignore missing sandbox primitives.

- **9.3 Prove output validation and quarantine before delivery.**
  - Design: Run scenarios for expected public output, undeclared artifact, oversized output, unexpected executable/archive, sensitive marker, private ref, regulated marker, unavailable validator, and manual release/deny.
  - Output: Output validation fixtures, quarantine records, release/deny decision refs, Overwatch refs, and requester/provider/operator redacted views.
  - Validation: Tests prove no unsafe or unvalidated artifact reaches normal delivery, provider retrieval, payout finality, or public summaries.

- **9.4 Prove fraud, challenge, payout-hold, and correction handoffs.**
  - Design: Attach Fraud Control, Challenge Task Service, Provider Payout Service, Overclaim, Overwatch, Reputation and Anti-Sybil Service, and Public Provider Onboarding refs for suspicious artifacts, failed enforcement, challenge failures, payout holds, dispute evidence, and provider corrections.
  - Output: Handoff matrix, fraud/challenge/payout/dispute fixtures, stable reason codes, provider-safe summaries, and replay bundles.
  - Validation: Tests prove Public Sandbox Profile emits evidence refs and hold triggers without adjudicating fraud, running challenges, mutating payouts, deciding final reputation, or deleting history.

- **9.5 Define Phase 13 governance and scale hardening gates.**
  - Design: Specify governance work for public sandbox reporting, compliance boundaries, threat modeling, incident response, audit export, redaction review, emergency disable drills, retention, validator provenance, and scale-readiness review.
  - Output: Governance checklist, report classes, retention classes, incident/compliance handoff matrix, threat-review targets, emergency-drill fixtures, and scale-readiness gate.
  - Validation: Review confirms public sandboxing is deny-by-default, explainable, replayable, privacy-preserving, correctable, and proportionate before wider public-provider scale.

## Phase 10: Validation, Queue Closure, And Handoff Readiness

### Work Items

- **10.1 Validate plan structure and local links.**
  - Design: Check title prefix, attached SDS link, phases 1 through 10, five work items per phase, Design/Output/Validation structure, final newline, tabs, and local Markdown links.
  - Output: Validation command output and progress evidence.
  - Validation: Focused script passes for this file and every changed cross-document link.

- **10.2 Validate tech-stack and authority guardrails.**
  - Design: Scan this plan and changed docs for conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain/NFT, speculative-market, pricing, revenue, customer-count, raw-secret-storage, private-workload hosting, regulated-workload hosting, system-service hosting, provider onboarding, scheduling, execution, trust verification, fraud adjudication, payout mutation, and external-provider-as-core-boundary drift.
  - Output: Guardrail scan output with only allowed negative-control, native Overrid service-name, authority-boundary, or explicit non-choice references.
  - Validation: Review confirms the plan keeps Rust-first/native Overrid service boundaries and does not turn comparison tools, local stubs, owner-service refs, or downstream APIs into Public Sandbox Profile's product boundary.

- **10.3 Validate SDS #56 build-breakdown coverage.**
  - Design: Map every SDS #56 build-breakdown item to this plan: schemas, profile lifecycle APIs, deny-by-default evaluation, Workload Classifier/Overguard/Public Provider Onboarding/Oversched/Overrun integrations, secret/mount denial evidence, output validation, quarantine, operations, replay, emergency disablement, and validation tests.
  - Output: Coverage matrix, API checklist, workflow checklist, fixture checklist, integration-test targets, and authority-boundary checklist.
  - Validation: Review proves no SDS #56 build-breakdown item is missing and the plan preserves Public Sandbox Profile as a Phase 11 public low-sensitivity runtime safety contract.

- **10.4 Validate SDS, service catalog, master plan, crosswalk, and queue alignment.**
  - Design: Confirm this plan is linked from the SDS source table, service catalog sub-build-plan section, master per-SDS table, build-plan crosswalk, queue state, queue progress, and build-plan progress.
  - Output: Updated backlinks and index rows for SDS #56, queue state update, queue progress update, and build-plan progress evidence.
  - Validation: JSON validation passes; local link checks pass; queue validation confirms `056-build-plan` is complete, no materialized task is running, and `057-build-plan` is the next incomplete build-plan task.

- **10.5 Validate implementation handoff readiness.**
  - Design: Confirm the 10-phase plan gives builders source alignment, contracts, schemas, profile lifecycle APIs, restrictions, denials, evaluation prechecks, runner enforcement, output validation, quarantine, redaction, replay, emergency lifecycle, proof scenarios, governance gates, and validation work in dependency order without changing master Phase 0 through Phase 13 order.
  - Output: Handoff checklist, Docdex index refresh, search evidence, DAG export reference, and test-runner status.
  - Validation: Docdex targeted index refresh succeeds, Docdex retrieval finds the plan, `git diff --check` passes, and any `docdexd run-tests` blocker is recorded explicitly.

## Alignment Review

- The sub-build plan keeps Public Sandbox Profile first build work in master Phase 11 because unknown or semi-trusted public nodes require strict public low-sensitivity workload boundaries, no-secret/no-private/no-regulated/no-system-service denial, hardened sandbox profiles, output validation, quarantine, redacted evidence, fraud/challenge handoffs, payout holds, and local runner enforcement.
- The plan treats Phase 10 trusted federation and public-interest pools as upstream context and downstream consumers, not as the first build point for unknown public-node sandbox enforcement. Known-participant federation remains separate from adversarial public supply.
- The plan treats Public Provider Onboarding as the provider eligibility owner; Public Sandbox Profile consumes provider/node compatibility refs without enrolling providers or publishing capability records.
- The plan treats Workload Classifier and Overguard as workload/data-class and policy owners; Public Sandbox Profile consumes their refs and denies unsafe public placement without becoming a general policy engine.
- The plan treats Oversched and Overlease as placement/reservation owners; Public Sandbox Profile returns evaluation and compatibility refs without scoring nodes or issuing leases.
- The plan treats Overrun and Overcell as local execution-enforcement owners; Public Sandbox Profile creates the selected versioned contract and rejects missing enforcement evidence without running workloads itself.
- The plan treats Overstore and Overvault as object/private/secret owners; Public Sandbox Profile denies unsafe refs, validates outputs, and records quarantine refs without owning object persistence or raw secrets.
- The plan treats Fraud Control Service, Challenge Task Service, Provider Payout Service, Reputation and Anti-Sybil Service, Overclaim, and Overwatch as fraud/challenge/payout/reputation/dispute/audit owners; Public Sandbox Profile stores refs and emits evidence without adjudicating their outcomes.
- The plan preserves master Phase 0 through Phase 13 ordering and uses earlier phases as prerequisites, Phase 11 as the first public sandbox build, and Phase 13 as governance/security/compliance hardening.
- The plan aligns with `docs/overrid_tech_stack_choice.md`: Rust-first core service/contracts, native Overrid boundaries, canonical JSON/JSON Schema, optional Protobuf, Ed25519 signatures, BLAKE3/content hashes, signed envelopes, and no conventional cloud, blockchain, NFT, pricing, revenue, or customer-count assumptions.
