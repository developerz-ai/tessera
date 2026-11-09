# auth-segments

`segments.tes` — pins the #6 missing-`constraint`-production resolution together
with #8: `segments: Table<Session>` is the fixed G4 partitioned-store shape,
with `shard: Int[0..3]` refined so the law's per-shard comparison has
a bounded range to check against, and
the `new n: Session where n.id == s, n.user == owner.id, n.token == tok,
n.shard == 1` constraint list (the chained `where` form §5 fixes) is the
#6-constraint-list form the follow-up grammar allows at admission time.
Note there is no `max_shards` symbol in this module: the refinement's upper
bound is the literal `3`, so nothing undeclared is reachable from the type.

`admit` opens with `let owner = users[whom] or fail NotFound;` (§4,
`docs/spec/v0.1.md:638`), so the rule reads a table and the declared
`fails NotFound` has a reachable failure site: `state users: Table<User> key id;`
is empty on the fresh store, `users[bob]` misses, and `admit` aborts with
`NotFound` before its `new` runs. `owner` is then projected (`n.user == owner.id`)
rather than the `whom` parameter read directly — the same local-binding shape
[`auth-login/`](../auth-login/login.tes) uses for `s.user == u.id`. Without that
lookup the body would be one `new` whose fields are all bound by parameters and
literals, and the declared failure could never occur.

## Issue anchors

| Issue | Anchor (resolution link) | Used? |
|---|---|---|
| #5 keyword list | [#5 comment](https://github.com/developerz-ai/tessera/issues/5#issuecomment-6571502968) | NONE |
| #6 keywords without productions (constraints) | [#6 comment](https://github.com/developerz-ai/tessera/issues/6#issuecomment-6571732818) | YES |
| #7 statement separator | [#7 comment](https://github.com/developerz-ai/tessera/issues/7#issuecomment-6571620854) | YES — `;` terminates every statement, including the `or fail` forms at :15 |
| #8 G4-fragment productions | [#8 comment](https://github.com/developerz-ai/tessera/issues/8#issuecomment-6571764343) | YES |
| #9 parameterized capability grammar | [#9 comment](https://github.com/developerz-ai/tessera/issues/9#issuecomment-6571798479) | NONE |
| #10 literal collisions / id-literal typing | [#10 comment](https://github.com/developerz-ai/tessera/issues/10#issuecomment-6571629208) | YES — ascribed id literal :9, :10 |
| #15 example fixture origin | [#15 comment](https://github.com/developerz-ai/tessera/issues/15#issuecomment-6572277019) | YES — fixture-only example args |
