# todo-ascribe-boundary

`ascribe.tes` — pins the *ascription side* of the #10 literal-typing boundary:
`fixture alice_task: Id<Task> = #12 : Id<Task>;` is the id-literal-with-ascription
shape, and `fixture opened_on: Date = 2026-10-22;` is the Date-literal one. Per §4,
those fixtures are used only in the `example` declaration and its initializers;
the law and rule bodies inline the values (`t.id == #12 : Id<Task>` in the law,
`on >= 2026-10-22` in the rule body), so no fixture name crosses into a law or
rule body. The law compares the inline value to a table-typed row, so the boundary
through `Id<Task>` and `Date` is exercised together. The mirror edge is
`todo-negative-shape`.

## Issue anchors

| Issue | Used? |
|---|---|
| #5 keyword list | NONE |
| #6 keywords without productions | NONE |
| #7 statement separator | YES — `;` terminates every statement, including the `or fail` forms at :17, :18, :19 |
| #8 G4-fragment productions | NONE |
| #9 parameterized capability grammar | YES — `needs` clause (`:10`) |
| #10 literal collisions / id-literal typing | YES — ascribed id literal :12, :13; `Date` literal :14, :19; the law's ascribed id literal :23 |
| #15 example fixture origin | YES — fixture-only example args (`bob, alice_task, opened_on` at :27) |
