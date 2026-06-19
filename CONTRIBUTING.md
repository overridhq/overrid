# Contributing to Overrid

Thanks for taking the time to contribute.

Overrid is still in the design/specification stage. Contributions should make the public system clearer, more buildable, and more coherent with the whitepaper, build plan, service catalog, SDS files, and tech-stack decision.

## Ground Rules

- Keep Overrid primitives as native Overrid services and protocols.
- Do not replace the core architecture with SaaS, cloud-first, blockchain, NFT, speculative-token, or per-transaction-fee assumptions.
- Do not add revenue projections, pricing assumptions, user-count assumptions, or fundraising claims to canonical docs.
- Do not commit local agent state, scratch plans, raw source imports, generated build output, logs, caches, secrets, or private notes.
- Keep related docs aligned when changing system behavior. A service-catalog change usually needs a matching SDS and build-plan check.
- Prefer concrete interfaces, state models, events, validation rules, and failure modes over broad slogans.

## Getting Started

```bash
git clone git@github.com:overridhq/overrid.git
cd overrid
```

Read these first:

- [README.md](README.md)
- [docs/whitepaper.md](docs/whitepaper.md)
- [docs/overrid_tech_stack_choice.md](docs/overrid_tech_stack_choice.md)
- [docs/build_plan/master_plan.md](docs/build_plan/master_plan.md)
- [docs/service_catalog/master_services.md](docs/service_catalog/master_services.md)
- [docs/sds/master_sds.md](docs/sds/master_sds.md)

## Making Changes

1. Create a branch with a clear name.
2. Keep changes scoped to one topic.
3. Update every affected document in the same change.
4. Check links and markdown fences before opening a pull request.
5. Explain what changed, why it changed, and which docs were updated.

Suggested local checks:

```bash
git status --short
rg -n "TODO|TBD|Claude note|Gemini note|revenue calculation|per transaction fee|NFT|blockchain" docs README.md CONTRIBUTING.md
```

The second command is a review aid, not an automatic rejection. Some words may be valid when the document is explicitly rejecting those mechanics.

## Pull Requests

Good pull requests are small, traceable, and easy to review. Include:

- A short summary.
- The affected docs or services.
- Any changed assumptions.
- Any open question that still needs a decision.

## License and Contribution Rights

By submitting a contribution, you confirm that you have the right to submit it and that it may be included in Overrid under the MIT License.

You grant Bekir Dağ and the Overrid project the rights needed to use, modify, sublicense, and distribute your contribution as part of the project. The project copyright notice remains held by Bekir Dağ unless a separate written agreement says otherwise.
