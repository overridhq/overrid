# Phase 9: Overpack Deployment Platform

## Objective

Make application deployment intent-driven and repeatable.

Developers should describe what the app needs; Overrid should validate, authorize, allocate, deploy, route, meter, observe, scale, update, and recover it.

## Depends On

- Phase 8 data, storage, namespace, and route primitives.
- Phase 7 grid-resident system workloads.
- Phase 5 accounting and billing rails.
- Phase 4 policy engine.

## Build Order

1. Define Overpack application-intent manifest.
2. Add package validation, signing, and provenance.
3. Add deployment planner.
4. Add provisioning for runtime, data, storage, routes, policy, and billing.
5. Add health checks and observability.
6. Add Release Strategy Service plans, channels, health gates, version pins, rolling, blue-green, canary, manual rollout, and route-weight strategies.
7. Add rollback, freeze, and recovery handoffs.
8. Prove one app deployment from a signed manifest.

## Workstream 1: Application-Intent Manifest

Define manifest sections:

- App identity.
- Services.
- Runtime cards.
- Data needs.
- Storage needs.
- Model needs.
- Permissions.
- Wallet budget.
- Billing rules.
- Routes.
- Geography.
- Scaling rules.
- Security rules.
- Health checks.
- Observability.

The manifest should express intent, not exact machine placement.

## Workstream 2: Validation And Provenance

Validate:

- Schema.
- Signature.
- Version.
- Dependency locks.
- SBOM.
- Artifact hashes.
- Permission declarations.
- Policy compatibility.
- Budget compatibility.
- Route ownership.

Store provenance so an operator can answer what was deployed, by whom, from what artifact, under which policy version.

## Workstream 3: Deployment Planner

Implement a planner that produces ordered steps:

1. Validate manifest.
2. Authorize actor and tenant.
3. Reserve budget.
4. Allocate runtime.
5. Allocate data stores.
6. Allocate storage.
7. Bind routes.
8. Deploy services.
9. Activate traffic.
10. Observe health.
11. Confirm settlement hooks.

Each step should be resumable or safely reversible.

## Workstream 4: Provisioning

Provision:

- Workload runtime.
- Overbase collections.
- Overstore buckets or object scopes.
- Overvault scopes.
- Namespace records.
- Overmesh routes.
- Policy bindings.
- Metering bindings.
- Billing account links.

The platform should not require manual infrastructure edits for normal deployment.

## Workstream 5: Release Strategy Service

Release Strategy Service owns rollout intent:

- Release plans.
- Strategy templates.
- Release channels.
- Traffic steps.
- Health gates.
- Promotion rules.
- Freeze records.
- Version pins.
- Rollback triggers.
- Approval records.
- Release evidence refs.

Support:

- Rolling update.
- Blue-green deployment.
- Canary deployment.
- Manual rollout.
- Manual rollback.
- Automatic rollback on health failure.
- Route-weight changes.
- Version pinning.

Keep release operations auditable and tied to signed deployment commands, package validation refs, deployment plan refs, Overmesh route refs, Overguard policy refs, Overwatch health evidence, restore refs when required, and metering/billing hook readiness where billable traffic is involved.

## Workstream 6: AI-Generated Package Compatibility

Prepare for AI-generated deployments:

- Strict manifest validation.
- Permission minimization.
- Policy dry-run before deployment.
- Human-readable diff.
- Artifact provenance.
- Test environment deployment.
- Resource budget preview.

AI-generated apps must not bypass trust, policy, or billing rails.

## Validation

- Developer deploys an app from one signed Overpack manifest.
- Runtime, data, storage, routes, policy, metering, and billing bind automatically.
- Health checks determine readiness.
- Canary or blue-green update works.
- Rollback works without manual infrastructure edits.
- Deployment event history is replayable.

## Exit Gate

Phase 9 is complete when a developer can deploy and update an app by changing a signed manifest instead of hand-building infrastructure.

## Handoff To Phase 10

Phase 10 uses the deployment platform and trust rails to support known external swarms and public-interest resource pools.
