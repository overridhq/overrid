# Deploy New App

## Slug

`deploy-new-app`

## Title

Deploy New App

## Navigation Group

Apps, Deployment, And Operations

## Description

Deploy New App is the Overdesk deployment wizard for publishing or updating Overrid apps without manually coordinating every backend service. It guides builders through source selection, manifest validation, resource estimation, policy dry-run, namespace routes, release strategy, deployment planning, final signed confirmation, and launch monitoring.

## Primary Users

- Developers
- App owners
- Delegated app managers
- Organization admins
- Institution app operators
- Grant/project builders
- Support operators helping an app owner

## Primary User Goals

- Start from a package, repo/import, template, existing app version, or local build output.
- Validate the app manifest before any deployment planning.
- See required permissions, data classes, resource needs, storage refs, namespace needs, workload class, and policy refs.
- Estimate compute, GPU, RAM, storage, bandwidth, wallet budget, and grant availability.
- Run policy dry-runs before signing deployment.
- Choose namespace routes, app routes, subroutes, visibility, and dispute/contact info.
- Review deployment graph, release strategy, health checks, backup/restore refs, and rollback plan.
- Launch the deployment and monitor status, logs, health, errors, rollback readiness, and replay refs.

## Entry Points

- Owned Apps primary action.
- App Detail deploy action.
- Developer Console deploy preview.
- Native App Catalog developer action.
- Release And Rollback Manager new release action.
- Grants And Public-Interest Projects build/deploy handoff.
- Address bar command: `/deploy-app`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active owner scope.
- Deployment draft state.
- Current wizard step.
- Validation state.
- Wallet/grant precheck state.
- Primary action: Continue.
- Secondary actions: Save Draft, Validate, Cancel.

### Wizard Stepper

Content:

- Select Source.
- Validate Manifest.
- Estimate Resources.
- Policy Dry Run.
- Namespace And Routes.
- Deployment Plan.
- Final Confirmation.
- Launch Monitor.

### Select Source

Content:

- Overpack manifest selector.
- Repo/package import.
- Template selector.
- Existing app version selector.
- Local build output selector.
- Source ref.
- Package hash/ref.
- Import status.
- Source trust marker.
- Unsupported source reason codes.

Links and handoffs:

- Developer Console.
- Overasset Assets.

### Manifest Validation

Content:

- Package identity.
- App name and version.
- Declared permissions.
- Data classes.
- Storage refs.
- Namespace needs.
- Resource needs.
- Workload class.
- Policy refs.
- Package Validator results.
- Stable reason codes.
- Blocking and warning results.

Links and handoffs:

- Developer Console.
- Privacy And Permissions Center.
- Security And Compliance Reviews.

### Resource Estimate

Content:

- Compute estimate.
- GPU estimate.
- RAM estimate.
- Storage estimate.
- Bandwidth estimate.
- Model-route or AI/RAG estimate where applicable.
- Expected ORU dimensions.
- Wallet precheck.
- Grant availability.
- Sponsored credits.
- Budget warning.
- Estimate confidence.

Links and handoffs:

- Wallet.
- Grants And Public-Interest Projects.

### Policy Dry Run

Content:

- Overguard decision.
- Workload Classifier output.
- Package Validator output.
- Oververify requirement marker.
- Compliance Boundary marker.
- Data-class policy results.
- Resource eligibility result.
- Reason codes.
- Required fixes.
- Dry-run replay refs.

Links and handoffs:

- Security And Compliance Reviews.
- Activity And Receipts Timeline.

### Namespace And Routes

Content:

- Namespace route selector.
- `/name` choice.
- App route.
- Subroutes.
- Visibility: public, private, institution, organization, or grant-linked.
- Route conflict state.
- Route trust marker.
- Contact and dispute info.
- Route policy dry-run state.

Links and handoffs:

- Namespace Manager.
- Overrid Browser.
- Native App Catalog.

### Deployment Plan

Content:

- Deployment Planner graph.
- Release strategy.
- Rollout mode.
- Health checks.
- Backup/restore refs.
- Rollback plan.
- Route-shift plan.
- Storage/data migration marker.
- Usage refs.
- Expected launch timeline.
- Blockers and warnings.

Links and handoffs:

- Release And Rollback Manager.
- Developer Console.
- App Detail.

### Final Confirmation

Content:

- App/source refs.
- Owner account/scope.
- Wallet/grant precheck.
- Namespace routes.
- Permissions and data classes.
- Resource estimate.
- Policy dry-run result.
- Release strategy.
- Rollback path.
- Audit refs.
- Signed command confirmation.

### Launch Monitor

Content:

- Deployment state.
- Current step.
- Logs with redaction.
- Health checks.
- Route binding state.
- Error list.
- Retry/cancel/rollback options where allowed.
- Support bundle/replay refs.
- Open app action.
- Open app detail action.

Links and handoffs:

- App Detail.
- Release And Rollback Manager.
- App Incidents And Support.
- Activity And Receipts Timeline.

## Primary Actions

- Select source.
- Validate manifest.
- Estimate resources.
- Run policy dry-run.
- Choose namespace route.
- Generate deployment plan.
- Sign deployment.
- Launch deployment.
- Open launch monitor.

## Secondary Actions

- Save draft.
- Cancel draft.
- Import template.
- Open developer console.
- Open wallet precheck.
- Open grant details.
- Export deployment preview.
- Ask AI to explain blockers.

## States

- Empty draft.
- Source selected.
- Import running.
- Manifest invalid.
- Manifest valid.
- Estimate running.
- Underfunded.
- Grant eligible.
- Policy dry-run running.
- Policy blocked.
- Route conflict.
- Plan generated.
- Awaiting signature.
- Deployment queued.
- Deploying.
- Partially deployed.
- Deployment healthy.
- Deployment failed.
- Rollback available.
- Permission denied.
- Partial owner-service outage.
- Offline draft only.
- Error with retry.

## Permissions And Privacy Behavior

- Deploy New App owns wizard state and desktop display only. Deployment Planner, Overpack, Package Validator, Release Strategy Service, Overguard, Overregistry, Overgate, Overbase, Overstore, Overvault, Overmeter, Wallet/ORU, and owner services own deployment authority.
- Secrets must stay in approved credential providers or Overvault refs and must never appear in local drafts, logs, exports, or support bundles.
- Invalid manifests, unsafe policy results, unauthorized actors, underfunded wallets, unsupported data classes, and blocked compliance boundaries must not advance to signed deployment.
- Final confirmation must show affected account, app, namespace, permissions, cost estimate, policy result, rollback path, and audit refs.
- Offline mode can save safe drafts but cannot validate, sign, launch, or mutate deployment state.

## Design Notes

- Use a stepper wizard with a persistent right-side review panel.
- Use blocking status rows for validation and policy results so builders know exactly what must be fixed.
- Keep logs compact and redacted by default.
- Make launch monitor a real operational state, not only a success screen.
