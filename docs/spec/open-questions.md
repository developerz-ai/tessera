# Open questions

Spec gaps found while scaffolding, `As of 2026-10-22`. Swarm rule (spec §13): **ambiguity is filed as an issue to the spec owner, never guessed.** Each row is a GitHub issue labelled `spec:decision` or `spec:ambiguity` on milestone **M0**. Resolved → fix the spec in the same PR, delete the row.

## Decisions (spec §14 "Open choices")

| ID | Question | Blocks |
|---|---|---|
| [D1](https://github.com/developerz-ai/tessera/issues/1) | Postgres in v0.1, or SQLite only until later? | WP E store trait shape |
| [D3](https://github.com/developerz-ai/tessera/issues/3) | Exact decidable fragment for `T where p` and laws. | WP B refinement checking, WP D encoding |
| [D4](https://github.com/developerz-ai/tessera/issues/4) | Does `tessera fill` ship in v0.1? | WP K scope, CLI table |

## Grammar and lexing (spec §2)

| ID | Gap | Why it blocks WP A |
|---|---|---|

## Packages

K2 resolved in [spec §13](v0.1.md#13-swarm-work-packages-and-milestones) ([#19](https://github.com/developerz-ai/tessera/issues/19)): WP J owns registry-free git resolution, hashing, content-addressed storage, lockfile and audit; discovery is an optional git short-name-to-URL index, not required for dependency resolution.
