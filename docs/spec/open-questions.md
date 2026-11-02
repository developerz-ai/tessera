# Open questions

Spec gaps found while scaffolding, `As of 2026-10-22`. Swarm rule (spec §13): **ambiguity is filed as an issue to the spec owner, never guessed.** Each row is a GitHub issue labelled `spec:decision` or `spec:ambiguity` on milestone **M0**. Resolved → fix the spec in the same PR, delete the row.

## Decisions (spec §14 "Open choices")

| ID | Question | Blocks |
|---|---|---|
| [D1](https://github.com/developerz-ai/tessera/issues/1) | Postgres in v0.1, or SQLite only until later? | WP E store trait shape |
| [D2](https://github.com/developerz-ai/tessera/issues/2) | Confirm UTC-only `time` for v0.1. | stdlib `time`, `Date` literal semantics |
| [D3](https://github.com/developerz-ai/tessera/issues/3) | Exact decidable fragment for `T where p` and laws. | WP B refinement checking, WP D encoding |
| [D4](https://github.com/developerz-ai/tessera/issues/4) | Does `tessera fill` ship in v0.1? | WP K scope, CLI table |

## Grammar and lexing (spec §2)

| ID | Gap | Why it blocks WP A |
|---|---|---|

## Semantics (spec §4–§5)

| ID | Gap | Why it blocks |
|---|---|---|
| [S2](https://github.com/developerz-ai/tessera/issues/12) | Rule-level `needs`: §6 says needs are part of a rule's signature, but the `rule` production has no needs clause. Inferred from the body? | WP B signatures, WP J compat diff |
| [S5](https://github.com/developerz-ai/tessera/issues/15) | `example` fixtures: where do `bob` and `alice_task` come from? | Corpus format (M0) |
| [S6](https://github.com/developerz-ai/tessera/issues/16) | Variant narrowing: `t.status is Done => t.status.at` needs flow typing after `is`. Specified? | WP B |

## Packages

| ID | Gap | Why it blocks |
|---|---|---|
| [K1](https://github.com/developerz-ai/tessera/issues/18) | §8/§11 list `publish`; [packages.md](packages.md) has no upload step and adds `tessera release`. Drop `publish`? | CLI contract |
| [K2](https://github.com/developerz-ai/tessera/issues/19) | WP J says "registry"; packages.md removes the central registry. Rescope WP J to resolver + lockfile + audit + optional discovery index? | WP J scope |
