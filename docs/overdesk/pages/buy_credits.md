# Buy Credits

## Slug

`buy-credits`

## Title

Buy Credits

## Navigation Group

Wallet, Credits, And Ownership

## Description

Buy Credits is the Overdesk surface for creating credit-purchase intents into an authorized ORU account. It should make funding an account easy while preserving the ORU, Seal Ledger, Overbill, and payment-provider boundaries that prevent per-operation payment friction.

## Primary Users

- Regular users
- Organization admins
- Institution account managers
- App owners
- Delegated account managers
- Resource providers buying usage credits

## Primary User Goals

- Choose the correct account to fund.
- Select ORU dimension or approved bundle.
- Understand the target account, payment handoff, expected crediting path, and receipt state.
- Complete payment without exposing payment secrets to Overdesk.
- See failed, pending, credited, cancelled, refunded, or disputed states.
- Return to Wallet with updated projections and receipts.

## Entry Points

- Wallet low-balance action.
- Home Dashboard wallet warning.
- Notifications Center low-balance or precheck failure alert.
- App deployment budget precheck.
- Native app usage precheck.
- Provider payout or account management flows.
- Address bar command: `/buy-credits`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active account/scope.
- Purchase-intent state.
- Payment handoff state.
- Primary action: Continue Payment.
- Secondary actions: Cancel, View Wallet, View Receipts.

### Account And Scope

Content:

- Funding target account.
- Personal, organization, institution, app-owner, or delegated account marker.
- Role and authorization state.
- Current balance projection.
- Low-balance reason where launched from a warning.
- Account restrictions or compliance markers.

Links and handoffs:

- Wallet.
- Settings And Security.

### Credit Dimension Selector

Content:

- Available ORU dimensions.
- Approved bundles where policy allows.
- Purpose scope where applicable.
- Grant or sponsored-credit alternatives.
- Budget/precheck context when launched from another page.

Links and handoffs:

- Grants And Public-Interest Projects.
- Central AI Stewardship for grant/public-interest refs.

### Amount And Preview

Content:

- Amount input.
- Preset amount controls.
- Expected post-purchase projection.
- External payment-provider fee note where applicable.
- Cancellation/refund path summary.
- Usage estimate context if launched from app deployment or usage precheck.

### Payment Handoff

Content:

- Payment method refs.
- External payment-provider route.
- Handoff status.
- Redirect/return state.
- Payment pending marker.
- Timeout warning.
- Retry state.

### Confirmation Review

Content:

- Target account.
- ORU dimension/bundle.
- Payment-provider handoff ref.
- Expected ORU crediting path.
- Overbill intent ref.
- Seal Ledger checkpoint display.
- Cancellation/refund path.
- Confirmation checkbox for account-sensitive purchases.

### Receipt And Status

Content:

- Purchase intent status.
- Overbill receipt refs.
- ORU projection update refs.
- Seal Ledger checkpoint refs.
- Wallet refresh action.
- Statement/export handoff.
- Activity timeline handoff.

Links and handoffs:

- Wallet.
- Activity And Receipts Timeline.
- Disputes And Appeals.

### Failure, Refund, And Retry Panel

Content:

- Payment failed state.
- Payment pending too long state.
- Reconciliation mismatch state.
- Cancelled state.
- Refund requested state.
- Dispute state.
- Retry payment action.
- Open dispute action.

### Safety And Policy Notes

Content:

- No per-operation payment reminder.
- ORU is an internal non-speculative utility credit.
- Credits settle usage internally through ORU and Seal Ledger.
- Overdesk does not store payment secrets.
- Failed payments do not locally credit the wallet.

## Primary Actions

- Select account.
- Select credit dimension.
- Enter amount.
- Create purchase intent.
- Continue payment.
- Refresh wallet.
- Open receipt.

## Secondary Actions

- Cancel purchase.
- Retry payment.
- Open dispute.
- View grant alternatives.
- Export receipt.
- Return to Wallet.

## States

- Empty form.
- Loading account.
- Account not authorized.
- Amount invalid.
- Policy restricted.
- Intent draft.
- Intent created.
- Payment handoff pending.
- Payment pending.
- Payment failed.
- Reconciliation pending.
- Credited.
- Cancelled.
- Refund requested.
- Disputed.
- Offline blocked.
- Error with retry.

## Permissions And Privacy Behavior

- Buy Credits must never store card data, bank credentials, payment secrets, vault secrets, or raw payment-provider tokens in Overdesk.
- Creating a purchase intent requires signed actor identity, account authorization, idempotency key, policy refs, and payment-provider handoff refs.
- Spendable credit must not appear until payment-provider event state, Overbill record, Seal Ledger entry, and ORU projection reconcile.
- Overdesk owns the purchase-intent form and status display; Overbill, ORU Account Service, Seal Ledger, and external payment integrations own settlement, crediting, receipts, refunds, and reconciliation.

## Design Notes

- Use a short stepper: Account, Credits, Payment, Receipt.
- Keep external payment status and wallet credit status visually separate.
- Make the target account impossible to miss on every step.
- Avoid sales-language framing; this is account funding for resource usage.
- Put failed, pending, refund, and dispute paths in the same screen so users do not lose context.
