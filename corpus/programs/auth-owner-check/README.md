# auth-owner-check

`owner-check.tes` — the §4 owner-guard shape: `require t.owner == by or fail NotFound;`
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

| Issue | Used? |
|---|---|
| #5 keyword list | NONE |
| #6 keywords without productions | NONE |
| #7 statement separator | YES — `;` terminates every statement, including the `or fail` forms at :16, :17 |
| #8 G4-fragment productions | NONE |
| #9 parameterized capability grammar | YES — `needs` clause (`:10`) |
| #10 literal collisions / id-literal typing | YES — ascribed id literal :12, :13 |
| #15 example fixture origin | YES — fixture-only example args (`alice, some_task` at :23) |
