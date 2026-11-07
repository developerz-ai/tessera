# todo-early-return

`early-return.tes` — `return` used as a statement in a plain sequential rule body,
mixed with `or fail` paths (#7). There is no `if`/`else if` block in the body: v0.1
has no block statement (§2 — the spec defines neither `if` nor `else`), so the
early-exit reading comes from `return t.id;` statements and `or fail` alone.
Anchor: the #5 resolution's decision to keep `return` reserved even in rule bodies
so it reads as a value-producing exit rather than being re-interpretable as an
identifier. The example's third argument is the inline Date literal `2026-10-22`
for the `on: Date` parameter — no undeclared fixture name is referenced.

## Issue anchors

| Issue | Anchor (resolution link) | Used? |
|---|---|---|
| #5 keyword list / `return` reserved | [#5 comment](https://github.com/developerz-ai/tessera/issues/5#issuecomment-6571502968)    | YES — `return t.id;` in the sequential rule body (no `if`/`else if` arms) |
| #6 keywords without productions | [#6 comment](https://github.com/developerz-ai/tessera/issues/6#issuecomment-6571732818)    | NONE |
| #7 statement separator | [#7 comment](https://github.com/developerz-ai/tessera/issues/7#issuecomment-6571620854) | YES — `or fail` paths mix with returns |
| #8 G4-fragment productions | [#8 comment](https://github.com/developerz-ai/tessera/issues/8#issuecomment-6571764343)    | NONE |
| #9 parameterized capability grammar | [#9 comment](https://github.com/developerz-ai/tessera/issues/9#issuecomment-6571798479)    | NONE |
| #10 literal collisions / id-literal typing | [#10 comment](https://github.com/developerz-ai/tessera/issues/10#issuecomment-6571629208) | NONE |
| #15 example fixture origin | [#15 comment](https://github.com/developerz-ai/tessera/issues/15#issuecomment-6572277019) | YES — fixture-only example args, with the date inlined |
