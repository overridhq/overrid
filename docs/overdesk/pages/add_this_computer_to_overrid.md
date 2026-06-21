# Add This Computer To Overrid

## Slug

`add-this-computer-to-overrid`

## Title

Add This Computer To Overrid

## Navigation Group

Network Contribution

## Description

Add This Computer To Overrid is the Overdesk onboarding flow for turning the current device into an Overrid resource provider without terminal work. It guides the user through system checks, identity and tenant scope, installer readiness, Overcell state, hardware discovery, benchmark readiness, first sharing preset, access preview, and final signed confirmation.

## Primary Users

- Individual device owners
- Founder hardware operators
- University lab admins
- Organization admins
- Resource providers
- Technical support operators helping a provider
- Delegated device managers

## Primary User Goals

- Check whether the computer can safely join Overrid.
- Choose which account, organization, institution, or tenant scope owns the node.
- Install or verify Overcell and the required local services.
- See detected CPU, GPU, RAM, storage, network, battery, thermal, and runtime facts.
- Run or schedule benchmark and verification steps.
- Choose a first resource-sharing preset.
- Review allowed users, institutions, tags, private UUIDs, resource caps, policy preview, and audit refs.
- Join the network only after owner services return authoritative registration refs.

## Entry Points

- Network Contribution navigation.
- Home Dashboard This Computer card.
- Wallet provider contribution summary.
- Provider Earnings And Payouts empty-provider state.
- Node Fleet Manager add-node action.
- Notifications Center local setup prompt.
- Address bar command: `/add-computer`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active account/scope.
- Current device name.
- Onboarding step state.
- Local service state.
- Primary action: Continue Setup.
- Secondary actions: Run Checks, Pause Setup, View Diagnostics.

### Stepper

Content:

- System Check.
- Identity Scope.
- Installer Check.
- Capability Discovery.
- Benchmark And Verification.
- First Sharing Rule.
- Access Preview.
- Final Review.
- Joined State.

### System Check

Content:

- Operating system.
- Architecture.
- CPU summary.
- GPU summary.
- RAM.
- Storage.
- Network.
- Virtualization/sandbox readiness.
- Battery profile.
- Thermal profile.
- Disk encryption state where visible.
- Unsupported or warning reasons.

### Identity And Tenant Scope

Content:

- Personal account.
- Organization account.
- Institution scope.
- Delegated setup role.
- Node owner refs.
- Node display name.
- Provider profile state.
- Signed actor identity.

Links and handoffs:

- Identity And Profile Center.
- Wallet.
- Provider Earnings And Payouts.

### Installer And Local Service Check

Content:

- Overcell state.
- Node Installer state.
- Hardware Discovery availability.
- Updater state.
- Benchmark runner state.
- Local supervisor state.
- Required OS permissions.
- Signed bundle verification state.
- Enrollment preflight state.
- Remediation actions.

Links and handoffs:

- Local Device Settings.
- Diagnostics And Support Bundles.

### Capability Discovery

Content:

- Detected compute capability.
- GPU/runtime support.
- Memory capacity.
- Storage capacity.
- Network bandwidth class.
- Uptime profile.
- Locality/coarse region where allowed.
- Safety limits.
- Redacted host detail marker.
- Capability evidence refs.

### Benchmark And Verification

Content:

- Benchmark readiness.
- Last benchmark result.
- Oververify requirements.
- Challenge-task readiness.
- Reputation bootstrap state.
- Verification blockers.
- Run benchmark action.
- Skip or schedule later action where allowed.

Links and handoffs:

- Activity And Receipts Timeline.
- Security And Compliance Reviews.

### First Sharing Rule

Content:

- Presets: idle only, night only, office hours, institution only, private UUID only, or paused until enabled.
- CPU/GPU/RAM/storage/network caps.
- Battery/thermal safety defaults.
- Allowed workload classes.
- Public low-sensitivity toggle where policy allows.
- Usage and earning projection marker.

Links and handoffs:

- Resource Sharing Rules.
- Provider Earnings And Payouts.

### Access Preview

Content:

- Institution allowlist preview.
- Organization allowlist preview.
- User allowlist preview.
- Purpose tag preview.
- Private UUID preview.
- Deny rules.
- Policy dry-run result.
- Missing prerequisite reasons.

Links and handoffs:

- Access Rules.
- Privacy And Permissions Center.

### Final Review

Content:

- Node name.
- Owner account/scope.
- Resource caps.
- First sharing rule.
- Access rules.
- Installer command summary.
- Policy result.
- Expected usage dimensions.
- Audit refs.
- Signed confirmation.
- Start/join button.

## Primary Actions

- Run system checks.
- Select owner scope.
- Install or verify Overcell.
- Run hardware discovery.
- Run benchmark.
- Choose sharing preset.
- Preview access policy.
- Join network.

## Secondary Actions

- Pause setup.
- Save draft.
- Open diagnostics.
- Export redacted support bundle.
- Open Resource Sharing Rules.
- Open Access Rules.
- Open provider payout setup.

## States

- Not started.
- Checking system.
- Unsupported device.
- Missing permission.
- Installer unavailable.
- Signed bundle verified.
- Enrollment pending.
- Hardware discovery running.
- Capability summary ready.
- Benchmark pending.
- Verification pending.
- Sharing rule draft.
- Policy preview failed.
- Ready for final review.
- Joining.
- Joined.
- Registration denied.
- Partial local-service outage.
- Offline prepared mode.
- Error with retry.

## Permissions And Privacy Behavior

- Overdesk must not expose raw full host command output, precise private locality, secrets, raw credentials, or private UUID values in logs or support bundles.
- Private UUIDs and access grants are sensitive and must be redacted by default.
- Joining requires signed actor identity, tenant/account scope, installer refs, policy refs, trace refs, and owner-service acknowledgement.
- A device cannot be marked joined until owner services return authoritative registration and audit refs.
- Overdesk may start the install/update flow and submit signed onboarding commands; Node Installer, Overcell, Hardware Discovery, Oververify, Overguard, Overregistry, Overkey, Overgate, and Overwatch own authoritative registration, capability, policy, credential, route, and audit state.

## Design Notes

- Use a linear setup stepper with a persistent summary panel.
- Keep remediation actions next to failed checks.
- Show "paused until I enable" as a safe default option.
- Do not ask users to understand scheduler internals during onboarding.
- Make final review explicit: account, device, resources, access, policy, and audit refs must be visible before join.
