# SUB BUILD PLAN #85 - Internal KYC Service

Attached SDS: [SDS #85 - Internal KYC Service](../sds/governance_ops/internal_kyc_service.md)

## Purpose

Turn the Internal KYC Service SDS into the implementation sequence for Overrid's KYC, KYB, beneficial-owner, source-of-funds, screening, cooling-period, and cash-out eligibility fact service.

The service exists to stop ORU, Seal Ledger, Overbill, and Provider Payout flows from becoming anonymous laundering rails.

It must protect, not remove, the ORU-first economy. Users may earn ORU through approved resource contribution or legitimate services and spend ORU on other Overrid services. KYC/AML controls gate high-risk funding and cash-out; they do not prevent normal ORU spending on valid apps, subscriptions, one-time charges, resource usage, or machine-to-machine service calls.

## Source Alignment

| Source | Role |
| --- | --- |
| [Overrid AML Rules](../aml_rules.md) | Defines laundering threat model, Turkish-law baseline, funding limits, cooling periods, fake-app detection, and enforcement rules. |
| [SDS #85: Internal KYC Service](../sds/governance_ops/internal_kyc_service.md) | Controls service scope, records, APIs, events, workflows, privacy, validation, and build order. |
| [Internal KYC Service implementation plan](../service_catalog/governance_ops/internal_kyc_service.md) | Service-catalog summary for builders. |
| [Phase 13](phase_13_governance_compliance_scale_hardening.md) | First full build phase for governance and compliance hardening. |
| [Phase 5](phase_05_metering_oru_seal_ledger_overbill.md) | Money-moving services that must consume KYC/AML placeholder refs before public cash-out exists. |

## First Build Phase

Full implementation starts in [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md).

Phase 5 must reserve integration points for KYC, AML holds, manual high-credit requests, and payout eligibility facts so later public payout behavior does not require accounting redesign.

## Dependencies

- Overpass and Overtenant identity/organization refs.
- Overkey signed fact bundles.
- Overvault protected evidence refs.
- Overwatch audit and replay evidence.
- Compliance Boundary Service policy bundles.
- Overbill funding and payment-provider refs.
- ORU Account Service and Seal Ledger state refs.
- Provider Payout Service payout period and batch refs.
- Fraud Control and Reputation/Anti-Sybil risk signals.
- Overguard deny-by-default policy enforcement.

## Build Sequence

### 1. Policy And Schema Contract

Deliver:

- `kyc_subject` schema.
- Person KYC profile schema.
- Business KYB profile schema.
- Beneficial-owner schema.
- Verification attempt schema.
- Payout destination ownership schema.
- Source-of-funds/source-of-wealth schema.
- Cash-out eligibility fact schema.
- Policy bundle reference fields.

Acceptance:

- Every money-moving fact cites a policy version.
- Schemas carry refs instead of raw PII.
- Missing policy bundle results in deny-by-default.

### 2. Protected Evidence Storage

Deliver:

- Overvault ref model for identity documents, liveness media, bank/payment destination evidence, tax data, and screening payloads.
- Redacted logging rules.
- Operator evidence access audit.

Acceptance:

- No raw identity document, bank, card, tax, or screening data appears in ordinary service records or logs.
- Every sensitive evidence read writes Overwatch audit refs.

### 3. Person KYC Workflow

Deliver:

- Subject creation.
- Verification attempt lifecycle.
- Provider callback/result refs.
- Manual review cases.
- KYC status and expiry.

Acceptance:

- KYC completion produces signed fact bundles.
- Failed, expired, revoked, or stale KYC facts block cash-out.

### 4. Business KYB And Beneficial Ownership

Deliver:

- Business KYB intake.
- Authorized representative checks.
- Beneficial-owner records.
- App/provider ownership binding.
- KYB expiry and refresh.

Acceptance:

- Organization-owned app payout cannot become eligible without current KYB and beneficial-owner facts.
- Beneficial-owner changes invalidate relevant payout eligibility until review.

### 5. Payout Destination Ownership

Deliver:

- Tokenized payout destination refs.
- Name/ownership match state.
- Supported rail/currency/region facts.
- Destination expiry and revocation.

Acceptance:

- Payout destination mismatch, expiry, or recent change blocks payout.
- Raw payout credentials stay outside the service.

### 6. Source-Of-Funds And Manual High-Credit Review

Deliver:

- Manual high-credit request intake from Overbill and Wallet/Overdesk.
- Source-of-funds/source-of-wealth evidence records.
- Operator review decisions.
- Amount, purpose, rail, expiry, and evidence scopes.

Acceptance:

- Automated credit purchases above active policy caps are denied and routed to manual review.
- Manual approval is amount-bounded, purpose-scoped, time-bounded, and replayable.

### 7. Cash-Out Eligibility Engine

Deliver:

- Evaluation API for Provider Payout Service.
- KYC/KYB currentness checks.
- Cooling-period checks.
- Related-party, app-legitimacy, chargeback, dispute, and reconciliation fact inputs.
- `allow`, `hold`, `deny`, `manual_review` decision states.

Acceptance:

- KYC alone never returns `allow`.
- First payout and post-funding cool-off rules block otherwise valid payout items.
- Bought credits cannot be cashed out directly by the buyer.
- Bought ORU and earned ORU remain spendable inside Overrid when not frozen, held, or prohibited by policy.

### 8. Money-Service Integrations

Deliver:

- Overbill manual high-credit and funding-hold integration.
- Provider Payout Service eligibility integration.
- ORU Account Service held/available projection refs.
- Seal Ledger hold/release/correction reason refs.
- Overguard policy enforcement integration.

Acceptance:

- Provider payout batch creation fails when cash-out eligibility facts are missing, stale, expired, or deny/hold/manual-review.
- Funding state and payout state can be replayed from Seal Ledger and Overwatch refs.

### 9. User And Operator Surfaces

Deliver:

- Wallet and Usage Center verification status.
- Overdesk KYC/KYB and payout readiness surfaces.
- Admin/operator manual review queue.
- Redacted reason-code catalog.

Acceptance:

- Users see clear remediation without sensitive AML heuristics.
- Operators see evidence only when assigned and authorized.
- No screen reveals suspicious-transaction reporting status.

### 10. Reporting, Refresh, And Drills

Deliver:

- KYC/KYB refresh jobs.
- Payout destination refresh jobs.
- Compliance exports.
- AML drill fixtures.
- Public aggregate reporting inputs.

Acceptance:

- Stale KYC/KYB and stale payout destination facts expire automatically.
- Compliance exports include policy version, decision refs, and redaction profile.
- Fake-app laundering fixture is detected and blocked before payout.

## Validation Matrix

| Scenario | Expected result |
| --- | --- |
| Unverified person requests payout | Denied with `kyc_required`. |
| Verified buyer tries to cash out bought credits directly | Denied with `bought_credit_not_cashout_eligible`. |
| New verified app receives most spend from linked buyer | Held with `related_party_review_required` and `app_review_required`. |
| Organization app lacks KYB beneficial-owner facts | Held with `kyb_required`. |
| Payout destination was changed yesterday | Held with `payout_destination_review_required`. |
| Manual high-credit request lacks source-of-funds evidence | Held with `source_of_funds_required`. |
| Chargeback appears on linked funding | Affected funding and payout items held. |
| KYC expires before payout batch submission | Payout batch item blocked. |

## Exit Criteria

Internal KYC Service is ready when:

- All core schemas and APIs are versioned.
- Raw PII is isolated behind Overvault or approved tokenization refs.
- KYC/KYB fact bundles are signed and replayable.
- Provider Payout Service cannot submit payouts without current cash-out eligibility.
- Overbill cannot auto-approve high-credit purchases above active policy caps.
- Wallet and Overdesk expose safe status and manual review paths.
- Fake-app laundering, structuring, mule, chargeback, and related-party fixtures block payout.
