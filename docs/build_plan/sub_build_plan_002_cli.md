# SUB BUILD PLAN #2 - CLI

Attached SDS: [docs/sds/foundation/cli.md](../sds/foundation/cli.md)

## Purpose

This sub-build plan turns SDS #2 into an implementation sequence for the Overrid CLI. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

The CLI is a Rust developer/operator client over the SDK and Overgate. It must make signing, idempotency, trace ids, retries, stable output envelopes, phase availability, and diagnostics routine for humans and automation. It must never become a direct storage client, secret workaround, or hidden production bypass.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #2: CLI](../sds/foundation/cli.md) | Controls command scope, profile behavior, credential references, output envelopes, exit codes, phase availability, validation, and resolved open-question decisions. |
| [CLI service plan](../service_catalog/foundation/cli.md) | Controls service-catalog objective, first build phase, dependencies, contracts, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical build order from master Phase 0 through master Phase 13. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Controls the first build point for basic CLI commands and synthetic queued workload submission. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Controls the hardened CLI path for Docdex, Mcoda, Codali, SDK, and operator automation. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps CLI aligned to master Phase 1 as the first build phase, with Phase 6 as product hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires a Rust CLI using generated contracts and SDK behavior, not a Node.js/TypeScript core client or direct database/queue/object-store access. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phase 0 and Phase 1 | Freeze CLI scope, links, command availability, and phase gates before implementation. |
| 2 | Master Phase 0 and Phase 1 | Create Rust CLI crate, shared command framework, generated contracts, and SDK integration. |
| 3 | Master Phase 1 | Build safe profiles, credential references, environment separation, and signer handoff. |
| 4 | Master Phase 1 | Define output envelopes, exit-code registry, diagnostics, and command lifecycle plumbing. |
| 5 | Master Phase 1 | Implement control-plane bootstrap commands and synthetic workload pending-state path. |
| 6 | Master Phase 1 with Phase 6 hardening | Harden signing, idempotency, trace ids, retries, reason decoding, and audit refs for automation. |
| 7 | Master Phases 2 and 3 | Add node registration, health, real workload state, logs, result, follow, cancellation, and execution diagnostics. |
| 8 | Master Phases 4, 5, and 9 | Add policy dry-run, package validation, usage, ORU, receipt, ledger, dispute, and deployment-adjacent commands as owning services exist. |
| 9 | Master Phase 6 | Harden Docdex, Mcoda, Codali, SDK, CI, and product-integration workflows without direct internal API calls. |
| 10 | Master Phase 6 with handoff to Phases 7 and 13 | Validate security, automation compatibility, release readiness, diagnostics, and later backbone/governance handoff. |

## Tech Stack Guardrails

- The CLI must be implemented in Rust and share generated contracts, transport behavior, signing helpers, retry handling, and error decoding with the Rust SDK.
- TypeScript may consume generated web bindings elsewhere, but it must not be the CLI runtime or core command implementation.
- Every platform call must pass through SDK/Overgate contracts; no command may read or mutate Overbase, Overstore, Overvault, Overqueue, Overwatch, Seal Ledger, node-agent state, or local service stores directly.
- Local development helpers may orchestrate loopback services only when they preserve final Overrid contract shapes and visible test-environment markers.
- The CLI must fail closed with stable reason codes when a backing service is unavailable in the current master phase.
- The CLI must not introduce PostgreSQL, Redis, NATS, Kafka, S3, MinIO, Vault, blockchain, NFT, pricing, revenue, or customer-count assumptions.

## Phase 1: SDS Alignment, Command Scope, And Phase Gates

### Work Items

- **1.1 Attach the build plan to SDS #2.**
  - Design: Link this document from the numbered CLI SDS, CLI service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/foundation/cli.md`, `docs/service_catalog/foundation/cli.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #2 returns both the CLI SDS and this sub-build plan.

- **1.2 Freeze the CLI boundary.**
  - Design: Record that the CLI is a Rust client over SDK/Overgate, not an owning service, private API caller, or direct storage reader.
  - Output: Boundary guardrails documented in this plan and enforced by later work items.
  - Validation: Architecture review confirms no work item requires direct service storage, queue, ledger, vault, object-store, or node-agent mutation.

- **1.3 Define command availability by master phase.**
  - Design: Treat Phase 1 as the first build point for version, doctor, profile, auth, tenant, identity, key, manifest, and synthetic workload pending-state commands. Gate later node, execution, policy, usage, receipt, package, and deployment commands by owning-service readiness.
  - Output: Command availability matrix with `available`, `hidden`, `documented_planned`, and `not_available_in_phase` states.
  - Validation: Help and `--all-phases` tests prove unavailable commands are either hidden or fail with stable `not_available_in_phase` output.

- **1.4 Align resolved open-question decisions.**
  - Design: Carry forward SDS decisions for platform-native credential storage, Phase 1 command limits, deterministic idempotency fingerprints, small numeric exit-code registry, and profile environment separation.
  - Output: Implementation checklist tied to each resolved decision.
  - Validation: Review proves the plan does not re-open or contradict the SDS resolved decisions.

- **1.5 Define documentation-update rules.**
  - Design: When a CLI command needs a new backend contract, update the owning SDS/service plan first, then update this sub-build plan and command docs before implementation.
  - Output: Cross-document maintenance rule for CLI command expansion.
  - Validation: Review checklist rejects CLI-only command behavior that is not backed by SDK/Overgate contracts.

### Phase 1 Gate Outputs

#### Link Attachment Matrix

| Document | Required attachment | Phase 1 validation |
| --- | --- | --- |
| `docs/sds/foundation/cli.md` | Links to this sub-build plan, the CLI service plan, master SDS, master service catalog, crosswalk, and Phase 1/Phase 6 build docs. | Local Markdown link check and Docdex search must return the CLI SDS and this sub-build plan for SDS #2 queries. |
| `docs/service_catalog/foundation/cli.md` | Links to the CLI SDS and this sub-build plan while preserving Phase 1 as the first build point and Phase 6 as hardening. | Service-catalog review confirms the CLI remains a terminal client over SDK/Overgate, not an owning service. |
| `docs/build_plan/master_plan.md` | Keeps SDS #2 indexed under per-SDS sub-build plans with Phase 1 as the first build point. | Master-plan row must reference `SUB BUILD PLAN #2 - CLI` and must not move CLI first build work later than Phase 1. |
| `docs/build_plan/service_catalog_alignment.md` | Keeps the CLI row and Phase 1 crosswalk entry linked to the service plan and this sub-build plan. | Crosswalk validation confirms CLI appears in Phase 1 and Phase 6 only for hardening/product integration. |
| `docs/overrid_tech_stack_choice.md` | Keeps Rust-first CLI, generated contracts, SDK behavior, signing, idempotency, trace propagation, and stable JSON output as accepted stack constraints. | Stack review rejects Node.js/TypeScript as the CLI runtime and rejects direct database, queue, object-store, vault, ledger, or node-agent access. |

#### Frozen CLI Boundary

| Boundary area | Allowed in CLI | Rejected in CLI | Enforcement note |
| --- | --- | --- | --- |
| Runtime | Rust binary crate using generated contracts and shared SDK behavior. | Node.js/TypeScript core CLI runtime or shell-only protocol implementation. | Phase 2 crate work must enter the Cargo workspace and share SDK transport/signing helpers. |
| Platform access | SDK/Overgate request construction, capability discovery, signed command envelopes, stable response decoding. | Direct reads or writes against Overbase, Overstore, Overvault, Overqueue, Overwatch, Seal Ledger, node-agent state, or service-local files. | Any command requiring state must cite the owning SDK/Overgate contract before implementation. |
| Secrets | Credential references, signer handoff, platform-native secret storage, owner-only local encrypted stores where needed. | Raw private keys, ambient CI keychains, world-readable config secrets, printed signatures, or decrypted payload output. | Profile and credential commands must fail closed before request construction when references are unsafe. |
| Local development | Loopback-only helpers with explicit fixture markers and final Overrid contract shapes. | Hidden production bypasses, private service URLs, or local mocks that drift from command envelope/output schemas. | `dev` commands are local/profile-gated and cannot be reused for seed or production-like profiles. |
| Product automation | Phase 6 hardening over the same CLI/SDK path used by humans. | Product-specific private API clients, hardcoded model/provider routing, or direct adapter internals. | Docdex, Mcoda, Codali, and later products must use documented CLI/SDK/Overgate contracts. |

#### Command Availability Matrix

| Command family | Phase 1 state | Backing contract | Help and run behavior |
| --- | --- | --- | --- |
| `overrid version` | `available` | Local CLI build, SDK compatibility, schema pins. | Visible in normal help and runnable without platform mutation. |
| `overrid doctor` | `available` | Profile, endpoint, credential-reference, schema, clock, and local config checks. | Visible in normal help; diagnostics are redacted and secret-free. |
| `overrid profile create|list|select|inspect|reset` | `available` | Local profile schema and owner-only config rules. | Visible in normal help; unsafe permissions or ambiguous profile state fail closed. |
| `overrid auth login|whoami` | `available` | Overgate plus Overkey/Overpass/Overtenant through SDK contracts. | Visible once Phase 1 capabilities exist; failures include trace id and stable reason codes where available. |
| `overrid credential enroll|inspect` | `available` | Credential-reference resolver and Overkey-lite enrollment metadata. | Visible for Phase 1; raw key material is never printed or persisted in CLI config. |
| `overrid tenant create|list|inspect` | `available` | Overtenant Phase 1 command/read contracts. | Visible for Phase 1; mutating commands require signed envelopes, expected state where relevant, reason where required, and idempotency. |
| `overrid identity create|list|inspect|disable` | `available` | Overpass-lite identity contracts. | Visible for Phase 1; tenant/actor scope is required before payload construction. |
| `overrid key enroll|list|rotate|revoke` | `available` | Overkey-lite lifecycle contracts. | Visible for Phase 1; revocation/rotation require signed command envelopes and audit refs. |
| `overrid manifest validate|submit|inspect` | `available` | Shared schema package plus Overregistry Phase 1 manifest contracts. | Visible for Phase 1; validation can run locally, submit/inspect use SDK/Overgate. |
| `overrid workload submit|status|timeline` | `available` | Overqueue pending-state and Overwatch audit contracts. | Visible for Phase 1 only as synthetic pending-state workflow; no real execution is implied. |
| `overrid dev start|stop|reset|seed|smoke` | `available` for local profiles only | Phase 0/1 local-stack helper contracts and fixture markers. | Hidden outside local/test profiles and rejected for seed, staging, production-like, and CI unless explicitly test-harness marked. |
| `overrid node register|inspect|health` | `hidden` until Phase 2 | Overcell, node credential enrollment, and Overregistry capability records. | Hidden in normal Phase 1 help; `--all-phases` may document the command and run attempts return `not_available_in_phase`. |
| `overrid workload logs|cancel|result|follow` and real execution states | `hidden` until Phase 3 | Overpack, Oversched, Overlease, Overrun, Overmeter, Overqueue, Overwatch. | Hidden in normal Phase 1 help; status/timeline must not pretend real execution exists. |
| `overrid policy dry-run` | `documented_planned` for Phase 4 | Overguard and Policy Dry-Run API contracts. | Documented in planned command refs; run attempts before capability readiness return `not_available_in_phase`. |
| `overrid usage show`, `overrid receipt show`, `overrid dispute list|inspect` | `documented_planned` for Phase 5/6 | Overmeter, ORU Account Service, Seal Ledger, Overbill, Overclaim read models. | Documented as read-only planned commands; no pricing, revenue, blockchain, or direct ledger assumptions. |
| `overrid package validate` and deployment-adjacent commands | `not_available_in_phase` until Phase 9 | Package Validator, Overpack, Deployment Planner, Release Strategy Service. | May appear only through `--all-phases` or docs; invocation before owning contracts fails with stable `not_available_in_phase`. |
| Governance, incident, compliance, migration, and backbone commands | `not_available_in_phase` until Phase 7/13 owning contracts exist | Grid-resident backbone, incident, compliance, migration, and governance service contracts. | Not visible in normal Phase 1 help and never implemented as CLI-only privileged shortcuts. |

#### Resolved SDS Decision Checklist

| SDS decision | Phase 1 implementation checklist | Validation |
| --- | --- | --- |
| Platform-native credential storage by default. | macOS uses Keychain, Linux desktop uses Secret Service/libsecret where available, headless Linux uses an Overrid-owned encrypted local store or signer socket with owner-only permissions, and CI uses explicit short-lived references. | Credential tests must prove no raw key material appears in profile config, output, debug logs, diagnostics, or fixtures. |
| Phase 1 command limits. | Expose only version, doctor, profile, auth, credential, tenant, identity, key, manifest, synthetic workload pending-state, and local-only dev helpers. | Help tests and `--all-phases` tests must prove later commands are hidden, documented planned, or fail with `not_available_in_phase`. |
| Deterministic idempotency fingerprints. | Generate default keys after local schema validation from environment class, endpoint identity, tenant id, actor id, command type, target ref, canonical payload hash, expected current state, reason, and schema version. | Idempotency tests must prove safe retries reuse the same key and intentional new operations require changed payload or explicit new-key behavior. |
| Small numeric exit-code registry. | Preserve `0`, `2`, `3`, `4`, `5`, `6`, `7`, `8`, `9`, `10`, `11`, and `12` with named exit classes in every JSON envelope. | Exit-code tests lock the registry and reject accidental renumbering before external automation depends on it. |
| Profile environment separation. | Profiles carry `local`, `seed`, `staging`, `production_like`, or `ci`, endpoint fingerprint, tenant id, actor id, credential namespace, allowed credential classes, fixture allowance, confirmation policy, and schema pins. | Profile tests prove fixture credentials cannot cross into seed/staging/production-like/CI and endpoint overrides cannot silently affect seed or production-like profiles. |

#### Documentation Update Rule

When a CLI command needs new platform behavior, update documentation in this order before implementation:

1. Update the owning SDS and service-catalog file for the backend contract or read model.
2. Update the master phase document or crosswalk if the command's owning phase or service alignment changes.
3. Update this sub-build plan's command availability matrix and the future command reference.
4. Add or update schema/fixture/SDK contract validation before wiring CLI parser behavior.
5. Reject any CLI-only command behavior that is not backed by SDK/Overgate contracts and an owning service document.

## Phase 2: Rust CLI Crate, Generated Contracts, And SDK Integration

### Work Items

- **2.1 Create the Rust CLI crate and package boundary.**
  - Design: Place the CLI in the Cargo workspace as a Rust binary crate that depends on shared contracts, SDK transport, signing helpers, and test utilities through stable internal crate boundaries.
  - Output: CLI crate skeleton with command module layout, version command, build metadata, and workspace integration.
  - Validation: `cargo check` for the CLI crate succeeds once implementation exists and no Node.js/TypeScript runtime dependency is introduced.

- **2.2 Integrate generated contracts.**
  - Design: Consume canonical JSON Schema/Protobuf-derived Rust types for commands, manifests, errors, output envelopes, idempotency records, trace context, and diagnostics instead of hand-shaped ad hoc structs.
  - Output: Generated contract import layer and schema-version compatibility checks.
  - Validation: Contract tests prove generated types reject unknown or incompatible schema versions before request submission.

- **2.3 Integrate SDK transport middleware.**
  - Design: Reuse SDK middleware for endpoint selection, Overgate request construction, auth headers, trace propagation, retries, timeout policy, stable error decoding, and response validation.
  - Output: CLI SDK client wrapper with explicit Overgate-only target validation.
  - Validation: Tests prove commands cannot target private service URLs or bypass Overgate.

- **2.4 Define command parser conventions.**
  - Design: Standardize command groups, flags, aliases, environment-variable behavior, help output, confirmation prompts, dry-run flags, and mutually exclusive options.
  - Output: Parser conventions for all initial command modules.
  - Validation: CLI usage tests cover required flags, invalid combinations, hidden commands, `--json`, `--no-color`, `--verbose`, and `--profile`.

- **2.5 Define fixture strategy.**
  - Design: Use local-stack and contract fixtures that preserve final Overrid envelopes rather than CLI-only mock payloads.
  - Output: Valid/invalid fixture set for profile, command, manifest, output, error, and diagnostic bundles.
  - Validation: Fixture tests prove the CLI and SDK agree on request and response shapes.

### Phase 2 Gate Outputs

| Gate | Artifact | Validation |
| --- | --- | --- |
| Rust workspace boundary | `Cargo.toml`, `packages/cli`, `packages/sdk`, and `packages/schemas/overrid_contracts` are Cargo workspace members. | `cargo check -p overrid-cli` succeeds and no Node.js/TypeScript CLI runtime artifact exists under `packages/cli`. |
| Generated contract projection | `packages/schemas/overrid_contracts/v0/cli_command.schema.json` and `packages/schemas/overrid_contracts/codegen_manifest.json` are the canonical schema source, and `packages/schemas/overrid_contracts/src/lib.rs` exposes CLI schema-version compatibility, output envelope, API error, trace context, idempotency, and diagnostic bundle shapes as the Rust projection. | Contract tests reject missing, malformed, unknown-family, or future CLI schema versions before request submission, and validation proves the Rust projection remains non-authoritative relative to the JSON Schema source. |
| SDK/Overgate wrapper | `packages/sdk` exposes `OvergateEndpoint`, `ClientConfig`, `OverridSdkClient`, and Overgate request metadata with private service target rejection. | SDK tests prove direct targets such as Overqueue, Overvault, Overwatch, node-agent, or service-local URLs cannot bypass Overgate. |
| Parser conventions | `packages/cli` supports `version`, `help`, `--json`, `--output`, `--no-color`, `--verbose`, `--profile`, and `--all-phases`, with stable `not_available_in_phase` output for phase-gated command families. | CLI tests cover JSON output, hidden normal help, all-phase help for every gated family, invalid and conflicting output modes in either flag order, missing profile values, and phase-gated node command behavior. |
| Fixture strategy | `packages/cli/fixtures` contains valid version output and invalid private-service endpoint fixtures that preserve final Overrid envelope fields. | `scripts/validate_cli_phase2.py` verifies fixture shape, Cargo workspace membership, Rust-first guardrails, Overgate-only behavior, no forbidden runtime terms, and exact parity between `overrid version --json` and the valid fixture. |

## Phase 3: Profiles, Credential References, And Environment Separation

### Work Items

- **3.1 Implement profile schema and storage.**
  - Design: Store named `cli_profile` records with endpoint, endpoint fingerprint, environment class, tenant id, actor id, credential namespace, schema pins, default output mode, confirmation policy, and fixture allowance.
  - Output: Profile read/write/list/select commands with owner-only file permissions for file-backed config.
  - Validation: Tests prove ambiguous profile, missing tenant, missing actor, unknown endpoint mode, and unsafe file permissions fail closed.

- **3.2 Implement credential-reference resolution.**
  - Design: Support platform-native secret stores where available, encrypted local key-store or signing-agent references where appropriate, hardware-token references where configured, and explicit CI references without raw key material in CLI config.
  - Output: `credential_reference` resolver with `keychain`, `secret_service`, `encrypted_store`, `signing_agent`, `hardware_token`, `fixture`, and `ci_reference` classes.
  - Validation: Credential tests cover missing key, revoked key, expired key, fixture isolation, local file permissions, and no raw private-key output.

- **3.3 Enforce environment class boundaries.**
  - Design: Separate `local`, `seed`, `staging`, `production_like`, and `ci` profiles before any request is built. Reject silent endpoint override for seed and production-like profiles.
  - Output: Environment guard that runs before command payload construction.
  - Validation: Tests prove local/test fixture credentials cannot be sent to seed, staging, production-like, or CI profiles unless explicitly marked as test harness profiles.

- **3.4 Implement signer handoff.**
  - Design: Route mutating commands through approved signer flows that return signature refs without exposing private key material to output, logs, or diagnostics.
  - Output: Signing abstraction used by all mutating command builders.
  - Validation: Tests prove unsigned, wrong-key, revoked-key, and mismatched-tenant signatures are rejected before submission or by Overgate before side effects.

- **3.5 Build profile safety prompts and confirmation policy.**
  - Design: Require explicit profile selection or `--confirm-profile` for mutating commands that cross from local/test to seed or production-like infrastructure; require `--reason` for admin-impacting operations.
  - Output: Confirmation and reason enforcement shared across command groups.
  - Validation: Usage tests prove high-risk operations cannot proceed through defaults, aliases, or environment variables.

### Phase 3 Gate Outputs

| Gate | Artifact | Validation |
| --- | --- | --- |
| Profile contract gate | `packages/schemas/overrid_contracts` exposes `EnvironmentClass`, `CliProfile`, `FixtureAllowance`, `ConfirmationPolicy`, owner-only file mode policy, and schema-pinned profile validation. | Contract tests reject missing tenant/actor data, unsupported schema pins, fixture credentials outside allowed environments, and unsafe group/world file permission bits. |
| Credential-reference gate | `CredentialReference` supports `keychain`, `secret_service`, `encrypted_store`, `signing_agent`, `hardware_token`, `fixture`, and `ci_reference` classes without raw key material. | Contract and CLI tests reject revoked/expired credentials, namespace mismatch, raw private-key markers, and fixture credentials outside local or explicitly marked test-harness profiles. |
| Environment guard gate | `packages/sdk` validates profile and credential references before request construction and rejects seed/production-like endpoint overrides. | SDK tests prove local fixture profiles pass, seed endpoint override fails, seed mutating commands require explicit profile confirmation, and admin-impacting mutations require `--reason`. |
| Signer handoff gate | `packages/sdk` and `packages/cli` produce signer handoff refs for mutating credential commands without exposing private key material. | Rust tests and `scripts/validate_cli_phase3.py` prove `credential enroll --json` emits `signature_ref`, `exposes_key_material:false`, and no private-key/token markers. |
| CLI command gate | `packages/cli` supports Phase 3 `profile create|list|select|inspect|reset`, `credential enroll|inspect`, `--environment`, `--endpoint`, `--endpoint-fingerprint`, `--credential-*`, `--confirm-profile`, `--reason`, and `--test-harness-profile` parser/runtime paths. | CLI tests and validator runs cover sanitized `profile inspect`, profile storage policy output, credential signer handoff, missing seed confirmation, and stable Phase 3 JSON error output. |
| Validation gate | `scripts/validate_cli_phase3.py` is wired into `scripts/validate_overrid.py` and checks docs, schema source, Rust symbols/strings, CLI command output, Cargo tests, and secret-redaction guardrails. | `docdexd run-tests --repo /Users/bekirdag/Documents/apps/overrid` must pass through the full validation wrapper before Phase 3 is reported complete. |

## Phase 4: Command Lifecycle, Output Envelope, Exit Codes, And Diagnostics

### Work Items

- **4.1 Implement command lifecycle state tracking.**
  - Design: Track parsed, profile_loaded, credential_ready, payload_validated, signed, submitted, accepted, waiting, completed, denied, and failed states for every command.
  - Output: Internal `command_context` and lifecycle recorder.
  - Validation: Tests prove every terminal path yields a state, trace id when available, reason code, retry class, and exit code.

- **4.2 Implement stable `--json` output envelope.**
  - Design: Render `ok`, `result`, `error`, `trace_id`, `audit_refs`, `reason_code`, `retry_class`, `schema_version`, `warnings`, timing, and named exit class from one canonical output envelope.
  - Output: JSON renderer plus human renderer fed by the same response model.
  - Validation: Snapshot and contract tests prove JSON remains schema-compatible and human output includes trace ids for failures.

- **4.3 Implement exit-code registry.**
  - Design: Lock the small numeric registry from the SDS: success, usage/config/credential/schema/policy/phase/idempotency/transport/timeout/platform/local I/O classes.
  - Output: Versioned `exit_code_registry` with named classes in JSON output.
  - Validation: Exit-code tests cover all initial classes and prevent accidental renumbering.

- **4.4 Implement redacted diagnostic bundles.**
  - Design: Generate support bundles with version, profile name, endpoint fingerprint, command name, schema versions, trace ids, reason codes, retry counts, and dependency status while excluding secrets and private payloads.
  - Output: `overrid doctor` and diagnostic-bundle generation.
  - Validation: Redaction scans prove bundles do not contain private keys, tokens, signatures, decrypted payloads, raw prompts, secrets, or private file contents.

- **4.5 Implement dependency and compatibility discovery.**
  - Design: Let commands query Overgate or local capability data for route availability, schema versions, and phase support before presenting or running phase-dependent command groups.
  - Output: Capability cache with visible stale age and fail-closed behavior.
  - Validation: Tests prove absent dependencies produce `not_available_in_phase` or capability errors instead of private shortcuts.

### Phase 4 Gate Outputs

| Gate | Artifact | Validation |
| --- | --- | --- |
| Lifecycle gate | `packages/schemas/overrid_contracts` exposes `CommandLifecycleState`, `CommandLifecycle`, and `CommandContext`, and `packages/cli` renders lifecycle states for success, validation failure, and phase-gated failure paths. | Rust tests and `scripts/validate_cli_phase4.py` prove terminal paths include `completed`, `denied`, or `failed` lifecycle state and never omit lifecycle metadata from JSON output. |
| Output-envelope gate | `packages/cli` renders `schema_version`, `ok`, `result`, `error`, `trace_id`, `reason_code`, `retry_class`, `exit_code`, `exit_class`, `timing_ms`, `lifecycle`, `diagnostic_bundle`, `capabilities`, `audit_refs`, and `warnings` from one canonical envelope helper. | Version, doctor, profile, credential, and phase-gated command JSON checks prove the shared envelope shape is stable and schema-compatible. |
| Exit-registry gate | `packages/schemas/overrid_contracts` locks the numeric exit classes for success, usage, config, credential, schema, policy, phase, idempotency, transport, timeout, platform, and local I/O. | Contract tests and the Phase 4 validator reject missing classes, renumbered classes, or CLI JSON output that omits the named exit class. |
| Diagnostic-bundle gate | `overrid doctor --json` and all JSON envelopes include redacted diagnostic metadata with command name, schema versions, trace ids, reason codes, retry count, dependency status, and `secret_free_refs_only` redaction policy. | Redaction scans prove diagnostic output excludes raw keys, tokens, signatures, decrypted payloads, raw prompts, private payloads, and private file contents. |
| Capability-discovery gate | CLI capability metadata reports local cache source, stale age, schema versions, route availability, phase gate, and fail-closed behavior for unavailable routes. | `policy dry-run --json` and other unavailable routes return `not_available_in_phase`, `exit_class:"phase"`, `fail_closed:true`, and no private-service shortcut output. |
| Validation gate | `scripts/validate_cli_phase4.py` is wired into `scripts/validate_overrid.py` and checks docs, schema source, manifest, Rust symbols, emitted CLI JSON, fixtures, redaction, and Cargo tests. | `docdexd run-tests --repo /Users/bekirdag/Documents/apps/overrid` must pass through the full validation wrapper before Phase 4 is reported complete. |

## Phase 5: Phase 1 Control-Plane Bootstrap Commands

### Work Items

- **5.1 Implement `version`, `doctor`, and profile commands.**
  - Design: Provide basic observability and local configuration workflows before any mutating platform operation.
  - Output: `overrid version`, `overrid doctor`, and profile create/list/select/inspect/reset commands.
  - Validation: Smoke tests prove a clean local profile can be created, inspected, diagnosed, and selected without secrets in output.

- **5.2 Implement `auth` and credential commands.**
  - Design: Build `auth login`, `auth whoami`, credential enrollment, and credential inspection on top of Overkey/Overpass/Overtenant through the SDK path.
  - Output: Auth command group that resolves actor, tenant, roles, key id, endpoint, and schema version.
  - Validation: Tests cover missing credential, disabled actor, revoked key, wrong tenant, and successful whoami.

- **5.3 Implement tenant, identity, and key lifecycle commands.**
  - Design: Support Phase 1 tenant setup and identity/key lifecycle with signed mutating envelopes, expected-state fields, reasons where needed, and idempotency keys.
  - Output: `tenant create|list|inspect|suspend`, `identity create|list|inspect|disable`, and `key enroll|list|rotate|revoke`.
  - Validation: Integration tests prove the CLI can create tenant, identity, and credential records and that duplicate idempotency keys are deterministic.

- **5.4 Implement manifest validation and submission.**
  - Design: Validate resource, workload, package, provider, and native app manifests locally, then submit accepted Phase 1 manifests through Overgate/Overregistry.
  - Output: `manifest validate|submit|inspect` with local validation, signed submission, and versioned immutable refs.
  - Validation: Tests cover invalid schema, unsupported manifest version, missing signature, duplicate submit, and accepted manifest refs.

- **5.5 Implement synthetic workload pending-state commands.**
  - Design: Let Phase 1 submit synthetic workload commands that stop at durable Overqueue pending state and expose status/timeline without pretending real execution exists.
  - Output: `workload submit|status|timeline` for Phase 1 synthetic work.
  - Validation: End-to-end test proves signed workload command reaches pending queue state with complete Overwatch audit chain.

## Phase 6: Signing, Idempotency, Retries, Traceability, And Error Decoding

### Work Items

- **6.1 Implement canonical idempotency fingerprints.**
  - Design: Generate default idempotency keys from environment class, endpoint identity, tenant id, actor id, command type, target ref, canonical payload hash, expected current state, reason, and schema version after local schema validation.
  - Output: Deterministic idempotency module with `--idempotency-key` and `--new-idempotency-key` behavior.
  - Validation: Tests prove safe retries reuse the same key and intentional new operations require changed payload or explicit new-key behavior.

- **6.2 Implement retry and timeout policy.**
  - Design: Use SDK retry classes and bounded timeouts so retryable transport/platform failures are retried safely while schema, auth, policy, and phase-unavailable failures stop immediately.
  - Output: Shared retry controller used by every command group.
  - Validation: Tests cover retryable transport errors, platform unavailable, timeout while waiting, and non-retryable denials.

- **6.3 Implement trace propagation and audit refs.**
  - Design: Generate or accept trace ids, pass them through SDK/Overgate, and preserve returned Overwatch/audit refs in human and JSON output.
  - Output: Trace context module plus output rendering for audit refs.
  - Validation: Tests prove failures and successful mutating commands display trace ids and audit refs where available.

- **6.4 Implement stable error decoding.**
  - Design: Decode Overgate, policy, schema, idempotency, credential, dependency, timeout, and platform terminal errors into stable reason codes, retry classes, exit classes, and remediation hints.
  - Output: Error decoder shared by all commands.
  - Validation: Golden tests lock output for expected reason-code families and prevent raw internal errors from leaking.

- **6.5 Implement local idempotency cache.**
  - Design: Optionally persist command fingerprint to idempotency key mapping for safe retries, scoped by profile/environment and never containing private payloads.
  - Output: Owner-only local cache with reset/inspect behavior.
  - Validation: Tests prove profile switches and tenant switches cannot reuse cached keys across unsafe boundaries.

## Phase 7: Seed Private Swarm And Real Execution Commands

### Work Items

- **7.1 Add Phase 2 node commands.**
  - Design: Expose node registration, inspection, and health only after Overcell, Overregistry capability records, and node credential enrollment contracts exist.
  - Output: `node register|inspect|health` with profile-scoped credential checks and capability refs.
  - Validation: Tests prove Phase 1 profiles receive `not_available_in_phase` and Phase 2 fixtures show live, stale, expired, draining, and disabled node states.

- **7.2 Add Phase 3 workload execution commands.**
  - Design: Extend workload commands from synthetic pending state to scheduled, leased, running, succeeded, failed, cancelled, timed out, and dead-lettered real execution states.
  - Output: `workload status|timeline|logs|cancel|result|follow` backed by Overgate, Overqueue, Oversched, Overlease, Overrun, and Overwatch refs.
  - Validation: End-to-end tests cover successful, retryable failed, final failed, cancelled, timed-out, and dead-lettered workloads.

- **7.3 Add log and result retrieval.**
  - Design: Retrieve logs and results through authorized control-plane refs and SDK/Overgate contracts, not direct node callbacks or object-store paths.
  - Output: Log/result commands with redaction, bounded streaming, and trace-linked refs.
  - Validation: Tests prove private payloads and raw node paths are not printed unless owning-service authorization explicitly permits the view.

- **7.4 Add follow-mode and polling behavior.**
  - Design: Use capability-gated event streams where available, otherwise fall back to bounded polling with visible stale age and timeout behavior.
  - Output: `--wait`, `--timeout`, `--poll-interval`, and `--follow` behavior for long-running commands.
  - Validation: Tests cover streaming unavailable, polling timeout, terminal success, terminal failure, and interrupted follow mode.

- **7.5 Add execution diagnostics.**
  - Design: Surface scheduler, lease, runner, node heartbeat, package, and result-state reason codes in one trace-linked command timeline without exposing private storage.
  - Output: CLI timeline renderer for execution diagnostics.
  - Validation: Fixture tests prove each state cites owning-service refs and stable reason codes.

## Phase 8: Policy, Package, Usage, Receipt, And Accounting Commands

### Work Items

- **8.1 Add policy dry-run commands.**
  - Design: Expose Phase 4 policy dry-run through Overguard/Policy Dry-Run API with workload class, data sensitivity, quota, package trust, egress, provider eligibility, budget placeholder, and reason-code output.
  - Output: `policy dry-run` with human and JSON renderers.
  - Validation: Tests cover denied egress, wrong tenant, insufficient trust, quota exhaustion, unsupported workload class, and accepted dry-run.

- **8.2 Add package validation commands.**
  - Design: Expose package validation before submit, and later Phase 9 package/deployment validation, without accepting unsigned or unprovenanceable artifacts.
  - Output: `package validate` with schema, signature, hash, dependency, permission, SBOM/provenance, and policy compatibility checks as backing contracts mature.
  - Validation: Tests prove invalid package, unsupported version, missing provenance, and policy-incompatible package fail before submit.

- **8.3 Add usage and ORU read commands.**
  - Design: Show Overmeter rollups, ORU balances, holds, reservations, corrections, and budget denials through read APIs without creating payment behavior.
  - Output: `usage show` and balance-oriented read output where owning services expose the view.
  - Validation: Tests cover CPU-ORU, GPU-ORU, STOR-ORU, NET-ORU, MEM-ORU, DATA-ORU, budget exhausted, and disputed usage.

- **8.4 Add receipt and ledger reference commands.**
  - Design: Render Seal Ledger/Overbill refs, receipts, invoice status, refund refs, correction refs, payout-hold refs, and audit refs without encoding pricing assumptions.
  - Output: `receipt show` and ledger-ref inspection commands.
  - Validation: Revenue-assumption scans confirm no pricing, customer-count, or market-volume projections enter CLI docs or output fixtures.

- **8.5 Add dispute evidence read commands.**
  - Design: Surface Overclaim cases, evidence refs, hold status, correction refs, and resolution state through authorized read models.
  - Output: `dispute list|inspect` or equivalent command group once owning service contracts exist.
  - Validation: Tests prove the CLI cannot directly mutate ledger/dispute state and all reads are tenant/role filtered.

## Phase 9: Product Integration And Automation Hardening

### Work Items

- **9.1 Harden Docdex workflows.**
  - Design: Provide CLI/SDK examples and commands for Docdex encrypted indexing, search/retrieval job submission, result inspection, cancellation, and usage/receipt lookup without direct internal APIs.
  - Output: Product fixtures and command recipes for Docdex workloads.
  - Validation: Product tests prove Docdex can submit jobs, inspect trace state, retrieve authorized results, and see usage through CLI/SDK paths.

- **9.2 Harden Mcoda workflows.**
  - Design: Support agent workload submission, model/resource-selection metadata, tool-boundary declarations, failure reason propagation, budget checks, and usage reporting.
  - Output: Mcoda product fixtures and command recipes.
  - Validation: Tests prove Mcoda workloads remain provider/model dynamic and do not hardcode one model, node, or paid-service assumption.

- **9.3 Harden Codali workflows.**
  - Design: Support code-agent package execution, repository context refs, logs/artifacts, structured results, retry/repair boundaries, and per-agent-phase usage output.
  - Output: Codali product fixtures and command recipes.
  - Validation: Tests prove logs and artifacts are retrieved through authorized refs and policy/resource failures surface actionable reason codes.

- **9.4 Harden CI and script automation.**
  - Design: Treat CI as an explicit `profile_kind=ci` with short-lived service-account, signing-agent, fixture, hardware-token, or mounted-secret references; never use ambient persistent keychain defaults.
  - Output: CI-safe examples, JSON-output stability tests, and non-interactive confirmation behavior.
  - Validation: Automation tests prove `--json` output is stable, secret-free, and sufficient for scripts to branch on exit classes.

- **9.5 Harden product command docs.**
  - Design: Document supported product workflows, phase requirements, expected failure modes, and safe retry patterns without leaking internal services or requiring raw HTTP calls.
  - Output: CLI command reference and product integration examples aligned to SDS #2.
  - Validation: Docs review confirms examples route through SDK/Overgate and use no direct internal API or private storage access.

## Phase 10: Validation, Release Readiness, Security Review, And Handoff

### Work Items

- **10.1 Run contract and snapshot validation.**
  - Design: Lock command schemas, output envelopes, exit codes, help text, JSON output, human output, and error decoding before external automation depends on them.
  - Output: Contract, golden, snapshot, and compatibility test suite.
  - Validation: Tests cover every command group, every exit-code class, every reason-code family, and backward-compatible `--json` output.

- **10.2 Run security and redaction validation.**
  - Design: Exercise credential storage, signer handoff, file permissions, environment separation, debug output, diagnostics, log/result redaction, and cross-tenant access.
  - Output: CLI security test report tied to SDS validation.
  - Validation: Tests and scans prove raw keys, tokens, signatures, secrets, private payloads, decrypted content, and unsafe endpoints do not appear in output, logs, or bundles.

- **10.3 Run phase-availability validation.**
  - Design: Test the CLI against fixtures representing Phases 1, 2, 3, 4, 5, 6, and unavailable later services.
  - Output: Phase matrix proving each command is available, hidden, or denied exactly as planned.
  - Validation: `not_available_in_phase` behavior is stable and no command silently uses private shortcuts.

- **10.4 Run integration and product reliability validation.**
  - Design: Exercise tenant setup, identity/key lifecycle, manifest submit, synthetic workload, real private job, policy dry-run, package validation, usage/receipt lookup, cancellation, timeout, retry, and product workflows.
  - Output: End-to-end validation matrix with trace ids and audit refs.
  - Validation: `docdexd run-tests` or the repo's canonical test command passes once implementation exists; missing test-runner configuration remains recorded as an environment blocker until configured.

- **10.5 Prepare Phase 7 and Phase 13 handoff.**
  - Design: Document which CLI commands remain disabled or read-only until grid-resident backbone operations, governance, compliance, incident response, break-glass, and migration tooling have owning-service contracts.
  - Output: Handoff notes for system-service operations, migration, incident, compliance, and governance command families.
  - Validation: Handoff review confirms Phase 6 CLI completion does not authorize high-risk Phase 7 or Phase 13 operations early.

## Alignment Review

- The sub-build plan keeps CLI first build work in master Phase 1, matching the SDS, service catalog entry, and build-plan crosswalk.
- The plan treats master Phase 6 as hardening for real product integrations, matching both SDS #2 and Phase 6 Workstream 2.
- The plan explicitly gates node, execution, policy, accounting, package, deployment, governance, and backbone operations behind their owning master phases.
- The plan does not require any change to the master Phase 0 through Phase 13 dependency order.
- The plan adds only a more detailed per-SDS implementation layer under `docs/build_plan`.
- The plan respects the accepted tech stack: Rust owns the CLI, SDK integration, generated contract use, signing, idempotency, trace handling, diagnostics, and stable JSON output.

## Exit Gate

SUB BUILD PLAN #2 is complete when a builder can implement the CLI as a Rust SDK/Overgate client that bootstraps Phase 1 control-plane workflows, expands safely through later owning-service contracts, and hardens Phase 6 product automation without violating credential safety, phase gates, tenant isolation, auditability, tech-stack boundaries, or the canonical master build order.
