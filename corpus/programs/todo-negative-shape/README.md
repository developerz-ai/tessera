# todo-negative-shape

`boundary.tes` — the *negative* side of the #10 literal-typing boundary: the law
keeps `Id<Task>` and `Id<User>` id domains disjoint inside one rule, and no
law or rule treats them as interchangeable. The disjointness comparison is
`t.owner == u.id` — the `Id<User>` projection of the quantified `User` row — not
`t.owner == u`: comparing `Id<User>` against a `Task` row is an ordinary nominal
type mismatch (docs/spec/v0.1.md:524), not a domain-disjointness edge. The mirror
of `todo-ascribe-boundary` (the ascription side); the fixture block uses the same
`#1`/`#12` id shapes as that program.

## Issue anchors

| Issue | Used? |
|---|---|
| #5 keyword list | NONE |
| #6 keywords without productions | NONE |
| #7 statement separator | YES — `;` terminates every statement, including the `or fail` forms at :26, :27, :28, :29 |
| #8 G4-fragment productions | NONE |
| #9 parameterized capability grammar | YES — `needs` clause (`:10`) |
| #10 literal collisions / id-literal typing | YES — ascribed id literal :12, :13 |
| #15 example fixture origin | YES — fixture-only example args (`bob, alice_task` at :23) |
