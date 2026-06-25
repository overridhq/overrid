# Overrid AML Rules

## Purpose

Overrid must not become a cheap laundering path where a person buys credits, a related person runs a fake app, the buyer spends credits in that app, the fake app books ORU earnings, and the operator cashes out through Seal Ledger money movement.

The rule is simple: ORU must fund real resource usage and lawful services, but it must not become an anonymous cash-in/cash-out rail.

Overrid is intentionally an ORU-first economy. Users can earn ORU by contributing approved computer resources or legitimate services, then spend ORU on other Overrid services. Users can also buy ORU and spend it inside the system. Native services, third-party apps, subscriptions, one-time charges, paid listings, resource usage, AI calls, and machine-to-machine calls should all use ORU as the internal payment medium. AML controls must protect that loop instead of removing it.

This document is a product and technical control document, not legal advice. Turkish law and MASAK guidance are the default baseline for payment, identity, transaction-limit, suspicious-transaction, and reporting rules. The active legal thresholds must be loaded from a versioned policy table reviewed by counsel or the legal steward, not hardcoded into application logic.

The wider Turkish-law surface is tracked in [Turkish Law Compliance Matrix](turkish_law_compliance_matrix.md). This AML document owns the laundering-specific controls; the matrix owns the cross-law launch checklist.

The app payment-bypass rule is tracked in [App Monetization Terms Policy](app_monetization_terms_policy.md). AML treats app-level external checkout as a laundering and evidence-gap signal because it can hide the real payer, purpose, amount, and service-delivery path.

Regulatory account or credit freezes are covered in [Regulatory Freezes Without Breaking Distribution](regulatory_freeze_distributed_system.md). Freeze support must work through signed legal-order facts, Overguard policy epochs, and append-only Seal Ledger entries, not direct authority access to accounts or nodes.

## Legal Baseline

Overrid AML controls must follow Turkish law by default and apply stricter internal risk controls where needed.

Primary source references:

- MASAK obligations overview: <https://masak.hmb.gov.tr/yukumlulukler>
- MASAK threshold announcement for identity limits in the Measures Regulation and MASAK Communique No. 5: <https://masak.hmb.gov.tr/duyuru/tedbirler-yonetmeliginde-ve-5-sira-no-lu-masak-tebliginde-yapilan-degisikliklerle-parasal-sinirlar-yeniden-belirlendi>
- MASAK Measures Regulation: <https://masak.hmb.gov.tr/suc-gelirlerinin-aklanmasinin-ve-terorun-finansmaninin-onlenmesine-dair-tedbirler-hakkinda-yonetmelik-3/>
- MASAK Law No. 5549 page: <https://masak.hmb.gov.tr/5549-sayili-suc-gelirlerinin-aklanmasinin-onlenmesi-hakkinda-kanun-2/>
- MASAK payment and electronic-money suspicious transaction guide update: <https://masak.hmb.gov.tr/duyuru/odeme-ve-elektronik-para-kuruluslari-supheli-islem-bildirimi-rehberi-ve-masak-online-sistemi-guncellendi>
- MASAK remote identity verification guidance for payment and electronic-money organizations: <https://masak.hmb.gov.tr/masak-genel-tebligi-sira-no-19/>

As of the current source scan, MASAK's public threshold announcement identifies these important amounts:

| Policy key | Current referenced amount | Use in Overrid |
| --- | ---: | --- |
| `turkey.electronic_transfer_identity_threshold_tl` | 15,000 TL | Identity and connected-transaction review trigger for electronic transfer style funding or payout flows. |
| `turkey.general_identity_transaction_threshold_tl` | 185,000 TL | High-value identity trigger for applicable transactions or connected transaction totals. |

The implementation rule is:

```text
automated_limit = min(current_turkish_law_limit, overrid_internal_risk_cap)
```

If MASAK changes thresholds, Overrid updates the signed AML policy bundle. Services must consume the policy bundle by version and produce replayable evidence showing which version was active.

## Hard Rules

1. No anonymous cash-out.

Every payout, redemption, provider cash-out, settlement to bank account, or conversion from earned ORU into external money requires completed KYC or KYB, verified payout destination ownership, active sanctions or blocked-party screening where required, and an allow decision from the AML policy path.

2. No automatic high-value credit purchases.

Automated credit purchase limits must stay below the active Turkish-law threshold and may be much lower for new, unverified, or risky accounts. Users can request larger credit purchases through manual support. Manual purchase review should be helpful and fast, but it must collect the evidence needed for compliance, source-of-funds checks, invoice/business-purpose review, and risk approval.

3. Connected transactions count together.

Overrid must aggregate connected transactions across account, identity, organization, app, provider, device, payment instrument, bank account, phone, email, IP/network, namespace, and beneficial-owner links. Splitting purchases below a threshold is treated as structuring risk, not as safe activity.

4. Bought credits are not cash-out eligible.

ORU or credits bought with fiat may be spent on real Overrid resources, native services, third-party apps, subscriptions, one-time service charges, paid listings, AI calls, storage, hosting, and machine-to-machine usage. They cannot be directly cashed out by the buyer. Cash-out eligibility belongs only to verified providers or app owners with real, evidence-backed service delivery, passed dispute windows, settled payment-provider events, and completed AML checks.

5. App monetization is ORU-only.

Apps, native services, and providers must not collect subscription fees, in-app purchases, one-time purchases, paid unlocks, paid listings, service-unit charges, app access fees, or app-specific support payments through third-party payment rails. Card checkout, bank-transfer instructions, crypto or stablecoin wallet payments, external subscription links, QR-code payments, payment handles, and "contact me to pay" flows are prohibited for anything delivered through Overrid. External payment rails are boundary rails for Overbill-operated ORU funding, refunds, chargebacks, tax records, regulated settlement, and eligible provider payouts.

6. Provider earnings are held by default until risk clears.

App/service earnings must start as `earnings_pending`. They become `cashout_eligible` only after app legitimacy, usage evidence, connected-party checks, dispute windows, chargeback windows, KYC/KYB status, cool-off requirements, and payout destination checks clear.

7. KYC does not remove cooling periods.

KYC proves identity. It does not prove that a transaction is legitimate. First payout, high-risk payout, and post-funding payout paths must still observe cooling periods and risk review.

8. Fake apps are blocked before they become payout rails.

Apps that exist mainly to receive ORU from linked users, provide no measurable service, have circular usage, produce abnormal conversion from bought credits to provider earnings, or show related-party spend concentration are not payout-eligible. This does not weaken the ORU-first economy: legitimate subscriptions, one-time payments, paid app features, paid listings, and service charges remain allowed when they are backed by real service delivery and normal user behavior.

9. No tipping off.

User-facing messages may provide coarse remediation steps such as "verification required" or "payout review pending." They must not reveal suspicious-transaction reporting, exact internal thresholds, graph-cluster membership, model weights, private evidence, or law-enforcement/reporting status.

## Funding And Cash-Out State Machines

Credit funding state:

1. `payment_intent_created`: user starts a credit purchase through Overbill.
2. `payment_pending`: external payment is not settled.
3. `funded_pending`: payment event is received, but credits are not fully spendable.
4. `spendable_limited`: low-risk spend is allowed within policy, but cash-out paths remain blocked.
5. `spendable`: funding is settled, risk checks pass, and normal usage is allowed.
6. `funding_held`: funding is blocked by AML, chargeback, dispute, sanction, payment mismatch, or manual review.
7. `funding_reversed`: payment failed, was refunded, charged back, or reversed with Seal Ledger correction refs.

Provider earning state:

1. `earning_observed`: Seal Ledger records usage-backed provider earning refs.
2. `earnings_pending`: payment settlement, dispute, chargeback, and risk windows are open.
3. `earnings_held`: a fraud, AML, KYC, dispute, app-legitimacy, chargeback, or manual-review hold is active.
4. `cooloff_active`: KYC/KYB is complete, but minimum cooling period has not expired.
5. `cashout_eligible`: all current requirements pass and a payout batch may include the item.
6. `payout_submitted`: Provider Payout Service has submitted an approved payout instruction through Overbill/payment-provider refs.
7. `paid`: payment-provider refs confirm settlement.
8. `reversed_or_corrected`: reversal, clawback, chargeback, or correction refs supersede earlier state.

Seal Ledger must record every balance-relevant transition as append-only entries. Corrections create new entries; history is never rewritten.

## Cooling Period Policy

Turkish law may impose specific recordkeeping, identity, and suspicious-transaction duties, but Overrid must also maintain internal cooling periods to reduce laundering risk. Unless counsel or the signed policy bundle requires stricter values, the default internal starting policy should be:

| Scenario | Minimum hold | Rationale |
| --- | ---: | --- |
| First provider payout after first received earning | 30 days | Blocks immediate cash-out of staged fake-app flows. |
| Cash-out after a recent fiat credit funding connected to the same actor graph | 7 days minimum, 30 days for new or high-risk clusters | Prevents buy, spend, earn, cash-out loops. |
| New app/provider payout eligibility | 30 days or one complete payout period after real usage evidence appears | Gives enough time for dispute, chargeback, and fake-service signals. |
| High-risk related-party spend concentration | Manual review; no automatic release | Related-party patterns are the core laundering path. |
| Chargeback or payment mismatch on linked accounts | Hold until finality plus manual review | Prevents external payment loss and laundering through reversals. |

The policy bundle may extend these periods by account age, KYC tier, app category, geography, payment rail, provider risk, or evidence gaps. Shortening a hold requires an explicit manual review record with source evidence.

## KYC And KYB Requirements

Overrid must build an Internal KYC Service as service SDS #85.

Minimum person KYC requirements:

- Legal name, date of birth, nationality or citizenship context where required, and unique identity document refs.
- Remote identity verification and liveness refs where allowed.
- Verified phone and email.
- Verified payout destination ownership.
- Sanctions, blocked-party, and PEP screening refs where required.
- Source-of-funds declaration for high-value funding, suspicious patterns, manual high-credit purchase, or payout review.
- Periodic refresh and event-driven refresh after material risk changes.

Minimum business KYB requirements:

- Legal entity name, registry number, tax number where applicable, address, and authorized representative refs.
- Beneficial ownership records.
- Director or signatory authority refs.
- Business purpose and expected Overrid use.
- App/service ownership refs and payout destination refs.
- Source-of-funds/source-of-wealth evidence for high-value or unusual activity.

Raw identity documents, bank details, tax forms, and sensitive screening payloads must stay in Overvault or approved provider/tokenization systems. Business services should receive signed verification facts and private evidence refs, not raw PII.

## Manual High-Credit Purchase Path

Users who need larger credit purchases should not be blocked by a bad user experience. They should be routed to a manual review path:

1. User requests a larger credit purchase from Wallet and Usage Center or Overdesk.
2. Overbill creates a `manual_high_credit_request` with amount, purpose, funding rail, tenant, and trace refs.
3. Internal KYC Service checks KYC/KYB status and required source-of-funds evidence.
4. Compliance Boundary Service attaches the active Turkish-law and jurisdiction policy bundle.
5. Fraud Control and Reputation/Anti-Sybil provide risk signals without revealing private heuristics.
6. A compliance operator approves, reduces, delays, or denies the purchase with reason codes.
7. Seal Ledger records the funding, spendability limits, holds, and eventual releases or reversals.

Approval should be explicit, time-bounded, amount-bounded, purpose-scoped, and replayable.

## Fake-App Laundering Detection

The platform must treat app monetization as adversarial once public cash-out exists.

Signals to detect:

- App receives most revenue from one buyer or a tight buyer cluster.
- Buyer and app owner share identity attributes, payment instruments, bank accounts, devices, networks, addresses, domains, recovery contacts, beneficial owners, company officers, or prior account links.
- Recently funded accounts spend heavily in one new app.
- A new app converts a high percentage of bought credits into provider earnings without normal engagement.
- App has no real service outputs, low runtime cost, weak user retention, fake traffic, abnormal refund/chargeback patterns, or templated user behavior.
- Multiple apps receive funds from the same buyer graph and cash out to related destinations.
- Buyers split purchases across accounts just below threshold values.
- App operators repeatedly close and reopen apps, namespaces, or payout destinations.
- Payout destination is changed shortly before cash-out.
- Internal ORU dimensions do not match the app's claimed service. For example, large Service-ORU earnings with almost no compute, storage, network, or human-visible usage.
- App UI, catalog text, support messages, outbound links, QR codes, dependencies, or namespace routes point users to card, bank-transfer, crypto, stablecoin, payment-link, external subscription, or private payment flows.
- Users report that features, support priority, content, rank, or service units are unlocked only after off-platform payment.

Required actions:

- New monetized apps start with low payout caps and longer holds.
- First payout for every app requires manual or enhanced review until enough clean history exists.
- Related-party payments may count as usage for product metrics but must not become automatic payout eligibility.
- App review records must include service description, pricing model, expected resource usage, refund policy, prohibited-category attestation, and owner identity refs.
- Monetized app review records must include ORU-only monetization attestation, accepted terms-policy version, and external-checkout absence evidence.
- Cash-out eligibility must depend on real service evidence, not only ledger earnings.
- Payment bypass must suspend app monetization and hold affected provider earnings until investigation, correction, and appeal paths resolve.

## Graph And Pattern Guardrails

The AML graph should connect:

- Person identity refs.
- Organization and beneficial-owner refs.
- App owner refs.
- Payment instruments and payout destinations.
- Device and session refs.
- Network/IP risk refs.
- Namespace/domain refs.
- Recovery email/phone refs.
- Invite/referral/organization membership refs.
- Wallet and ORU account refs.
- App usage and provider earning refs.
- Dispute, refund, chargeback, and reversal refs.

Risk patterns:

- Circular flows: A funds B's app, B funds C's app, C pays A's app.
- Fan-out/fan-in: one funding source spreads to many accounts that converge into one payout destination.
- Threshold structuring: repeated transactions just below policy limits.
- Rapid in/out movement: funding to cash-out path within short windows.
- Mule behavior: verified accounts receive earnings but show no real app operation or normal account use.
- Cross-service laundering: directory listings, messaging, social promotions, or fake subscriptions used as cover for transfers.

Central AI may summarize evidence and propose risk actions, but high-impact restrictions, account closures, suspicious-transaction decisions, and cash-out approvals require policy-defined review and audit refs.

## Service Integration Rules

| Service | AML responsibility |
| --- | --- |
| Overbill | Enforce funding limits, payment finality, chargeback state, manual high-credit requests, refund/reversal refs, and payment-provider reconciliation. |
| ORU Account Service | Project bought, spent, earned, held, and payout-eligible balances from Seal Ledger without allowing direct balance mutation. |
| Seal Ledger | Append funding, spending, earning, hold, release, correction, reversal, and payout settlement entries. |
| Provider Payout Service | Require KYC/KYB, payout destination ownership, cool-off, dispute-window closure, app-legitimacy checks, and AML allow decisions before payout batches. |
| Internal KYC Service | Own KYC/KYB profiles, verification facts, beneficial-owner refs, source-of-funds refs, screening refs, refresh requirements, and cash-out eligibility facts. |
| Compliance Boundary Service | Publish Turkish-law and jurisdiction-specific AML policy bundles, threshold versions, reporting markers, and compliance fact bundles. |
| Fraud Control Service | Detect fake-app, structuring, mule, circular-flow, chargeback, and related-party risk. |
| Reputation and Anti-Sybil Service | Provide identity/linkage/risk signals for public providers and payout hold recommendations. |
| Overguard | Enforce deny-by-default policy decisions based on AML fact bundles. |
| Overwatch | Record audit evidence, alerts, review timelines, and replay bundles. |
| Wallet and Usage Center | Show safe user-facing status, credit limits, manual request path, payout holds, and remediation without exposing sensitive AML logic. |
| Overdesk | Expose wallet, KYC, app earnings, payout status, and manual review workflows in the desktop product. |
| Central AI Service | Analyze suspicious patterns and package evidence; it does not unilaterally release funds or file reports. |

## Enforcement Actions

AML controls must support scoped and proportional actions:

- Block automated credit purchase.
- Route to manual high-credit review.
- Limit spendability of recently funded credits.
- Hold provider earnings.
- Deny payout batch inclusion.
- Freeze payout destination.
- Require KYC/KYB refresh.
- Require source-of-funds or source-of-wealth evidence.
- Suspend app monetization while keeping read-only evidence.
- Remove app from public discovery if it is likely a fake payout rail.
- Create Overclaim appeal path where appropriate.
- Preserve evidence and submit suspicious-transaction reports when legally required.
- Close accounts or organizations after final policy process.

Actions must be scoped by account, app, provider, payout destination, period, transaction, or connected cluster. Broad ecosystem-wide blocks require incident or stewardship review.

## Reporting And Evidence

Every AML decision must be replayable:

- Input facts and policy bundle version.
- Legal threshold version.
- KYC/KYB status and refresh timestamp.
- Risk signal refs.
- Related-party graph summary refs.
- Funding, usage, earning, and payout refs.
- Human/steward review refs where required.
- User-facing reason codes.
- Private operator notes and sensitive evidence refs.
- Appeal/correction refs.

Suspicious-transaction handling must preserve confidentiality and avoid tipping off. Public reports may show aggregate AML outcomes, but never private identities, raw evidence, exact thresholds beyond public legal thresholds, graph memberships, or reporting status.

## Validation

AML rules are production-ready only when the following tests pass:

- Unverified users cannot cash out.
- Automated credit purchases above the active policy cap are denied and routed to manual review.
- Connected transactions aggregate across account, device, payment, organization, and beneficial-owner refs.
- Bought credits cannot be cashed out directly by the buyer.
- A fake app receiving spend from a linked buyer cluster cannot become payout-eligible.
- First payout cool-off and post-funding cool-off are enforced.
- KYC completion alone does not release AML holds.
- Chargebacks and payment-provider mismatches block affected funding and payout items.
- Provider payout batches require fresh KYC/KYB, AML, dispute, chargeback, and reconciliation facts.
- User-facing reason codes do not reveal sensitive AML heuristics or reporting status.
- Seal Ledger replay proves every funding, hold, release, payout, reversal, and correction transition.
