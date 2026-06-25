# Trust and Safety Abuse Prevention

## Purpose

Overrid cannot stop all abuse with certainty, but it can make abuse hard, expensive, short-lived, non-monetizable, and legally reportable.

The platform should not rely on per-app moderation alone. Money laundering, illegal listings, trafficking, gambling, adult content, and underage exploitation must be handled through a shared Trust and Safety control plane used by public apps, wallet flows, directory listings, search, messaging, hosting, app deployment, and AI services.

This document is an architecture note, not legal advice. Jurisdiction-specific wallet, AML, child safety, privacy, and reporting obligations require qualified legal review before launch. Porn, casinos, betting, and gambling are excluded by Overrid policy even where they are legal in a specific jurisdiction.

The wider Turkish-law checklist is tracked in [Turkish Law Compliance Matrix](turkish_law_compliance_matrix.md). Trust and Safety owns abuse prevention; the compliance matrix owns cross-law launch gates and service ownership.

App-level monetization rules are tracked in [App Monetization Terms Policy](app_monetization_terms_policy.md). Trust and Safety treats third-party payment bypass as economic abuse because it can hide illegal flows, weaken AML visibility, and starve the shared maintenance budget that keeps the network operating.

## Core Rule

Overrid should be open for lawful public use, but not neutral toward crime.

Public apps, wallet flows, directory listings, search, messaging, hosting, and AI should all share the same abuse-control layer.

Overrid is not a medium for porn, casinos, betting, gambling, or wager-like products. These categories must be blocked at the policy, app registry, search, payment, routing, and hosting layers instead of treated as future licensed business lines.

```text
Normal use should preserve privacy.
Risky public or monetized use must require stronger verification.
Prohibited public content must not become searchable, payable, routable, or persistent.
Serious illegal content must trigger containment, evidence preservation, and required reporting.
```

## Shared Abuse-Control Plane

The abuse-control plane should coordinate:

- Overpass identity and account state.
- Overguard policy enforcement.
- Overwatch evidence and audit records.
- Fraud Control risk cases and recommendations.
- Reputation and Anti-Sybil signals.
- Directory Listings moderation.
- Search indexing permissions.
- Messaging Center contact abuse controls.
- Wallet, ORU, Seal Ledger, Overbill, and payout controls.
- Central AI detection, analysis, and evidence summaries.
- Overclaim dispute and appeal paths.

Central AI may help detect and package evidence, but final serious actions should follow policy, evidence, human or steward review, and appeal paths.

## Risk-Tiered Identity

Do not give the same permissions to every account.

Recommended access tiers:

| Activity | Required control |
| --- | --- |
| Read public content | Low friction, rate-limited abuse controls |
| Basic personal use | Pseudonymous identity, device/session reputation |
| Public posting | Verified account or reputation threshold |
| Directory listings | Category policy, posting reputation, review for risky categories |
| Receiving money | KYC/KYB before payout or high-volume monetization |
| Running apps/services | Verified developer or operator identity |
| Public marketplace activity | stronger verification, fraud monitoring, dispute support |
| High-risk categories | manual approval or blocked |
| Payouts/redemptions | strongest KYC/KYB, sanctions screening, fraud checks |

This keeps normal usage private while preventing fully anonymous monetized abuse.

## AML Controls for ORU and Wallet

ORU and wallet flows must not become anonymous laundering rails.

The detailed control document is [Overrid AML Rules](aml_rules.md). Turkish law and MASAK guidance are the default baseline for identity thresholds, suspicious-transaction handling, reporting boundaries, and payment/e-money style risk controls. Active legal limits must be consumed from Compliance Boundary policy bundles, not hardcoded into wallet or payout logic.

Required controls:

- No anonymous cash-out.
- No anonymous high-volume transfer, resale, redemption, or payout.
- Transaction limits for new or unverified accounts.
- KYC/KYB before payouts, high-volume transfers, business use, or credit resale.
- Manual review for credit purchases above the active automated policy cap.
- Cooling periods between funding, earned ORU eligibility, and cash-out.
- No direct cash-out of bought credits by the buyer.
- Fake-app laundering detection before provider earnings become payout-eligible.
- Sanctions and blocked-entity screening where legally required.
- Payout holds for suspicious activity.
- ORU-only monetization enforcement for app subscriptions, in-app purchases, one-time purchases, paid unlocks, listings, and service units.
- Source-of-funds review for high-risk accounts or transactions.
- Suspicious pattern detection across account, wallet, listing, device, app, and graph behavior.
- Suspicious activity reporting where legally required.
- Audit-preserving correction entries instead of ledger rewrites.

Risk patterns to detect:

- Rapid in/out movement.
- Circular transfers.
- Layering across many accounts.
- Many low-value transactions designed to avoid thresholds.
- Mule-account behavior.
- Repeated chargeback/refund abuse.
- Newly created accounts receiving abnormal flows.
- App/service accounts used only to move credits.
- Directory listings that appear designed only to justify payments.
- Related-party spend from recently funded accounts into newly created apps.
- Apps that route users to card, bank-transfer, crypto/stablecoin, payment-link, QR-code, external subscription, or private payment flows for Overrid-delivered services.
- Payout destination changes shortly before cash-out.

Turkish-law source references are tracked in [Overrid AML Rules](aml_rules.md), including MASAK obligations, identity thresholds, suspicious-transaction guidance, and remote identity verification guidance.

## Prohibited Category Registry

Directory Listings and public apps should not rely on free-form category creation.

Overrid should maintain a signed policy registry:

```text
category -> allowed | prohibited | review_required | licensed_only
jurisdiction -> local legality rules
identity_level_required -> none | account | verified | kyb | licensed
payment_allowed -> true | false | restricted
search_index_allowed -> true | false | restricted
manual_review_required -> true | false
reporting_required -> true | false | conditional
```

Default category examples:

| Category | Default policy |
| --- | --- |
| Illegal drugs | Prohibited |
| Human trafficking or exploitation | Prohibited and escalated |
| Sexual services exploitation | Prohibited and escalated |
| Weapons | Blocked or jurisdiction-gated |
| Gambling, casino, betting, wagers, odds, or games of chance for value | Permanently prohibited; no licensed exception in Overrid-native services |
| Pornographic/adult sexual content | Permanently prohibited in Overrid-native public services |
| Jobs | Allowed with scam and discrimination controls |
| Housing | Allowed with scam, privacy, and discrimination controls |
| Local services | Allowed with fraud controls |
| Events/community groups | Allowed with abuse and safety controls |

`licensed_only` exists for categories that Overrid may legally support in the future, such as regulated professional services. It must not be used as a bypass for porn, casinos, betting, or gambling.

## Public Content Pre-Moderation

Anything public and searchable should pass checks before publication.

Required checks:

- Text classification.
- ADES/entity extraction for suspicious entities, coded terms, locations, payment terms, service claims, names, organizations, and contact patterns.
- Media moderation for images and video.
- Known illegal-content hash matching where legally and technically possible.
- Listing-template validation.
- Account, seller, and organization reputation checks.
- Contact/payment behavior checks.
- Manual review for risky categories.

Search should not index pending, risky, prohibited, or review-required content until it is cleared.

## Directory Listings Controls

Directory Listings should have hard rules because it is the highest-risk native app for illegal goods and services.

Required controls:

- Allowlisted launch categories.
- Prohibited-category registry.
- Location privacy defaults.
- Contact handoff through Messaging Center instead of raw public phone/email exposure where possible.
- Report button on every listing and profile.
- Scam and duplicate detection.
- Listing velocity limits.
- New-account posting limits.
- Risk review before search indexing.
- Payment suppression for suspicious listings.
- Evidence snapshots for removed listings.
- Appeals through Overclaim.

High-risk listings should be blocked or held before they become searchable.

## Child Safety

Child sexual exploitation and underage sexualized content must be treated as absolute prohibited content.

Rules:

- No sexualized minor content, ever.
- No grooming, enticement, extortion, or sexual solicitation of minors.
- No adult-to-minor risky contact patterns in public or reported surfaces.
- Immediate containment for suspected CSAM or child exploitation.
- Evidence preservation with strict access controls.
- Required legal reporting where applicable.
- No public redisplay of suspected illegal material during review.
- No normal appeal path that exposes illegal material to unqualified reviewers.

Controls:

- Known CSAM hash matching where legally and technically possible.
- Image/video/text classification.
- Age-risk and contact-pattern detection.
- Report button everywhere.
- Dedicated child-safety escalation queue.
- Human review by trained personnel.
- Law-enforcement or national hotline reporting paths where required.

NCMEC's CyberTipline is the U.S. centralized reporting system for suspected online child exploitation and can receive reports from the public and electronic service providers: <https://www.missingkids.org/gethelpnow/cybertipline>

## Human Trafficking and Exploitation

Overrid must treat trafficking indicators as serious safety events, not ordinary policy violations.

Controls:

- Block categories and wording patterns associated with trafficking or exploitation.
- Detect coercive or coded listing language.
- Detect repeated listings across locations, accounts, images, devices, or payment routes.
- Detect suspicious contact funnels from directory listings to messaging and wallet payments.
- Preserve evidence.
- Escalate to trained review.
- Report or refer through appropriate legal channels where required.

The U.S. Department of Justice lists reporting paths for trafficking crimes, including the National Human Trafficking Hotline and FBI tips: <https://www.justice.gov/action-center/report-crime-or-submit-complaint>

## Gambling and Casino

Permanent policy:

```text
Do not allow gambling, casinos, betting, wagers, odds markets, or games of chance for value anywhere in Overrid-native services.
```

This prohibition applies even where the activity is legal, licensed, taxed, or culturally normalized in a specific country.

Blocked surfaces:

- Native app registry.
- Directory Listings.
- Search indexing.
- Messaging promotions and public groups.
- Wallet/ORU payments and payouts.
- App deployment.
- Public routing and namespace discovery.
- Ads, sponsorships, referral links, and affiliate flows.
- AI assistant routing that would help operate or promote those services.

Overrid should not provide the medium, infrastructure, discovery, payment rails, or promotion layer for gambling businesses.

## Porn and Adult Content

Permanent policy:

```text
Do not allow pornographic or adult sexual content in Overrid-native public services.
```

This is not a launch-only restriction and not a future adult-module placeholder. Overrid should not host, index, monetize, recommend, route, promote, or provide native app infrastructure for porn services.

Blocked surfaces:

- Native app registry.
- Directory Listings.
- Search indexing.
- Social public feeds.
- Messaging promotions and public groups.
- Wallet/ORU payments and payouts.
- App deployment.
- Public routing and namespace discovery.
- AI assistant routing that would help create, distribute, monetize, or promote pornographic services.

Narrow exceptions may exist only for non-pornographic safety, medical, educational, legal, or evidence-handling contexts. Those exceptions must be policy-reviewed, non-promotional, and unavailable for monetized adult entertainment.

## Enforcement Flow

Use one standard enforcement pipeline:

```text
detect -> block/hide/hold -> evidence package -> review -> action -> appeal -> transparency report
```

Possible actions:

- Reject listing.
- Hide from search.
- Disable contact handoff.
- Disable payment.
- Freeze payout.
- Suspend account.
- Block app deployment.
- Deny node workload.
- Require re-verification.
- Escalate to trained human review.
- Report to authorities or official hotlines where legally required.

## Role of Central AI

Central AI should help detect, score, explain, and summarize abuse evidence.

It should not act as sole judge.

```text
Central AI: detect, score, explain, recommend
Overguard: enforce policy
Overwatch: record evidence
Fraud Control: build case and recommendation
Human/steward review: serious decisions
Overclaim: appeal/dispute
```

Central AI can be powerful without becoming an opaque ruler.

## Privacy and Safety Balance

Overrid should be privacy-preserving, but not abuse-blind.

The correct balance:

- Pseudonymous for normal use.
- Verified for public monetized risk.
- Strongly verified for payouts and regulated activity.
- Policy-gated for public listings and search indexing.
- Evidence-preserving for serious abuse.
- Legally reportable for CSAM, trafficking, suspicious finance, and other reportable harms.

## Launch Recommendation

Launch public apps with conservative rules:

- Directory Listings starts with low-compliance allowed categories only.
- No gambling, casino, betting, wager, odds, or game-of-chance-for-value services.
- No pornographic or adult sexual content services.
- No anonymous payout.
- No public searchable listing without policy screening.
- No high-risk category without manual review.
- No central AI unilateral punishment.
- Appeals must exist, except where illegal material cannot be redistributed or exposed.

## Bottom Line

Overrid should be privacy-preserving, but not abuse-blind.

The design target is:

```text
Pseudonymous for normal use.
Verified for risk.
Impossible to monetize crime anonymously.
Impossible to make prohibited public content discoverable.
Impossible to use Overrid-native services as porn, casino, betting, or gambling infrastructure.
Fast to contain serious harm.
Auditable and appealable where appeal is legally safe.
```
