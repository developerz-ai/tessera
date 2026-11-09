# auth-permission-check

`permission.tes` — pins the "permission-check shape" composition: a
`require p or fail Name;` guard, a `let x = e or fail Name;` bound lookup, a law
combining `in` (membership set literal) with `or` (the #9 precedence pairs), and a
two-fixture example. The law `admins_write` names both operands it reads: it
binds `t` through `forall t in tasks` and `u` through `forall u in users` (§4
quantifier form), then reads `u.permission` and `u.name` directly off the bound
row — `t.owner == u.id` in the `with (...)` guard is what ties the two `forall`
bounds together (`t.owner` is an `Id<User>`, never a `User` row, so no law may
compare it to `u` itself).

The rule binds its two lookups before it guards, so the declared `fails NotFound`
is reachable: `let t = tasks[id] or fail NotFound;` and
`let u = users[by] or fail NotFound;` run first, and only then the two `or fail
Forbidden` guards, which read the rows those lookups bound. On the fresh store the
first lookup misses and the rule aborts with `NotFound`, so ordering the
`Forbidden` guards ahead of it would make the declared failure unreachable —
the invariant [`../README.md`](../README.md#corpus-invariant-a-declared-failure-must-be-reachable)
records and `corpus_programs.rs` enforces. Rule bodies read only rule parameters
and table lookups — the guards compare `u.permission` and `t.owner` directly,
with no fixture referenced inside a rule body, per §4's restriction of fixture
uses to `example` declarations and fixture initializers — so the module declares
exactly the fixtures its example passes (`bob`, `alice_task`). No fresh construct
rides outside the existing records.

## Issue anchors

| Issue | Anchor (resolution link) | Used? |
|---|---|---|
| #5 keyword list | [#5 comment](https://github.com/developerz-ai/tessera/issues/5#issuecomment-6571502968) | NONE |
| #6 keywords without productions | [#6 comment](https://github.com/developerz-ai/tessera/issues/6#issuecomment-6571732818)    | NONE |
| #7 statement separator | [#7 comment](https://github.com/developerz-ai/tessera/issues/7#issuecomment-6571620854) | YES — `;` terminates every statement, including the `or fail` forms at :16, :17, :18 |
| #8 G4-fragment productions | [#8 comment](https://github.com/developerz-ai/tessera/issues/8#issuecomment-6571764343)    | NONE |
| #9 parameterized capability grammar | [#9 comment](https://github.com/developerz-ai/tessera/issues/9#issuecomment-6571798479) | YES — `needs` clause (`:10`) |
| #10 literal collisions / id-literal typing | [#10 comment](https://github.com/developerz-ai/tessera/issues/10#issuecomment-6571629208) | YES — ascribed id literal :12, :13 |
| #15 example fixture origin | [#15 comment](https://github.com/developerz-ai/tessera/issues/15#issuecomment-6572277019) | YES — fixture-only example args; rule bodies use table lookups, not fixtures |
