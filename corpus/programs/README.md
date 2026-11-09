# corpus/programs — domain programs

Twenty spec-conformant example programs (`todo_*` / `auth_*` domains), one
`.tes` + one README per directory. These are corpus data for M0 sign-off: no
checker reads this directory yet, and the layouts + constructs used are all
anchored against the seven M0 spec issues below — each link names the exact
GitHub comment where the resolution is recorded, so a maintainer can veto one
row without touching the others.

Not runnable: no command compiles `.tes` files yet — the plans
(`docs/plans/README.md`) schedule the parser and interpreter against WP A/C.

| Issue | Topic | Resolution link (anchored) |
|---|---|---|
| #5 | Keyword list, `return` reserved | [#5 (comment)](https://github.com/developerz-ai/tessera/issues/5#issuecomment-6571502968) |
| #6 | Keywords with no grammar production | [#6 (comment)](https://github.com/developerz-ai/tessera/issues/6#issuecomment-6571732818) |
| #7 | Statement separator when whitespace is insignificant | [#7 (comment)](https://github.com/developerz-ai/tessera/issues/7#issuecomment-6571620854) |
| #8 | Missing productions (store, surface, app, use, formula, constraints, pat, place, bounds) | [#8 (comment)](https://github.com/developerz-ai/tessera/issues/8#issuecomment-6571764343) |
| #9 | `needs` grammar & parameterized capabilities | [#9 (comment)](https://github.com/developerz-ai/tessera/issues/9#issuecomment-6571798479) |
| #10 | Literal collisions / typing of id literals | [#10 (comment)](https://github.com/developerz-ai/tessera/issues/10#issuecomment-6571629208) |
| #15 | Where example fixtures come from | [#15 (comment)](https://github.com/developerz-ai/tessera/issues/15#issuecomment-6572277019) |

## Programs

| Directory | Demonstrates | Construct set |
|---|---|---|
| [`todo-basic/`](todo-basic/README.md) | Minimal §4 skeleton: state, one rule, one law, failing example | #10, #15 |
| [`todo-due-date/`](todo-due-date/README.md) | Date literal `2026-10-22` as a Date value, date-comparison law | #10, #15 |
| [`todo-locked/`](todo-locked/README.md) | Section-4 rule body with `require`/`let` + `or fail Name` | #7, #15 |
| [`auth-login/`](auth-login/README.md) | Login rule, `or fail NotFound` on lookup + password check | #15 |
| [`auth-token/`](auth-token/README.md) | `return` as a reserved keyword; `in` precedence in laws | #5, #9, #15 |
| [`todo-output-stream/`](todo-output-stream/README.md) | `output ledger: Stream<Entry>;` + `output ledger += e;` | #7, #9, #15 |
| [`todo-precedence/`](todo-precedence/README.md) | `forall ... in`, `=>`, `or`/`\|\|`, `in` mixed in one law | #9, #15 |
| [`auth-segments/`](auth-segments/README.md) | Partitioned table with admission constraints | #6, #8, #15 |
| [`todo-backlog/`](todo-backlog/README.md) | All seven anchors in one program | #5, #6, #7, #8, #9, #10, #15 |
| [`todo-extended-state/`](todo-extended-state/README.md) | Two state tables + a third audit table, three-fixture example | #10, #15 |
| [`todo-early-return/`](todo-early-return/README.md) | `return` mid-rule, `or fail` paths in the same rule | #5, #7, #15 |
| [`todo-ascribe-boundary/`](todo-ascribe-boundary/README.md) | Id+Date literal ascriptions (`#12 : Id<Task>`, Date fixture) | #10, #15 |
| [`todo-negative-shape/`](todo-negative-shape/README.md) | `Id<Task>` vs `Id<User>` disjoint in one rule (negative edge) | #10, #15 |
| [`auth-session-expiry/`](auth-session-expiry/README.md) | Clock-driven law (`clock.today`, Date comparisons) | #10, #15 |
| [`intake-domains/`](intake-domains/README.md) | Multi-table composition, spanning intake rule | #7, #10, #15 |
| [`auth-role-table/`](auth-role-table/README.md) | Record-valued fields, `users[id].role` update | #7, #15 |
| [`todo-shard/`](todo-shard/README.md) | Partitioned `state segments: Table<Task>;`, per-shard law | #8, #10, #15 |
| [`auth-owner-check/`](auth-owner-check/README.md) | `require u.owner == by or fail NotFound;` guard shape | #7, #10, #15 |
| [`todo-archive/`](todo-archive/README.md) | Archive status transition + stream append + `forall` law | #7, #9, #10, #15 |
| [`auth-permission-check/`](auth-permission-check/README.md) | Permission-check shape: two `or fail` forms across a rule | #7, #9, #15 |

Per-program READMEs list — per issue row — whether that program's syntax rests on
that resolution (`YES`), composes two (`YES / YES` with a note), or does not use
it (`NONE`). `corpus/semantics/` is not touched: that directory is pinned by
`packages/tessera-interp/tests/semantics_schema.rs` (exactly three cases).

## Corpus invariant: a declared failure must be reachable

Every `example <rule>(...) fails <Name>;` MUST name a failure its rule can
actually produce: the rule body carries a matching `... or fail <Name>`, or a
`require ... or fail <Name>`. `new t: T where ...;` only creates a value — it
never reads a table (`docs/spec/v0.1.md:640`) — so a body that is a single `new`
whose fields are all bound by parameters and literals has no failure site and
cannot satisfy `fails <Name>` on the empty store. Where a table read is the point
of the example, bind the lookup first: `let x = <table>[<key>] or fail NotFound;`
(`docs/spec/v0.1.md:638`), then use `x` in the `new` constraints.

No checker reads this directory yet, so this invariant is review-enforced; it
becomes a corpus test when the parser lands (WP A).
