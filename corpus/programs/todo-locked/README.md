# todo-locked

`locked.tes` — exercises the #7 statement-boundary resolution: the
rule body mixes `require ... or fail Name;`, `let x = e or fail Name;`, an
`unless` assignment, `return e;`, and a parenthesized law. Every statement ends
with `;` — the separator the whole corpus encodes — so multi-line rule bodies
parse unambiguously without significant whitespace.

## Issue anchors

| Issue | Used? |
|---|---|
| #5 keyword list | YES — `return` at statement position (`:22`) |
| #6 keywords without productions | NONE |
| #7 statement separator | YES — `;` terminates every statement, including the `or fail` forms at :16, :17, :18, :19, :20 |
| #8 G4-fragment productions | NONE |
| #9 parameterized capability grammar | YES — `needs` clause (`:10`) |
| #10 literal collisions / id-literal typing | YES — ascribed id literal :12, :13; `Date` literal :20 |
| #15 example fixture origin | YES — fixture-only example args (`bob, alice_task` at :27) |
