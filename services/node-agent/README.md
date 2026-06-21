# Node Agent Service Contract

`services/node-agent` is reserved for the Overcell node agent and node simulator code when the seed private swarm phases start implementation.

Ownership rules:

- Own local node identity bootstrap, capability discovery handoff, supervised agent behavior, and simulator boundaries once their SDS phases are active.
- Keep package, API, event, and execution contracts in `packages/schemas` and `docs/specs` before runtime code depends on them.
- Do not bypass Overgate, Overmesh, Overlease, Overrun, Overmeter, or Overwatch contracts with direct private storage or queue access.
- Keep simulator fixtures test-scoped and separate from local state or secrets.
- Repository Layout Phase 6 keeps node-agent modules in the `node_agent_modules` dependency group. They may consume contracts but must not import integration harness internals, local stack internals, fixture writers, integration artifacts, `infra/local`, or docs files as executable configuration.
- Any future node-agent service split requires measured pressure, updated SDS/service-catalog/build-plan/spec evidence, and validation before it is accepted.

This scaffold is a path and ownership contract only. It is not an early node-agent runtime.
