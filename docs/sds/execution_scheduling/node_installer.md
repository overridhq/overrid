SDS #18

# Node Installer SDS

## Purpose

Make Overrid node onboarding repeatable, verifiable, reversible, and auditable for founder hardware first and later provider hardware.

Node Installer is the bootstrap and lifecycle tooling that installs the Overcell node agent, verifies the installer bundle, enrolls credentials, writes local configuration, registers the node, installs a supervised service, and captures diagnostics. It is not a remote root-control system and not a scheduler.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [node_installer.md](../../service_catalog/execution_scheduling/node_installer.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 2: Seed Private Swarm](../../build_plan/phase_02_seed_private_swarm.md) |

## Service Family

- Family: Execution and scheduling
- Owning layer: Node bootstrap, enrollment, upgrade, drain, diagnostics, and uninstall
- Primary data scope: installer bundles, install sessions, enrollment tokens, node-agent config, supervisor units, health checks, rollback records, diagnostics bundles, and audit refs
- First build phase from service plan: [Phase 2: Seed Private Swarm](../../build_plan/phase_02_seed_private_swarm.md)

## Problem Statement

Overrid starts with founder servers and GPUs, but the installation process must not become a pile of undocumented shell commands. Every node needs a reproducible path from clean host to enrolled Overcell agent, with verifiable packages, safe credential enrollment, health checks, rollback, and uninstall.

Without a proper installer contract, early bootstrap work will leak into production operations, making public provider onboarding unsafe and making grid-resident system services harder to trust.

## Goals

- Provide a repeatable install command for supported operating systems, starting with the seed hardware target.
- Verify installer bundle integrity, version, platform compatibility, and signing metadata before side effects.
- Enroll node credentials through Overkey without storing raw private material in control-plane records.
- Write a minimal node-agent config with endpoint, node id, tenant/provider scope, trust class, and feature flags.
- Install a long-running supervised Overcell service rather than a one-off script.
- Support idempotent reruns, upgrade, drain, restart, diagnostics, rollback, and uninstall flows.
- Emit Overwatch evidence for installer version, host facts, enrollment, health, and lifecycle commands.

## Non-Goals

- Do not provide arbitrary remote shell access to provider machines.
- Do not bypass host operator consent or required local privileges.
- Do not schedule workloads or decide node eligibility.
- Do not manage provider payouts, pricing, billing, or market assumptions.
- Do not store raw enrollment secrets, private keys, or host credentials in installer records.
- Do not require every future platform to be solved in Phase 2; add platforms behind explicit compatibility records.

## Primary Actors And Clients

- Founder/operator running the installer on seed servers and GPU nodes.
- Public provider onboarding flows in later phases.
- Overgate registration endpoints receiving enrollment commands.
- Overkey credential enrollment and rotation flows.
- Overcell node agent installed and supervised by the installer.
- Hardware Discovery invoked after enrollment.
- Overwatch recording install, rollback, uninstall, and diagnostics evidence.

## Dependencies

- [Overcell](overcell.md) node-agent package and local service interface.
- [Overkey](../control_plane/overkey.md) for enrollment token validation and node credential lifecycle.
- [Overgate](../control_plane/overgate.md) for registration, config, and control-plane admission.
- [Overtenant](../control_plane/overtenant.md) for tenant/provider scope and node ownership checks.
- [Overwatch](../control_plane/overwatch.md) for audit events and diagnostics refs.
- Host service supervisor such as systemd or a supported platform equivalent.
- [Hardware Discovery](hardware_discovery.md) for post-install inventory verification.

The installer may be distributed as a CLI command, package script, or signed bundle, but all forms must implement the same install-session contract.

## Owned Responsibilities

Node Installer owns:

- Installer bundle metadata, supported platform matrix, checksums, signatures, and compatibility rules.
- Install-session lifecycle and idempotency handling.
- Enrollment-token use, node-agent config rendering, local directory layout, and file permissions.
- Supervisor service installation, start, stop, drain, restart, upgrade, rollback, and uninstall commands.
- Local preflight checks for OS, kernel, disk, network, GPU runtime, permissions, and clock health.
- Diagnostics bundle generation with redaction rules.
- Operator-readable output and stable exit codes for automation.

Node Installer must call platform services through documented interfaces and leave durable audit evidence for every state-changing action.

## Data Model

The first implementation should define:

- `installer_bundle`: bundle id, version, platform, architecture, package refs, checksums, signing keys, minimum OS/runtime requirements, and deprecation state.
- `install_session`: session id, node host fingerprint hash, operator actor, provider/tenant scope, bundle id/version, state, idempotency key, trace id, timestamps, and audit refs.
- `enrollment_token_use`: token id/ref, scope, expiry, single-use status, node id issued, credential binding ref, and denial reason if rejected.
- `node_agent_config`: node id, control-plane endpoint, tenant/provider scope, trust class, feature flags, cache paths, log paths, update channel, and secret refs.
- `supervisor_unit_record`: unit name, service user, install path, environment refs, restart policy, resource limits, and installed version.
- `install_health_check`: control-plane reachability, credential validity, service status, heartbeat confirmation, discovery trigger status, and diagnostics refs.
- `rollback_record`: previous version, target version, files changed, supervisor action, reason code, and final state.
- `diagnostics_bundle`: redacted logs, config hashes, service status, host fact summary, command transcript refs, and retention policy.

Common envelope fields:

- `id`, `node_id` when known, `tenant_id` or provider scope.
- `actor_id`, `trace_id`, `idempotency_key`.
- `state`, `created_at`, `updated_at`, `policy_refs`, and `audit_refs`.

## API Surface

Node Installer has both local commands and control-plane APIs.

Local commands:

- `overrid-node install`: verify bundle, enroll, write config, install service, start agent, run health check.
- `overrid-node status`: print stable human and JSON status for automation.
- `overrid-node upgrade`: verify and apply a newer bundle with rollback plan.
- `overrid-node drain`: request node drain before shutdown, upgrade, or uninstall.
- `overrid-node diagnostics`: create a redacted diagnostics bundle.
- `overrid-node uninstall`: stop service, optionally revoke local credentials, preserve audit refs, and remove files.

Control-plane APIs:

- `POST /installer/bundles`: register installer bundle metadata. Operator/system-service only.
- `GET /installer/bundles/{platform}`: read latest compatible bundle and verification metadata.
- `POST /node-enrollments`: create or validate an enrollment token for a provider/tenant scope.
- `POST /install-sessions`: record install start, progress, health, rollback, or uninstall evidence.
- `GET /install-sessions/{session_id}`: read install status and diagnostics refs.

API and command requirements:

- Mutating control-plane calls require actor identity, tenant/provider scope, trace id, and idempotency key.
- Local commands must be rerunnable without duplicating node identities or corrupting config.
- Installer output must include stable reason codes and machine-readable JSON mode.
- Enrollment secrets may appear only in local memory or local protected files; control-plane records store refs and hashes.

## Event Surface

- `node_installer.bundle_registered`: signed bundle metadata accepted.
- `node_installer.preflight_started`: local preflight checks began.
- `node_installer.preflight_failed`: preflight failed with remediation reason.
- `node_installer.enrollment_requested`: node credential enrollment started.
- `node_installer.service_installed`: supervisor service installed or updated.
- `node_installer.health_check_passed`: Overcell heartbeat and control-plane contact confirmed.
- `node_installer.rollback_started`: rollback began for a failed install or upgrade.
- `node_installer.uninstalled`: node agent removed or disabled with audit refs.
- `node_installer.diagnostics_created`: redacted diagnostics bundle created.

Events should preserve enough evidence for later provider onboarding and incident review without exposing local secrets.

## Core Workflow

1. Operator downloads or invokes a signed installer bundle for the host platform.
2. Installer verifies signature, checksum, version, and compatibility.
3. Installer runs local preflight checks and reports clear remediation for failures.
4. Operator supplies or fetches an enrollment token.
5. Installer enrolls node credentials through Overkey/Overgate and receives node id/config refs.
6. Installer writes node-agent config with strict file permissions.
7. Installer installs the supervised Overcell service and starts it.
8. Overcell sends heartbeat and triggers Hardware Discovery.
9. Installer records session evidence and final health state in Overwatch/control-plane records.
10. Later reruns can upgrade, drain, repair, rollback, diagnose, or uninstall the node.

## State Machine

Install session lifecycle:

1. `prepared`: bundle metadata and install intent exist.
2. `verified`: bundle signature, checksum, and compatibility passed.
3. `preflight_failed`: host prerequisites failed before side effects.
4. `installing`: files and local directories are being written.
5. `enrolling`: node credential enrollment is in progress.
6. `configured`: node-agent config and permissions are in place.
7. `supervised`: host service unit is installed.
8. `healthy`: service heartbeat and registration are confirmed.
9. `degraded`: service installed but health or discovery is incomplete.
10. `failed`: install failed with rollback eligibility and reason code.
11. `rolled_back`: previous known-good state restored.
12. `uninstalled`: service removed or disabled while preserving audit evidence.

Every transition must be recorded locally and, when reachable, in control-plane audit events. Offline installs must upload pending evidence after reconnect.

## Policy And Security

- Installer bundles must be signed and checksummed; unsigned bundles are rejected.
- Enrollment tokens must be scoped, expiring, single-use or tightly bounded, and never logged.
- Local private keys must be generated or stored with OS-appropriate protection and never sent to control-plane storage.
- Config files must avoid raw secrets where Overvault or local protected refs can be used.
- Local commands that change service state require explicit operator privileges on the host.
- Diagnostics bundles must redact tokens, private keys, secret env vars, exact private IPs where policy requires, and unrelated logs.
- Upgrade and rollback must verify target versions before changing service files.
- Public-provider phases must add stronger supply-chain policy, revocation, and fraud checks before allowing unknown hosts.

## Metering And Accounting

Installer work is operational overhead, not a user workload, but usage visibility is still required:

- Emit usage facts for download size, install duration, diagnostics bundle size, control-plane calls, and discovery/heartbeat bootstrap cost.
- Attribute events to provider/operator scope and installer version.
- Keep founder hardware bootstrap overhead visible without encoding pricing or business-volume projections.
- Later provider onboarding can use the same facts for support load, eligibility, and operational reporting.

## Observability And Operations

- Operators need status for installed version, service state, node id, heartbeat age, last discovery, enrollment state, and pending upgrade.
- Health checks should verify service supervisor, config permissions, credential validity, control-plane reachability, Overcell heartbeat, and discovery trigger.
- Logs must have stable reason codes and remediation hints.
- The installer should support dry-run/preflight mode before side effects.
- Diagnostics should be small, redacted, and trace-linked to install sessions.
- Rollback and uninstall must preserve enough evidence for later disputes or incident response.

## Failure Modes And Recovery

- Unsupported OS or architecture: fail during preflight with remediation and no side effects.
- Bundle verification failure: abort before unpacking or service changes.
- Enrollment token expired: stop enrollment and allow operator to supply a new token without reinstalling from scratch.
- Control plane unreachable: allow local prepared state only if policy permits; retry enrollment later.
- Service start failure: collect diagnostics, rollback if previous version exists, and leave audit evidence.
- Health check failure: mark session degraded with clear next command.
- Upgrade failure: rollback to previous verified bundle and keep both evidence records.
- Uninstall interruption: rerun idempotently and preserve final audit state.

## Validation Plan

The service implementation plan lists these requirements:

- Installer can enroll a clean seed node.
- Re-running installer is idempotent.
- Drain and uninstall preserve audit records.

Additional SDS-level validation:

- Clean host install test for one seed server and one GPU node.
- Rerun tests proving no duplicate node identity or duplicate service unit is created.
- Bundle signature and checksum failure tests.
- Expired, reused, wrong-scope, and revoked enrollment token tests.
- Supervisor restart, host reboot, and heartbeat preservation tests.
- Upgrade, rollback, drain, diagnostics, and uninstall tests.
- Redaction tests for diagnostics bundles.
- Offline/reconnect evidence upload test if offline install mode is supported.

## Build Breakdown

1. Define bundle metadata, install-session schema, config schema, health schema, diagnostics schema, and exit codes.
2. Build Linux installer for the first founder server profile with signed bundle verification.
3. Add enrollment-token flow through Overkey/Overgate and node-agent config rendering.
4. Install Overcell as a supervised service with heartbeat confirmation.
5. Trigger Hardware Discovery and record final install health.
6. Add idempotent status, drain, uninstall, diagnostics, and rollback commands.
7. Add upgrade channels, platform matrix expansion, and public-provider onboarding integration later.

The Phase 2 deliverable is a reliable private-swarm bootstrap path, not a broad package-management platform.

## Handoff And Downstream Use

Node Installer feeds Overcell registration, Hardware Discovery, Benchmark Runner prerequisites, Overwatch audit evidence, and later public provider onboarding. Downstream systems should use install-session and node records rather than assuming a host was configured manually.

## Open Design Questions

- Phase 2 supports a narrow Linux-first seed matrix rather than a broad package-management platform. The first required target is the founder seed server profile plus one founder GPU-node profile on an explicitly named Ubuntu LTS server baseline, using the Rust Overcell agent as a supervised systemd service and the Linux probe/runtime contracts already used by Hardware Discovery. GPU support starts with the actual first founder GPU stack: NVIDIA driver plus CUDA/container-toolkit runtime when the first seed GPU is NVIDIA, or ROCm only if the first seed GPU is AMD. Other Linux distributions, ROCm when not needed by the first seed GPU, mixed-vendor GPU fleets, non-systemd supervisors, and non-Linux hosts are later compatibility records with their own signed bundle metadata, preflight probes, discovery support, benchmark evidence, and rollback tests.
- Phase 2 requires live control-plane enrollment before issuing node identity, node credentials, Overregistry node records, or scheduler-visible capability records. The first installer may support an offline prepared mode only for side-effect-bounded local work: verifying the signed bundle, checking OS/runtime prerequisites, rendering a pending install plan, caching package artifacts, and writing a local prepared-state record without registering the node or storing usable enrollment credentials. When connectivity returns, the installer must complete scoped Overkey/Overgate enrollment, bind credentials, start or restart the supervised Overcell service, upload pending Overwatch evidence, trigger Hardware Discovery, and only then allow the node to become visible for Phase 2 inventory and later Phase 3 placement.
- Which local files must remain after uninstall for audit and future reinstall continuity?
- What exact fields belong in the public provider diagnostics bundle versus restricted operator evidence?
- How should installer channels map to stable, beta, and emergency rollback releases?
