# todo-negative-shape

`boundary.tes` — the *negative* side of the #10 literal-typing boundary: the law
keeps every id comparison inside one nominal domain, and no law or rule treats
`Id<Task>` and `Id<User>` as interchangeable. Its only id comparison is
`t.owner == u.id` — two `Id<User>` values, the `owner` column of the quantified
`Task` row and the key of the quantified `User` row — not `t.owner == u`:
comparing `Id<User>` against a `User` row is an ordinary nominal type mismatch
(docs/spec/v0.1.md:524), not a domain edge. The form the law deliberately does
not take is `t.id != u.id`: `Id<Task>` against `Id<User>` is that same nominal
type mismatch (§3 keeps the two distinct), so it certifies nothing about id
domains. The rule `tag(by: Id<User>, of: Id<Task>)` is the positive side: it
carries both id domains through one rule — `tasks[of]`, `require t.owner == by`,
`users[by]` — without ever comparing an `Id<Task>` value with an `Id<User>` one.
The mirror of `todo-ascribe-boundary` (the ascription side); the fixture block
uses the same `#1`/`#12` id shapes as that program.

## Issue anchors

| Issue | Used? |
|---|---|
| #5 keyword list | NONE |
| #6 keywords without productions | NONE |
| #7 statement separator | YES — `;` terminates every statement, including the `or fail` forms at :30, :31, :32 |
| #8 G4-fragment productions | NONE |
| #9 parameterized capability grammar | YES — `needs` clause (`:10`) |
| #10 literal collisions / id-literal typing | YES — ascribed id literal :12, :13 |
| #15 example fixture origin | YES — fixture-only example args (`bob, alice_task` at :27) |
