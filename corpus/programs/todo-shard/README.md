# todo-shard

`shard.tes` — a partitioned store shape: `state segments: Table<Task>;` keyed by
`id` with a bounds-refined `shard` discriminator (`Int[0..3]`), a law over one
shard (`t.shard == 1`) whose date guard is the inlined `2026-10-22` literal (§4
keeps fixture uses inside `example` declarations and fixture initializers, so the
cutoff fixture does not appear in the law body), and an example by id. Rides the
#8 store production plus the #10 id-literal and Date-typing records — no fresh
grammar.

## Issue anchors

| Issue | Anchor (resolution link) | Used? |
|---|---|---|
| #5 keyword list | [#5 comment](https://github.com/developerz-ai/tessera/issues/5#issuecomment-6571502968)    | NONE |
| #6 keywords without productions | [#6 comment](https://github.com/developerz-ai/tessera/issues/6#issuecomment-6571732818)    | NONE |
| #7 statement separator | [#7 comment](https://github.com/developerz-ai/tessera/issues/7#issuecomment-6571620854) | NONE |
| #8 G4-fragment productions | [#8 comment](https://github.com/developerz-ai/tessera/issues/8#issuecomment-6571764343)    | YES |
| #9 parameterized capability grammar | [#9 comment](https://github.com/developerz-ai/tessera/issues/9#issuecomment-6571798479)    | NONE |
| #10 literal collisions / Date+id literal typing | [#10 comment](https://github.com/developerz-ai/tessera/issues/10#issuecomment-6571629208) | YES — `fixture cutoff: Date = 2026-10-22;` declared, with the same literal inlined in the law |
| #15 example fixture origin | [#15 comment](https://github.com/developerz-ai/tessera/issues/15#issuecomment-6572277019) | YES — fixture-only example args; fixtures stay out of law bodies |
