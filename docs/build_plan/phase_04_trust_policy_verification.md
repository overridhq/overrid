# Phase 4: Trust, Policy, and Verification

## Objective

Make Overrid safe enough for multiple tenants and real workloads by adding policy enforcement, evidence-backed provider/node verification, challenge-result consumption, dispute handling, cache trust scopes, and private mesh controls.

This phase prevents the system from becoming a blind job marketplace.

## Depends On

- Phase 3 private execution loop.
- Audit events and state transitions.
- Raw metering events.
- Node capability and benchmark records.

## Build Order

1. Build Workload Classifier workload/data sensitivity classes and classification facts.
2. Build Overguard policy engine.
3. Add policy dry-run API.
4. Add Oververify verification records, evidence validation, trust signals, and eligibility signals.
5. Add Challenge Task Service outcomes and Benchmark Runner evidence consumption.
6. Add Overclaim disputes and holds.
7. Add Overmesh private connectivity.
8. Add cache trust scopes.

## Workstream 1: Workload Sensitivity Classes

Define initial classes:

- System service.
- Private tenant workload.
- Trusted federation workload.
- Public low-sensitivity workload.
- Research/public-interest workload.
- Regulated or secret-bearing workload.

Workload Classifier produces the deterministic classification facts for these classes. Every workload must declare a class, and the classifier must be allowed to downgrade eligibility, deny placement, mark facts unknown, or require review before Overguard evaluates admission.

## Workstream 2: Overguard Policy Engine

Build rules for:

- Tenant quota.
- Workload class.
- Data sensitivity.
- Secrets use.
- Egress policy.
- Package trust.
- Provider eligibility.
- Node trust class.
- Region or jurisdiction constraints.
- Cache scope.
- Budget and ORU reservation placeholders.

Policy output should include decision, reason codes, matched rules, policy version, and evidence references.

## Workstream 3: Policy Dry Run

Expose a dry-run endpoint that returns:

- Would allow, deny, block, or require review.
- Reasons.
- Required trust class.
- Expected placement class.
- Estimated reservation requirements with no accounting authority.
- Missing prerequisites.
- Policy version.

Dry runs must use the same Overguard evaluator as real admission while remaining side-effect-free: no queue items, leases, reservations, vault mounts, bills, payouts, ORU transitions, or workload records.

This endpoint will become important for developers, native apps, and central AI governance.

## Workstream 4: Oververify V0

Store verification evidence:

- Provider identity verification level.
- Node enrollment evidence.
- Benchmark evidence.
- Challenge task outcomes.
- Historical reliability.
- Dispute history.
- Abuse markers.
- Trust and eligibility signals.

Trust and eligibility signals must be explainable from evidence, reason codes, and policy/evaluator versions. Avoid opaque trust numbers that cannot be appealed. Oververify consumes Benchmark Runner and Challenge Task Service evidence; it does not run benchmarks or orchestrate challenges.

## Workstream 5: Challenge Checks

Add challenge tasks for:

- Proving node liveness.
- Validating claimed GPU class.
- Detecting impossible benchmark claims.
- Checking result consistency for selected workloads.
- Measuring reliability under repeated controlled jobs.

Challenge failure should reduce eligibility or trigger manual/admin review depending on severity.

## Workstream 6: Overclaim V0

Build dispute records:

- Dispute id.
- Workload id.
- Tenant id.
- Provider id.
- Evidence links.
- Claim type.
- Current state.
- Hold status.
- Proposed correction.
- Final resolution.

Disputes should be linked to settlement holds in Phase 5, but the record model starts here.

## Workstream 7: Overmesh Private Connectivity

Add private discovery and service routing for trusted private nodes:

- Node endpoint registration.
- Health-aware route selection.
- Tenant-scoped service discovery.
- Deny-by-default access between tenants.
- Audit events for route changes.

Keep the first mesh narrow: enough for control-plane to node-agent and trusted private service connectivity.

## Workstream 8: Cache Trust Scopes

Define cache scopes:

- Private tenant.
- Trusted swarm.
- Federation grant.
- Public low-sensitivity content.

No cached artifact may cross into a broader trust scope without explicit policy approval.

## Validation

- Policy decisions can be replayed from stored facts and policy version.
- Invalid package, denied egress, wrong tenant, insufficient trust, and missing secrets policy are rejected before execution.
- Verification evidence changes scheduler eligibility.
- Disputed job can block settlement progression.
- Cache scope prevents cross-tenant artifact exposure.

## Exit Gate

Phase 4 is complete when execution is governed by replayable policy decisions and provider eligibility is based on stored verification evidence rather than trust by assumption.

## Handoff To Phase 5

Phase 5 converts usage and dispute evidence into ORU accounting, Seal Ledger state, billing records, holds, refunds, and receipts.
