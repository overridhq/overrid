# Internal KYC Service Implementation Plan

## Objective

Provide Overrid-owned KYC, KYB, beneficial-owner, source-of-funds, screening, refresh, and cash-out eligibility facts so ORU, Seal Ledger, Overbill, and Provider Payout flows cannot become anonymous laundering rails.

## First Build Phase

[Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md), with Phase 5 accounting services consuming placeholder verification and hold refs before public cash-out is enabled.

## Dependencies

- Overpass.
- Overtenant.
- Overkey.
- Overvault.
- Overwatch.
- Overguard.
- Compliance Boundary Service.
- Overbill.
- ORU Account Service.
- Provider Payout Service.
- Fraud Control Service.
- Reputation and Anti-Sybil Service.

## Development Order

1. Define KYC subject, KYB subject, beneficial-owner, verification-attempt, screening, source-of-funds, risk-tier, refresh, and cash-out eligibility schemas.
2. Add Overvault-backed document and sensitive payload refs so raw identity, bank, tax, and screening data never lands in ordinary service records or logs.
3. Build person KYC intake, remote identity verification refs, liveness refs, and manual review records.
4. Build business KYB intake, authorized representative checks, beneficial-owner records, and app/provider ownership links.
5. Build sanctions, blocked-party, PEP, source-of-funds, and source-of-wealth evidence refs where required by policy.
6. Produce signed KYC/KYB fact bundles for Overguard, Overbill, Provider Payout Service, Wallet and Usage Center, and Overdesk.
7. Produce cash-out eligibility facts that include KYC/KYB state, payout destination ownership, cooling period state, source-of-funds status, and active AML holds.
8. Add refresh, expiry, revocation, appeal, export, and audit workflows.

## Contracts And Interfaces

- KYC/KYB profile schema.
- Beneficial-owner schema.
- Verification-attempt schema.
- Payout destination ownership fact.
- Source-of-funds/source-of-wealth evidence ref.
- Screening result ref.
- Cash-out eligibility fact bundle.
- KYC refresh and expiry event contract.

## Validation

- No payout can become eligible without a current KYC/KYB allow fact.
- Raw identity documents and payout credentials are stored only behind Overvault or tokenized provider refs.
- Cooling periods remain active even after KYC completion until the policy window expires.
- Source-of-funds review is required for high-value or suspicious funding and payout flows.
- Fact bundles are signed, versioned, and replayable.

## Handoff

Internal KYC Service gives money-moving services verified identity and AML facts without moving payment authority, accounting authority, or raw PII into those services.

## Detailed SDS

The design contract is expanded in [Internal KYC Service SDS](../../sds/governance_ops/internal_kyc_service.md).

## Sub-Build Plan

- [SUB BUILD PLAN #85 - Internal KYC Service](../../build_plan/sub_build_plan_085_internal_kyc_service.md)

## SDS Design Alignment

- Treat the service as identity-verification and AML-fact authority, not as a payment processor, legal-advice engine, ledger, wallet, or final fraud-enforcement authority.
- Keep Turkish-law thresholds, cooling periods, and reporting markers in Compliance Boundary policy bundles that this service consumes by version.
- Make KYC completion necessary but not sufficient for cash-out; payout eligibility also requires cooling-period, app-legitimacy, related-party, dispute, chargeback, and reconciliation facts.
- Preserve privacy through data minimization, Overvault refs, redacted events, strict operator roles, and no sensitive AML heuristic exposure to users.
