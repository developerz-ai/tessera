# todo-precedence

`precedence.tes` — pins the #9 expression-precedence reading: `forall t in tasks`
mixes `||`, `=>`, `in` ({...} membership), `&&` conjunction and `or fail`-independent
`or` in one expression, every law hand-checked against a fresh-empty-store state.
The `guarded_owner` guard is part of the predicate — written as an `&&` conjunct
beside the positive test, because the claim grammar (§2.1: a claim is a quantifier
and a predicate) has no `with (...)` production — and it sits to the right of the
positive test so the §5 ordered-conjunction rule carries the fact inward.

## Issue anchors

| Issue | Used? |
|---|---|
| #5 keyword list | NONE |
| #6 keywords without productions | NONE |
| #7 statement separator | YES — `;` terminates every statement, including the `or fail` forms at :24, :25 |
| #8 G4-fragment productions | NONE |
| #9 parameterized capability grammar | YES — `needs` clause (`:10`) |
| #10 literal collisions / id-literal typing | YES — ascribed id literal :12, :13; `Date` literal :15, :19 |
| #15 example fixture origin | YES — fixture-only example args (`bob, alice_task` at :21) |
