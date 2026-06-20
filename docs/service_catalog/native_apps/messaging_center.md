# Messaging Center Implementation Plan

## Objective

Build a username-addressed replacement for fragmented email, phone, and chat identities.

## First Build Phase

[Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md).

## Dependencies

- Overpass namespace.
- Overvault.
- Overstore.
- Personal AI assistant.
- Overguard.
- Overwatch.

## Development Order

1. Build direct username addressing and organization inboxes.
2. Add app notifications and attachments through Overstore.
3. Add encrypted personal messages where appropriate.
4. Add spam, abuse, and identity verification controls.
5. Add optional AI assistant triage with permission.

## Contracts And Interfaces

- Message schema.
- Inbox schema.
- Attachment refs.
- Notification contract.
- Abuse report event.

## Validation

- Messages route to usernames and organization inboxes.
- Attachments obey Overstore and Overvault policy.
- Spam and abuse controls create audit evidence.

## Handoff

Messaging becomes a protocol-level utility for directory, workspace, native apps, and mobile clients.

## Detailed SDS

The detailed design contract is [Messaging Center SDS](../../sds/native_apps/messaging_center.md).

## Sub-Build Plan

- [SUB BUILD PLAN #71 - Messaging Center](../../build_plan/sub_build_plan_071_messaging_center.md)

## Design Alignment

- Treat Messaging Center as a username-addressed communication utility, not as a social feed, identity authority, ad surface, storage system, or payment service.
- Require inboxes, threads, message envelopes, organization inbox routing, app notifications, attachment refs, encrypted payload refs, contact/block prefs, AI triage permissions, abuse reports, and usage refs.
- Integrate with Directory Listings, Workspace, Social, Maps, Personal AI Assistant, AI Gateway Router, Overvault, Overstore, Overguard, Fraud Control, and Wallet/Usage Center through explicit refs and events.
- Encrypted personal messages, org inbox roles, first-contact rules, notification preferences, and AI triage must be permissioned, auditable, and revocable.
