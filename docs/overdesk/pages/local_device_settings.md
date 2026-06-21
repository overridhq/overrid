# Local Device Settings

## Slug

`local-device-settings`

## Title

Local Device Settings

## Navigation Group

System And Help

## Description

Local Device Settings is the Overdesk page for the current desktop device profile, local permissions, hardware discovery summary, network state, local stack targets, resource-contribution readiness, storage paths, platform integrations, power behavior, and device-level troubleshooting. It shows what the desktop shell can observe or configure locally without turning Overdesk into the hardware discovery authority, node registration authority, scheduler, installer executor, or trust scorer.

## Primary Users

- Regular users
- Provider operators
- Developers
- App owners
- Organization admins
- Institution admins
- Support operators
- Node operators

## Primary User Goals

- See local device identity, trust, compatibility, and platform state.
- Review hardware capability summaries used by Overdesk and resource-contribution flows.
- Manage local permissions for notifications, files, network, location, camera/media where relevant, and background tasks.
- Inspect connectivity to Overgate, local stack endpoints, node services, cache, vault/key providers, and update services.
- Troubleshoot installer, local stack, hardware discovery, network, and permission issues.
- Open resource sharing, cache, diagnostics, and update pages from one device hub.

## Entry Points

- System And Help navigation.
- Settings And Security.
- Add This Computer To Overrid.
- Node Detail.
- Node Fleet Manager.
- Developer Console.
- Diagnostics And Support Bundles.
- Updates And Release Notes.
- Address bar command: `/device`.

## Sections To Have

### Page Header

Content:

- Page title.
- Device display name.
- Device ref.
- Platform and OS version.
- App version.
- Device trust state.
- Hardware discovery state.
- Network state.
- Local stack state.
- Primary action: Run Device Check.
- Secondary actions: Permissions, Local Stack, Storage, Diagnostics, Updates.

Links and handoffs:

- Settings And Security.
- Diagnostics And Support Bundles.
- Updates And Release Notes.

### Device Identity And Trust

Content:

- Device ref.
- Local install id.
- Signed account scope.
- Organization/institution scope.
- Trust marker.
- Last verified time.
- Enrollment state.
- Revocation state.
- Owner-service acknowledgement refs.
- Fresh verification requirement for sensitive actions.

Links and handoffs:

- Identity And Profile Center.
- Settings And Security.
- Activity And Receipts Timeline.

### Hardware Capability Summary

Content:

- CPU class.
- Memory class.
- GPU capability class.
- Storage class.
- Network class.
- Runtime support flags.
- Accelerator availability.
- Thermal/power constraints.
- Last discovery time.
- Discovery error codes.

Links and handoffs:

- Add This Computer To Overrid.
- Resource Sharing Rules.
- Node Detail.
- Diagnostics And Support Bundles.

### Local Permissions

Content:

- Notification permission.
- File/folder access permission.
- Background task permission.
- Network permission.
- Location permission where allowed.
- Camera/media permission where needed by native apps.
- Platform credential-provider permission.
- Startup/login-item permission.
- Required/recommended/optional marker.
- Fix permission controls.

Links and handoffs:

- Settings And Security.
- Social Photo/Video.
- Maps And Navigation.
- Overvault Secure Storage Center.

### Network And Routing

Content:

- Overgate reachability.
- DNS/address resolution state.
- Local route health.
- Proxy/VPN marker where detectable.
- Latency class.
- Offline state.
- Captive network warning.
- Public/private network marker.
- Last connection receipts.

Links and handoffs:

- Overrid Browser.
- Global Search.
- Local Cache And Offline Sync.
- Diagnostics And Support Bundles.

### Local Stack And Developer Targets

Content:

- Local stack enabled state.
- Local Overrid services.
- Endpoint refs.
- Health state.
- Test environment marker.
- SDK/CLI version marker.
- Manifest preview setting.
- Debug log state.
- Local data reset controls.

Links and handoffs:

- Developer Console.
- Deploy New App.
- Settings And Security.
- Diagnostics And Support Bundles.

### Resource Contribution Readiness

Content:

- Node registration state.
- Resource sharing rule summary.
- Access rule summary.
- Benchmark state.
- Installer state.
- Eligibility warnings.
- Day/night/hour rule summary.
- Resource percentage summary.
- Private UUID/tag/institution access markers.

Links and handoffs:

- Add This Computer To Overrid.
- Resource Sharing Rules.
- Access Rules.
- Node Fleet Manager.

### Storage And Local Paths

Content:

- App data path.
- Cache path.
- Offline queue path.
- Diagnostics path.
- Download/export path.
- Available storage.
- Encrypted-store state.
- Cleanup recommendations.
- Open safe folder action.

Links and handoffs:

- Local Cache And Offline Sync.
- Diagnostics And Support Bundles.
- Settings And Security.

### Power Thermal And Background Behavior

Content:

- Battery/AC state.
- Low-power mode marker.
- Background execution permission.
- Thermal pressure marker.
- Pause contribution on battery setting.
- Night/day rule interaction.
- Sleep/wake behavior.
- Resource throttling hints.

Links and handoffs:

- Resource Sharing Rules.
- Node Detail.
- Settings And Security.

### Device Actions And Repair

Content:

- Run device check.
- Refresh hardware discovery.
- Recheck permissions.
- Reconnect local stack.
- Repair local cache refs.
- Reset local shell state.
- Export diagnostics.
- Open support bundle flow.
- Unenroll device where allowed.

Links and handoffs:

- Diagnostics And Support Bundles.
- Local Cache And Offline Sync.
- Settings And Security.

## Primary Actions

- Run device check.
- Refresh hardware discovery.
- Recheck permissions.
- Open local stack.
- Open resource contribution setup.
- Open cache/offline sync.
- Export diagnostics.
- Open updates.

## Secondary Actions

- Rename local device display name.
- Copy safe device ref.
- Open device activity.
- Reset local shell state.
- Reconnect Overgate.
- Open support bundle.
- Ask AI to explain a device issue.

## States

- Loading.
- Live.
- Device unsupported.
- Permission missing.
- Hardware discovery unavailable.
- Network offline.
- Overgate unreachable.
- Local stack unavailable.
- Device not trusted.
- Device enrolled.
- Device revoked.
- Repair required.
- Diagnostics ready.
- Action denied.

## Permissions And Privacy Behavior

- Overdesk may store local shell settings, non-secret device refs, local path preferences, and device-display metadata.
- Hardware discovery summaries must be bounded and classed; unrestricted host detail, secrets, raw serials, private hardware identifiers, and unrelated local files must not be collected.
- Hardware Discovery owns normalized capability observations; Benchmark Runner owns measured performance evidence; schedulers and trust services own placement and eligibility decisions.
- Node registration and contribution state remain owned by node/install/resource services, not by Overdesk.
- Permission changes must use platform-approved prompts and must explain why a permission is required.
- Diagnostic exports must be redacted and user-reviewed before leaving the device.

## Design Notes

- Use a compact device-health dashboard with clear problem rows and direct repair actions.
- Show capability classes instead of raw host dumps.
- Keep local developer controls visually separated from normal user controls.
- Use warning states for missing permissions only when a current workflow needs the permission.
- Avoid showing private UUIDs, secret refs, raw tokens, or full hardware fingerprints.
- Make this page feel like a practical control panel, not a generic preferences page.
