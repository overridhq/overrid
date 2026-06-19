SDS #57

# Purpose Tag Registry SDS

## Purpose

Define stewarded purpose tags for science, education, medical, opensource, climate, public infrastructure, and later approved categories.

Purpose Tag Registry owns the versioned catalog of public-interest purpose tags, eligibility criteria, evidence requirements, steward assignments, review history, deprecation rules, and policy export facts. It gives Overgrant, Public-Interest Pool Service, Overguard, central AI stewardship, and reporting tools a stable purpose vocabulary. It does not authorize grants, allocate resources, score project quality, decide central AI priorities, or adjudicate disputes.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [purpose_tag_registry.md](../../service_catalog/federation_public/purpose_tag_registry.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 10: Trusted Federation and Public-Interest Pools](../../build_plan/phase_10_trusted_federation_public_interest_pools.md) |

## Service Family

- Family: Federation and public capacity
- Owning layer: Stewarded purpose taxonomy, eligibility definitions, and evidence requirements
- Primary data scope: purpose tag definitions, tag versions, eligibility criteria, evidence requirement bundles, steward records, review proposals, policy export refs, deprecations, and reporting refs
- First build phase from service plan: [Phase 10: Trusted Federation and Public-Interest Pools](../../build_plan/phase_10_trusted_federation_public_interest_pools.md)

## Problem Statement

Public-interest pools and grant-backed workloads need a shared purpose vocabulary. Without strict tag definitions, any workload can claim to be science, education, medical, opensource, climate, or public infrastructure work without evidence. That weakens grant integrity, public reporting, and fraud controls.

The registry must make purpose tags stewarded, versioned, evidence-backed, and auditable. It must also keep tag definition separate from resource allocation: a workload matching a purpose tag can become eligible for a downstream grant decision, but this service does not spend resources or approve work by itself.

## Goals

- Define canonical purpose tags and versioned eligibility criteria.
- Attach required evidence schemas to each purpose tag and tag version.
- Track steward ownership, review history, approvals, deprecations, and supersessions.
- Provide policy export facts for Overguard, Overgrant, and Public-Interest Pool Service.
- Support workload purpose-claim validation without allocating grants or resources.
- Preserve auditability when criteria change over time.
- Provide public reporting hooks that expose tag definitions without leaking private workload evidence.

## Non-Goals

- Do not authorize grants, allocate resources, or create Overgrant authorizations.
- Do not decide project merit, central AI investment priority, or public-interest pool funding level.
- Do not adjudicate fraud, abuse, or disputes; Fraud Control Service and Overclaim own those flows.
- Do not store raw sensitive evidence when a reference to Overstore, Overvault, or Overwatch is sufficient.
- Do not turn purpose tags into speculative assets, NFTs, tokens, or tradable rights.
- Do not add pricing, financial projections, customer-count assumptions, or per-transaction fee economics.

## Primary Actors And Clients

- Stewardship reviewers creating and maintaining purpose tag definitions.
- Overgrant checking purpose eligibility before grant authorization.
- Public-Interest Pool Service binding pools to allowed purpose tags.
- Overguard consuming tag policy export facts for admission and dry runs.
- Workload owners submitting purpose claims and evidence refs.
- Central AI Service and Stewardship Reporting Service consuming purpose definitions and report refs.
- Fraud Control Service and Overclaim consuming tag evidence when claims are suspicious or disputed.
- SDK, CLI, admin UI, and public reporting surfaces displaying tag rules.

## Dependencies

- [Overregistry](../control_plane/overregistry.md) for canonical tag publication and version refs.
- [Overgrant](../accounting/overgrant.md) for grant eligibility and authorization handoff.
- [Public-Interest Pool Service](public_interest_pool_service.md) for pool-to-purpose binding.
- [Overguard](../trust_policy_verification/overguard.md) and [Policy Dry-Run API](../trust_policy_verification/policy_dry_run_api.md) for policy evaluation and previews.
- [Overwatch](../control_plane/overwatch.md) for review, approval, and change evidence.
- [Overstore](../data_storage_namespace/overstore.md) and [Overvault](../data_storage_namespace/overvault.md) for evidence references.
- [Central AI Service](../ai_rag_model_routing/central_ai_service.md), [Stewardship Reporting Service](../governance_ops/stewardship_reporting_service.md), and [Protocol Improvement Proposal Registry](../governance_ops/pip_registry.md) for governance and reporting integration.

## Owned Responsibilities

Purpose Tag Registry owns:

- Purpose tag ids, labels, descriptions, categories, and lifecycle state.
- Versioned eligibility criteria and evidence requirement bundles.
- Steward assignment and review participation records.
- Change proposals, approval records, deprecation records, and supersession links.
- Tag policy export refs for Overguard, Overgrant, and public-interest pools.
- Purpose claim validation results at the tag-definition level.
- Redacted public tag documentation and reporting metadata.
- Replay bundles for tag changes and workload purpose-claim validation.

## Data Model

- `purpose_tag`: stable tag id, slug, title, description, category, visibility, current version ref, owner/steward refs, state, and public documentation refs.
- `purpose_tag_version`: version id, tag id, criteria hash, evidence requirement hash, activation window, compatibility notes, signer refs, predecessor refs, and deprecation refs.
- `eligibility_criteria_bundle`: required predicates, disallowed predicates, accepted organization/workload types, evidence thresholds, freshness requirements, and policy export facts.
- `evidence_requirement_bundle`: required evidence types, accepted evidence refs, redaction rules, retention class, reviewer visibility, and user-facing explanation text.
- `steward_assignment`: steward identity refs, scope, review authority, conflict-of-interest refs, start/end times, and audit refs.
- `tag_change_proposal`: proposed change, motivation, compatibility impact, privacy impact, affected pools/grants, review state, comments refs, and approval refs.
- `purpose_claim_validation`: workload ref, claimed tag/version, submitted evidence refs, matched criteria, missing evidence, denial reason codes, policy refs, and replay refs.
- `tag_publication_ref`: Overregistry refs, report refs, policy bundle refs, and downstream subscriber update refs.

Accepted tag versions are immutable. Updates create new tag versions with explicit activation windows and supersession links.

## API Surface

- `POST /purpose-tags`: creates a draft purpose tag.
- `POST /purpose-tags/{tag_id}/versions`: creates a new draft tag version with criteria and evidence requirements.
- `POST /purpose-tags/{tag_id}/review`: opens or updates a stewardship review proposal.
- `POST /purpose-tags/{tag_id}/versions/{version_id}/activate`: activates an approved tag version.
- `POST /purpose-tags/{tag_id}/versions/{version_id}/deprecate`: deprecates or supersedes a tag version.
- `GET /purpose-tags`: lists active and public tag summaries.
- `GET /purpose-tags/{tag_id}`: returns tag metadata, current version, and public evidence requirements.
- `POST /purpose-tags/{tag_id}/validate-claim`: validates a workload's purpose claim against a specific tag version.
- `GET /purpose-tags/{tag_id}/policy-export`: returns policy facts for Overguard, Overgrant, and pools.
- `GET /purpose-tags/replay/{validation_id}`: reconstructs a validation or tag-change decision.

Mutating APIs require steward or service identity, trace id, idempotency key, governance refs, policy refs, and Overwatch evidence. Stable errors include `tag_exists`, `tag_unknown`, `version_not_active`, `criteria_invalid`, `evidence_requirement_missing`, `steward_conflict`, `review_not_approved`, `claim_evidence_missing`, `claim_ineligible`, and `tag_deprecated`.

## Event Surface

- `purpose_tag_registry.tag_created`: draft tag created.
- `purpose_tag_registry.version_created`: new tag version drafted.
- `purpose_tag_registry.review_opened`: stewardship review opened.
- `purpose_tag_registry.review_completed`: review completed with decision refs.
- `purpose_tag_registry.version_activated`: tag version activated.
- `purpose_tag_registry.version_deprecated`: tag version deprecated or superseded.
- `purpose_tag_registry.claim_validated`: workload purpose claim validation completed.
- `purpose_tag_registry.claim_denied`: purpose claim denied with reason codes.
- `purpose_tag_registry.policy_export_updated`: downstream policy export refs changed.

Events include tag refs, version refs, steward refs, governance refs, policy refs, public/private evidence refs, and redaction class. They must not include private evidence content.

## Core Workflow

1. Steward proposes a purpose tag or version update with motivation, criteria, evidence requirements, and affected downstream uses.
2. Registry validates schema, conflict-of-interest metadata, and compatibility with existing tag versions.
3. Stewardship review records approval, rejection, requested changes, or deprecation.
4. Approved versions are published to Overregistry and exported as policy facts.
5. Overgrant, Public-Interest Pool Service, Overguard, and Policy Dry-Run consume active version refs.
6. Workloads submit purpose claims with evidence refs; the registry validates criteria and returns matched/missing facts.
7. Downstream grant and pool services decide allocation using validation refs, quotas, fairness rules, accounting state, and policy.
8. Reports use public tag metadata and aggregated validation refs without exposing private evidence.

## State Machine

Tag lifecycle:

1. `draft`
2. `review_pending`
3. `active`
4. `deprecated`
5. `superseded`
6. `rejected`

Version lifecycle:

1. `draft`
2. `criteria_validated`
3. `review_pending`
4. `approved`
5. `active`
6. `deprecated`
7. `superseded`
8. `revoked`

Claim validation lifecycle:

1. `submitted`
2. `evidence_loaded`
3. `criteria_checked`
4. `eligible`
5. `ineligible`
6. `needs_more_evidence`
7. `expired`

## Policy And Security

- Require steward authorization and conflict-of-interest metadata for tag changes.
- Keep raw workload evidence in Overstore or Overvault; store only refs, hashes, validation facts, and redacted summaries.
- Require explicit tag version refs in Overgrant and public-interest pool policies so criteria changes cannot silently affect historical grants.
- Deny purpose claims when evidence is missing, stale, unverifiable, or tied to a deprecated tag version.
- Maintain public definitions while protecting private workload evidence, medical data, personal data, and sensitive research details.
- Make deprecations and supersessions explicit so reports can explain which tag rules applied at the time.

## Metering And Accounting

- Emit usage events for tag validation, review work, policy export generation, and report generation where material.
- Link claim validation usage to workload refs, tenant refs, tag refs, version refs, and evidence refs.
- Purpose tags do not create ORU balances, grants, payouts, or ledger entries directly.
- Overgrant, Public-Interest Pool Service, Seal Ledger, and Stewardship Reporting consume tag refs and validation refs downstream.
- Do not encode fees, financial projections, price assumptions, or revenue logic in tag records.

## Observability And Operations

- Expose active tags, pending reviews, deprecated versions, claims needing evidence, denied claims by reason code, affected pools/grants, and policy export freshness.
- Alert when active pools or grants reference deprecated tag versions.
- Provide replay for tag review, activation, deprecation, and workload claim validation.
- Provide public documentation generation for active tags and evidence requirements.
- Provide internal reviewer views that separate raw evidence access from redacted public summaries.

## Failure Modes And Recovery

- Tag version activated with incorrect criteria: supersede with a corrected version and preserve historical application.
- Evidence requirement too broad or too narrow: open a review proposal and publish updated criteria after approval.
- Downstream service references stale tag: emit policy export stale event and require refresh before new authorization.
- Missing evidence: return `needs_more_evidence` with required evidence ids.
- Conflicting steward decision: keep tag in review pending state and require governance escalation.
- Privacy leak risk in evidence summary: block public publication and require redaction review.
- Claim validation uses deprecated version: return current version ref and require caller decision on historical versus new rules.

## Validation Plan

- Workloads claiming a purpose tag must supply required evidence refs.
- Ineligible workloads are denied before grant authorization or pool allocation.
- Tag changes are versioned, reviewed, and auditable.
- Deprecated tag versions cannot be used for new grants unless a policy explicitly allows historical continuation.
- Public reports show tag definitions without raw private evidence.
- Policy export includes active version ids and criteria hashes.
- Claim replay reconstructs criteria, evidence refs, matched predicates, missing predicates, and denial reason codes.

## Build Breakdown

1. Define purpose tag, tag version, criteria bundle, evidence requirement, steward assignment, review proposal, claim validation, and policy export schemas.
2. Implement tag/version CRUD for drafts with steward authorization.
3. Implement review, activation, deprecation, supersession, and public publication flows.
4. Implement purpose claim validation with evidence refs and stable reason codes.
5. Integrate policy exports with Overguard, Overgrant, Public-Interest Pool Service, and reporting.
6. Add deprecation alerts, replay views, and public documentation generation.
7. Prove Phase 10 validation with eligible and ineligible public-interest workloads.

## Handoff And Downstream Use

Purpose Tag Registry hands tag version refs, criteria hashes, evidence requirement refs, claim validation refs, and policy export refs to Overgrant, Public-Interest Pool Service, Overguard, Policy Dry-Run API, Central AI Service, Stewardship Reporting Service, Fraud Control Service, Overclaim, SDK, CLI, and admin UI.

Downstream services must reference a concrete tag version rather than a mutable tag label.

## Open Design Questions

Resolved decisions:

- The first Phase 10 build should create the six stewarded tag records named in the build plan: `science`, `education`, `medical`, `opensource`, `climate`, and `public_infrastructure`. The proof public-interest pool may allocate only against active `science`, `education`, and `opensource` versions at first, because those can be validated from public project, institution, curriculum, repository, license, publication, or research-output refs. `medical`, `climate`, and `public_infrastructure` should exist as inactive or review-only tag versions until compliance markers, outcome-report rules, evidence redaction, steward review, and emergency/public-service governance are mature enough for real allocation.
- Public evidence may include tag ids and versions, public criteria summaries, evidence requirement names, public organization or project refs, repository/license refs, curriculum or public research-output refs, signed public attestations, criteria hashes, active/deprecated status, aggregate validation counts, public-safe denial reason totals, and redacted Overwatch or Stewardship Reporting refs. Steward-only or participant-only evidence includes raw submitted documents, private organization records, identity/contact refs, non-public grant or sponsor letters, sensitive research protocols, compliance/IRB-style attestations, private dataset/model refs, fraud or abuse evidence, Overvault refs, payment or payout details, central-AI private context, and any evidence whose disclosure would reveal private workload, medical, security, or anti-abuse facts.
- Medical and sensitive research claims must be validated by refs and redacted fact bundles rather than raw public payloads. Purpose Tag Registry should require Compliance Boundary facts, workload/data-class refs, signed steward-visible evidence refs, Overvault or protected Overstore storage for raw evidence, hash/integrity refs, and Overwatch replay evidence before returning an eligible validation. Public views should show only a high-level purpose summary, tag version, active/inactive state, redacted reason codes, and aggregate reporting refs. Missing compliance facts, private-data handling rules, or authorized reviewer refs must return `needs_more_evidence` or `claim_ineligible`, and public-provider placement remains denied unless a later trusted policy explicitly allows a narrow low-sensitivity derivative.
- Adding, deprecating, or splitting a purpose tag requires a `tag_change_proposal` with compatibility impact, affected pool/grant/policy refs, evidence requirement diffs, privacy and compliance impact, migration/deprecation plan, and Overwatch audit refs. Phase 10 can approve low-risk criteria or evidence-bundle revisions through Purpose Tag Registry steward review plus affected owner-service review from Overgrant, Public-Interest Pool Service, Overguard, and Stewardship Reporting. A new public tag, a tag split, semantic eligibility widening, medical/regulatory/public-infrastructure activation, or any change that affects public reporting, allocation, or cross-service policy exports requires the stricter governance path: required domain reviews, no blocking security/privacy/compliance/accounting findings, and a PIP once the PIP Registry is available. Central AI may recommend changes, but it cannot activate, deprecate, or split tags directly.
- Historical tag versions should remain visible as redacted public archive entries for as long as any public report, pool, grant, policy export, deprecation notice, or claim-validation replay cites them, and normally indefinitely for version id, criteria hash, activation/deprecation window, supersession link, public criteria summary, and correction/retraction notices. Raw or steward-only evidence follows its owning Overstore, Overvault, Overwatch, Compliance Boundary, and Overclaim retention policy and must not be copied into public reports. If a historical public summary leaks restricted data or becomes misleading, publish a corrected or retracted public view linked to the original version instead of deleting the version history.
