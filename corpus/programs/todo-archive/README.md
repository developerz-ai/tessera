# todo-archive

`archive.tes` — the status-transition + stream-append composition over already-
anchored resolutions: a `status` transition rule, the `output ledger += e;` append
shape, a `forall` law, and a three-fixture example. `created` is a `Date`-typed
field (docs/spec/v0.1.md:104 — v0.1 has no `Instant` literal; `Instant` comes only
from `clock.now`), so the law's `t.created < 2026-10-22` compares `Date` to `Date`.
No fresh construct appears; the #9 anchor gives the law's parenthesized precedence.

## Issue anchors

| Issue | Used? |
|---|---|
| #5 keyword list | NONE |
| #6 keywords without productions | NONE |
| #7 statement separator | YES — `;` terminates every statement, including the `or fail` forms at :18, :19, :20, :21 |
| #8 G4-fragment productions | NONE |
| #9 parameterized capability grammar | YES — `needs` clause (`:11`) |
| #10 literal collisions / Date-literal typing | YES — ascribed id literal :13, :14; `Date` literal :15, :26; the `on: Date` parameter is compared by a guard |
| #15 example fixture origin | YES — fixture-only example args (`bob, done_task, opened_on` at :28) |
