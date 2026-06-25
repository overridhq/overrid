# App Monetization Terms Policy

## Purpose

Overrid apps must monetize through ORU only. The platform cannot stay alive if apps use Overrid distribution, compute, storage, discovery, namespace, wallet, and trust rails while collecting their own subscription, purchase, or service fees outside the system.

This document is the canonical product and terms policy for app-level monetization. It should be reflected in publisher terms, user-facing Terms of Service, Overpack validation rules, Overregistry app records, Overbill billing rules, ORU Account Service spending rules, Overguard policy bundles, Overdesk publisher screens, and payout eligibility checks.

## Hard Rule

ORU is the only approved internal fee collection method for Overrid apps and native services.

The rule applies to:

- subscriptions;
- in-app purchases;
- one-time purchases;
- paid feature unlocks;
- usage-based app charges;
- paid listings and paid placement where policy allows them;
- API or service-unit charges;
- machine-to-machine calls;
- app-specific support, tip, donation, or sponsorship flows if those flows are ever allowed.

Apps must not collect or request payment through:

- credit card checkout;
- bank transfer instructions;
- crypto or stablecoin wallet payments;
- PayPal, Stripe, Iyzico, virtual POS, payment links, or similar third-party checkout links;
- external subscription pages;
- QR codes or payment handles;
- "contact me to pay" flows;
- direct messages used for payment coordination;
- any off-platform payment arrangement that unlocks app features, app access, app content, service units, or preferential treatment inside Overrid.

External payment rails remain allowed only for platform boundary flows operated by Overbill or an approved licensed/authorized partner: ORU funding, refunds, chargebacks, tax documents, regulated settlement, and eligible provider payouts. They are not app-level checkout rails.

## Terms Of Service Requirements

Publisher terms and user-facing Terms of Service must explicitly state that:

- ORU is the sole approved monetization and fee collection method inside Overrid.
- Third-party payment collection is prohibited for subscriptions, in-app purchases, one-time purchases, paid unlocks, paid listings, service units, app-specific donations/support, and app access.
- Apps may not redirect users to external card, bank, crypto, stablecoin, payment-link, or subscription systems to avoid ORU.
- Apps may not accept off-platform payment in exchange for access, rank, promotion, features, content, credits, support priority, or service units delivered through Overrid.
- Attempts to bypass ORU can lead to manifest rejection, catalog hiding, route suspension, monetization suspension, payout hold, earnings ineligibility, app termination, and account enforcement, subject to dispute and legal process.
- Repeated or intentional bypass is treated as both economic abuse and AML risk because it can starve platform maintenance and hide real transaction flows.

## Why This Must Be Strict

Overrid's economic loop is simple: users fund or earn ORU, spend ORU on useful services, providers earn ORU for real work, and eligible providers cash out through compliant payout rails. If app owners can collect fees directly by card, bank transfer, crypto, payment links, or private arrangements, the loop breaks.

That would remove the budget needed for maintenance, fraud control, support, stewardship, public-interest work, and the shared infrastructure that makes the apps possible. ORU-only monetization is therefore not optional. It is a survival rule for the ecosystem.

## Enforcement Model

Overrid should enforce this policy before launch, during operation, and before payout.

- Overpack manifests must declare monetization model, billing rules, ORU-only payment intent, external-payment absence, and accepted terms-policy version.
- Package Validator must reject manifests, package metadata, links, dependencies, routes, or UI declarations that expose external checkout for app monetization.
- Overregistry must store the accepted monetization policy version, ORU-only attestation, and bypass enforcement state on app and catalog records.
- Overguard must block routes, APIs, or policy grants that enable prohibited payment collection.
- Overbill and ORU Account Service must require ORU-backed records for app subscriptions, purchases, paid unlocks, paid listings, and service-unit settlement.
- Search, Directory, Native App Catalog, and Overdesk must hide or mark apps whose monetization is suspended.
- Provider Payout Service must hold or deny earnings when app revenue is linked to payment bypass, fake service delivery, or related-party laundering.
- Overwatch must preserve evidence, and Overclaim must provide appeal and correction paths.

## Allowed Boundary Flows

The policy does not ban Overrid from using payment providers. It bans apps from using them as their own in-system checkout.

Allowed boundary flows include:

- user buys ORU through Overbill and an approved payment provider;
- user receives refund or chargeback correction through Overbill;
- provider receives eligible payout after KYC/KYB, AML checks, dispute windows, tax handling, and payout approval;
- Overbill creates invoices, receipts, payment-provider refs, and tax/accounting records;
- licensed or authorized partners perform regulated payment or e-money functions where required.

## Required Evidence

Every monetized app should have replayable evidence for:

- app owner identity and authority;
- accepted publisher terms version;
- active monetization policy version;
- ORU-only attestation;
- declared subscription, purchase, paid feature, listing, service-unit, and machine-payment rules;
- absence of external checkout routes in manifest, package metadata, catalog content, and app UI declarations;
- Overbill and ORU Account Service refs for every paid user-facing action;
- enforcement state, warning history, holds, appeals, and final decisions.
