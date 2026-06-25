# SUB BUILD PLAN #85 - Internal KYC Service

Attached SDS: [SDS #85 - Internal KYC Service](../sds/governance_ops/internal_kyc_service.md)

## Purpose

This sub-build plan turns SDS #85 into an implementation sequence for Internal KYC Service. It stays aligned with the master build plan, the service catalog, the SDS layer, the AML rules document, and the accepted Rust-first tech stack.

Internal KYC Service is the Phase 13 KYC, KYB, beneficial-owner, source-of-funds, screening, cooling-period, manual high-credit, and cash-out eligibility fact service. It exists to stop ORU, Seal Ledger, Overbill, and Provider Payout flows from becoming anonymous laundering rails.

The service protects the ORU-first economy without replacing it. Users may earn ORU through approved resource contribution or legitimate services and spend ORU on valid Overrid services. KYC/AML controls gate high-risk funding, payout destinations, source-of-funds review, and cash-out eligibility; they do not block normal ORU spending unless policy, holds, disputes, freezes, or compliance facts require it.

Internal KYC Service does not move money, mutate ORU balances, write Seal Ledger entries, approve payout batches by itself, provide legal advice, own fraud graph authority, expose raw private evidence, or decide suspicious-activity finality without the legal/steward process.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #85: Internal KYC Service](../sds/governance_ops/internal_kyc_service.md) | Controls purpose, scope, non-goals, actors, dependencies, policy inputs, records, APIs, events, workflows, state model, AML guardrails, privacy, security, validation, build order, and resolved implementation decisions. |
| [Internal KYC Service plan](../service_catalog/governance_ops/internal_kyc_service.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, SDS design alignment, and handoff. |
| [Overrid AML Rules](../aml_rules.md) | Defines laundering patterns, Turkish-law baseline, funding limits, cooling periods, fake-app detection, cash-out controls, and enforcement expectations. |
| [Master build plan](master_plan.md) | Controls the canonical Phase 0 through Phase 13 build order and keeps SDS #85 first build work in Phase 13 with earlier Phase 5 hooks. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies accounting, funding, payout-hold, ORU projection, Seal Ledger, Overbill, and Provider Payout integration points that must reserve KYC/AML refs before public cash-out exists. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Controls the first full Internal KYC Service build point, including KYC/KYB, AML, manual high-credit review, cooling periods, cash-out eligibility, exports, reporting, threat review, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #85 discoverable from the numbered service set and aligned to Phase 13 with Phase 5 accounting hooks. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, generated contracts, canonical JSON plus JSON Schema where appropriate, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and TypeScript only for client/operator surfaces. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 8, 11, 12, and 13 | Attach SDS #85, preserve Phase 13 as first full build, record Phase 5 hooks, and freeze KYC/AML authority boundaries. |
| 2 | Master Phases 0, 1, 4, 5, 8, and 13 | Define Rust contracts, KYC/KYB schemas, policy refs, stable errors, signed fact bundles, and deterministic fixtures. |
| 3 | Master Phases 1, 5, 8, and 13 | Implement protected evidence refs, provider tokenization, redaction, operator access controls, retention, and no-tipping-off projections. |
| 4 | Master Phases 1, 4, 8, and 13 | Implement person KYC subjects, verification attempts, screening refs, manual review, risk tiers, signed KYC facts, and expiry. |
| 5 | Master Phases 1, 4, 5, 8, and 13 | Implement business KYB, authorized representatives, beneficial owners, app/provider ownership binding, refresh, and revocation. |
| 6 | Master Phases 5, 8, 12, and 13 | Implement payout destination ownership, destination-change holds, source-of-funds/source-of-wealth records, manual high-credit review, and connected-transaction aggregation. |
| 7 | Master Phases 4, 5, 11, and 13 | Implement cash-out eligibility evaluation from KYC/KYB, cooling periods, related-party, app-legitimacy, dispute, chargeback, reconciliation, and risk refs. |
| 8 | Master Phases 4, 5, 8, 11, 12, and 13 | Integrate Overbill, Provider Payout, ORU Account Service, Seal Ledger, Overguard, Wallet, Overdesk, Overwatch, Fraud Control, and Reputation/Anti-Sybil through refs only. |
| 9 | Master Phases 8, 12, and 13 | Implement user/operator surfaces, redacted APIs, review queues, compliance exports, refresh jobs, expiry jobs, and safe reporting inputs. |
| 10 | Master Phase 13 with evidence from Phases 0 through 12 | Validate AML drills, privacy controls, replay, policy freshness, threat/security review, incident hooks, docs links, and launch readiness. |

## Tech Stack Guardrails

- Internal KYC Service uses Rust-first shared contracts and service-facing APIs for KYC subjects, profiles, verification attempts, evidence refs, screening refs, source-of-funds records, payout destinations, manual reviews, cash-out eligibility facts, exports, refresh jobs, and replay bundles. TypeScript is acceptable only for generated client, Wallet, Overdesk, admin, or operator surfaces and must call Overrid APIs without becoming a KYC authority.
- Contracts, profile records, fact bundles, stable errors, event envelopes, replay bundles, export manifests, redaction profiles, policy examples, and deterministic fixtures use canonical JSON plus JSON Schema where docs-facing contracts or fixtures are needed. Compact Protobuf may be generated later only through the shared contract layer.
- Mutating APIs require signed actor/service envelopes, tenant or system scope, trace id, idempotency key, subject refs, policy refs, evidence refs, schema version, stable reason codes, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for evidence refs, provider payload refs, replay bundles, export manifests, fixture inputs, and deterministic comparison artifacts. BLAKE3 must not be described as encryption.
- Raw identity documents, liveness media, bank/payment credentials, tax data, source-of-funds evidence, screening payloads, and sensitive operator notes must stay behind Overvault or approved tokenized provider refs. Ordinary service records carry hashes, refs, coarse statuses, reason codes, timestamps, freshness windows, and audit refs.
- External KYC, liveness, sanctions, PEP, bank-account, payment-destination, or tax providers may be used only behind narrow tokenized adapters. They must not become the product boundary, policy authority, evidence store, ledger authority, payout authority, or source of user-facing truth.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, an external KYC/compliance SaaS, external legal-advice tooling, external payment custody, Kubernetes-first orchestration, blockchain, NFT mechanics, hardcoded pricing, revenue forecasts, customer-count assumptions, raw private-data exports, suspicious-activity tipping-off, fraud finality, accounting mutation, or payout approval the Internal KYC Service authority.

## Phase 1: SDS Attachment, Phase 13 Scope, And KYC/AML Authority Boundary

### Work Items

- **1.1 Attach the build plan to SDS #85.**
  - Design: Keep this plan reachable from the Internal KYC SDS, service catalog plan, master build plan, Phase 13 plan, Phase 5 accounting plan, build-plan crosswalk, AML rules, and tech-stack decision.
  - Output: Stable links between this file, `docs/sds/governance_ops/internal_kyc_service.md`, `docs/service_catalog/governance_ops/internal_kyc_service.md`, `docs/build_plan/master_plan.md`, `docs/build_plan/phase_13_governance_compliance_scale_hardening.md`, `docs/build_plan/phase_05_metering_oru_seal_ledger_overbill.md`, `docs/build_plan/service_catalog_alignment.md`, `docs/aml_rules.md`, and `docs/overrid_tech_stack_choice.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #85 returns the SDS, service plan, crosswalk, Phase 13 workstream, and this sub-build plan.

- **1.2 Preserve Phase 13 as the first full build point.**
  - Design: Keep full KYC/KYB, AML, manual high-credit, and cash-out eligibility implementation in Phase 13 because it depends on identity, tenancy, signing, audit, policy, accounting, vault, fraud, public-provider, native-app, and governance prerequisites.
  - Output: Phase-gate note that Phase 5 reserves accounting hooks for KYC/AML refs, holds, and payout eligibility while Phase 13 builds the full KYC/AML fact authority.
  - Validation: Review confirms the plan does not move full KYC authority into Phase 5 accounting, Phase 4 policy enforcement, Phase 8 vault/storage, Phase 11 fraud/reputation, or Phase 12 Wallet/Overdesk surfaces.

- **1.3 Freeze Internal KYC Service ownership boundaries.**
  - Design: Record that Internal KYC owns KYC/KYB subjects, person/business profiles, beneficial-owner records, authorized-representative refs, verification attempts, payout destination ownership facts, source-of-funds/source-of-wealth refs, screening refs, risk tiers, manual review cases, cash-out eligibility facts, refresh records, exports, and replay bundles.
  - Output: Ownership checklist covering architecture, APIs, records, events, operator flows, privacy controls, downstream handoffs, and review gates.
  - Validation: Checklist rejects payment processing, payout execution, Seal Ledger writes, ORU mutation, billing documents, fraud-model ownership, legal advice, final suspicious-transaction filing authority, raw evidence storage, or public-facing AML heuristic exposure.

- **1.4 Carry forward resolved SDS #85 decisions.**
  - Design: Preserve the resolved decisions that external providers are tokenized adapters only, policy thresholds come from Compliance Boundary bundles, cash-out eligibility is composed and not equal to completed KYC, connected-party analysis uses refs from owner services, and user/operator visibility is split by audience.
  - Output: Resolved-decision checklist for provider boundaries, policy bundle freshness, composed allow facts, connected-transaction refs, redacted user states, operator access gates, and retention/deletion behavior.
  - Validation: Review rejects hardcoded thresholds, raw evidence in ordinary records, provider truth leakage, KYC-only payout allow, hidden graph authority, exact risk-threshold display, suspicious-reporting disclosure, and deletion paths that erase legally required AML records.

- **1.5 Define upstream and downstream service boundaries.**
  - Design: Map Overpass, Overtenant, Overkey, Overvault, Overwatch, Overguard, Compliance Boundary Service, Overbill, ORU Account Service, Seal Ledger, Provider Payout Service, Fraud Control, Reputation and Anti-Sybil, Overclaim, Wallet and Usage Center, Overdesk, Central AI, SDK, CLI, and Admin/Developer UI interactions through refs and signed facts.
  - Output: Boundary matrix naming allowed reads, owned writes, required refs, denied direct mutations, redaction classes, evidence refs, usage refs, audit refs, replay refs, and owner-service finality.
  - Validation: Review confirms Internal KYC exchanges signed refs/events/fact bundles and never copies private internals or grants itself mutation authority owned by another service.

## Phase 2: Contracts, Schemas, Policy Inputs, Stable Errors, And Fixtures

### Work Items

- **2.1 Create the Internal KYC Rust contract module.**
  - Design: Add contract types for KYC subjects, person profiles, business profiles, beneficial owners, authorized representatives, verification attempts, payout destinations, source-of-funds/source-of-wealth records, screening refs, cash-out facts, manual reviews, refresh jobs, exports, replay bundles, stable errors, and lifecycle states.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, subject/state/reason-code enums, policy-ref fields, fact-bundle signature metadata, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from payment, ledger, payout, fraud-authority, legal-advice, and raw-evidence storage code.

- **2.2 Define subject and profile schemas.**
  - Design: Model `kyc_subject`, `person_kyc_profile`, `business_kyb_profile`, `beneficial_owner`, and authorized representative projections with Overpass refs, Overtenant refs, organization refs, jurisdiction refs, risk tier, state, refresh due time, and audit refs.
  - Output: JSON Schema files, valid examples, invalid examples, stable errors, migration notes, privacy notes, and deterministic fixtures for person, organization, beneficial-owner, app-provider, and system-actor subjects.
  - Validation: Schema tests reject raw legal names, raw identity numbers, raw birth dates, raw tax numbers, raw registry numbers, missing tenant/identity refs, unversioned records, missing jurisdiction refs, missing audit refs, and unbounded private payloads.

- **2.3 Define verification, evidence, screening, payout-destination, and source-of-funds schemas.**
  - Design: Model verification attempts, identity/liveness/provider result refs, payout destination ownership facts, source-of-funds/source-of-wealth records, screening bundle refs, operator review refs, and expiry/freshness windows.
  - Output: Schema set, lifecycle state matrix, reason-code catalog, provider callback envelope, Overvault ref requirements, tokenization fields, ownership match fields, source trigger fields, and golden fixtures.
  - Validation: Tests reject raw document images, bank/card/payment credentials, tax forms, screening payloads, missing provider signatures, missing Overvault refs, missing expiry, unsupported rail/currency/region facts, and unsigned provider results.

- **2.4 Define cash-out eligibility, policy input, export, and replay schemas.**
  - Design: Model `cashout_eligibility_fact` with KYC/KYB currentness, beneficial-owner status, payout destination ownership, source-of-funds status, screening status, cooloff state, related-party state, app-legitimacy state, chargeback/dispute/reconciliation facts, policy version, decision, reason codes, expiry, and signature ref.
  - Output: Fact bundle schema, policy input schema, export manifest schema, replay bundle schema, stable errors, audience projections, deterministic examples, and policy-version compatibility notes.
  - Validation: Tests prove KYC alone cannot produce `allow`, missing/stale/contradictory policy bundles fail closed, fact bundles include policy version and generated timestamp, and replay reconstructs decisions from stored refs rather than live policy state.

- **2.5 Create deterministic fixtures and golden traces.**
  - Design: Build fixtures for person KYC, business KYB, beneficial-owner change, payout destination ownership, source-of-funds review, manual high-credit request, cash-out allow, cash-out hold, cash-out deny, manual review, refresh, export, and replay.
  - Output: Fixture directory, canonical inputs, expected states, expected events, expected stable errors, BLAKE3 hashes, usage refs, audit refs, redacted projections, and replay bundles.
  - Validation: Fixture tests produce stable ids, hashes, states, reason codes, redaction behavior, usage refs, audit refs, and replay output across repeated runs.

## Phase 3: Protected Evidence, Provider Adapters, Redaction, And Retention

### Work Items

- **3.1 Implement Overvault-backed evidence refs.**
  - Design: Store only evidence refs, hashes, scopes, freshness windows, redaction classes, and access policy metadata in Internal KYC records while raw identity, liveness, bank, tax, source-of-funds, and screening payloads remain in Overvault or approved tokenized provider stores.
  - Output: Evidence-ref model, evidence class registry, Overvault grant requirements, evidence freshness checks, hash metadata, and degraded-state diagnostics.
  - Validation: Tests prove ordinary records, events, logs, replay bundles, exports, and user projections do not include raw identity documents, liveness media, bank numbers, tax forms, screening details, or private operator notes.

- **3.2 Implement PII-safe logging and redacted projections.**
  - Design: Create redaction rules for user, affected-party, operator, steward, auditor, service-to-service, and public aggregate audiences with no-tipping-off constraints.
  - Output: Redactor module, audience projection schemas, safe status codes, support/debug log filters, export redaction profiles, and invalid examples.
  - Validation: Tests reject exact thresholds, graph cluster membership, sanctions hit details, suspicious-transaction status, private operator notes, provider payloads, and law-enforcement/reporting status in user-facing or broad operator surfaces.

- **3.3 Implement tokenized provider adapter contracts.**
  - Design: Accept external KYC, liveness, sanctions, PEP, bank-account, payment-destination, and tax-provider results only through signed, idempotent, tokenized adapter callbacks with provider refs, evidence refs, result refs, and replay hashes.
  - Output: Provider adapter envelope, callback verification, idempotency handling, retry/failure states, provider health records, stale-result errors, and Overwatch audit events.
  - Validation: Tests reject unsigned callbacks, replayed callbacks with mismatched payload hashes, direct raw provider data writes, provider decisions without policy refs, stale callbacks, and callbacks that attempt to mutate money-service state.

- **3.4 Implement operator evidence access control.**
  - Design: Require role, purpose, case assignment, tenant/system scope, evidence class, time bound, and Overwatch audit before operator evidence access.
  - Output: Evidence access request flow, access grant refs, denied-state reason codes, break-glass request records, audit events, and evidence-read replay links.
  - Validation: Tests prove unassigned operators, expired roles, missing purpose, unsupported evidence class, and stale case refs cannot read private evidence; every approved read writes an audit ref.

- **3.5 Implement retention, deletion, and correction behavior.**
  - Design: Consume Compliance Boundary retention/deletion policy bundles so legally required AML records are preserved while non-required copies and projections can be redacted or deleted.
  - Output: Retention job contracts, deletion/redaction command envelopes, correction records, supersession refs, audit refs, export redaction updates, and replay notes.
  - Validation: Tests prove deletion requests do not erase legally required AML records, non-required copies are redacted according to policy, corrections append new facts instead of rewriting history, and replay remains deterministic after redaction.

## Phase 4: Person KYC, Screening, Manual Review, Risk Tiers, And KYC Fact Bundles

### Work Items

- **4.1 Implement person subject resolution and KYC intake.**
  - Design: Resolve or create person subjects from Overpass identity refs, tenant scope, jurisdiction profile, required activity matrix, and requested monetized/funding/payout capability.
  - Output: `POST /kyc/subjects`, `GET /kyc/subjects/{id}`, person profile intake, state transitions from `not_started` through `collecting`, and safe user-facing next-step states.
  - Validation: Tests reject missing identity, missing tenant scope, missing jurisdiction profile, unsupported subject type, stale session refs, and raw PII in ordinary profile records.

- **4.2 Implement verification attempt lifecycle.**
  - Design: Track verification attempts for person KYC, identity documents, liveness, phone/email/address evidence, provider callbacks, manual results, expiry, and supersession.
  - Output: `POST /kyc/verification-attempts`, `POST /kyc/verification-attempts/{id}/result`, attempt states, idempotent callback handling, reason codes, audit refs, and replay refs.
  - Validation: Tests prove attempts are append-only, provider callbacks are signed, duplicate idempotency keys return original results, failed/expired/superseded attempts cannot produce current allow facts, and manual results require authorized operator refs.

- **4.3 Implement screening refs and risk tier assignment.**
  - Design: Attach sanctions, blocked-party, PEP, jurisdiction, and policy-required screening refs as evidence inputs without exposing raw screening hits outside authorized review.
  - Output: `POST /kyc/screening`, screening bundle refs, risk-tier state changes, refresh requirements, redacted reason codes, and Overwatch events.
  - Validation: Tests prove screening-required flows fail closed when refs are missing, risk-tier changes are signed and audited, and user projections do not reveal sensitive screening payloads or suspicious-reporting status.

- **4.4 Implement manual KYC review cases.**
  - Design: Route failed, ambiguous, high-risk, stale, or contradicted KYC evidence into assigned manual review cases with purpose, evidence refs, operator notes, decision refs, and escalation state.
  - Output: `POST /kyc/manual-reviews`, review queue records, assignment records, decision states, escalation refs, safe user statuses, and replay bundles.
  - Validation: Tests prove manual review cannot be completed without assignment, evidence refs, reason codes, and audit refs; broad surfaces only show coarse remediation or pending-review states.

- **4.5 Implement signed person KYC fact bundles.**
  - Design: Produce signed KYC facts only after current evidence, screening, policy bundle, risk tier, and review state satisfy the required activity matrix.
  - Output: KYC fact bundle builder, signature refs, expiry, freshness windows, policy version, reason-code mapping, current/stale/expired lookup APIs, and Overguard handoff envelope.
  - Validation: Tests prove facts are signed, versioned, replayable, deny-by-default on missing policy, expire at the configured refresh point, and block cash-out when failed, expired, revoked, stale, or under review.

## Phase 5: Business KYB, Authorized Representatives, Beneficial Owners, And App Ownership

### Work Items

- **5.1 Implement business KYB intake.**
  - Design: Capture business entity refs, registry/tax refs, registered address refs, business purpose refs, expected Overrid use refs, app-provider refs, screening refs, and source-of-funds/source-of-wealth refs without raw business evidence in ordinary records.
  - Output: `POST /kyc/business-profiles`, KYB profile states, required evidence matrix, jurisdiction refs, status reason codes, and safe user/operator projections.
  - Validation: Tests reject raw registry documents, tax documents, missing jurisdiction refs, missing authority refs, unscoped app-provider refs, and unsigned profile updates.

- **5.2 Implement authorized representative checks.**
  - Design: Bind organization actions to authorized representatives who have current person KYC, authority evidence refs, role scope, tenant/org scope, expiry, and audit refs.
  - Output: Representative record schema, authority-ref validation, representative KYC currentness checks, lifecycle states, and review queue handoffs.
  - Validation: Tests prove organization-owned app or payout flows cannot advance when representative KYC is missing, stale, revoked, expired, or outside scope.

- **5.3 Implement beneficial-owner records.**
  - Design: Track ownership/control type, ownership band, authority refs, person subject refs, screening refs, verification status, effective windows, and supersession without rewriting history.
  - Output: `POST /kyc/beneficial-owners`, beneficial-owner lifecycle, append-only changes, screening requirements, invalidation refs, and replay bundles.
  - Validation: Tests prove beneficial-owner changes invalidate relevant payout eligibility until reviewed and cannot silently alter previous KYB or payout facts.

- **5.4 Implement app/provider ownership binding.**
  - Design: Bind KYB subjects, authorized representatives, beneficial owners, app refs, provider refs, Overasset ownership refs, Overregistry records, and payout eligibility scopes.
  - Output: App/provider ownership binding records, ownership freshness checks, app/provider payout-scope refs, missing-fact diagnostics, and owner-service handoff refs.
  - Validation: Tests prove organization-owned app payout cannot become eligible without current KYB, beneficial-owner, representative, app ownership, and provider-scope facts.

- **5.5 Implement KYB refresh, revocation, and suspended-state behavior.**
  - Design: Schedule KYB and beneficial-owner refreshes from policy bundles, revoke or suspend stale/contradicted facts, and append replacement facts rather than mutating prior records.
  - Output: KYB refresh jobs, risk-tier update events, revoked/suspended/expired states, safe user statuses, downstream invalidation events, and replay refs.
  - Validation: Tests prove stale KYB facts expire automatically, revoked or suspended KYB blocks payout, and downstream Provider Payout and Overguard consumers receive invalidation refs.

## Phase 6: Payout Destinations, Source Of Funds, Manual High-Credit Review, And Connected Transactions

### Work Items

- **6.1 Implement payout destination ownership facts.**
  - Design: Verify tokenized payout destination refs, ownership evidence refs, name match state, rail region, supported currency refs, tokenization refs, state, verification time, and expiry.
  - Output: `POST /kyc/payout-destinations`, destination ownership schema, ownership match reason codes, supported rail/currency facts, expiry jobs, and audit refs.
  - Validation: Tests prove destination mismatch, missing ownership evidence, unsupported rail/currency, stale destination, or missing tokenization blocks payout without exposing raw payout credentials.

- **6.2 Implement payout destination change holds.**
  - Design: Apply policy-driven holds or manual review when payout destination ownership changes, high-risk sessions precede destination changes, or account takeover signals are present.
  - Output: Destination change event, hold fact, review trigger, Wallet/Overdesk safe status, Provider Payout stale-fact response, and replay bundle.
  - Validation: Tests prove recent destination changes block payout until the active policy window expires or authorized review completes.

- **6.3 Implement source-of-funds and source-of-wealth records.**
  - Design: Capture trigger refs, declarations, evidence refs, reviewer refs, reason codes, effective windows, source scope, amount/purpose/rail bounds, and audit refs for high-value or suspicious flows.
  - Output: `POST /kyc/source-of-funds`, source evidence records, review states, expiry windows, redacted projections, and export/replay refs.
  - Validation: Tests prove high-value, suspicious, manual high-credit, large funding, large payout, or policy-refresh paths cannot proceed when required source evidence is missing, stale, rejected, or out of scope.

- **6.4 Implement manual high-credit review.**
  - Design: Let Overbill, Wallet, and Overdesk create manual high-credit requests that cite active policy caps, funding history, connected transaction totals, source-of-funds evidence, Fraud Control refs, and operator decision refs.
  - Output: Manual high-credit request schema, review queue state, amount-bounded approvals, purpose-scoped approvals, time-bounded approvals, denial/reduction/delay states, and Overbill handoff refs.
  - Validation: Tests prove auto credit purchases above active caps are denied and routed to manual review, approvals are bounded and replayable, and Overbill cannot treat missing KYC/AML facts as approved.

- **6.5 Implement connected-transaction aggregation.**
  - Design: Aggregate signed refs from Overpass, Overtenant, ORU Account Service, Seal Ledger, Overbill, Provider Payout, Fraud Control, Reputation/Anti-Sybil, Overclaim, app/provider ownership, and Compliance Boundary policy windows without creating hidden graph authority.
  - Output: Connected transaction snapshot, related-party state, fake-app risk refs, structuring-window refs, reason codes, and cash-out eligibility inputs.
  - Validation: Tests prove linked buyer/app-owner loops, threshold structuring, related accounts, mule-like patterns, chargebacks, and disputes create hold/manual-review facts without exposing graph internals to users.

## Phase 7: Cash-Out Eligibility Engine, Cooling Periods, Fraud Inputs, And Replay

### Work Items

- **7.1 Implement cash-out eligibility evaluation API.**
  - Design: Accept Provider Payout evaluation requests with provider refs, app refs, payout period refs, earning refs, subject refs, policy refs, trace id, idempotency key, and requested audience.
  - Output: `POST /kyc/cashout-eligibility/evaluate`, `GET /kyc/facts/{subject_id}/cashout`, submitted/facts_validated/evaluated states, stable errors, and audit events.
  - Validation: Tests reject requests without provider/app/payout period/earning refs, missing policy refs, stale facts, unsupported tenant scope, or duplicate idempotency mismatches.

- **7.2 Implement composed allow-fact evaluation.**
  - Design: Require current KYC/KYB, beneficial-owner status, authorized representative status, payout destination ownership, source-of-funds/source-of-wealth where required, screening status, and active policy bundle before considering allow.
  - Output: Evaluation pipeline, fact currentness checks, missing-prerequisite states, deny/hold/manual-review/allow decisions, and reason-code mapping.
  - Validation: Tests prove KYC completion alone never returns `allow`, organization payout requires KYB and beneficial owners, missing source-of-funds blocks high-risk flows, and stale facts fail closed.

- **7.3 Implement cooling periods and bought-credit cash-out controls.**
  - Design: Enforce first-payout cooloff, post-funding cash-out holds, new-app payout probation, payout-destination refresh windows, connected transaction windows, and the rule that bought credits are not directly cash-out eligible for the buyer.
  - Output: Cooloff state calculator, bought/earned/held ORU input refs, app-age refs, funding refs, policy version refs, hold decisions, and replay notes.
  - Validation: Tests prove first payout and post-funding cooloffs block otherwise valid payout items, buyer-funded ORU cannot be cashed out directly by the buyer, and bought ORU remains spendable inside Overrid when not frozen, held, or prohibited by policy.

- **7.4 Integrate fraud, reputation, dispute, chargeback, and reconciliation inputs.**
  - Design: Consume Fraud Control, Reputation/Anti-Sybil, Overclaim, Overbill, ORU Account Service, Seal Ledger, Provider Payout, app legitimacy, chargeback, dispute, and reconciliation refs as inputs to cash-out eligibility.
  - Output: Risk input manifest, input freshness checks, hold/manual-review reason codes, owner-service response refs, and replayable input snapshots.
  - Validation: Tests prove active fraud, related-party, app-review, challenge, dispute, chargeback, refund, reversal, or reconciliation mismatch refs block or hold affected payout items.

- **7.5 Implement deterministic replay and fail-closed behavior.**
  - Design: Store policy version, input fact refs, output fact refs, decision state, reason codes, signatures, redaction profile, and generated timestamp for every cash-out evaluation.
  - Output: `GET /kyc/replay/{fact_id}`, replay bundle, redacted replay projection, integrity hashes, and stale-policy diagnostics.
  - Validation: Tests prove replay reconstructs each decision from stored refs, stale or missing policy inputs never replay as allow, and unauthorized replay projections exclude private evidence.

## Phase 8: Money-Service, Policy, Audit, Wallet, And Desktop Integrations

### Work Items

- **8.1 Integrate Overbill funding and high-credit hooks.**
  - Design: Connect Overbill funding requests, receipts, chargebacks, refunds, manual high-credit requests, and payment-provider refs to Internal KYC through signed refs and fact lookups only.
  - Output: Overbill handoff envelope, funding hold refs, manual review request refs, source-of-funds trigger refs, KYC/AML fact lookup responses, and replay links.
  - Validation: Tests prove Overbill cannot auto-approve high-credit purchases above active caps, cannot bypass source-of-funds review, and cannot mutate KYC facts directly.

- **8.2 Integrate Provider Payout eligibility gates.**
  - Design: Make Provider Payout Service require current cash-out eligibility facts before payout batch inclusion while still owning payout batch lifecycle, payment instruction refs, result tracking, reversals, and correction coordination.
  - Output: Provider Payout precheck contract, batch item block/hold/allow states, stale-fact errors, idempotent lookup flow, and Overwatch refs.
  - Validation: Tests prove payout batch creation fails when cash-out facts are missing, stale, expired, hold, deny, or manual-review, and Internal KYC does not execute payout.

- **8.3 Integrate ORU Account Service and Seal Ledger refs.**
  - Design: Consume bought/spent/earned/held ORU projections and Seal Ledger funding, hold, release, correction, earning, chargeback, reversal, and settlement refs without mutating ledger/accounting truth.
  - Output: ORU projection input manifest, Seal Ledger ref mapping, hold/release reason-code handoff, reconciliation input snapshot, and replay refs.
  - Validation: Tests prove Internal KYC can explain cash-out decisions from ORU/ledger refs but cannot change balances, write ledger entries, or settle payouts.

- **8.4 Integrate Overguard and policy dry-run handoffs.**
  - Design: Publish signed KYC/KYB, AML, source-of-funds, and cash-out fact bundles as policy inputs for Overguard and side-effect-free previews for SDK, CLI, Admin/Developer UI, Wallet, and Overdesk.
  - Output: Overguard input envelope, policy dry-run projection, reason-code mapping, deny-by-default states, and replay links.
  - Validation: Tests prove Overguard remains policy enforcement authority and Internal KYC cannot directly allow execution, funding, payout, publication, or public-provider admission.

- **8.5 Integrate Wallet, Overdesk, audit, and owner-service recovery paths.**
  - Design: Provide Wallet and Overdesk safe verification, payout-readiness, source-of-funds, and review statuses while Overwatch records every operator action and downstream services retain owner truth.
  - Output: Safe read APIs, owner-service handoff events, retry states, degraded diagnostics, Wallet/Overdesk projection schemas, and audit/replay refs.
  - Validation: Tests simulate consuming-service outage and prove fact bundles are retained for retry without widening access, copying raw private facts, or mutating owner-service records.

## Phase 9: User Surfaces, Operator Queues, Refresh Jobs, Exports, And Reporting Inputs

### Work Items

- **9.1 Implement safe user-facing APIs.**
  - Design: Expose only coarse, remediable verification and payout-readiness states for users, organizations, app owners, and providers.
  - Output: `GET /me/verification-status`, `POST /me/verification/start`, `POST /me/source-of-funds`, `GET /me/payout-readiness`, safe status codes, localized reason-code hooks, and denial/remediation states.
  - Validation: Tests prove user APIs do not expose exact thresholds, screening hits, suspicious-reporting status, internal graph refs, private operator notes, or provider payloads.

- **9.2 Build Wallet and Overdesk KYC/KYB surfaces.**
  - Design: Render verification status, KYB requirements, source-of-funds upload prompts, payout destination state, payout readiness, manual review pending states, and remediation paths through owner-service APIs.
  - Output: Wallet/Overdesk page contracts, safe projection schemas, action-intent envelopes, stale/offline states, denied states, and audit refs.
  - Validation: Review confirms Wallet and Overdesk are client surfaces only and cannot approve KYC, alter facts, bypass policy, read raw evidence, or approve payouts.

- **9.3 Build admin/operator review queues.**
  - Design: Provide assigned operator queues for KYC failures, KYB exceptions, beneficial-owner changes, payout destinations, source-of-funds, high-credit requests, cash-out holds, refresh failures, and export review.
  - Output: Review queue API, assignment workflow, evidence access flow, decision records, escalation states, SLA/age diagnostics, and no-tipping-off projections.
  - Validation: Tests prove operators see evidence only when assigned and authorized, decisions require reason codes and audit refs, and severe or ambiguous cases escalate according to policy.

- **9.4 Implement compliance exports and reporting inputs.**
  - Design: Produce authorized compliance export refs, public aggregate reporting inputs, private audit packages, redacted report inputs, and stewardship/auditor projections according to Compliance Boundary policy bundles.
  - Output: `GET /kyc/exports/{period}`, export job records, redaction profiles, integrity hashes, period refs, policy versions, review records, and replay refs.
  - Validation: Tests prove exports include policy version, decision refs, redaction profile, generated timestamp, evidence manifests, and integrity hashes while excluding unauthorized raw evidence.

- **9.5 Implement refresh, expiry, revocation, and alert jobs.**
  - Design: Schedule KYC refresh, KYB refresh, beneficial-owner refresh, payout destination refresh, source-of-funds expiry, screening refresh, policy refresh, and stale-fact invalidation from signed policy bundles.
  - Output: Refresh job worker, due/overdue states, invalidation events, alert refs, owner-service notifications, Wallet/Overdesk statuses, and replay notes.
  - Validation: Tests prove stale KYC/KYB, stale payout destination, stale screening, stale policy, and expired source evidence automatically block affected cash-out or manual high-credit flows.

## Phase 10: AML Drills, Privacy Validation, Threat Review, Operations, And Launch Readiness

### Work Items

- **10.1 Build AML laundering drill fixtures.**
  - Design: Model the fake-app laundering path from credit purchase through related app spend, ORU earning, cash-out request, and payout hold, plus structuring, mule, chargeback, destination switching, and new-app probation scenarios.
  - Output: Drill fixture set, expected fact states, expected reason codes, expected Overbill/Seal Ledger/Provider Payout/Overguard refs, and replay bundles.
  - Validation: Tests prove fake-app laundering, direct bought-credit cash-out, linked buyer/app-owner flows, threshold structuring, mule-like accounts, chargebacks, and destination switches are blocked before payout.

- **10.2 Validate privacy, redaction, and evidence isolation.**
  - Design: Run privacy tests for records, events, logs, exports, replay bundles, support/debug outputs, Wallet/Overdesk projections, operator projections, and public aggregate reporting inputs.
  - Output: Privacy validation suite, forbidden-field scan, redaction fixture results, operator-access audit proofs, deletion/redaction test evidence, and remediation checklist.
  - Validation: Tests prove no raw PII, payment credentials, tax data, liveness media, screening payloads, source evidence, suspicious-report status, or private operator notes leak into unauthorized surfaces.

- **10.3 Validate policy freshness, signatures, and replay.**
  - Design: Verify every mutating command, fact bundle, export, replay bundle, and downstream handoff carries signed envelopes, policy versions, idempotency keys, trace ids, schema versions, reason codes, and audit refs.
  - Output: Signature tests, stale-policy tests, idempotency tests, replay determinism tests, downgrade/rollback tests, and compatibility fixtures.
  - Validation: Tests prove missing/stale/superseded policy, bad signatures, mismatched hashes, replayed callbacks, unscoped requests, or non-idempotent retries fail closed.

- **10.4 Run threat/security review and incident integration.**
  - Design: Cover fake KYC callbacks, account takeover before payout destination change, related-party laundering, threshold structuring, mule accounts, insider evidence access, tipping off, stale fact replay, provider compromise, and jurisdiction policy drift.
  - Output: Threat model records, security review findings, mitigation refs, accepted-risk records, Incident Response hooks, Fraud Control handoffs, Overclaim handoffs, and Stewardship Reporting inputs.
  - Validation: Review confirms every threat has mitigation, test coverage, monitoring, owner assignment, accepted-risk expiry, or incident-response hook before launch.

- **10.5 Complete launch readiness and documentation validation.**
  - Design: Validate structure, links, stack guardrails, authority boundaries, cross-document alignment, queue/progress records, Docdex retrieval, and handoff readiness before declaring SDS #85 build planning complete.
  - Output: Final validation checklist, link-check evidence, structure-check evidence, stack-guardrail scan, Docdex index refresh, Docdex search evidence, progress update, and handoff note.
  - Validation: Checks confirm title prefix `SUB BUILD PLAN #85`, attached SDS link, phases 1 through 10, five work items per phase, Design/Output/Validation structure, local links, Phase 13 alignment, Phase 5 hooks, tech-stack compliance, and no unresolved documentation drift.

## Exit Criteria

Internal KYC Service is ready for implementation handoff when:

- All 10 sub-build phases are documented with concrete work items and validation expectations.
- The first full build point remains Phase 13 and Phase 5 accounting hooks are explicitly reserved.
- Raw PII, payment credentials, tax data, liveness data, screening payloads, and source evidence are isolated behind Overvault or tokenized provider refs.
- KYC/KYB, source-of-funds, payout destination, screening, and cash-out eligibility facts are signed, versioned, policy-linked, and replayable.
- Provider Payout cannot include payout items without current cash-out eligibility facts.
- Overbill cannot auto-approve high-credit purchases above active policy caps.
- Wallet and Overdesk expose safe statuses without revealing sensitive AML heuristics or suspicious-reporting status.
- Fake-app laundering, structuring, mule, chargeback, destination-switching, stale-fact, and related-party fixtures block payout before money leaves the system.
