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
| [G2](https://github.com/developerz-ai/tessera/issues/6) | Keywords with no production: `output`, `schedule`, `assume`, `hint`, `foreign`, `plugin`, `never`, `old`, `exists`, `match`, `then` (as a statement). | Nothing to parse them into. |
| [G4](https://github.com/developerz-ai/tessera/issues/8) | §2.1 resolves the eight non-formula productions: `store`, `surface`, `app`, `use`, `constraints`, `pat`, `place`, `bounds`, with boundary cases and syntactic AST requirements in §2.2. Only `formula` remains outstanding, delegated to S4/[#14](https://github.com/developerz-ai/tessera/issues/14), including proof goals. Retain this row until formula lands; #8 is not fully closed. | Formula syntax still blocks AST schema freeze; the scaffold has no implemented AST schema to validate. |
| [G5](https://github.com/developerz-ai/tessera/issues/9) | `needs` allows only `Ident ":" Ident`, but §6 uses parameterized capabilities (`net.listen(port)`, `fs(path)`, `store:Db`). | Grammar contradicts §6. |
| [G6](https://github.com/developerz-ai/tessera/issues/10) | Literal collisions: date `2026-10-22` vs subtraction, duration `7d` vs identifier, id `#12` — and how `#12` gets its `T` in `Id<T>`. | Lexer rules and typing of id literals. |

## Semantics (spec §4–§5)

| ID | Gap | Why it blocks |
|---|---|---|
| [S1](https://github.com/developerz-ai/tessera/issues/11) | `view` vs `fact`: the spec gives them one row and one production. What differs? | WP B, C |
| [S2](https://github.com/developerz-ai/tessera/issues/12) | Rule-level `needs`: §6 says needs are part of a rule's signature, but the `rule` production has no needs clause. Inferred from the body? | WP B signatures, WP J compat diff |
| [S4](https://github.com/developerz-ai/tessera/issues/14) | `formula` grammar for `law`/`prove`, including `is idempotent` and cost goals (`time <= 5ms`). | WP A, D |
| [S5](https://github.com/developerz-ai/tessera/issues/15) | `example` fixtures: where do `bob` and `alice_task` come from? | Corpus format (M0) |
| [S6](https://github.com/developerz-ai/tessera/issues/16) | Variant narrowing: `t.status is Done => t.status.at` needs flow typing after `is`. Specified? | WP B |
| [S7](https://github.com/developerz-ai/tessera/issues/17) | Numerics: `Decimal(n)` division and mixed precision; runtime out-of-range on refined `Int` from external input — compile error is impossible there, so which failure? | WP B, C |

## Packages

| ID | Gap | Why it blocks |
|---|---|---|
| [K1](https://github.com/developerz-ai/tessera/issues/18) | §8/§11 list `publish`; [packages.md](packages.md) has no upload step and adds `tessera release`. Drop `publish`? | CLI contract |
| [K2](https://github.com/developerz-ai/tessera/issues/19) | WP J says "registry"; packages.md removes the central registry. Rescope WP J to resolver + lockfile + audit + optional discovery index? | WP J scope |
