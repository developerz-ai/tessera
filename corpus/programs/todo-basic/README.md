# todo-basic

`basic.tes` — minimal todo program covering the §4 skeleton: `state` tables, a
`complete` rule with an `or fail` lookup and a snapshot `unless` write, a
`forall t in tasks` law, and an `example ... fails NotFound;` scenario (fresh
store ⇒ both tables empty ⇒ lookup fails). There is no `create`/`new` rule and no
`prove` block here.

## Issue anchors

| Issue | Used? |
|---|---|
| #5 keyword list / `return` reserved | NONE |
| #6 keywords without productions | NONE |
| #7 statement separator | YES — `;` terminates every statement, including the `or fail` forms at :16, :17 |
| #8 G4-fragment productions | NONE |
| #9 parameterized capability grammar | YES — `needs` clause (`:10`) |
| #10 literal collisions / id-literal typing | YES — ascribed id literal :12, :13 |
| #15 example fixture origin | YES — fixture-only example args (`bob, alice_task` at :23) |
