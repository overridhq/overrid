SDS #52

# Federation Template Service SDS

## Purpose

Define reusable templates that let known organizations share capacity under explicit tenant, policy, accounting, reporting, and dispute boundaries.

Federation Template Service is the trusted-partner capacity contract layer. It does not onboard unknown public providers, schedule work, execute workloads, maintain accounts, adjudicate disputes, or decide final policy. It records the federation pattern, participant roles, capacity scope, workload eligibility, sponsorship/accounting boundary, evidence owner, and dispute path that other services enforce.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [federation_template_service.md](../../service_catalog/federation_public/federation_template_service.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 10: Trusted Federation and Public-Interest Pools](../../build_plan/phase_10_trusted_federation_public_interest_pools.md) |

## Service Family

- Family: Federation and public capacity
- Owning layer: Trusted federation templates and cross-tenant capacity boundaries
- Primary data scope: federation template definitions, participant roles, capacity contribution scopes, workload eligibility rules, sponsor/accounting boundaries, reporting requirements, dispute boundary refs, and federation instance records
- First build phase from service plan: [Phase 10: Trusted Federation and Public-Interest Pools](../../build_plan/phase_10_trusted_federation_public_interest_pools.md)

## Problem Statement

Overrid should expand beyond a private founder swarm without jumping straight to adversarial public supply. Universities, companies, research labs, nonprofits, family/community clouds, trusted partner swarms, and public-service partners need repeatable ways to share capacity while preserving tenant boundaries, purpose limits, evidence ownership, accounting visibility, and dispute responsibility.

Without federation templates, each trusted capacity relationship would become custom operator work. That would make policy enforcement inconsistent and cross-tenant usage hard to audit.

## Goals

- Define versioned templates for capacity donation, accountable near-cost or agreed-term capacity, research-only pools, education-only pools, internal organization swarms, partner-only overflow, and emergency/disaster-response pools.
- Capture participant identity, operator records, resource contribution scope, workload/data-class eligibility, provider obligations, reporting, accounting behavior, and dispute contact.
- Produce federation instance records when a template is bound to actual participants and tenants.
- Make cross-tenant usage cite a template, participant, policy version, grant/accounting boundary, and dispute boundary.
- Feed Overtenant, Overguard, Overgrant, Overbill, Overclaim, Oververify, Overwatch, and Deployment Planner with stable federation refs.
- Keep trusted federation separate from unknown public-provider onboarding.

## Non-Goals

- Do not verify organization identity directly; Overpass, Overtenant, Overkey, and Oververify own identity and evidence.
- Do not create grants, balances, invoices, payouts, or ledger entries; Overgrant, ORU Account Service, Overbill, Provider Payout Service, and Seal Ledger own accounting.
- Do not adjudicate disputes; Overclaim owns claims, holds, corrections, and finality.
- Do not schedule or run workloads; Oversched, Overlease, Overcell, and Overrun own execution.
- Do not admit unknown public providers; Public Provider Onboarding owns that path.
- Do not create pricing forecasts, revenue projections, blockchain mechanics, NFT rights, or per-transaction fee economics.

## Primary Actors And Clients

- Trusted organizations proposing capacity-sharing relationships.
- Overtenant binding participant organizations, tenant scopes, roles, and members.
- Overguard enforcing template workload and data-class eligibility.
- Overgrant creating cross-tenant resource allocation from template facts.
- Overbill, ORU Account Service, and Seal Ledger linking accounting behavior to federation boundaries.
- Overclaim resolving disputes that cite participant, tenant, workload, and accounting scopes.
- Overwatch storing audit events and evidence refs.
- Admin/developer UI and CLI showing template status and instance boundaries.

## Dependencies

- [Overtenant](../control_plane/overtenant.md) for tenant and organization boundaries, participant roles, membership, and suspension.
- [Overpass](../control_plane/overpass.md), [Overkey](../control_plane/overkey.md), and [Oververify](../trust_policy_verification/oververify.md) for participant identity and verification evidence.
- [Overguard](../trust_policy_verification/overguard.md) for template policy, workload class, data-class, and cross-tenant admission decisions.
- [Overgrant](../accounting/overgrant.md) for programmable cross-tenant resource allocation.
- [Overbill](../accounting/overbill.md), [ORU Account Service](../accounting/oru_account_service.md), [Seal Ledger](../accounting/seal_ledger.md), and [Provider Payout Service](../accounting/provider_payout_service.md) for accounting boundaries and settlement refs.
- [Overclaim](../trust_policy_verification/overclaim.md) for dispute paths and correction refs.
- [Purpose Tag Registry](purpose_tag_registry.md) and [Public-Interest Pool Service](public_interest_pool_service.md) for purpose-limited federation and public-interest capacity.
- [Overwatch](../control_plane/overwatch.md) for template lifecycle, activation, and usage evidence.

## Owned Responsibilities

Federation Template Service owns:

- Federation template schema and versioned template definitions.
- Participant type definitions and role requirements.
- Capacity contribution scope records for compute, GPU, storage, bandwidth, model, and service capacity.
- Workload and data-class eligibility rules attached to each template.
- Provider obligation requirements such as uptime, reporting, support contact, evidence retention, and dispute cooperation.
- Accounting boundary refs that state who sponsors, pays, receives provider earnings, and owns evidence.
- Federation instance records that bind a template to verified participants, tenants, and effective dates.
- Template validation reports and activation readiness records.

## Data Model

- `federation_template`: `template_id`, `version`, `template_type`, `participant_types`, `eligible_workload_classes`, `allowed_data_classes`, `capacity_dimensions`, `accounting_behavior`, `reporting_requirements`, `dispute_boundary_ref`, `state`, and `policy_refs`.
- `participant_role_requirement`: role name, required identity tier, operator contact requirement, verification refs, signing requirements, and allowed administrative actions.
- `capacity_contribution_scope`: capacity source, resource dimensions, geography/region, availability window, quota, maintenance window, reliability expectation, and allowed recipient scope.
- `workload_eligibility_rule`: workload class, data class, purpose tag refs, tenant eligibility, route/storage/vault restrictions, and denial reason codes.
- `federation_accounting_boundary`: sponsor account ref, grantee scope, provider earning recipient, billing document refs, hold rules, and Seal Ledger stream refs.
- `federation_dispute_boundary`: dispute owner, evidence owner, logs owner, response SLA, hold behavior, appeal route, and Overclaim refs.
- `federation_instance`: concrete template binding with participants, tenants, effective window, template version, policy refs, grant refs, status, and audit refs.

Templates are immutable by version once approved. Federation instances can be suspended, renewed, superseded, or retired through explicit lifecycle transitions.

## API Surface

- `POST /federation-templates`: creates a draft template with participant types, capacity scopes, workload rules, accounting, reporting, and dispute boundaries.
- `GET /federation-templates/{template_id}/versions/{version}`: returns a template version and redacted policy/accounting/dispute summaries.
- `POST /federation-templates/{template_id}/submit`: freezes a draft template for validation and approval.
- `POST /federation-templates/{template_id}/approve`: activates a template version after Overguard and operator approval.
- `POST /federation-instances`: binds an active template to verified participants, tenants, effective dates, and grant/accounting refs.
- `POST /federation-instances/{instance_id}/preflight`: checks identity, tenant, policy, accounting, purpose, and dispute readiness without scheduling work.
- `GET /federation-instances/{instance_id}`: returns instance status, participants, allowed scopes, reporting rules, and evidence refs.
- `POST /federation-instances/{instance_id}/suspend`: suspends usage under a template with reason code and policy refs.
- `GET /federation-instances/{instance_id}/usage-boundary`: returns the refs downstream services must attach to cross-tenant usage.

Mutating APIs require verified operator identity, tenant context, trace id, idempotency key, and Overguard policy decision refs. Stable errors include `participant_not_verified`, `template_not_active`, `workload_class_not_allowed`, `data_class_not_allowed`, `accounting_boundary_missing`, `dispute_boundary_missing`, `purpose_tag_not_verified`, and `participant_suspended`.

## Event Surface

- `federation_template.drafted`: draft template created.
- `federation_template.submitted`: template frozen for validation.
- `federation_template.approved`: template version became active.
- `federation_template.denied`: template approval denied with reason codes.
- `federation_template.instance_created`: template bound to participants and tenants.
- `federation_template.instance_preflight_passed`: instance is ready for controlled usage.
- `federation_template.instance_preflight_failed`: instance has missing identity, policy, accounting, purpose, or dispute facts.
- `federation_template.instance_suspended`: instance usage suspended.
- `federation_template.instance_retired`: instance closed with final evidence.

Events include template id/version, instance id, participant refs, tenant refs, workload/data-class refs, accounting boundary refs, dispute refs, policy refs, and redacted evidence refs.

## Core Workflow

1. Draft a federation template for a known participant pattern such as university, research lab, nonprofit, trusted partner swarm, or emergency pool.
2. Define participant roles, identity tiers, operator contacts, capacity contribution scopes, and allowed workload/data classes.
3. Attach accounting, grant, reporting, evidence, and dispute boundaries.
4. Ask Overguard to validate the template policy and deny-by-default behavior.
5. Approve and version the template.
6. Bind verified participants and tenants into a federation instance.
7. Preflight instance readiness before any cross-tenant workload uses the capacity.
8. Provide usage-boundary refs to Overgrant, Overguard, Overmeter, Overbill, Overclaim, and scheduling/deployment services.

## State Machine

Template lifecycle:

1. `draft`
2. `submitted`
3. `under_policy_review`
4. `approved`
5. `active`
6. `superseded`
7. `retired`
8. `revoked`

Federation instance lifecycle:

1. `proposed`
2. `awaiting_participant_evidence`
3. `preflighting`
4. `active`
5. `suspended`
6. `renewal_pending`
7. `retired`
8. `revoked`

Capacity scope lifecycle:

1. `declared`
2. `verified`
3. `available`
4. `exhausted`
5. `paused`
6. `expired`

Historical template versions and instance usage-boundary refs remain readable after retirement or revocation.

## Policy And Security

- Only verified known participants can instantiate federation templates.
- Enforce workload class, data class, route/storage/vault, geography, purpose tag, and tenant boundary rules before scheduling.
- Treat cross-tenant usage as deny-by-default unless a federation instance, Overguard policy decision, and accounting boundary are present.
- Keep private, regulated, secret-bearing, and system-service workloads out of templates that do not explicitly allow them with required controls.
- Keep raw secrets out of federation records; store only refs and requirements.
- Require participant suspension to propagate to Overguard, Overgrant, Overbill, Overclaim, and scheduling prechecks.
- Redact organization-sensitive contacts, routes, and evidence in user-facing reports.

## Metering And Accounting

- Every cross-tenant workload under a federation instance must carry template id, instance id, participant refs, sponsor/payer refs, provider earning refs, purpose tags where applicable, and dispute boundary refs.
- Overmeter emits usage dimensions; ORU Account Service derives account views; Seal Ledger stores immutable accounting entries; Overbill manages billing refs.
- Federation Template Service stores accounting boundary refs, not balances, invoices, prices, payouts, or ledger entries.
- Public-interest or donated capacity may route through Overgrant and Public-Interest Pool Service without speculative token mechanics.
- Do not encode revenue projections, customer counts, or market assumptions.

## Observability And Operations

- Expose active templates, active federation instances, suspended participants, missing evidence, accounting-boundary gaps, dispute-boundary gaps, and policy-denial reason counts.
- Provide operator views for participant roles, capacity scopes, allowed workload/data classes, purpose tags, and instance readiness.
- Support simulation mode showing whether a proposed workload would be allowed under an instance.
- Track cross-tenant usage refs and dispute refs for audits without storing private workload payloads.
- Emit stale evidence alerts before effective windows or verification refs expire.

## Failure Modes And Recovery

- Participant identity missing or stale: keep instance blocked.
- Accounting boundary missing: deny cross-tenant usage before scheduling.
- Dispute boundary missing: deny activation until evidence and hold behavior are defined.
- Template policy conflict: keep template in denied state and require revision.
- Participant suspension: suspend all affected instances and publish reason-coded events.
- Purpose tag not verified: deny purpose-limited usage until Purpose Tag Registry evidence exists.
- Overguard unavailable: block activation and cross-tenant scheduling rather than guessing.
- Incorrect template version used: reject request with required version and compatibility refs.

## Validation Plan

- Known organization capacity is visible only under an approved active federation instance.
- Cross-tenant usage cites template id, instance id, participant refs, policy refs, grant/accounting refs, and dispute boundary refs.
- Ineligible workload class or data class is denied with stable reason code.
- Missing participant verification blocks instance activation.
- Suspended participant prevents new usage without deleting historical usage records.
- Disputes can be traced to participant, tenant, workload, evidence owner, accounting scope, and Overclaim refs.
- Template version replay reconstructs why a workload was allowed or denied.

## Build Breakdown

1. Define federation template, participant role, capacity scope, eligibility rule, accounting boundary, dispute boundary, and instance schemas.
2. Implement draft, submit, approve, retire, and revoke lifecycle for template versions.
3. Implement federation instance creation and readiness preflight.
4. Add Overtenant, Oververify, Overguard, Overgrant, Overbill, Overclaim, and Overwatch refs.
5. Add usage-boundary query consumed by Overguard, Overgrant, Overmeter, Overbill, and scheduling/deployment services.
6. Add admin UI/CLI summaries and simulation mode.
7. Prove one trusted partner swarm and one public-interest template with explicit policy and accounting boundaries.

## Handoff And Downstream Use

Federation Template Service prepares Overrid for trusted external capacity before public supply. It hands template and instance boundary refs to Overguard, Overgrant, Overmeter, Overbill, Overclaim, Oversched, Deployment Planner, Public-Interest Pool Service, stewardship reporting, SDK, CLI, and admin UI.

Downstream services should attach federation refs to work and accounting events instead of copying template policy into local logic.

## Open Design Questions

Resolved decisions:

- The first proof should implement a trusted partner swarm template. It is the shortest safe extension from the private founder swarm and Phase 9 deployable-app platform because the participant can be known, operator-managed, package-validated, and bound to explicit Overtenant, Overguard, Overgrant, Overmeter, Overbill, Overclaim, and Overwatch refs before capacity is exposed. University, research-lab, nonprofit, education-only, and emergency templates should follow after the same template and instance lifecycle is proven and Purpose Tag Registry / Public-Interest Pool Service evidence is available.
- Trusted federation support should be tiered before Phase 11 public supply exists. The proof/default allowlist is `public`, `public_low_sensitivity`, and `grant_funded_public_interest` data classes on verified known participants. A later Phase 10 trusted-organization template may allow `organization_private` or tightly scoped `tenant_private` only when the federation instance names the participants, tenant boundary, storage/vault restrictions, route restrictions, evidence owner, dispute owner, accounting boundary, and Overguard decision refs. `user_private`, `secret_bearing`, `regulated`, and `system_service` remain denied for trusted federation until a separate policy/template version explicitly proves the required controls; none of these classes may fall through to Phase 11 public-provider paths.
- Templates express agreed terms as versioned operational terms, not prices or forecasts. The template may record `terms_kind` values such as donated capacity, reciprocal overflow, accountable near-cost, invoice-backed sponsor, grant-sponsored, or emergency sponsored; resource dimensions, quota windows, sponsor/payer refs, provider-earning recipient refs, settlement/hold windows, evidence retention, reporting duties, and compatible Overgrant/Overbill/Seal Ledger stream refs. Concrete prices, invoices, balances, payouts, and settlement entries stay in Overbill, ORU Account Service, Provider Payout Service, and Seal Ledger, while the template stores only signed refs, hashes, and stable term categories.
- The minimum dispute boundary before cross-tenant work can run is a concrete `federation_dispute_boundary` with claimant/respondent roles, participant contact refs, evidence owner, logs owner, accounting/hold target refs, response SLA, challenge or appeal window, redaction profile, escalation route, Overclaim claim-type refs, and finality behavior. Instance preflight must fail with `dispute_boundary_missing` when any of these fields are absent, stale, or unsupported for the selected workload/data class; cross-tenant scheduling must not proceed on informal operator promises.
- Federation reports use audience-specific redaction profiles. Public reports show only approved template names/types, active versions, public purpose tags, aggregate contributed/used dimensions, high-level allow/deny reason-code totals, public-interest status, and redacted audit refs. Participant-only reports show that participant's instance scope, capacity contribution, quota/usage rollups, reporting duties, accounting/grant refs, dispute status, and evidence obligations. Operator-only reports include verified participant refs, contacts, policy decisions, suspension state, accounting/dispute boundary details, topology-sensitive capacity scopes, and Overwatch evidence refs. Central-AI-only reports are policy-granted stewardship summaries for public-interest allocation, anomaly review, and governance reporting; they receive redacted evidence packages and aggregate signals, not raw secrets, private payloads, provider-private topology, payment details, or final decision authority.
