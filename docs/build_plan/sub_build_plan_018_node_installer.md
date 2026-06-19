# SUB BUILD PLAN #18 - Node Installer

Attached SDS: [docs/sds/execution_scheduling/node_installer.md](../sds/execution_scheduling/node_installer.md)

## Purpose

This sub-build plan turns SDS #18 into an implementation sequence for Node Installer. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Node Installer is the signed, auditable bootstrap and lifecycle tool for Overrid nodes. It verifies installer bundles, runs bounded preflight checks, enrolls scoped node credentials through Overkey and Overgate, writes protected Overcell config, installs a supervised Rust Overcell service, records install-session evidence, triggers Hardware Discovery after health confirmation, and supports idempotent rerun, upgrade, drain, rollback, diagnostics, and uninstall flows. It is not remote shell access, a scheduler, a trust scorer, a billing service, or a general package-management platform.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #18: Node Installer](../sds/execution_scheduling/node_installer.md) | Controls Node Installer purpose, responsibilities, data model, commands, APIs, events, state machine, security rules, validation, resolved open-question decisions, and downstream handoff. |
| [Node Installer service plan](../service_catalog/execution_scheduling/node_installer.md) | Controls service-catalog objective, first build phase, dependencies, development order, contracts, validation, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical build order from master Phase 0 through master Phase 13. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared schema, CLI, fixture, local-stack, idempotency, trace, signed-envelope, and integration-harness prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies identity, tenant, key, registry, queue, audit, Overwatch, and Overgate primitives that Node Installer depends on but does not own. |
| [Phase 2: Seed Private Swarm](phase_02_seed_private_swarm.md) | Controls the first build point for Node Installer as the seed-node bootstrap path for supervised Overcell agents, alongside Hardware Discovery, Benchmark Runner, and Overregistry capability publication. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Consumes installed, enrolled, healthy nodes as prerequisites for private execution, leases, runtime compatibility, and placement. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies policy and verification consumers for installer evidence, enrollment validity, suspicious install behavior, and challenge-triggered reinstall or repair. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies usage visibility for installer overhead without moving billing, settlement, payout, or ORU mutation into Node Installer. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies protected system-service operation, rolling updates, rollback, backup/restore, failover, signer handling, and maintenance-mode requirements. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Supplies known-organization and public-interest provider onboarding rules that later consume installer compatibility and evidence records. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public-provider onboarding, anti-abuse, sandbox, fraud, and payout-hold hardening that requires stricter installer channels and diagnostics controls. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies compliance, threat-model, incident, migration, reporting, and retention hardening for installer evidence and supply-chain controls. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #18 first build work aligned to master Phase 2, with later handoffs through private execution readiness, verification, metering visibility, grid-resident operation, provider onboarding, public-provider hardening, and governance. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services and CLI, Tokio, Axum/Tower/Hyper-style HTTP, signed envelopes, Ed25519, BLAKE3/content hashes, canonical JSON plus JSON Schema, explicit GPU/runtime adapters, Rust Overcell supervision, and native Overrid boundaries. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, and 2 | Attach SDS #18, freeze Node Installer as bootstrap/lifecycle tooling, and preserve first build in Phase 2. |
| 2 | Master Phases 0 and 2 | Build Rust CLI/service skeleton, schemas, fixtures, stable exit codes, and local harness scenarios. |
| 3 | Master Phase 2 | Implement signed bundle metadata, verification, platform compatibility, package cache, and upgrade-channel policy. |
| 4 | Master Phase 2 | Implement preflight checks, dry-run/prepared mode, side-effect boundaries, local state, and remediation output. |
| 5 | Master Phases 1 and 2 | Implement scoped enrollment, credential binding, protected config rendering, and control-plane session records. |
| 6 | Master Phase 2 | Install and operate the supervised Overcell service with health confirmation and Hardware Discovery trigger. |
| 7 | Master Phases 2 and 3 | Add idempotent status, repair, drain, restart, uninstall, rollback, diagnostics, and Phase 3 readiness checks. |
| 8 | Master Phases 4, 5, and 7 | Add audit, restricted evidence, usage facts, channel controls, and grid-resident operational hardening. |
| 9 | Master Phases 6, 10, 11, and 13 | Harden SDK, CLI, admin, trusted-provider, public-provider, incident, and governance handoffs. |
| 10 | Master Phase 2 through Phase 13 | Validate structure, tech-stack alignment, documentation links, queue state, and final implementation gates. |

## Tech Stack Guardrails

- Node Installer core is a Rust CLI and supporting execution/scheduling service module using shared contracts, Tokio where async control-plane calls are required, and Axum/Tower/Hyper-style HTTP for control-plane APIs.
- Installer bundle manifests, install sessions, enrollment-token-use records, node-agent config, supervisor records, health checks, rollback records, diagnostics manifests, API errors, exit codes, and fixtures use canonical JSON plus JSON Schema. Compact Protobuf contracts may be added only where the shared contract layer requires them.
- Mutating control-plane calls require signed command or service-account envelopes, idempotency keys, trace ids, tenant/provider scope, actor/service refs, stable reason codes, schema versions, policy/evidence refs, and append-only Overwatch events.
- Installer bundles, agent packages, diagnostics manifests, rollback manifests, package-cache entries, and local prepared-state plans use BLAKE3/content hashes and signed metadata. Ed25519 is used where operator, service-account, bundle, or node signatures are required.
- The installed node agent is the Rust Overcell process running under a host supervisor such as systemd for Phase 2 Linux-first targets. Non-systemd and non-Linux targets are later compatibility records, not Phase 2 assumptions.
- GPU/runtime support is explicit and adapter-controlled through the same Linux probe/runtime contracts used by Hardware Discovery. Phase 2 supports only the first founder seed server profile and one founder GPU-node profile on an explicitly named Ubuntu LTS baseline.
- Installer state, command spools, evidence refs, and local prepared records use Overrid-owned boundaries or Overrid-shaped local stubs during early phases. PostgreSQL, Redis, Kafka, NATS, RabbitMQ, S3, MinIO, Vault, cloud KMS, blockchain, NFT, market-token, pricing, revenue, or customer-count mechanics must not become Node Installer's product boundary.
- Node Installer does not own scheduling placement, benchmark measurement, Hardware Discovery normalization, trust scoring, policy finality, provider eligibility, ORU balances, Seal Ledger transitions, billing, payout eligibility, or public-provider admission.
- Node Installer must not provide arbitrary remote shell access. Every state-changing local command requires explicit local operator authority, stable side-effect boundaries, reason codes, and audit evidence.
- Diagnostics are redacted by default. Public provider diagnostics expose only safe summaries, hashes, state, and reason codes; restricted operator evidence requires purpose, authority, and audit.

## Phase 1: SDS Attachment, Installer Scope, And Boundary Rules

### Work Items

- **1.1 Attach the build plan to SDS #18.**
  - Design: Link this document from the numbered Node Installer SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/execution_scheduling/node_installer.md`, `docs/service_catalog/execution_scheduling/node_installer.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #18 returns both the Node Installer SDS and this sub-build plan.

- **1.2 Freeze Node Installer as bootstrap and lifecycle tooling.**
  - Design: Record that Node Installer owns signed bundle verification, install-session lifecycle, bounded preflight, enrollment-token use, protected config rendering, supervisor installation, health checks, lifecycle commands, rollback records, diagnostics manifests, and installer evidence.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms Node Installer does not own remote shell, scheduling, benchmark measurement, trust scoring, public-provider admission, billing, payouts, or arbitrary package management.

- **1.3 Preserve master Phase 2 as the first build point.**
  - Design: Keep first implementation in master Phase 2 because seed hardware needs a repeatable, signed, auditable path from clean host to enrolled Overcell service before discovery, benchmarks, and Phase 3 private execution can consume it.
  - Output: Phase-gate note that master Phase 0 and Phase 1 are prerequisites, Phase 2 is first implementation, and later phases consume or harden installer evidence.
  - Validation: Review proves this plan does not move Node Installer into Phase 0 or Phase 1 and does not change the master Phase 0 through Phase 13 order.

- **1.4 Carry forward resolved SDS decisions.**
  - Design: Preserve SDS #18 decisions for Linux-first seed matrix, live enrollment before node identity, side-effect-bounded offline prepared mode, uninstall evidence retention, public versus restricted diagnostics, and stable/beta/emergency rollback channels.
  - Output: Resolved-decision checklist tied to SDS #18 open-question answers.
  - Validation: Review rejects plans that register offline nodes, store usable enrollment credentials in prepared mode, leave private keys after uninstall, expose raw host identifiers in public diagnostics, or allow unsigned channel changes.

- **1.5 Define runtime authority boundaries.**
  - Design: Create a boundary matrix for Overcell, Overkey, Overgate, Overtenant, Overwatch, Hardware Discovery, Benchmark Runner, Overregistry, Oversched, Overlease, Overrun, Overmeter, Overguard, Oververify, Overvault, SDK, CLI, admin UI, and public-provider onboarding.
  - Output: Boundary matrix listing read/write authority, local privilege requirements, allowed events, restricted evidence, required signatures, and ownership exclusions.
  - Validation: Design review rejects direct scheduler state writes, direct trust-score mutation, host mutation outside declared lifecycle commands, unrestricted restricted-evidence reads, and billing or payout decisions inside Node Installer.

## Phase 2: Rust CLI, Control-Plane Surface, Schemas, And Fixtures

### Work Items

- **2.1 Create the Node Installer Rust CLI crate.**
  - Design: Add a Rust CLI module using shared contract types, stable JSON output, human-readable output, tracing, exit codes, config loading, platform adapters, local privilege checks, and clients for Overkey, Overgate, Overwatch, Overregistry, and Overcell health.
  - Output: CLI crate, command modules, local state abstraction, control-plane client boundary, and integration-test hooks.
  - Validation: Compile and command-start checks pass once implementation exists; crate layout review confirms Node Installer remains separate from Overcell, Hardware Discovery, Benchmark Runner, Oversched, and accounting services.

- **2.2 Define installer contract schemas.**
  - Design: Add schemas for `installer_bundle`, `install_session`, `enrollment_token_use`, `node_agent_config`, `supervisor_unit_record`, `install_health_check`, `rollback_record`, `diagnostics_bundle`, API errors, exit codes, and lifecycle events.
  - Output: JSON Schema files, Rust types, fixtures, state enums, reason-code enums, redaction-class metadata, compatibility metadata, and schema-version rules.
  - Validation: Schema tests reject missing bundle id/version/platform, trace id, idempotency key, actor ref, tenant/provider scope, node id when required, signature refs, state, reason code, audit refs, and redaction class.

- **2.3 Define seed-node install fixtures.**
  - Design: Model clean CPU seed server, clean GPU seed node, unsupported OS, missing systemd, missing network, invalid bundle signature, expired token, service-start failure, rollback success, uninstall rerun, and diagnostics redaction fixtures.
  - Output: Valid and invalid fixtures with expected state transitions, command output, Overwatch events, local state, config hashes, and control-plane session records.
  - Validation: Fixture tests prove install, rerun, upgrade, rollback, diagnostics, and uninstall produce deterministic state, stable reason codes, and no duplicate node identity.

- **2.4 Implement control-plane installer APIs.**
  - Design: Add `POST /installer/bundles`, `GET /installer/bundles/{platform}`, `POST /node-enrollments`, `POST /install-sessions`, and `GET /install-sessions/{session_id}` with signed command envelopes and scoped authority.
  - Output: API handlers or service-contract requirements, request/response schemas, idempotency behavior, pagination/read filters, and Overwatch event payloads.
  - Validation: API tests cover valid calls, duplicate idempotency keys, wrong tenant/provider scope, revoked actor, unsupported platform, stale bundle, missing trace id, unauthorized bundle registration, and restricted diagnostics reads.

- **2.5 Wire local development and harness scenarios.**
  - Design: Add local-stack and integration-harness scenarios for dry-run, signed bundle verification, successful seed install, GPU install, live enrollment, service health, discovery trigger, rollback, diagnostics, uninstall, and reconnect evidence upload.
  - Output: Local service config, fake Overkey/Overgate/Overwatch clients, fake Overcell package, systemd-equivalent test adapter, deterministic fixtures, and scenario names.
  - Validation: Local smoke tests can run installer dry-run, enroll a test node, install a supervised test service adapter, confirm heartbeat, trigger Hardware Discovery, and inspect redacted installer evidence without production credentials.

## Phase 3: Signed Bundles, Compatibility, Package Cache, And Channels

### Work Items

- **3.1 Implement installer bundle registration.**
  - Design: Support immutable bundle metadata with bundle id, version, platform, architecture, package refs, checksums, signing key refs, minimum OS/runtime requirements, compatibility class, channel, deprecation state, and rollback eligibility.
  - Output: Bundle registry APIs, bundle hash calculation, signing metadata, compatibility filters, deprecation behavior, and `node_installer.bundle_registered` events.
  - Validation: API tests cover valid registration, duplicate version, mutable metadata rejection, missing signature, missing checksum, unsupported platform, stale signing key, unknown channel, and deprecated bundle filtering.

- **3.2 Implement local bundle verification.**
  - Design: Verify signature, BLAKE3/content hash, package ref, platform selector, architecture, minimum OS/runtime requirements, channel policy, and rollback manifest before unpacking or changing service files.
  - Output: Verification engine, verified state transition, error reasons, command output, and operator remediation hints.
  - Validation: Verification tests reject unsigned bundles, checksum mismatch, wrong architecture, wrong platform, revoked signing key, downgraded target without rollback authorization, expired emergency bundle, and malformed manifest.

- **3.3 Implement Linux-first platform matrix.**
  - Design: Start with the founder seed server profile and one founder GPU-node profile on an explicitly named Ubuntu LTS baseline, using systemd supervision and the Linux runtime contracts consumed by Hardware Discovery.
  - Output: Supported-platform matrix, compatibility record schema, GPU runtime selector, systemd unit expectations, and later-platform placeholder records.
  - Validation: Compatibility tests prove other Linux distributions, ROCm when not required by the first GPU, non-systemd supervisors, mixed-vendor GPU fleets, and non-Linux hosts are rejected or marked later-compatibility until explicit records exist.

- **3.4 Implement package cache and prepared artifact handling.**
  - Design: Allow verified package artifacts to be cached locally for side-effect-bounded prepared mode without issuing node identity, storing usable enrollment credentials, or creating scheduler-visible records.
  - Output: Cache manifest, artifact hash records, prepared-state record, cleanup behavior, and offline/reconnect state rules.
  - Validation: Offline tests prove prepared mode can verify bundle, check prerequisites, render a pending plan, cache artifacts, and write local prepared state without registering the node or leaking credentials.

- **3.5 Implement channel policy.**
  - Design: Map stable, beta, and emergency rollback channels to signed bundle metadata: stable is default for seed/private nodes, beta is explicit opt-in for test or pilot nodes, and emergency rollback is scoped, expiring, signed, and policy-gated.
  - Output: Channel state model, channel-change command, compatibility rules, rollback authorization refs, and Overwatch events.
  - Validation: Channel tests reject unsigned channel changes, beta on non-opt-in production nodes, expired emergency rollback bundles, incompatible rollback targets, channel downgrade without authorization, and public-provider beta use without policy approval.

## Phase 4: Preflight, Dry Run, Prepared Mode, And Side-Effect Boundaries

### Work Items

- **4.1 Implement local preflight checks.**
  - Design: Check OS baseline, architecture, kernel, service supervisor, disk space, network reachability, clock health, package permissions, GPU runtime prerequisites, container-toolkit readiness, local user/group availability, and protected directory permissions.
  - Output: Preflight engine, check registry, stable reason codes, remediation hints, and `node_installer.preflight_started` / `node_installer.preflight_failed` events.
  - Validation: Preflight tests cover unsupported OS, wrong architecture, missing systemd, insufficient disk, no control-plane reachability, clock drift, missing GPU runtime, bad permissions, and missing local privilege.

- **4.2 Implement dry-run mode.**
  - Design: Let operators validate bundle, platform compatibility, planned file writes, service changes, enrollment prerequisites, config paths, and required privileges without side effects.
  - Output: Dry-run report schema, human/JSON output, no-side-effect guard, and planned action list.
  - Validation: Dry-run tests prove no files, services, credentials, node identities, enrollment records, or Overregistry records are created while planned changes and blockers are reported clearly.

- **4.3 Implement prepared offline mode.**
  - Design: Support prepared mode only for local bounded work: verified bundle, prerequisite checks, pending install plan, package artifact cache, and local prepared-state record.
  - Output: Prepared state, reconnect command, pending evidence spool, policy switch, and explicit blocked states for enrollment, config activation, service start, and node visibility.
  - Validation: Tests prove prepared mode cannot issue node identity, store usable enrollment credentials, start Overcell as an enrolled service, publish capability records, or make scheduler-visible state.

- **4.4 Implement side-effect transaction planning.**
  - Design: Group file writes, config writes, service unit changes, service start, enrollment, evidence upload, discovery trigger, rollback, and uninstall into ordered steps with rollback eligibility and evidence refs.
  - Output: Install action plan, step state machine, local transaction log, rollback hints, and failure cleanup behavior.
  - Validation: Failure tests prove partial file writes, interrupted service install, enrollment failure, control-plane outage, service-start failure, and evidence-upload failure leave a rerunnable state with clear next command.

- **4.5 Implement operator output and exit-code discipline.**
  - Design: Provide stable human and JSON output for preflight, install, status, upgrade, drain, diagnostics, rollback, and uninstall with reason codes that automation can consume.
  - Output: Exit-code registry, JSON schemas, output examples, remediation text, and command transcript refs.
  - Validation: CLI tests prove every failure mode has a stable exit code, machine-readable reason code, trace id, idempotency key when applicable, and no raw token or private key output.

## Phase 5: Enrollment, Credentials, Protected Config, And Install Sessions

### Work Items

- **5.1 Implement enrollment token validation.**
  - Design: Validate scoped, expiring, single-use or tightly bounded enrollment tokens through Overkey and Overgate before issuing node identity, node credentials, or scheduler-visible records.
  - Output: Enrollment client, token-use record, denial reasons, credential binding refs, and `node_installer.enrollment_requested` events.
  - Validation: Enrollment tests reject expired, reused, wrong-scope, revoked, missing-tenant, missing-provider, stale actor, and offline tokens while allowing operator to retry with a new token without reinstalling from scratch.

- **5.2 Implement node credential binding.**
  - Design: Generate or bind node credentials with OS-appropriate local protection and store only refs, hashes, and binding evidence in control-plane records.
  - Output: Credential binding flow, local protected-file permissions, Overkey refs, rotation placeholder, and revocation behavior.
  - Validation: Security tests prove private keys and enrollment secrets are never logged, never stored in control-plane records, never included in diagnostics, and removed or revoked according to uninstall policy.

- **5.3 Implement node-agent config rendering.**
  - Design: Render minimal protected config with node id, control-plane endpoint, tenant/provider scope, trust class, feature flags, cache paths, log paths, update channel, secret refs, trace refs, and schema version.
  - Output: Config schema, renderer, file permissions, config hash, redacted read model, and config migration hook.
  - Validation: Config tests reject raw secrets, world-readable config, missing tenant/provider scope, missing update channel, unknown feature flags, stale endpoint, and duplicate node identity.

- **5.4 Implement install-session recording.**
  - Design: Record install start, progress, health, rollback, diagnostics, upgrade, drain, and uninstall evidence with actor id, trace id, idempotency key, bundle refs, state, timestamps, and audit refs.
  - Output: Install-session repository, append-only transition records, terminal states, pending evidence spool, and `POST /install-sessions` integration.
  - Validation: State tests prove every transition is append-only, illegal transitions are rejected, terminal states include reason and evidence refs, and offline pending evidence uploads after reconnect.

- **5.5 Implement registration and Overregistry handoff.**
  - Design: Register node records only after live enrollment and credential binding, then keep scheduler-visible capability records blocked until Overcell heartbeat and Hardware Discovery have passed.
  - Output: Registration handoff, node identity refs, Overregistry node record, visibility state, and blocked reason codes.
  - Validation: Integration tests prove offline prepared hosts and partially enrolled hosts are not scheduler-visible, and a healthy installed node becomes inventory-visible only after Overcell heartbeat and Hardware Discovery trigger.

## Phase 6: Supervised Overcell Service, Health, Discovery Trigger, And Heartbeat

### Work Items

- **6.1 Install the supervised Overcell service.**
  - Design: Install Overcell as a systemd service for Phase 2 Linux targets with service user, install path, environment refs, restart policy, resource limits, logging, and safe start/stop behavior.
  - Output: Supervisor unit record, unit file template, service installation command, local permission checks, and `node_installer.service_installed` events.
  - Validation: Service tests prove unit creation is idempotent, permissions are narrow, existing units are upgraded through plan steps, and missing supervisor support blocks Phase 2 install with remediation.

- **6.2 Implement service start and heartbeat confirmation.**
  - Design: Start or restart the supervised Overcell service, wait for heartbeat, validate node credential use, check control-plane reachability, and record health state.
  - Output: Health-check workflow, heartbeat confirmation, degraded/failed states, timeout behavior, and `node_installer.health_check_passed` events.
  - Validation: Tests cover service start failure, heartbeat timeout, invalid credential, unreachable control plane, duplicate node id, stale config, and restart after host reboot.

- **6.3 Trigger Hardware Discovery after health.**
  - Design: Trigger Hardware Discovery only after service supervision and heartbeat are confirmed, then record discovery trigger status without owning discovery normalization.
  - Output: Discovery trigger client, status field, pending/dispatched/failed states, and handoff refs.
  - Validation: Integration tests prove Hardware Discovery is not triggered before enrollment and heartbeat, trigger failure marks install degraded rather than healthy, and discovery owns inventory facts after trigger.

- **6.4 Implement status command and read model.**
  - Design: Provide `overrid-node status` with installed version, bundle hash, service state, node id, heartbeat age, enrollment state, discovery trigger state, pending upgrade, channel, and restricted evidence refs.
  - Output: Status schema, human output, JSON output, redaction rules, and automation-friendly reason codes.
  - Validation: Status tests cover clean install, prepared offline state, degraded service, stale heartbeat, failed discovery trigger, pending upgrade, uninstalled tombstone, and restricted field redaction.

- **6.5 Implement host reboot and restart preservation.**
  - Design: Preserve node identity, service state, config refs, heartbeat behavior, and pending evidence across supervisor restart and host reboot.
  - Output: Restart test hooks, boot-state detection, service health resync, pending upload retry, and reconnect evidence upload.
  - Validation: Reboot tests prove identity is not duplicated, service restarts under supervisor policy, Overcell heartbeat resumes, pending evidence uploads once connected, and the installer remains rerunnable.

## Phase 7: Idempotent Lifecycle, Drain, Repair, Rollback, Diagnostics, And Uninstall

### Work Items

- **7.1 Implement idempotent rerun and repair.**
  - Design: Make install and repair commands detect existing state, verify bundle/config/service/evidence consistency, avoid duplicate node identities, and repair only declared drift.
  - Output: Rerun detector, repair action plan, drift report, no-op state, and reason codes.
  - Validation: Rerun tests prove repeated install does not duplicate service units, node ids, credentials, install sessions, Overregistry records, or audit refs.

- **7.2 Implement drain and restart lifecycle commands.**
  - Design: Provide `overrid-node drain` and restart flows that request drain before shutdown, upgrade, or uninstall and coordinate with Overcell/Overlease without owning scheduling.
  - Output: Drain command, drain request schema, timeout behavior, forced local stop policy, and lifecycle events.
  - Validation: Drain tests prove active leases are respected, timeout produces explicit state, scheduler authority stays outside Node Installer, and forced stops require local operator confirmation and audit.

- **7.3 Implement upgrade and rollback.**
  - Design: Verify target bundle, create rollback plan, stop/drain service as required, apply files atomically where feasible, start health checks, and rollback to previous verified bundle on failure.
  - Output: Upgrade command, rollback manifest, rollback record, previous-version refs, failure states, and `node_installer.rollback_started` events.
  - Validation: Upgrade tests cover successful upgrade, signature failure, health-check failure, interrupted upgrade, rollback success, rollback failure, channel mismatch, and evidence preservation.

- **7.4 Implement diagnostics bundles.**
  - Design: Create redacted diagnostics with service status, config hashes, bundle refs, health summaries, command transcript refs, host fact summary, discovery trigger status, reason codes, and restricted evidence separation.
  - Output: Diagnostics manifest, redaction policy, public-provider bundle profile, restricted-operator bundle profile, and retention metadata.
  - Validation: Redaction tests prove diagnostics exclude tokens, private keys, secret env vars, raw enrollment material, exact private IPs where policy requires, hostnames, serials, local usernames, unrelated logs, and raw command output from public bundles.

- **7.5 Implement uninstall and local tombstone policy.**
  - Design: Stop service, optionally drain, revoke or remove local credentials, remove service files, preserve minimal audit continuity, and support rerunnable uninstall without leaving secrets.
  - Output: Uninstall command, tombstone manifest, removed-file report, retained-evidence refs, purge option rules, and `node_installer.uninstalled` events.
  - Validation: Uninstall tests prove service binaries, unit files, raw configs, keys, tokens, caches, and sensitive logs are removed while minimal audit refs, config hashes, bundle history, uninstall receipt, revocation refs, and Overwatch refs remain where policy requires.

## Phase 8: Evidence, Usage Visibility, Channels, And Grid-Resident Operations

### Work Items

- **8.1 Implement Overwatch audit evidence.**
  - Design: Emit append-only events for bundle registration, preflight, enrollment, service install, health check, rollback, diagnostics, uninstall, channel change, prepared mode, and restricted evidence access.
  - Output: Event payload contracts, evidence refs, stable severity mapping, restricted-event read policy, and local pending-event spool.
  - Validation: Audit tests prove every state-changing command has trace id, actor/service ref, tenant/provider scope, idempotency key where applicable, reason code, and no raw secrets.

- **8.2 Emit installer usage facts.**
  - Design: Emit usage facts for download size, cache bytes, install duration, control-plane calls, diagnostics bundle size, evidence upload bytes, discovery trigger attempts, upgrade duration, and uninstall duration.
  - Output: Overmeter raw usage event contract, attribution fields, usage client, and operator visibility checklist.
  - Validation: Metering tests prove installer overhead is visible to operators while no per-command charge, settlement, billing, provider earning, or ORU mutation logic lives inside Node Installer.

- **8.3 Harden channel and release operations.**
  - Design: Add stable/beta/emergency rollback channel enforcement, signer rotation, bundle revocation, compatibility deprecation, forced upgrade notices, and emergency rollback expiry.
  - Output: Channel policy engine, revocation list handling, signer refs, compatibility reports, and operator prompts.
  - Validation: Release tests reject revoked bundles, stale signing keys, expired rollback manifests, incompatible emergency rollback, beta on disallowed nodes, and unsigned forced upgrade notices.

- **8.4 Prepare grid-resident installer service operations.**
  - Design: Define Node Installer control-plane components as protected system-service workloads with backup, restore, failover, rolling update, rollback, maintenance mode, break-glass controls, signer handling, and incident runbooks.
  - Output: Phase 7 readiness checklist, system-service manifest requirements, backup/restore fields, failover evidence refs, and signer rotation behavior.
  - Validation: Grid-readiness review confirms founder seed hardware can later be removed from the normal path without changing bundle, enrollment, install-session, diagnostics, rollback, or evidence contracts.

- **8.5 Implement evidence retention and recovery.**
  - Design: Retain restricted installer evidence only as long as needed for diagnostics, challenges, incidents, provider onboarding, disputes, and governance while preserving compact hashes, signatures, reason codes, and lifecycle summaries.
  - Output: Retention policy records, archive refs, tombstone behavior, evidence expiration behavior, replay/rebuild command, and restricted access matrix.
  - Validation: Recovery tests rebuild install-session summaries from append-only events and prove historical installs remain explainable after bundle, schema, channel, or redaction policy upgrades.

## Phase 9: SDK, CLI, Admin, Provider, Public Pool, And Governance Handoffs

### Work Items

- **9.1 Harden generated SDK and CLI installer flows.**
  - Design: Provide generated Rust-first SDK and CLI flows for bundle reads, enrollment-token creation, install-session inspection, status, diagnostics, channel reads, upgrade status, and uninstall evidence.
  - Output: SDK/CLI contract examples, stable JSON output, pagination, reason-code mappings, and troubleshooting flows.
  - Validation: SDK/CLI tests prove clients pass trace ids and idempotency keys, decode reason codes, respect tenant/provider filters, and cannot access restricted diagnostics without authority.

- **9.2 Implement admin and operator visibility.**
  - Design: Expose dashboards for bundle compatibility, install success rate, failed preflights, unhealthy services, pending prepared installs, channel distribution, rollback events, diagnostics redaction, and uninstall tombstones.
  - Output: Admin read-model requirements, UI endpoint contracts, restricted evidence policy, and operator workflow checklist.
  - Validation: Admin tests prove operators can diagnose installation evidence while tenants and providers cannot see cross-tenant private host metadata or raw restricted evidence.

- **9.3 Define private execution and product handoffs.**
  - Design: Document how Oversched, Overlease, Overrun, Overmeter, SDK, CLI, admin UI, Docdex, Mcoda, Codali, AI gateway, encrypted RAG, and runtime bridge consume installed-node status through Overregistry and Overcell rather than installer private tables.
  - Output: Product readiness checklist, adapter fixture contracts, install-health prerequisites, and integration scenarios.
  - Validation: Product integration tests fail when consumers bypass Overregistry/Overcell status, treat prepared/offline installs as eligible, ignore degraded health, or depend on Node Installer private state.

- **9.4 Add trusted-provider and public-interest onboarding rules.**
  - Design: Support known-organization and public-interest pools by preserving installer provenance, provider refs, purpose/grant refs, channel policy, bundle compatibility, diagnostics profile, and evidence refs.
  - Output: Trusted-provider install checklist, grant-aware install summaries, provider evidence refs, and onboarding reporting refs.
  - Validation: Trusted-provider tests prove capacity remains policy-bound, evidence-backed, purpose-scoped, and explainable without exposing raw provider host details.

- **9.5 Harden public-provider installer controls.**
  - Design: Add stronger public-provider rules for stable-channel-only defaults, no offline node identity, random post-install verification, fraud refs, payout-hold refs, sandbox class limits, diagnostic minimization, and revocation handling.
  - Output: Public-provider installer hardening checklist, anti-abuse rule set, fraud/payout-hold handoff refs, and public sandbox compatibility matrix.
  - Validation: Public-provider tests prove unknown nodes cannot become eligible for private, regulated, secret-bearing, or system-service workloads and suspicious installer behavior reduces eligibility before payout-sensitive flows.

## Phase 10: Validation, Documentation Alignment, And Final Gates

### Work Items

- **10.1 Validate sub-build-plan structure.**
  - Design: Check title prefix, attached SDS link, phase headings 1 through 10, work-item structure, Design/Output/Validation fields, alignment review, and exit gate.
  - Output: Focused validation result for `SUB BUILD PLAN #18`.
  - Validation: Scripted text checks pass for required headings, numbered phases, local links, and work-item sections.

- **10.2 Validate tech-stack alignment.**
  - Design: Audit this plan and linked docs for Rust-first implementation, Tokio, Axum/Tower/Hyper-style HTTP, Rust CLI, Overcell supervision, explicit GPU/runtime adapters, signed envelopes, Ed25519, BLAKE3/content hashes, canonical JSON plus JSON Schema, native Overrid boundaries, and restricted diagnostics.
  - Output: Tech-stack alignment checklist for Node Installer.
  - Validation: Scans find no PostgreSQL, Redis, S3, MinIO, NATS, Kafka, Vault, blockchain, NFT, pricing, revenue, or customer-count assumptions except lines that explicitly reject those assumptions.

- **10.3 Validate master-plan alignment.**
  - Design: Confirm the master Phase 0 through Phase 13 order remains unchanged and SDS #18 is represented as a Phase 2 execution/scheduling service with later handoffs through private execution readiness, trust/verification, accounting visibility, grid-resident operation, provider onboarding, public-provider hardening, and governance.
  - Output: Updated master-plan, Phase 2 plan, and crosswalk rows for SDS #18.
  - Validation: Review confirms only per-SDS sub-build indexing and explicit Phase 2 Node Installer wording changed; no master phase-order dependency was altered.

- **10.4 Validate service-catalog and SDS alignment.**
  - Design: Ensure SDS #18 and the Node Installer service plan link back to this sub-build plan and preserve Node Installer as the bootstrap/lifecycle tooling for supervised Overcell node agents.
  - Output: Updated source-document and sub-build-plan references.
  - Validation: Local Markdown link validation across changed docs returns no missing local targets.

- **10.5 Prepare Node Installer implementation gates.**
  - Design: Require tests for bundle registration, signature/hash verification, compatibility matrix, channel policy, package cache, preflight, dry-run, prepared mode, transaction planning, exit codes, enrollment token validation, credential binding, protected config, install sessions, Overregistry registration handoff, supervised Overcell service install, heartbeat, Hardware Discovery trigger, status, reboot preservation, rerun/repair, drain, upgrade, rollback, diagnostics, uninstall, Overwatch evidence, usage facts, release operations, grid-resident readiness, retention/recovery, SDK/CLI/admin/provider/public-provider handoffs, and documentation links.
  - Output: Final validation checklist for Node Installer implementation.
  - Validation: Handoff review confirms Overcell, Overkey, Overgate, Overtenant, Overwatch, Hardware Discovery, Benchmark Runner, Overregistry, Oversched, Overlease, Overrun, Overmeter, Overguard, Oververify, Overvault, SDK, CLI, admin UI, adapters, federation services, public-provider services, and governance services can depend on Node Installer install evidence without moving their runtime authority into Node Installer.

## Alignment Review

- The sub-build plan keeps Node Installer first build work in master Phase 2, matching SDS #18, the service catalog entry, Phase 2 plan, master build plan, and build-plan crosswalk.
- The plan treats master Phase 0 and Phase 1 as prerequisites for shared schemas, local harnesses, CLI foundations, identity, tenant, key, registry, queue, audit, Overgate, and Overwatch primitives, not as the Node Installer implementation phase.
- The plan treats later phases as consumer or hardening gates: private execution readiness in Phase 3, policy/verification and challenge evidence in Phase 4, installer overhead visibility in Phase 5, grid-resident update/rollback operations in Phase 7, product/client reads in Phase 6, trusted-provider onboarding in Phase 10, public-provider anti-abuse in Phase 11, and governance/compliance hardening in Phase 13.
- The plan carries forward SDS #18 resolved decisions for Linux-first seed support, live control-plane enrollment, side-effect-bounded prepared mode, uninstall audit continuity, public versus restricted diagnostics, and stable/beta/emergency rollback channels.
- The plan keeps Node Installer narrow: no arbitrary remote shell, no broad package-management platform, no scheduling placement ownership, no benchmark measurement ownership, no Hardware Discovery normalization ownership, no policy finality, no trust score ownership, no provider eligibility judgment, no ORU or Seal Ledger mutation, no billing or payout ownership, and no public-provider admission authority.
- The plan does not require any change to the master Phase 0 through Phase 13 dependency order.

## Exit Gate

SUB BUILD PLAN #18 is complete when a builder can implement Node Installer as the Phase 2 Rust signed bootstrap and lifecycle tool with immutable installer bundle metadata, signature and BLAKE3 verification, Linux-first compatibility records, stable/beta/emergency rollback channel policy, side-effect-bounded preflight and prepared mode, scoped Overkey/Overgate enrollment, protected node credential binding, minimal Overcell config rendering, install-session recording, supervised systemd Overcell service installation, heartbeat confirmation, Hardware Discovery trigger, idempotent rerun and repair, drain, upgrade, rollback, diagnostics redaction, uninstall tombstone policy, append-only Overwatch evidence, installer overhead usage facts, grid-resident release readiness, retention and recovery behavior, SDK/CLI/admin/product/federation/public-provider handoffs, implementation validation gates, and documentation links that preserve the master Phase 0 through Phase 13 order.
