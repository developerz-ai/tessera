# auth-owner-check

`owner-check.tes` — the §4 owner-guard shape: `require u.owner == by or fail NotFound;`
inside a rule with a `forall`-law over the `tasks` table and a two-fixture example
(`complete(alice, some_task)`). The law `completedMeansOwner` binds its free
variable: it reads `(t.status is Done) => (t.owner in users)` with the
`forall t in tasks` bound `t` and the `users` table itself as the membership
operand, rather than an unquantified `u.owner == by` — the §4 `claim`
grammar (`claim = [quantifier] predicate`) has no open-lookaround form, so
every name in the predicate is bound by the declared `forall` or is a
declared state table name. Rides the #7 record for the `or fail` suffix and
the #10 id-literal typing record. Uses a null-snapshot-failure example.

## Issue anchors

| Issue | Anchor (resolution link) | Used? |
|---|---|---|
| #5 keyword list | [#5 comment](https://github.com/developerz-ai/tessera/issues/5#issuecomment-6571502968)    | NONE |
| #6 keywords without productions | [#6 comment](https://github.com/developerz-ai/tessera/issues/6#issuecomment-6571732818)    | NONE |
| #7 statement separator | [#7 comment](https://github.com/developerz-ai/tessera/issues/7#issuecomment-6571620854) | YES — `or fail` on the lookup and guard |
| #8 G4-fragment productions | [#8 comment](https://github.com/developerz-ai/tessera/issues/8#issuecomment-6571764343)    | NONE |
| #9 parameterized capability grammar | [#9 comment](https://github.com/developerz-ai/tessera/issues/9#issuecomment-6571798479)    | NONE |
| #10 literal collisions / id-literal typing | [#10 comment](https://github.com/developerz-ai/tessera/issues/10#issuecomment-6571629208) | YES — law's membership binds owners through the `users` table
| #15 example fixture origin | [#15 comment](https://github.com/developerz-ai/tessera/issues/15#issuecomment-6572277019) | YES — `complete(alice, some_task)` |
