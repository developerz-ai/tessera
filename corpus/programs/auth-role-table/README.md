# auth-role-table

`table.tes` — a `state users: Table<User> key id;` (keyed, like every other
program's tables) holding record-valued fields (`role`, `name`), a rule that
writes through `users[id].role`, a law reading the same field, a stream append
as an `Entry`-typed local (`let e: Entry; let e.user = id; let e.granted =
role; output ledger += e;`), and a two-fixture-argument `example`. All shapes
ride existing anchors — no fresh constructs; the `#15` anchor commits the
two-arg fixture form.

## Issue anchors

| Issue | Anchor (resolution link) | Used? |
|---|---|---|
| #5 keyword list | [#5 comment](https://github.com/developerz-ai/tessera/issues/5#issuecomment-6571502968)    | NONE |
| #6 keywords without productions | [#6 comment](https://github.com/developerz-ai/tessera/issues/6#issuecomment-6571732818)    | NONE |
| #7 statement separator | [#7 comment](https://github.com/developerz-ai/tessera/issues/7#issuecomment-6571620854) | YES — `or fail` on the lookup |
| #8 G4-fragment productions | [#8 comment](https://github.com/developerz-ai/tessera/issues/8#issuecomment-6571764343)    | NONE |
| #9 parameterized capability grammar | [#9 comment](https://github.com/developerz-ai/tessera/issues/9#issuecomment-6571798479)    | NONE |
| #10 literal collisions / id-literal typing | [#10 comment](https://github.com/developerz-ai/tessera/issues/10#issuecomment-6571629208) | NONE |
| #15 example fixture origin | [#15 comment](https://github.com/developerz-ai/tessera/issues/15#issuecomment-6572277019) | YES — two-fixture example args |
