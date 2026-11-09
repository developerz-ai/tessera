# todo-shard

`shard.tes` — a partitioned store shape: `state segments: Table<Task>;` keyed by
`id` with a bounds-refined `shard` discriminator (`Int[0..3]`), a law over one
shard (`t.shard == 1`) whose date guard is the inlined `2026-10-22` literal (§4
keeps fixture uses inside `example` declarations and fixture initializers, so no
fixture appears in the law body), and an example by id. Rides the
#8 store production plus the #10 id-literal and Date-typing records — no fresh
grammar.

## Issue anchors

| Issue | Used? |
|---|---|
| #5 keyword list | NONE |
| #6 keywords without productions | NONE |
| #7 statement separator | YES — `;` terminates every statement, including the `or fail` forms at :16, :17 |
| #8 G4-fragment productions | YES — the partitioned-store shape (`state segments: Table<Task> key id;`, `:7`) |
| #9 parameterized capability grammar | YES — `needs` clause (`:10`) |
| #10 literal collisions / Date+id literal typing | YES — ascribed id literal :12, :13; `Date` literal :21 |
| #15 example fixture origin | YES — fixture-only example args (`bob, shard_task` at :23) |
