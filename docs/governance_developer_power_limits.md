# Developer Power Limits in Mature Overrid

## Purpose

Overrid should not ask the public to trust developers permanently. The mature system must make developer overreach mathematically invalid at the client, node, ledger, routing, and governance layers.

The core rule is:

```text
Developers can write code, but they cannot make valid system state alone.
```

This document defines the control model that limits developer power after Overrid matures from founder-operated seed infrastructure into a grid-resident public system.

## Trust Model

Public trust should come from verifiable rules, not promises. A mature Overrid client or node must reject invalid authority even when the invalid request is signed by a developer-controlled key.

The public-trust formula is:

```text
Trust = open code
      + client-enforced invariants
      + threshold signatures
      + developer vote caps
      + timelocks
      + transparency logs
      + reproducible builds
      + forkability
```

## No Single Admin Key

Overrid must never have one master key that can change balances, routes, names, apps, policies, accounts, or ledger state.

Critical actions must require threshold signatures from independent groups.

Example:

```text
Critical protocol upgrade:
requires 9 of 15 signatures
developer keys counted max: 3
must include: node operators + universities/institutions + user/public stewards + security auditors
```

Even if all developers agree, they cannot pass the action alone.

## Class-Capped Governance

Threshold signing should not be simple `t-of-n`. It must include signer-class constraints so one social group cannot capture governance by stacking keys.

Example validation rule:

```text
Valid action =
  total signatures >= threshold
  AND developer signatures <= developer_cap
  AND at least N independent signer classes approved
  AND timelock expired
  AND public audit record exists
```

Signer classes may include:

- Core developers.
- Independent node operators.
- University or institutional operators.
- Public/user stewards.
- Security auditors.
- Native service operators.
- Legal/compliance stewards where required.

Developer signatures may contribute, but they must never be sufficient for critical governance actions.

## Timelocks and Challenge Windows

Dangerous actions must wait before activation.

Recommended windows:

- Normal protocol upgrade: 14 to 30 days.
- Economic rule change: 30 to 90 days.
- Namespace or root-policy change: 30 days.
- Emergency security patch: immediate or short delay, but time-limited and later ratified.

Timelocks give users, institutions, node operators, auditors, and competing client implementations time to inspect, object, fork, or refuse the update.

## Client-Enforced Constitution

The Overrid client and node software must contain hard protocol invariants. If an action violates these rules, the client rejects it even when developers signed it.

Required invariants:

- No arbitrary ORU minting.
- No arbitrary Seal Ledger rewriting.
- No forced wallet seizure without valid policy, evidence, quorum, and appeal path.
- No silent route hijack.
- No hidden update.
- No policy change without required quorum and timelock.
- No central AI unilateral punishment.
- No downgrade to weaker governance rules without the same or stronger approval threshold.
- No acceptance of unsigned or unverifiable critical state.

These rules must be part of the protocol, not only operational policy.

## Append-Only Transparency Logs

Every governance action must be written to an append-only public transparency log.

Covered records include:

- Protocol upgrades.
- Emergency interventions.
- Native app governance changes.
- Grant and donation allocations.
- Sanctions and appeals.
- Namespace changes.
- Ledger checkpoints.
- Release approvals.
- Root key changes.

The transparency flow is:

```text
action -> signed proposal -> public log -> Merkle root -> witnesses -> client verification
```

If developers or operators attempt to show different histories to different users, witness and gossip nodes should detect the split.

## Reproducible Signed Updates

Developers can publish software, but mature clients should only accept updates that are:

- Open source.
- Reproducibly built.
- Signed by threshold release keys.
- Included in the public update log.
- Compatible with rollback protection.
- Free of governance-rule downgrades unless approved through the highest threshold path.

This blocks silent backdoored updates from one developer, one compromised machine, or one captured release process.

## Capability-Based Administration

Overrid must not use broad super-admin accounts. Operational permissions should be scoped capabilities with expiry, evidence, and audit trails.

Examples:

```text
can_view_metrics
can_pause_app_for_2_hours
can_rotate_service_key
can_publish_release_candidate
can_open_dispute_case
can_execute_emergency_patch_until_timestamp
```

Every capability must define:

- Actor identity.
- Scope.
- Expiry.
- Allowed action class.
- Required signer set.
- Evidence reference.
- Audit log reference.

## ORU and Seal Ledger Immutability Boundaries

ORU and Seal Ledger must not expose manual mutation paths in production.

Invalid production functions:

```text
set_balance(user, amount)
delete_ledger_entry(id)
rewrite_history(...)
```

Ledger state may change only through valid state transitions, such as:

- Usage settlement.
- Refund.
- Dispute resolution.
- Grant allocation.
- Authorized correction.
- Provider payout.
- Wallet funding or withdrawal through approved rails.

Corrections must be append-only. The system should record compensating entries instead of rewriting history.

## Central AI as Recommender, Not Executor

Central AI may detect, recommend, score, explain, draft reports, and prepare evidence. It must not directly punish, seize, ban, spend, or rewrite state.

Serious actions require:

```text
central AI evidence
+ policy rule
+ governance or human quorum
+ appeal path
+ public log entry
```

This makes central AI powerful as a stewardship and fraud-detection mechanism without turning it into an opaque ruler.

## Emergency Powers

Emergency powers are necessary, but they must be bounded.

Emergency actions should:

- Use separate emergency keys.
- Require a smaller but still multi-class quorum.
- Expire automatically.
- Be publicly logged immediately.
- Require post-incident ratification.
- Be limited to containment, not permanent punishment.
- Never allow silent ORU, Seal Ledger, identity, or namespace rewrites.

Emergency authority should pause, isolate, or limit risk. It should not create permanent finality without normal governance review.

## Developer Role After Maturity

After maturity, developers should retain authority to:

- Maintain code.
- Publish proposals.
- Ship release candidates.
- Run their own nodes and services.
- Participate in governance as one signer class.
- Respond to security incidents within bounded rules.

Developers should not retain unilateral authority to:

- Freeze or seize wallets.
- Edit ORU or Seal Ledger balances.
- Redirect names, routes, or apps.
- Remove users without evidence-based process.
- Spend public-interest funds alone.
- Override node owner resource rules.
- Force hidden updates.
- Shut down the grid.

## Final Principle

Mature Overrid should be designed so developer signatures are useful but insufficient. Public clients, nodes, and services should accept only state transitions that satisfy the protocol's mathematical governance rules.

The target is:

```text
Developers control the code they write.
Node owners control their resources.
Users control their identities, wallets, data, and permissions.
Governance controls protocol-level rules.
Central AI recommends and audits, but does not rule alone.
The grid keeps running as long as enough independent nodes choose to run it.
```
