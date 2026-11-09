# todo-extended-state

`extended.tes` — compositions only, no fresh grammar: two `state` tables
(`tasks`, `users`), a `output ledger: Stream<Entry>;`, a law, and a
three-fixture-argument `example` (`bob`, `alice_task`, `opened_on: Date`).
The rule reads its `on: Date` parameter in the `Stale` guard at :21, so the
`Date` fixture feeds a value the body actually compares, not a dangling
argument. The `Date` fixture rides the #10 literal-typing record; every anchor
here is a composition of already-recorded resolutions.

## Issue anchors

| Issue | Used? |
|---|---|
| #5 keyword list | NONE |
| #6 keywords without productions | NONE |
| #7 statement separator | YES — `;` terminates every statement, including the `or fail` forms at :19, :20, :21 |
| #8 G4-fragment productions | NONE |
| #9 parameterized capability grammar | YES — `needs` clause (`:12`) |
| #10 literal collisions / id-literal typing | YES — ascribed id literal :14, :15; `Date` literal :16; the guard's `Date` literal :21 |
| #15 example fixture origin | YES — fixture-only example args (`bob, alice_task, opened_on` at :28) |
