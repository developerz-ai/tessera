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

| Issue | Used? |
|---|---|
| #5 keyword list | NONE |
| #6 keywords without productions (constraints) | YES — the `where`-chain constraint list at :16 |
| #7 statement separator | YES — `;` terminates every statement, including the `or fail` forms at :15 |
| #8 G4-fragment productions | YES — the partitioned-store shape (`state segments: Table<Session> key id;`, `:6`) |
| #9 parameterized capability grammar | NONE |
| #10 literal collisions / id-literal typing | YES — ascribed id literal :9, :10 |
| #15 example fixture origin | YES — fixture-only example args (`seg_one, bob, seg_token` at :21, :22) |
