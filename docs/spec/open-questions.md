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
| [G1](https://github.com/developerz-ai/tessera/issues/5) | Words used as syntax but missing from the keyword list: `return`, `in`, `or`, `key`, `index`, `bound`, `scope`, `as`. | Reserved or contextual? Decides the lexer. |
| [G2](https://github.com/developerz-ai/tessera/issues/6) | Keywords with no production: `output`, `schedule`, `assume`, `hint`, `foreign`, `plugin`, `never`, `old`, `exists`, `match`, `then` (as a statement). | Nothing to parse them into. |
| [G3](https://github.com/developerz-ai/tessera/issues/7) | Statement separator: whitespace is insignificant, yet every example separates statements by newline only (and `surface cli` uses `;`). | Without a terminator, `a = b\nc = d` has more than one parse. |
| [G4](https://github.com/developerz-ai/tessera/issues/8) | No productions for `store`, `surface`, `app`, `use`, `formula`, `constraints`, `pat`, `place`, `bounds`. | AST schema cannot freeze. |
| [G5](https://github.com/developerz-ai/tessera/issues/9) | `needs` allows only `Ident ":" Ident`, but §6 uses parameterized capabilities (`net.listen(port)`, `fs(path)`, `store:Db`). | Grammar contradicts §6. |
| [G6](https://github.com/developerz-ai/tessera/issues/10) | Literal collisions: date `2026-10-22` vs subtraction, duration `7d` vs identifier, id `#12` — and how `#12` gets its `T` in `Id<T>`. | Lexer rules and typing of id literals. |

## Semantics (spec §4–§5)

| ID | Gap | Why it blocks |
|---|---|---|
| [S1](https://github.com/developerz-ai/tessera/issues/11) | `view` vs `fact`: the spec gives them one row and one production. What differs? | WP B, C |
| [S2](https://github.com/developerz-ai/tessera/issues/12) | Rule-level `needs`: §6 says needs are part of a rule's signature, but the `rule` production has no needs clause. Inferred from the body? | WP B signatures, WP J compat diff |
| [S3](https://github.com/developerz-ai/tessera/issues/13) | A law that fails at commit at runtime: what failure name does the caller see? | WP C, semantics cases |
| [S4](https://github.com/developerz-ai/tessera/issues/14) | `formula` grammar for `law`/`prove`, including `is idempotent` and cost goals (`time <= 5ms`). | WP A, D |
| [S5](https://github.com/developerz-ai/tessera/issues/15) | `example` fixtures: where do `bob` and `alice_task` come from? | Corpus format (M0) |
| [S6](https://github.com/developerz-ai/tessera/issues/16) | Variant narrowing: `t.status is Done => t.status.at` needs flow typing after `is`. Specified? | WP B |
| [S7](https://github.com/developerz-ai/tessera/issues/17) | Numerics: `Decimal(n)` division and mixed precision; runtime out-of-range on refined `Int` from external input — compile error is impossible there, so which failure? | WP B, C |

## Packages

| ID | Gap | Why it blocks |
|---|---|---|
| [K1](https://github.com/developerz-ai/tessera/issues/18) | §8/§11 list `publish`; [packages.md](packages.md) has no upload step and adds `tessera release`. Drop `publish`? | CLI contract |
| [K2](https://github.com/developerz-ai/tessera/issues/19) | WP J says "registry"; packages.md removes the central registry. Rescope WP J to resolver + lockfile + audit + optional discovery index? | WP J scope |
