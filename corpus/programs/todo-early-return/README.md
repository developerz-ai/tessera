# todo-early-return

`early-return.tes` — `return` used as a statement in a plain sequential rule body,
mixed with `or fail` paths (#7). There is no `if`/`else if` block in the body: v0.1
defines no `if` or `else` production, and neither word is a keyword in §2's tables
(`docs/spec/v0.1.md:66`), so the early-exit reading comes from the `or fail` guards
and the final `return t.id;` — never from a conditional branch. The second guard's
predicate is one ordinary expression, `(t.status is Done) || (t.created >= on)`, so
§3.1's ordered conjunction and `||` rules apply to it unchanged.

Anchor: the #5 resolution's decision to keep `return` reserved even in rule bodies
(`docs/spec/v0.1.md:66`), so it reads as a value-producing exit rather than an
identifier. The law `resolved_from_open` binds every name it reads — `forall t in
tasks` binds `t`, and `users` is a declared state table — following
[`auth-owner-check/`](../auth-owner-check/owner-check.tes)'s law shape rather than
an open `with (...)` guard. `created` is a `Date` field and `on: Date` is the rule
parameter it is compared with, so `t.created >= on` compares `Date` to `Date`.

The example's third argument is the `opened_on` fixture for the `on: Date`
parameter — §4 (`docs/spec/v0.1.md:630`) requires every `example` argument to be a
declared fixture, so no inline `Date` literal appears there. On the fresh store
`tasks` is empty, `tasks[alice_task]` misses, and `fails NotFound` holds.

## Issue anchors

| Issue | Used? |
|---|---|
| #5 keyword list / `return` reserved | YES — `return` at statement position (`:21`) |
| #6 keywords without productions | NONE |
| #7 statement separator | YES — `;` terminates every statement, including the `or fail` forms at :17, :18, :19 |
| #8 G4-fragment productions | NONE |
| #9 parameterized capability grammar | YES — `needs` clause (`:10`) |
| #10 literal collisions / id-literal typing | YES — ascribed id literal :12, :13; `Date` literal :14 |
| #15 example fixture origin | YES — fixture-only example args (`bob, alice_task, opened_on` at :26) |
