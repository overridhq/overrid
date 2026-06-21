# Specs Directory Contract

`docs/specs` owns protocol, schema, API, service-contract, reason-code, event-contract, audit-record, and validation-artifact documents that implementation can cite.

Source contracts:

- [Contract authority](contract_authority.md)
- [Service contract template](service_contract_template.md)
- [Reason codes and events](reason_codes_and_events.md)

Rules:

- Hand-authored specs live in stable source files under this directory or documented SDS/service-catalog locations.
- Generated specs belong under `docs/specs/generated/` and stay ignored by default.
- Specs must not be executed as runtime configuration or hidden service discovery.
- Service contracts should include purpose, owned data, public API, events emitted, events consumed, security boundary, operational checks, test expectations, schema refs, and owning phase.
- Protobuf specs are optional internal compact contracts only when an owning SDS and spec justify them.
- Reason-code families, event envelopes, audit records, validation artifacts, and error shapes must cite `packages/schemas` canonical source contracts before service logic is accepted.
