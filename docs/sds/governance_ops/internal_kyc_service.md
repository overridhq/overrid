SDS #85

# Internal KYC Service SDS

## Summary

Internal KYC Service owns Overrid KYC, KYB, beneficial-owner, source-of-funds, screening, refresh, and cash-out eligibility facts. It exists because Overrid's ORU, Seal Ledger, Overbill, and Provider Payout flows must not become anonymous or cheap laundering rails.

The service does not move money. It does not mutate ORU balances. It does not write Seal Ledger entries. It does not decide every fraud case alone. It produces signed verification and AML fact bundles that Overguard, Overbill, Provider Payout Service, Wallet and Usage Center, Overdesk, Fraud Control, and stewardship tools can consume.

## Document Links

| Item | Link |
| --- | --- |
| Service implementation plan | [internal_kyc_service.md](../../service_catalog/governance_ops/internal_kyc_service.md) |
| Sub-build plan | [SUB BUILD PLAN #85 - Internal KYC Service](../../build_plan/sub_build_plan_085_internal_kyc_service.md) |
| Build phase alignment | [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md) |
| AML rules | [Overrid AML Rules](../../aml_rules.md) |
| Compliance boundary | [Compliance Boundary Service](compliance_boundary_service.md) |
| Provider payout | [Provider Payout Service](../accounting/provider_payout_service.md) |

## Context

The laundering pattern to stop is:

1. A user buys credits.
2. A related user or organization creates a fake app.
3. The first user spends credits inside that fake app.
4. The fake app books ORU earnings.
5. The app owner cashes out after paying only platform commission and external rail costs.

This service is the identity and AML fact layer required to break that path. It combines KYC/KYB state, payout destination ownership, source-of-funds evidence, beneficial-owner links, policy thresholds, cooling-period state, and review evidence into signed fact bundles. Money-moving services must deny by default when the required facts are missing, expired, stale, contradicted, or under review.

## Scope

Internal KYC Service owns:

- Person KYC profile state.
- Business KYB profile state.
- Beneficial-owner records.
- Authorized representative records.
- Verification attempt records.
- Identity document and liveness evidence refs.
- Payout destination ownership facts.
- Source-of-funds and source-of-wealth evidence refs.
- Sanctions, blocked-party, and PEP screening refs where required by policy.
- KYC/KYB risk tiers and refresh requirements.
- Cash-out eligibility facts.
- Manual review cases for verification and AML evidence.
- Redacted user-facing status and private operator evidence refs.

It must not own:

- Payment processing.
- Payout execution.
- Seal Ledger entries.
- ORU balance mutation.
- Billing documents.
- Legal advice.
- Final suspicious-transaction filing decisions without the legal/steward process.
- Raw document storage outside Overvault or approved tokenized providers.
- Fraud-model weights or graph-authority logic owned by Fraud Control or Reputation/Anti-Sybil.

## Actors

- Person user: wants to buy credits, run apps, or receive payouts.
- Organization owner: manages KYB and authorized representatives.
- App owner/provider: needs cash-out eligibility for earned ORU.
- Compliance operator: reviews high-risk identity, source-of-funds, and cash-out cases.
- Steward reviewer: reviews severe, ambiguous, or ecosystem-impacting cases.
- Overbill: requests funding and manual high-credit eligibility facts.
- Provider Payout Service: requests payout eligibility facts.
- Wallet and Usage Center: displays safe verification and payout status.
- Overdesk: exposes user-facing KYC, KYB, funding, app earnings, and payout workflows.
- Fraud Control and Reputation/Anti-Sybil: provide risk and linkage signals.
- Overguard: enforces policy decisions using fact bundles.

## Dependencies

- Overpass for identity, account, session, and authority refs.
- Overtenant for tenant and organization scope.
- Overkey for signed requests, operator actions, and fact-bundle signatures.
- Overvault for encrypted raw document, screening, bank, tax, and sensitive evidence refs.
- Overwatch for audit events, review timelines, and replay bundles.
- Overguard for deny-by-default policy decisions.
- Compliance Boundary Service for Turkish-law thresholds, jurisdiction profiles, regulated-scope markers, and reporting boundary facts.
- Overbill for funding, receipts, external payment refs, chargeback state, refunds, and manual high-credit requests.
- ORU Account Service for account projection refs and bought/spent/earned/held state.
- Seal Ledger for append-only funding, hold, release, correction, earning, and payout settlement refs.
- Provider Payout Service for payout batches and provider-facing cash-out state.
- Fraud Control Service for laundering, fake-app, chargeback, and related-party risk cases.
- Reputation and Anti-Sybil Service for public-provider linkage and Sybil-risk signals.
- Overclaim for disputes, appeals, corrections, and release evidence.

## Policy Inputs

Internal KYC Service consumes signed policy bundles instead of hardcoded limits.

Required policy inputs:

- `jurisdiction_profile_ref`.
- `aml_policy_version`.
- `kyc_required_activity_matrix`.
- `kyb_required_activity_matrix`.
- `turkey.electronic_transfer_identity_threshold_tl`.
- `turkey.general_identity_transaction_threshold_tl`.
- `overrid.unverified_auto_credit_buy_cap_tl`.
- `overrid.manual_high_credit_review_threshold_tl`.
- `overrid.first_payout_cooloff_days`.
- `overrid.post_funding_cashout_hold_days`.
- `overrid.new_app_payout_probation_days`.
- `overrid.source_of_funds_required_threshold_tl`.
- `overrid.connected_transaction_window`.
- `overrid.payout_destination_refresh_window`.
- `overrid.kyc_refresh_period`.
- `overrid.kyb_refresh_period`.

The active policy must be traceable to Compliance Boundary Service. A fact bundle must include the policy version and generated timestamp.

## Core Records

### `kyc_subject`

Represents a person, organization, app owner, provider, or authorized representative under KYC/KYB review.

Fields:

- `subject_id`.
- `subject_type`: `person`, `organization`, `authorized_representative`, `beneficial_owner`, `app_provider`, `system_actor`.
- `overpass_identity_ref`.
- `tenant_ref`.
- `organization_ref`.
- `country_of_residence_ref`.
- `jurisdiction_profile_ref`.
- `created_at`.
- `state`.
- `risk_tier`.
- `verification_profile_ref`.
- `refresh_due_at`.
- `audit_refs`.

### `person_kyc_profile`

Fields:

- `subject_id`.
- `legal_name_hash`.
- `birth_date_hash_or_ref`.
- `identity_document_ref`.
- `liveness_check_ref`.
- `remote_verification_ref`.
- `phone_verified_ref`.
- `email_verified_ref`.
- `address_evidence_ref`.
- `screening_bundle_ref`.
- `source_of_funds_ref`.
- `status`.
- `status_reason_codes`.
- `verified_at`.
- `expires_at`.

Hash or ref fields must not leak raw PII into ordinary service records.

### `business_kyb_profile`

Fields:

- `subject_id`.
- `legal_entity_name_hash`.
- `registry_number_hash_or_ref`.
- `tax_number_hash_or_ref`.
- `registered_address_ref`.
- `authorized_representative_refs`.
- `beneficial_owner_refs`.
- `business_purpose_ref`.
- `expected_overrid_use_ref`.
- `app_provider_refs`.
- `screening_bundle_ref`.
- `source_of_funds_ref`.
- `source_of_wealth_ref`.
- `status`.
- `verified_at`.
- `expires_at`.

### `beneficial_owner`

Fields:

- `beneficial_owner_id`.
- `organization_subject_id`.
- `person_subject_id`.
- `ownership_or_control_type`.
- `ownership_band`.
- `authority_ref`.
- `screening_bundle_ref`.
- `verification_status`.
- `effective_from`.
- `effective_to`.

### `verification_attempt`

Fields:

- `attempt_id`.
- `subject_id`.
- `attempt_type`: `person_kyc`, `business_kyb`, `representative`, `beneficial_owner`, `payout_destination`, `source_of_funds`, `refresh`.
- `provider_ref`.
- `input_evidence_refs`.
- `result_evidence_ref`.
- `state`: `created`, `submitted`, `passed`, `failed`, `needs_manual_review`, `expired`, `superseded`.
- `reason_codes`.
- `operator_review_ref`.
- `created_at`.
- `completed_at`.

### `payout_destination_ownership`

Fields:

- `destination_ref`.
- `subject_id`.
- `destination_type`: `bank_account`, `payment_account`, `provider_token`, `other_approved_rail`.
- `ownership_evidence_ref`.
- `name_match_state`.
- `rail_region_ref`.
- `supported_currency_refs`.
- `tokenization_ref`.
- `state`.
- `verified_at`.
- `expires_at`.

No raw bank, card, or payment credentials are stored in this record.

### `source_of_funds_record`

Fields:

- `source_ref`.
- `subject_id`.
- `trigger_ref`.
- `trigger_type`: `manual_high_credit`, `large_funding`, `large_payout`, `suspicious_pattern`, `operator_request`, `policy_refresh`.
- `declaration_ref`.
- `evidence_refs`.
- `review_state`.
- `reviewer_ref`.
- `reason_codes`.
- `effective_window`.
- `audit_refs`.

### `cashout_eligibility_fact`

Fields:

- `fact_id`.
- `subject_id`.
- `provider_ref`.
- `app_ref`.
- `payout_period_ref`.
- `kyc_status_ref`.
- `kyb_status_ref`.
- `beneficial_owner_status_ref`.
- `payout_destination_ref`.
- `source_of_funds_ref`.
- `screening_status_ref`.
- `cooloff_state`.
- `related_party_state`.
- `app_legitimacy_state`.
- `chargeback_state`.
- `dispute_state`.
- `reconciliation_state`.
- `policy_version`.
- `decision`: `allow`, `hold`, `deny`, `manual_review`.
- `reason_codes`.
- `expires_at`.
- `signature_ref`.

This fact is necessary but not sufficient for payout. Provider Payout Service must still require earning, dispute-window, reconciliation, and payout-batch facts.

## APIs

Internal APIs:

- `POST /kyc/subjects`: create or resolve a KYC subject.
- `GET /kyc/subjects/{id}`: read redacted subject state.
- `POST /kyc/person-profiles`: create or update a person KYC profile.
- `POST /kyc/business-profiles`: create or update a business KYB profile.
- `POST /kyc/beneficial-owners`: attach beneficial-owner records.
- `POST /kyc/verification-attempts`: start a verification attempt.
- `POST /kyc/verification-attempts/{id}/result`: record provider or manual result refs.
- `POST /kyc/payout-destinations`: record payout destination ownership evidence.
- `POST /kyc/source-of-funds`: create source-of-funds evidence records.
- `POST /kyc/screening`: attach sanctions, blocked-party, or PEP screening refs.
- `POST /kyc/cashout-eligibility/evaluate`: create a cash-out eligibility fact.
- `GET /kyc/facts/{subject_id}/cashout`: return current cash-out facts by scope.
- `POST /kyc/manual-reviews`: create or update manual review cases.
- `POST /kyc/refresh`: schedule or force refresh.
- `GET /kyc/replay/{fact_id}`: return redacted replay bundle for authorized reviewers.
- `GET /kyc/exports/{period}`: produce authorized compliance export refs.

User-facing APIs must expose only safe status:

- `GET /me/verification-status`.
- `POST /me/verification/start`.
- `POST /me/source-of-funds`.
- `GET /me/payout-readiness`.

The user-facing surface must not expose exact risk thresholds, suspicious-transaction status, cluster membership, internal graph refs, raw screening hits, or private operator notes.

## Events

Events must be signed, redacted by audience, and traceable through Overwatch:

- `kyc.subject_created`.
- `kyc.verification_started`.
- `kyc.verification_passed`.
- `kyc.verification_failed`.
- `kyc.manual_review_required`.
- `kyc.manual_review_completed`.
- `kyc.risk_tier_changed`.
- `kyc.refresh_due`.
- `kyc.refresh_completed`.
- `kyc.source_of_funds_requested`.
- `kyc.source_of_funds_accepted`.
- `kyc.source_of_funds_rejected`.
- `kyc.payout_destination_verified`.
- `kyc.payout_destination_revoked`.
- `kyc.cashout_eligibility_allow`.
- `kyc.cashout_eligibility_hold`.
- `kyc.cashout_eligibility_deny`.
- `kyc.cashout_eligibility_manual_review`.
- `kyc.fact_bundle_exported`.

Sensitive evidence refs must be private to authorized reviewers and should not appear in ordinary user, app, or provider events.

## Workflows

### Person KYC

1. User requests a monetized capability, high-credit purchase, payout, or verified provider status.
2. Internal KYC Service creates or resolves a `kyc_subject`.
3. Service checks the active Compliance Boundary policy bundle.
4. User submits required identity and liveness evidence through approved flows.
5. Raw evidence lands in Overvault or an approved tokenized provider path.
6. Verification attempt returns pass, fail, or manual review.
7. Screening refs and source-of-funds requirements are attached if required.
8. Service produces a signed KYC fact bundle.
9. Overguard and downstream services consume the fact.

### Business KYB

1. Organization or app provider requests monetized app ownership or payout eligibility.
2. Service captures legal entity, authority, beneficial-owner, and business purpose refs.
3. Authorized representative must pass person KYC.
4. Beneficial owners are verified and screened according to policy.
5. Source-of-funds or source-of-wealth evidence is requested for high-risk or high-value activity.
6. Service produces a signed KYB fact bundle.
7. App/provider payout eligibility can be evaluated only after KYB is current.

### Cash-Out Eligibility

1. Provider Payout Service requests evaluation for provider, app, payout period, and earning refs.
2. Internal KYC Service verifies KYC/KYB currentness.
3. It verifies payout destination ownership and expiry.
4. It checks cool-off state against funding, earning, app-age, and policy refs.
5. It consumes Fraud Control and Reputation/Anti-Sybil risk refs.
6. It consumes app legitimacy, chargeback, dispute, and reconciliation refs.
7. It returns `allow`, `hold`, `deny`, or `manual_review`.
8. Provider Payout Service can include payout items only when all other payout facts also pass.

### Manual High-Credit Purchase

1. Overbill or Wallet and Usage Center creates a manual high-credit request.
2. Internal KYC Service checks identity tier, funding history, connected transaction totals, and source-of-funds requirements.
3. Compliance Boundary Service supplies active Turkish-law thresholds and internal caps.
4. Fraud Control supplies structuring, fake-app, and related-party risk refs.
5. Operator approves, reduces, delays, or denies the request with evidence.
6. Overbill and Seal Ledger record funding, holds, releases, or reversals.

## State Model

KYC profile states:

- `not_started`.
- `collecting`.
- `provider_pending`.
- `manual_review`.
- `verified`.
- `limited`.
- `rejected`.
- `expired`.
- `revoked`.
- `suspended`.

Cash-out eligibility states:

- `not_requested`.
- `kyc_required`.
- `kyb_required`.
- `source_of_funds_required`.
- `payout_destination_required`.
- `cooloff_active`.
- `app_review_required`.
- `related_party_review_required`.
- `chargeback_or_dispute_hold`.
- `manual_review`.
- `eligible`.
- `denied`.
- `expired`.

Transitions must preserve old records and write new facts. No service may silently mutate history.

## AML Guardrails

Mandatory rules:

- Cash-out is impossible without current KYC or KYB.
- KYC/KYB completion is necessary but not sufficient for cash-out.
- Bought credits are not directly cash-out eligible.
- Connected transactions must aggregate across related accounts and entities.
- Credit purchase caps must be policy-driven and Turkish-law aware.
- Larger credit purchases go through manual review.
- First payout and post-funding payout flows require cooling periods.
- Related-party payments do not automatically create payout eligibility.
- New apps start under payout probation.
- Payout destination changes trigger a new hold or review.
- Source-of-funds evidence is required for high-value, suspicious, or manual high-credit paths.
- Chargeback, refund, reversal, or reconciliation mismatch blocks affected cash-out.

## Privacy And Data Retention

- Raw identity documents, liveness media, bank data, tax forms, and screening payloads must stay in Overvault or approved tokenized provider systems.
- Ordinary records store hashes, refs, status, reason codes, and expiry only.
- Logs must not contain raw PII, document images, bank numbers, identity numbers, or sensitive screening payloads.
- Operator access requires role, purpose, case assignment, and Overwatch audit.
- Public and user-facing reason codes must be coarse and remediable.
- Retention periods must come from Compliance Boundary policy bundles.
- Deletion requests must preserve legally required AML records while redacting non-required copies.

## Security And Abuse Considerations

Threats:

- Fake KYC provider callback.
- Stolen verified account used for payout.
- Account takeover shortly before payout destination change.
- Related-party laundering through apps.
- Threshold structuring.
- Mule accounts with real KYC.
- Insider access to raw identity evidence.
- Tipping off suspicious-transaction investigations.
- KYC fact forgery or stale fact replay.
- Jurisdiction policy drift.

Mitigations:

- Signed provider callbacks and idempotent verification attempts.
- Overkey-signed fact bundles with expiry.
- Payout destination change hold.
- Fresh KYC/KYB check before payout batch inclusion.
- Related-party and connected-transaction aggregation.
- Overwatch audit for every operator evidence access.
- Redacted reason codes for users.
- Deny-by-default on missing, stale, or conflicting facts.
- Policy bundle version checks.
- Replay tests for every cash-out decision.

## Validation

Required tests:

- Unverified user cannot cash out.
- Verified user cannot cash out bought credits directly.
- KYB is required before organization-owned app payout.
- Beneficial-owner change invalidates existing payout eligibility until reviewed.
- Payout destination ownership mismatch blocks payout.
- First payout cool-off blocks otherwise valid payout.
- Post-funding cool-off blocks linked buyer to app-owner cash-out loops.
- Source-of-funds requirement blocks manual high-credit approval until satisfied.
- KYC expiry invalidates cash-out fact bundles.
- Chargeback/refund mismatch blocks affected payout item.
- User-facing status does not expose sensitive AML heuristics.
- Replay bundle reconstructs decision from policy version and fact refs.

## Build Order

1. Define schemas and signed fact-bundle contract.
2. Implement Overvault ref handling and PII-safe logging.
3. Implement person KYC workflow.
4. Implement business KYB and beneficial-owner workflow.
5. Implement payout destination ownership facts.
6. Implement source-of-funds and source-of-wealth review records.
7. Implement cash-out eligibility evaluation.
8. Connect Overbill manual high-credit requests.
9. Connect Provider Payout eligibility checks.
10. Connect Wallet and Usage Center plus Overdesk user-facing status.
11. Add Overwatch replay, exports, and compliance reports.
12. Add policy refresh, KYC refresh, and expiry jobs.

## Implementation Guidance

The first production slice should be conservative:

- Manual review for first payouts.
- Low automatic credit limits for unverified users.
- KYC/KYB required for any payout.
- Long first-payout cool-off.
- Payout destination ownership required.
- Strict deny-by-default behavior for missing or stale facts.

Automation can improve after clean history, test evidence, legal review, and fraud metrics prove the controls work.

## Open Questions

Resolved decisions:

- The first implementation should be a Rust/native Overrid service that produces signed KYC/KYB and AML fact bundles from generated contracts. External KYC, liveness, sanctions, PEP, bank-account, payment-destination, and tax providers may be used only behind narrow tokenized adapters; they must not become the product boundary, policy authority, raw evidence store, ledger authority, payout authority, or source of user-facing truth. Raw identity, bank, tax, screening, and liveness evidence stays in Overvault or approved provider token stores, while ordinary records carry refs, hashes, reason codes, freshness windows, and Overwatch audit refs.
- Policy thresholds and review triggers should come only from active Compliance Boundary signed policy bundles, not hardcoded service constants. Turkish-law baseline values, Overrid internal caps, connected-transaction windows, KYC/KYB refresh periods, cooling periods, payout-destination refresh windows, and source-of-funds thresholds must be cited by policy version in every fact bundle. If the bundle is missing, stale, superseded, jurisdictionally incompatible, or not replayable, Internal KYC Service returns deny/hold/manual-review facts rather than allowing funding or payout paths.
- Cash-out eligibility is a composed allow fact, not a synonym for completed KYC. KYC or KYB currentness, beneficial-owner verification, payout-destination ownership, source-of-funds/source-of-wealth evidence where required, screening status, app legitimacy, related-party risk, first-payout and post-funding cool-off state, dispute and chargeback finality, reconciliation state, and Provider Payout earning-period refs must all be evaluated before `allow` can be emitted. Bought ORU remains spendable inside Overrid when not held or frozen, but it is never directly cash-out eligible for the buyer.
- Connected-party and fake-app analysis should use explicit refs from Overpass, Overtenant, ORU Account Service, Seal Ledger, Overbill, Provider Payout Service, Fraud Control, Reputation/Anti-Sybil, Overclaim, and app/provider ownership records instead of a hidden KYC-only graph authority. Internal KYC Service may aggregate those refs into source-of-funds, related-party, cool-off, and cash-out facts, but Fraud Control owns fraud cases, Overguard owns policy enforcement, Provider Payout owns payout inclusion, and Central AI remains advisory/evidence summarizing only.
- User-facing and operator-facing visibility must be split by audience. Users receive coarse, remediable states such as `verification_required`, `source_of_funds_required`, `payout_review_pending`, or `payout_destination_review_required`; they must not see exact thresholds, graph-cluster membership, sanctions hit details, suspicious-transaction status, private operator notes, or reporting/law-enforcement status. Operator evidence access requires role, purpose, case assignment, and Overwatch audit, and deletion or correction flows preserve legally required AML records while redacting non-required copies according to Compliance Boundary retention policy.
