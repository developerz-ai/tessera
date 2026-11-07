# auth-segments

`segments.tes` — pins the #6 missing-`constraint`-production resolution together
with #8: `segments: Table<Session>` is the fixed G4 partitioned-store shape,
with `shard: Int[0..3]` refined so the law's per-shard comparison has
a bounded range to check against, and
the `new n: Session where n.id == s, n.user == whom, n.token == tok,
n.shard == 1` constraint list (the chained `where` form §5 fixes) is the
#6-constraint-list form the follow-up grammar allows at admission time.
Note there is no `max_shards` symbol in this module: the refinement's upper
bound is the literal `3`, so nothing undeclared is reachable from the type.

## Issue anchors

| Issue | Anchor (resolution link) | Used? |
|---|---|---|
| #5 keyword list | [#5 comment](https://github.com/developerz-ai/tessera/issues/5#issuecomment-6571502968) | NONE |
| #6 keywords without productions (constraints) | [#6 comment](https://github.com/developerz-ai/tessera/issues/6#issuecomment-6571732818) | YES |
| #7 statement separator | [#7 comment](https://github.com/developerz-ai/tessera/issues/7#issuecomment-6571620854) | NONE |
| #8 G4-fragment productions | [#8 comment](https://github.com/developerz-ai/tessera/issues/8#issuecomment-6571764343) | YES |
| #9 parameterized capability grammar | [#9 comment](https://github.com/developerz-ai/tessera/issues/9#issuecomment-6571798479) | NONE |
| #10 literal collisions / id-literal typing | [#10 comment](https://github.com/developerz-ai/tessera/issues/10#issuecomment-6571629208) | NONE |
| #15 example fixture origin | [#15 comment](https://github.com/developerz-ai/tessera/issues/15#issuecomment-6572277019) | YES — fixture-only example args |
