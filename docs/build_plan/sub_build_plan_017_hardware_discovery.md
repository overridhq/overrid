# SUB BUILD PLAN #17 - Hardware Discovery

Attached SDS: [docs/sds/execution_scheduling/hardware_discovery.md](../sds/execution_scheduling/hardware_discovery.md)

## Purpose

This sub-build plan turns SDS #17 into an implementation sequence for Hardware Discovery. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Hardware Discovery is the observed-inventory evidence producer for Overrid nodes. It defines approved probes, coordinates bounded discovery through Overcell-managed nodes, ingests signed reports, redacts sensitive host details, normalizes device/runtime/health facts, publishes capability observations to Overregistry, emits Overwatch events, and preserves enough evidence for Benchmark Runner, Oversched, Overguard, Oververify, operators, and later federation/public-provider controls to consume observed facts without trusting raw provider claims.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #17: Hardware Discovery](../sds/execution_scheduling/hardware_discovery.md) | Controls Hardware Discovery purpose, responsibilities, data model, APIs, events, state machine, privacy/security rules, metering facts, validation, resolved open-question decisions, and downstream handoff. |
| [Hardware Discovery service plan](../service_catalog/execution_scheduling/hardware_discovery.md) | Controls service-catalog objective, first build phase, dependencies, development order, contracts, validation, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical build order from master Phase 0 through master Phase 13. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared schema, fixture, local-stack, idempotency, trace, signed-envelope, and integration-harness prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies identity, tenant, key, registry, queue, audit, and Overwatch primitives that Hardware Discovery depends on but does not own. |
| [Phase 2: Seed Private Swarm](phase_02_seed_private_swarm.md) | Controls the first build point for Hardware Discovery as seed-node observed inventory, alongside Overcell, Node Installer, Benchmark Runner, and Overregistry capability publication. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Consumes current signed discovery facts as placement prerequisites for Oversched, Overlease, Overrun, and Overpack runtime compatibility checks. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard, Oververify, challenge checks, Overmesh private connectivity, and policy evidence consumers that use discovery facts without turning them into trust scores. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies usage rollup and accounting visibility for discovery overhead without moving billing or settlement into Hardware Discovery. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies system-service workload, backup, restore, failover, and grid-resident operation requirements that depend on stricter runtime-support flags. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Supplies known-organization and public-interest pool rules that later consume normalized capability observations. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public-provider anti-gaming, sandbox, fraud, reputation, and payout-hold hardening that requires stricter discovery validation. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies compliance, threat modeling, migration, incident, reporting, and governance hardening for restricted host evidence and discovery replay. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #17 first build work aligned to master Phase 2, with later handoffs through execution, verification, accounting visibility, grid-resident operation, federation/public-provider hardening, and governance. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP, signed envelopes, Ed25519, BLAKE3/content hashes, canonical JSON plus JSON Schema, explicit GPU/runtime adapters, Rust Overcell agent coordination, and native Overrid boundaries. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, and 2 | Attach SDS #17, freeze Hardware Discovery as observed-inventory normalization, and preserve first build in Phase 2. |
| 2 | Master Phases 0 and 2 | Build the Rust service skeleton, discovery contract schemas, local fixtures, and deterministic probe-report tests. |
| 3 | Master Phase 2 | Implement probe registry, platform adapters, probe safety limits, redaction rules, and supported-platform matrix. |
| 4 | Master Phase 2 | Implement discovery session lifecycle, Overcell trigger path, signed report upload, report validation, and terminal states. |
| 5 | Master Phases 2 and 3 | Normalize reports into capability observations, detect changes/staleness, and publish Overregistry records consumed by private execution. |
| 6 | Master Phases 3 and 4 | Add runtime-support flags, fail-closed eligibility inputs, Overguard/Oververify evidence handoffs, and challenge-triggered refresh hooks. |
| 7 | Master Phases 2, 5, and 7 | Add security, privacy, restricted evidence, usage facts, operator diagnostics, and system-service runtime flag readiness. |
| 8 | Master Phases 2, 7, and 13 | Add refresh cadence, replay/rebuild, schema migration, backup/restore, and grid-resident service-readiness behavior. |
| 9 | Master Phases 6, 10, and 11 | Harden SDK, CLI, admin, trusted-federation, public-provider, fraud, and anti-gaming handoffs. |
| 10 | Master Phase 2 through Phase 13 | Validate structure, tech-stack alignment, documentation links, queue state, and final implementation gates. |

## Tech Stack Guardrails

- Hardware Discovery core is a Rust execution/scheduling service using Tokio and Axum/Tower/Hyper-style HTTP, with rustls/mTLS where early seed control-plane or node-agent paths require it.
- Probe definitions, discovery sessions, signed reports, inventory snapshots, device records, runtime support records, capability observations, health results, discovery errors, redaction manifests, API errors, and fixtures use canonical JSON plus JSON Schema. Compact Protobuf contracts may be added only where the shared contract layer requires them.
- Mutating commands require signed command or service-account envelopes, idempotency keys, trace ids, tenant or system scope, actor/service refs, stable reason codes, schema versions, policy/evidence refs, and append-only Overwatch events.
- Node report uploads require Overcell assignment or enrolled-node binding, node credential verification through Overkey, current enrollment binding, probe-version binding, redaction rules, and coordinator verification before normalization.
- Ed25519 is used where node, coordinator, service-account, or operator signatures are required. BLAKE3/content hashes are used for probe definitions, report bundles, redacted evidence bundles, normalized observations, and Overregistry publication refs.
- GPU/runtime integration is explicit and adapter-controlled for NVIDIA, ROCm, OCI/container runtime, WASI/sandbox runtime, cgroup/isolation, egress-control, filesystem, and future accelerator observations. Hardware Discovery must not hide arbitrary shell execution behind a generic host probe.
- Discovery storage and queues use Overrid-owned boundaries or Overrid-shaped local stubs during early phases. PostgreSQL, Redis, Kafka, NATS, RabbitMQ, S3, MinIO, Vault, cloud KMS, blockchain, NFT, market-token, pricing, revenue, or customer-count mechanics must not become Hardware Discovery's product boundary.
- Hardware Discovery does not own Benchmark Runner performance evidence, scheduler placement, policy finality, trust scoring, provider eligibility, disputes, reputation scoring, ORU accounting, Seal Ledger transitions, billing, payout decisions, or public-provider admission.
- Probes are allowlisted, versioned, least-privilege, read-only, timeout-bounded, and explicitly redacted. Any elevated host read requires an approved probe definition, operator evidence, and narrow data scope.
- General reads expose normalized resource class, capability value, confidence, freshness, degraded/unknown/unsupported state, and redacted reason codes only. Raw host identifiers remain restricted evidence or are never collected when they are outside SDS #17's allowed scope.

## Phase 1: SDS Attachment, Inventory Scope, And Boundary Rules

### Work Items

- **1.1 Attach the build plan to SDS #17.**
  - Design: Link this document from the numbered Hardware Discovery SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/execution_scheduling/hardware_discovery.md`, `docs/service_catalog/execution_scheduling/hardware_discovery.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #17 returns both the Hardware Discovery SDS and this sub-build plan.

- **1.2 Freeze Hardware Discovery as observed inventory.**
  - Design: Record that Hardware Discovery owns approved probe definitions, discovery session lifecycle, signed report ingestion, redaction, raw-to-normalized mapping, device/runtime/health facts, capability observations, error codes, change detection, and publication refs.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms Hardware Discovery does not own benchmark results, scheduler placement, trust score, policy finality, provider eligibility, ORU accounting, billing, payout holds, or dispute judgment.

- **1.3 Preserve master Phase 2 as the first build point.**
  - Design: Keep first implementation in master Phase 2 because seed nodes need normalized CPU, memory, GPU, storage, network, OS, runtime, driver, and locality observations before Benchmark Runner and Phase 3 private execution can safely consume them.
  - Output: Phase-gate note that master Phase 0 and Phase 1 are prerequisites, Phase 2 is first implementation, and later phases consume or harden discovery evidence.
  - Validation: Review proves this plan does not move Hardware Discovery into Phase 0 or Phase 1 and does not change the master Phase 0 through Phase 13 order.

- **1.4 Carry forward resolved SDS decisions.**
  - Design: Preserve SDS #17 decisions for restricted host inventory, never-collected data, Phase 3 fail-closed workload eligibility, discovery refresh cadence, normalized capability taxonomy, and Phase 7 system-service runtime flag bundles.
  - Output: Resolved-decision checklist tied to SDS #17 open-question answers.
  - Validation: Review rejects plans that collect secrets or private user data, expose raw host identifiers in general reads, treat unknown facts as eligible where certainty is required, or use unversioned capability names.

- **1.5 Define runtime authority boundaries.**
  - Design: Create a boundary matrix for Overcell, Node Installer, Overregistry, Benchmark Runner, Overwatch, Oversched, Overguard, Oververify, Overlease, Overrun, Overmeter, Overmesh, Overvault, SDK, CLI, admin UI, and federation/public-provider consumers.
  - Output: Boundary matrix listing read/write authority, allowed events, restricted evidence, required signatures, and ownership exclusions.
  - Validation: Design review rejects direct scheduler state writes, direct trust-score mutation, host configuration mutation, unrestricted raw evidence reads, and billing or payout decisions inside Hardware Discovery.

## Phase 2: Rust Service Skeleton, Schemas, And Fixtures

### Work Items

- **2.1 Create the Hardware Discovery Rust service crate.**
  - Design: Add a Rust service module using Tokio, Axum/Tower/Hyper-style HTTP, shared config loading, tracing setup, dependency injection, and clients for Overcell, Overkey, Overregistry, Overwatch, Overmeter, Oververify, and policy consumers.
  - Output: Service crate, handler modules, repository traits, local-stack entrypoint, background worker boundary, and integration-test hooks.
  - Validation: Compile and service-start checks pass once implementation exists; crate layout review confirms Hardware Discovery remains separate from Overcell, Benchmark Runner, Overregistry, Oversched, Oververify, and accounting services.

- **2.2 Define discovery contract schemas.**
  - Design: Add schemas for `discovery_session`, `probe_definition`, `hardware_inventory_snapshot`, `device_record`, `runtime_support_record`, `capability_observation`, `health_probe_result`, `discovery_error`, redaction manifests, API errors, and discovery events.
  - Output: JSON Schema files, Rust types, fixtures, stable reason-code enums, state enums, redaction-class metadata, and compatibility metadata.
  - Validation: Schema tests reject missing node id, provider id, trace id, idempotency key, actor/service ref, probe id/version, observed timestamp, support state, confidence, signature refs, redaction class, and reason codes where required.

- **2.3 Define seed-node discovery fixtures.**
  - Design: Model CPU-only, GPU-capable, missing-driver, unsupported-container-runtime, cgroup-v1-only, cgroup-v2-supported, multiple-disk, limited-network-visibility, sensor-unavailable, and redaction-blocked fixtures.
  - Output: Valid and invalid report fixtures with raw evidence refs, normalized snapshots, capability observations, health warnings, errors, publication outcomes, and redacted read models.
  - Validation: Fixture tests prove the same raw report, probe versions, redaction manifest, and normalization version reconstruct the same normalized snapshot and evidence hash.

- **2.4 Implement Overrid-owned storage boundaries.**
  - Design: Define repositories for probe definitions, sessions, signed report refs, normalized snapshots, device records, runtime support records, capability observations, discovery errors, redaction bundles, publication refs, and local Overrid-shaped storage stubs.
  - Output: Repository traits, local adapter, append-only source records, derived-index rebuild model, migration hooks, and restricted evidence refs.
  - Validation: Storage tests prove published observations cannot be silently edited, corrections append new records, restricted host details are stored only as protected refs, and never-collected data is rejected at ingestion.

- **2.5 Wire local development and harness scenarios.**
  - Design: Add local-stack and integration-harness scenarios for one CPU node, one GPU node, missing runtime, report upload, normalization, Overregistry publication, Overwatch events, and Benchmark Runner suite-selection consumption.
  - Output: Local service config, deterministic fixtures, fake Overcell node-agent channel, fake Overregistry/Overwatch clients, and scenario names.
  - Validation: Local smoke tests can request discovery, receive a signed report, normalize observations, publish capability refs, emit events, and inspect redacted latest inventory without production credentials.

## Phase 3: Probe Registry, Platform Adapters, And Redaction Rules

### Work Items

- **3.1 Implement versioned probe registration.**
  - Design: Support immutable probe definitions with probe id, semantic version, platform selector, command/module binding, required permission class, timeout, output schema, redaction rules, sensor scope, and deprecation state.
  - Output: Probe registry APIs, probe hash calculation, compatibility filters, supported-platform matrix, and `hardware_discovery.probe_registered` events.
  - Validation: API tests cover valid registration, duplicate version, mutable change rejection, missing output schema, missing redaction rule, excessive permission scope, unsupported platform, and deprecated probe filtering.

- **3.2 Implement Linux-first inventory probes.**
  - Design: Add Linux probe adapters for CPU architecture/model family/core counts, memory totals, OS/kernel, storage class/capacity/filesystem/health hints, private-network reachability hints, container runtime, cgroup/isolation, seccomp, and egress-control support.
  - Output: Rust probe modules, normalized output contracts, timeout behavior, permission declarations, fixture samples, and reason codes for unavailable commands.
  - Validation: Probe tests prove CPU-only servers, multiple disks, missing runtime dependencies, limited network visibility, unsupported features, and unavailable sensors produce explicit `supported`, `unsupported`, `unknown`, or `degraded` states.

- **3.3 Implement GPU and accelerator runtime adapters.**
  - Design: Add explicit NVIDIA/ROCm observation adapters for GPU count, model family, visible memory, driver version, runtime support, and health summary while leaving future accelerator classes extensible.
  - Output: Adapter interfaces, fixture reports, GPU runtime support records, device-record mapping, and redacted evidence refs.
  - Validation: Tests reject hidden generic shell probes, preserve missing-driver and broken-runtime errors, block GPU capability publication when required facts are unverifiable, and avoid leaking GPU UUIDs or serials through general reads.

- **3.4 Implement redaction and never-collect enforcement.**
  - Design: Enforce the SDS #17 field policy: restricted fields are hashed/redacted or stored only as protected evidence refs, while secrets, private keys, tokens, user home-directory contents, process memory, environment values, unrelated application data, packet contents, and raw personal files are never collected or persisted.
  - Output: Redaction policy engine, restricted evidence marker model, hash/salt handling, fixture denylist, and read-model field matrix.
  - Validation: Redaction tests prove serials, MAC/IP fields, hostnames, exact facility/rack/location, GPU UUIDs, disk WWNs, route tables, mount paths, local usernames, raw output, stderr, and sensor ids do not appear in general inventory reads.

- **3.5 Implement probe safety and least-privilege rules.**
  - Design: Bound every probe by allowlist status, read-only intent, timeout, command/module scope, permission class, output size, retry rules, and explicit operator approval where elevated host reads are unavoidable.
  - Output: Probe execution policy, rejection reason codes, operator approval refs, and `hardware_discovery.probe_failed` event payloads.
  - Validation: Safety tests reject arbitrary command execution, unbounded output, filesystem traversal, automatic privilege escalation, network packet capture, process inspection, user-directory reads, and host configuration mutation.

## Phase 4: Discovery Lifecycle, Overcell Triggering, And Signed Reports

### Work Items

- **4.1 Implement discovery session requests.**
  - Design: Support `POST /nodes/{node_id}/discovery-sessions` with node id, probe set, trigger type, requester, idempotency key, trace id, policy/evidence refs, maintenance context, and bounded batch behavior.
  - Output: Session request handler, requested state, idempotent responses, rate limits, and `hardware_discovery.session_requested` events.
  - Validation: API tests cover valid request, duplicate idempotency key, unsupported probe set, missing node, invalid trigger, unauthorized requester, unbounded batch, disabled node, and stale enrollment binding.

- **4.2 Implement Overcell probe assignment.**
  - Design: Send signed discovery assignments to Overcell with approved probe set, probe hashes, permission envelope, timeout, output schema refs, redaction manifest refs, trace id, and node-specific assignment binding.
  - Output: Assignment client, probing state transition, probe-start events, retry rules, and assignment failure reasons.
  - Validation: Integration tests prove a session moves from requested to probing only after Overcell accepts the signed assignment and rejects mismatched probe hash, expired assignment, wrong node id, unsupported platform, or missing permission approval.

- **4.3 Implement signed report upload.**
  - Design: Support internal `POST /discovery-sessions/{session_id}/reports` for node-originated signed reports bound to session id, node credential, probe set hash, probe versions, evidence bundle hash, and capture timestamps.
  - Output: Report upload handler, node signature verification, report schema validation, raw evidence ref storage, and `hardware_discovery.report_received` events.
  - Validation: Signature tests reject unsigned reports, wrong node credential, replayed reports, mismatched probe set, invalid timestamps, oversized evidence, and reports for sessions not in probing state.

- **4.4 Implement lifecycle state machine.**
  - Design: Enforce `requested`, `probing`, `reported`, `normalizing`, `published`, `unchanged`, `stale`, `suspect`, `failed`, and `cancelled` transitions with append-only transition records and stable reason codes.
  - Output: State machine validator, terminal-state events, retryability rules, cancellation behavior, and failure records.
  - Validation: State tests prove illegal transitions are rejected, published observations are corrected or superseded rather than edited, and every terminal state has reason, trace, and evidence refs.

- **4.5 Implement partial failure handling.**
  - Design: Let non-critical probe failures continue while required baseline failures produce explicit `unknown`, `unsupported`, `degraded`, `suspect`, or failed states according to trigger and workload class.
  - Output: Probe-failure aggregator, baseline-required matrix, error severity mapping, retryability model, and operator hints.
  - Validation: Failure tests prove unavailable probe commands, permission denied, node disconnect, signature mismatch, conflicting inventory, Overregistry outage, and redaction failure produce distinct states and do not silently infer capabilities.

## Phase 5: Normalization, Change Detection, And Overregistry Publication

### Work Items

- **5.1 Implement raw-to-normalized mapping.**
  - Design: Convert signed reports into stable `device_record`, `runtime_support_record`, `health_probe_result`, and `capability_observation` records using versioned mapping code and explicit confidence markers.
  - Output: Normalization engine, unit conversion table, capability taxonomy mapping, confidence model, and deterministic fixture cases.
  - Validation: Replay tests prove normalized observations can be reconstructed from report bundle, probe definitions, redaction rules, and normalization version.

- **5.2 Implement versioned capability taxonomy.**
  - Design: Use SDS #17 namespaced capability names such as `resource.cpu.arch`, `resource.memory.bytes_total`, `resource.gpu.count`, `runtime.container.oci`, `runtime.isolation.cgroup_v2`, `runtime.egress.policy_supported`, `locality.region`, and `locality.zone`, keeping vendor names and versions in values or metadata.
  - Output: Capability-name registry, schema version, metadata contract, migration rules, and compatibility tests.
  - Validation: Tests reject vendor/model/version/unit text inside capability names, unknown unregistered names, incompatible unit changes, and renaming existing CPU, GPU, storage, network, or runtime facts without migration records.

- **5.3 Implement change detection and staleness.**
  - Design: Detect newly found, missing, degraded, upgraded, downgraded, stale, and materially changed capabilities against previous snapshots and policy validity windows.
  - Output: Change detector, previous-value refs, stale-state markers, capability delta events, and policy-driven refresh windows.
  - Validation: Change tests prove additions, removals, driver upgrades, runtime changes, storage changes, network reachability changes, and sensor availability changes preserve history and produce correct reason codes.

- **5.4 Publish observations to Overregistry.**
  - Design: Publish normalized observations, snapshot refs, capability deltas, support states, confidence, freshness, probe version, redaction hash, and publication refs to Overregistry without overwriting historical records.
  - Output: Overregistry publication client, publication state, latest-summary projection, stale capability report, and `hardware_discovery.published` events.
  - Validation: Integration tests prove Overregistry receives versioned observations, keeps historical snapshots, distinguishes latest from stale/suspect/corrected observations, and exposes only scheduler-safe summaries.

- **5.5 Implement inventory and capability reads.**
  - Design: Support `GET /discovery-sessions/{session_id}`, `GET /nodes/{node_id}/inventory`, and `GET /nodes/{node_id}/capability-observations` with public summary, operator detail, restricted evidence, and tenant visibility filtering.
  - Output: Read handlers, field-policy matrix, redaction fixtures, pagination, stale markers, and stable reason-code mapping.
  - Validation: Privacy tests prove general reads hide serials, MACs, IPs, hostnames, rack/facility details, GPU UUIDs/serials, route tables, mount paths, local usernames, raw logs, stderr, and identifying sensor details.

## Phase 6: Runtime Support, Eligibility Inputs, And Verification Handoffs

### Work Items

- **6.1 Define Phase 3 fail-closed eligibility inputs.**
  - Design: Publish baseline facts required before Phase 3 private placement: node identity binding, OS/kernel compatibility, CPU architecture and cores, memory total, scratch storage, private-swarm reachability, Overcell/Overrun version and health, required runtime support, cgroup/isolation, filesystem features, and adapter readiness required by the manifest.
  - Output: Eligibility input schema, required-fact matrix, resource-class blocker reasons, and manifest-prerequisite mapping.
  - Validation: Tests prove missing, stale, unsigned, unverifiable, or `unknown` facts block only the affected workload class unless baseline node safety is affected.

- **6.2 Implement runtime support flag bundle.**
  - Design: Publish current support states for OCI/container execution, approved system-service runtime, WASI/sandbox runtime where feasible, cgroup v2 or equivalent isolation, namespace isolation, seccomp or equivalent syscall filtering, read-only root support, writable data mounts, egress policy support, and health/readiness probes.
  - Output: Runtime-support record contracts, flag taxonomy, denial reason codes, and fixture matrix.
  - Validation: Runtime tests prove missing, stale, or unknown mandatory flags return stable denials such as `runtime_fact_missing`, `workload_class_not_supported`, `system_runtime_fact_missing`, or `node_not_system_eligible`.

- **6.3 Implement Overguard policy evidence handoff.**
  - Design: Provide Overguard with normalized discovery facts, freshness, support states, redacted reason codes, and policy refs without making Hardware Discovery a policy engine.
  - Output: Policy evidence contract, dry-run input fixture, field redaction behavior, and denial reason mapping.
  - Validation: Overguard contract tests prove policy decisions cite discovery evidence but remain replayable from policy version, workload class, tenant state, package trust, and other policy-owned facts.

- **6.4 Implement Oververify challenge hooks.**
  - Design: Let Oververify and Challenge Task Service request targeted rediscovery, inspect suspect/conflicting facts, receive evidence refs, and compare discovery facts with benchmark or provider evidence without moving trust scoring into Hardware Discovery.
  - Output: Challenge trigger API/client, suspect-state linkage, refresh request events, and challenge evidence bundle refs.
  - Validation: Integration tests prove Oververify can trigger rediscovery and change eligibility through its own authority while Hardware Discovery remains observed-fact producer only.

- **6.5 Implement Benchmark Runner suite-selection handoff.**
  - Design: Provide Benchmark Runner with current discovery prerequisites, GPU/runtime facts, storage/network hints, stale markers, and required capability names for suite selection without producing measured performance.
  - Output: Benchmark prerequisite summary schema, fixture matrix, and suite-selection reason-code mapping.
  - Validation: Contract tests prove Benchmark Runner selects or blocks suites based on discovery facts and then produces separate measured-capacity evidence rather than overwriting inventory observations.

## Phase 7: Security, Privacy, Metering, And Operator Diagnostics

### Work Items

- **7.1 Implement restricted evidence bundles.**
  - Design: Build evidence bundles that link probe definitions, raw report refs, redaction manifests, normalized snapshots, signatures, Overwatch events, Overregistry refs, errors, corrections, and challenge refs.
  - Output: Bundle manifest schema, content hash, node signature refs, coordinator signature refs, redaction rules, and restricted reader policy.
  - Validation: Bundle tests prove raw host details stay restricted, public summaries are redacted, and Oververify/operator paths can prove integrity without exposing unnecessary private host data.

- **7.2 Implement operator diagnostics.**
  - Design: Provide diagnostics for stale nodes, missing prerequisites, degraded runtimes, failed probes, unsupported devices, redaction blocks, signature failures, and Overregistry publication lag.
  - Output: Operator read-model requirements, CLI/admin UI endpoint contracts, reason-code docs, and remediation hints.
  - Validation: Operator tests prove privileged diagnostics require explicit role, tenant/system scope, data class, evidence purpose, idempotency, audit, and service-account checks.

- **7.3 Emit discovery usage facts.**
  - Design: Emit system-service usage events for discovery duration, probe count, uploaded evidence bytes, normalized record writes, retry count, trigger type, requester, node id, provider id, and system service account.
  - Output: Overmeter raw usage event contract, usage event client, discovery-overhead dashboards, and trigger attribution.
  - Validation: Metering tests prove discovery overhead is visible to operators and future accounting while no per-probe charge, settlement, billing, or payout logic lives inside Hardware Discovery.

- **7.4 Implement privacy and abuse audit events.**
  - Design: Emit Overwatch events for sensitive-field redaction, blocked forbidden-field ingestion, restricted evidence access, redaction failure, signature mismatch, suspicious inventory conflict, and operator override.
  - Output: Event payload contracts, evidence refs, stable severity mapping, and restricted-event read policy.
  - Validation: Audit tests prove sensitive events include summaries and refs only, never raw host output, secrets, private files, packet contents, or unrelated personal data.

- **7.5 Add system-service runtime diagnostics.**
  - Design: Expose Phase 7 system-service readiness facts for approved runtime profile, durable local state paths, private Overmesh reachability, clock sync, audit spool availability, Overwatch path availability, Overvault secret-ref mounting, backup/snapshot readiness, and ingress/egress enforcement where required.
  - Output: System-service readiness summary, denial reasons, operator diagnostics, and candidate-node review fixtures.
  - Validation: Grid-readiness tests prove nodes missing mandatory system-service flags cannot host backbone services and ordinary workload placement does not bypass those denials.

## Phase 8: Refresh Cadence, Replay, Recovery, And Grid-Resident Readiness

### Work Items

- **8.1 Implement policy-driven refresh cadence.**
  - Design: Separate discovery refresh from Overcell heartbeat while enforcing lightweight daily drift checks, full weekly inventory refreshes, shorter volatile runtime checks, immediate refresh after install/upgrade/hardware/runtime changes, and stricter pre-placement checks for system-service candidates.
  - Output: Refresh policy engine, trigger reason codes, scheduling hooks, stale markers, and refresh-window configuration.
  - Validation: Cadence tests prove unchanged private nodes refresh on schedule, volatile GPU/runtime/storage/network facts refresh sooner, and challenge/operator/benchmark-anomaly triggers preempt normal windows.

- **8.2 Implement replay and rebuild.**
  - Design: Rebuild latest inventory, capability history, stale summaries, runtime flag summaries, and publication refs from append-only session, report, normalization, redaction, error, and publication records.
  - Output: Rebuild command, migration hooks, replay reports, corruption reports, and verification summaries.
  - Validation: Recovery tests destroy derived indexes in fixtures, rebuild them from source records, and verify snapshot, capability, runtime flag, error, publication, and latest-summary equivalence.

- **8.3 Implement schema migration and compatibility.**
  - Design: Preserve old snapshot schema versions, probe versions, redaction manifests, taxonomy versions, and conversion provenance as hardware classes and runtime support expand.
  - Output: Migration records, compatibility shims, conversion reports, and old-to-new capability mapping docs.
  - Validation: Migration tests prove old reports remain explainable, old snapshots are not silently rewritten, and new capability names do not break existing scheduler or verification consumers.

- **8.4 Prepare grid-resident operations.**
  - Design: Define Hardware Discovery as a system-service workload with protected placement, backup, restore, failover, rolling update, rollback, maintenance mode, break-glass controls, signer handling, and incident runbooks.
  - Output: Phase 7 readiness checklist, system-service manifest requirements, backup/restore fields, failover evidence refs, and signer rotation behavior.
  - Validation: Grid-readiness review confirms founder seed hardware can later be removed from the normal path without changing probe, session, report, normalization, observation, evidence, or publication contracts.

- **8.5 Implement retention and evidence lifecycle.**
  - Design: Retain raw restricted evidence only as long as needed for diagnostics, challenges, disputes, incident response, and governance while preserving compact normalized observations, content hashes, signatures, errors, and summary history.
  - Output: Retention policy records, archive refs, redaction marker behavior, evidence expiration behavior, and restricted access matrix.
  - Validation: Retention tests prove raw host details are not leaked through general reads and historical observations remain explainable after probe, schema, redaction, or taxonomy upgrades.

## Phase 9: Product, Federation, Public Provider, And Client Handoffs

### Work Items

- **9.1 Harden SDK and CLI discovery flows.**
  - Design: Provide generated Rust-first SDK and CLI flows for discovery request, session inspection, latest inventory reads, capability history reads, stale-state diagnostics, redacted evidence summaries, and operator refresh requests.
  - Output: SDK/CLI contract examples, stable JSON output, pagination, reason-code mappings, and troubleshooting flows.
  - Validation: SDK/CLI tests prove clients pass trace ids and idempotency keys, decode reason codes, respect tenant filters, and cannot invoke privileged report upload, redaction bypass, or overrides without authority.

- **9.2 Implement admin and operator visibility.**
  - Design: Expose dashboards for node inventory coverage, stale nodes, failed probes, missing prerequisites, runtime support gaps, sensitive-field redactions, publication lag, suspect reports, and system-service readiness.
  - Output: Admin read-model requirements, UI endpoint contracts, restricted evidence policy, and operator workflow checklist.
  - Validation: Admin tests prove operators can diagnose discovery evidence while tenants cannot see cross-tenant private host metadata or raw restricted evidence.

- **9.3 Define product and adapter handoffs.**
  - Design: Document how Docdex, Mcoda, Codali, AI gateway, encrypted RAG, runtime bridge, node agents, and product workloads consume scheduler-safe discovery facts through Overregistry.
  - Output: Product capability observation checklist, adapter fixture contracts, and integration scenarios.
  - Validation: Product integration tests fail when consumers bypass Overregistry, treat discovery claims as measured benchmark results, ignore stale/unknown/degraded states, or depend on Hardware Discovery private tables.

- **9.4 Add trusted federation and public-interest capacity rules.**
  - Design: Support known-organization and public-interest pools by preserving probe provenance, purpose scope, grant refs, provider refs, freshness windows, restricted evidence refs, and policy-visible capability summaries.
  - Output: Federation/public-interest discovery checklist, grant-aware observation summaries, and reporting refs.
  - Validation: Federation tests prove shared capacity remains policy-bound, evidence-backed, purpose-scoped, and explainable without exposing raw provider host details.

- **9.5 Harden public-provider anti-gaming controls.**
  - Design: Add stronger public-provider discovery controls for random refresh timing, impossible inventory claims, identity-binding suspicion, hardware-claim inconsistency, fraud refs, payout-hold refs, sandbox class limits, and low-sensitivity placement only.
  - Output: Public-provider discovery hardening checklist, anti-gaming rule set, challenge/fraud/payout-hold handoff refs, and public sandbox compatibility matrix.
  - Validation: Public-provider tests prove unknown nodes cannot become eligible for private, regulated, secret-bearing, or system-service workloads and suspicious discovery behavior reduces eligibility before payout-sensitive flows.

## Phase 10: Validation, Documentation Alignment, And Final Gates

### Work Items

- **10.1 Validate sub-build-plan structure.**
  - Design: Check title prefix, attached SDS link, phase headings 1 through 10, work-item structure, Design/Output/Validation fields, alignment review, and exit gate.
  - Output: Focused validation result for `SUB BUILD PLAN #17`.
  - Validation: Scripted text checks pass for required headings, numbered phases, local links, and work-item sections.

- **10.2 Validate tech-stack alignment.**
  - Design: Audit this plan and linked docs for Rust-first implementation, Tokio, Axum/Tower/Hyper-style HTTP, Overcell coordination, explicit GPU/runtime adapters, signed envelopes, Ed25519, BLAKE3/content hashes, canonical JSON plus JSON Schema, native Overrid boundaries, and restricted evidence handling.
  - Output: Tech-stack alignment checklist for Hardware Discovery.
  - Validation: Scans find no PostgreSQL, Redis, S3, MinIO, NATS, Kafka, Vault, blockchain, NFT, pricing, revenue, or customer-count assumptions except lines that explicitly reject those assumptions.

- **10.3 Validate master-plan alignment.**
  - Design: Confirm the master Phase 0 through Phase 13 order remains unchanged and SDS #17 is represented as a Phase 2 execution/scheduling service with later handoffs through private execution, trust/verification, accounting visibility, grid-resident operation, federation/public-provider hardening, client readers, and governance.
  - Output: Updated master-plan and crosswalk rows for SDS #17.
  - Validation: Review confirms only per-SDS sub-build indexing changed; no master phase-order dependency was altered.

- **10.4 Validate service-catalog and SDS alignment.**
  - Design: Ensure SDS #17 and the Hardware Discovery service plan link back to this sub-build plan and preserve Hardware Discovery as the observed-inventory normalization producer.
  - Output: Updated source-document and sub-build-plan references.
  - Validation: Local Markdown link validation across changed docs returns no missing local targets.

- **10.5 Prepare Hardware Discovery implementation gates.**
  - Design: Require tests for probe registration, probe compatibility, least-privilege safety, Linux CPU/memory/OS/storage/network probes, GPU/runtime adapters, redaction and never-collect enforcement, discovery request, Overcell assignment, signed report upload, lifecycle states, partial failures, normalization, capability taxonomy, change detection, Overregistry publication, read redaction, fail-closed eligibility inputs, runtime flags, Overguard/Oververify/Benchmark Runner handoffs, restricted evidence bundles, usage facts, refresh cadence, replay, retention, operator controls, SDK/CLI/admin/product/federation/public-provider handoffs, and documentation links.
  - Output: Final validation checklist for Hardware Discovery implementation.
  - Validation: Handoff review confirms Overcell, Node Installer, Overregistry, Benchmark Runner, Oversched, Overlease, Overrun, Overmeter, Overwatch, Overguard, Oververify, Challenge Task Service, Overmesh, Overvault, SDK, CLI, admin UI, adapters, federation services, public-provider services, and governance services can depend on Hardware Discovery observations without moving their runtime authority into Hardware Discovery.

## Alignment Review

- The sub-build plan keeps Hardware Discovery first build work in master Phase 2, matching SDS #17, the service catalog entry, Phase 2 plan, master build plan, and build-plan crosswalk.
- The plan treats master Phase 0 and Phase 1 as prerequisites for shared schemas, local harnesses, identity, tenant, key, registry, queue, audit, and Overwatch primitives, not as the Hardware Discovery implementation phase.
- The plan treats later phases as consumer or hardening gates: fail-closed eligibility inputs in Phase 3, policy/verification/challenge evidence in Phase 4, discovery overhead metering in Phase 5, system-service runtime flag readiness in Phase 7, trusted federation/public-interest consumption in Phase 10, public-provider anti-gaming in Phase 11, client/admin/product reads in Phase 6, and compliance/governance hardening in Phase 13.
- The plan carries forward SDS #17 resolved decisions for restricted host fields versus never-collected data, Phase 3 fail-closed eligibility gates, refresh cadence, normalized capability taxonomy, and Phase 7 system-service runtime flags.
- The plan keeps Hardware Discovery narrow: no Benchmark Runner performance measurement, no scheduler placement ownership, no policy finality, no trust score ownership, no provider eligibility judgment, no dispute adjudication, no reputation scoring, no ORU or Seal Ledger mutation, no billing or payout ownership, no arbitrary host command execution, no host mutation, and no public-provider admission authority.
- The plan does not require any change to the master Phase 0 through Phase 13 dependency order.

## Exit Gate

SUB BUILD PLAN #17 is complete when a builder can implement Hardware Discovery as the Phase 2 Rust observed-inventory evidence producer with immutable probe definitions, approved platform adapters, least-privilege probe safety, signed Overcell report upload, redaction and never-collect enforcement, deterministic normalization, versioned capability taxonomy, change and stale detection, Overregistry capability publication, redacted inventory reads, fail-closed eligibility inputs, runtime support flags, Overguard/Oververify/Benchmark Runner handoffs, restricted evidence bundles, discovery usage facts, refresh cadence, replay and migration behavior, retention policy, operator controls, SDK/CLI/admin/product/federation/public-provider handoffs, implementation validation gates, and documentation links that preserve the master Phase 0 through Phase 13 order.
