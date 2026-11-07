# auth-permission-check

`permission.tes` — pins the "permission-check shape" composition: a
`require p or fail Name;` guard, a `let x = e or fail Name;` bound lookup, a law
combining `in` (membership set literal) with `or` (the #9 precedence pairs), and a
two-fixture example. The law `admins_write` names both operands it reads: it
binds `t` through the declared `forall t in tasks` (§4 quantifier form) and
reads the user's permission via the `users` table (`t.owner` is an `Id<User>`,
not a `User` row, so the set literal is compared against the joined row: the
law's predicate runs over `users[u].permission` with `t.owner == u` bound in
the `forall u in users` guard). Rule bodies read only rule parameters and
table lookups — the guards use `users[by].permission == "tasks.write"` and
`t.owner == by` directly, with no fixture referenced inside a rule body, per
§4's restriction of fixture uses to `example` declarations and fixture
initializers — so the module declares exactly the fixtures its example passes
(`bob`, `alice_task`). No fresh construct rides outside the
existing records.

## Issue anchors

| Issue | Anchor (resolution link) | Used? |
|---|---|---|
| #5 keyword list | [#5 comment](https://github.com/developerz-ai/tessera/issues/5#issuecomment-6571502968)    | NONE |
| #6 keywords without productions | [#6 comment](https://github.com/developerz-ai/tessera/issues/6#issuecomment-6571732818)    | NONE |
| #7 statement separator | [#7 comment](https://github.com/developerz-ai/tessera/issues/7#issuecomment-6571620854) | YES — `require ... or fail` and `let ... or fail` |
| #8 G4-fragment productions | [#8 comment](https://github.com/developerz-ai/tessera/issues/8#issuecomment-6571764343)    | NONE |
| #9 parameterized capability grammar | [#9 comment](https://github.com/developerz-ai/tessera/issues/9#issuecomment-6571798479)    | YES — `in` + `or` precedence in one law |
| #10 literal collisions / id-literal typing | [#10 comment](https://github.com/developerz-ai/tessera/issues/10#issuecomment-6571629208) | NONE |
| #15 example fixture origin | [#15 comment](https://github.com/developerz-ai/tessera/issues/15#issuecomment-6572277019) | YES — fixture-only example args; rule bodies use table lookups, not fixtures |
