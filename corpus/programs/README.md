# corpus/programs — domain programs

Twenty spec-conformant example programs in three domains — `todo_*`, `auth_*`
and `intake_*` (the `intake-domains/` program) — one `.tes` + one README per
directory. These are corpus data for M0 sign-off: the layouts and constructs they
use are anchored against the seven M0 spec issues below, and each link names the
exact GitHub comment where the resolution is recorded, so a maintainer can veto
one row without touching the others.

Not runnable: no command parses `.tes` files yet — `docs/plans/` schedules the
parser and interpreter against WP A/C. Until then
`packages/tessera-interp/tests/corpus_programs.rs` checks the text-level
invariants of this directory (layout, declared failures, anchor rows), so a
drifted row fails a test instead of waiting for review.

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

The construct set is exactly the anchors a program's own README marks `YES`, so
the two cannot disagree: `corpus_programs.rs` derives `#5`, `#7`, `#9` and `#10`
from the `.tes` source and checks both against it. `#6`, `#8` and `#15` are a
maintainer's reading of the spec, not a text match.

| Directory | Demonstrates | Construct set |
|---|---|---|
| [`todo-basic/`](todo-basic/README.md) | Minimal §4 skeleton: state, one rule, one law, failing example | #7, #9, #10, #15 |
| [`todo-due-date/`](todo-due-date/README.md) | `Date` literal `2026-10-22` as a value; date-comparison law | #7, #9, #10, #15 |
| [`todo-locked/`](todo-locked/README.md) | Rule body with `require` / `let` + `or fail Name`, `return` | #5, #7, #9, #10, #15 |
| [`auth-login/`](auth-login/README.md) | Login rule: `or fail NotFound` lookup, password guard, token from `rng.token(32)` | #5, #7, #9, #10, #15 |
| [`auth-token/`](auth-token/README.md) | `return` at statement position; `in` precedence in laws | #5, #7, #9, #10, #15 |
| [`todo-output-stream/`](todo-output-stream/README.md) | `output ledger: Stream<Entry>;` + `output ledger += e;` | #7, #9, #10, #15 |
| [`todo-precedence/`](todo-precedence/README.md) | `or`, `=>`, `in` and `with` mixed in one `forall` law set | #7, #9, #10, #15 |
| [`auth-segments/`](auth-segments/README.md) | Bounded-int field (`Int[0..3]`), shard-uniqueness law | #6, #7, #8, #10, #15 |
| [`todo-backlog/`](todo-backlog/README.md) | Guarded `Maybe` payload in a `require` chain; five anchors in one program | #5, #7, #9, #10, #15 |
| [`todo-extended-state/`](todo-extended-state/README.md) | Two state tables plus a `ledger` stream, three-fixture example | #7, #9, #10, #15 |
| [`todo-early-return/`](todo-early-return/README.md) | `return` mid-rule, `or fail` paths in the same rule | #5, #7, #9, #10, #15 |
| [`todo-ascribe-boundary/`](todo-ascribe-boundary/README.md) | Id + `Date` literal ascriptions (`#12 : Id<Task>`) | #7, #9, #10, #15 |
| [`todo-negative-shape/`](todo-negative-shape/README.md) | `Id<Task>` vs `Id<User>` kept disjoint in one rule (negative edge) | #7, #9, #10, #15 |
| [`auth-session-expiry/`](auth-session-expiry/README.md) | Clock-driven `expire` rule, `Date`-comparison law | #7, #9, #10, #15 |
| [`intake-domains/`](intake-domains/README.md) | Three intake tables composed by one rule, spanning domains | #7, #9, #10, #15 |
| [`auth-role-table/`](auth-role-table/README.md) | Record-valued fields, role update, `return` of the new role | #5, #7, #9, #10, #15 |
| [`todo-shard/`](todo-shard/README.md) | `Int[0..3]` shard field, per-shard law | #7, #8, #9, #10, #15 |
| [`auth-owner-check/`](auth-owner-check/README.md) | `require t.owner == by or fail Forbidden;` guard shape | #7, #9, #10, #15 |
| [`todo-archive/`](todo-archive/README.md) | Status transition + stream append + `forall` law over the date | #7, #9, #10, #15 |
| [`auth-permission-check/`](auth-permission-check/README.md) | Two `or fail` guards after the lookup; declared failures reachable | #7, #9, #10, #15 |

Per-program READMEs list, against each of the seven issues, whether that
program's syntax rests on that resolution (`YES`, with the citing line) or does
not use it (`NONE`). `corpus/semantics/` holds the interpreter cases; the ones
that name a program in this directory are listed in
[`../README.md`](../README.md), which also records why a todo-domain case is
deferred and an auth-domain case is not. A program's case is registered by naming
it in the `CASES` list of
`packages/tessera-interp/tests/semantics_schema.rs`, the single place the count
lives.

## Corpus invariant: a declared failure must be reachable

Every `example <rule>(...) fails <Name>;` MUST name a failure its rule can
actually produce: the rule body carries a matching `... or fail <Name>`.

`new t: T where ...;` only creates a value and never reads a table
(`docs/spec/v0.1.md:640`), so a body that is one `new` whose fields are all bound
by parameters and literals has no failure site and cannot satisfy `fails <Name>`
on the empty store. Where reading a table is the point, bind the lookup first —
`let x = <table>[<key>] or fail NotFound;` (`docs/spec/v0.1.md:638`) — then use
`x` in the `new` constraints.

The order matters as much as the presence: a guard that precedes the lookup
fails on the empty store before the lookup is reached. A rule that runs
`require users[by].permission == … or fail Forbidden;` above
`let t = tasks[id] or fail NotFound;` can never produce the `NotFound` its own
example declares, so the table reads come first and the guards that depend on
them follow.

`packages/tessera-interp/tests/corpus_programs.rs` enforces this invariant
together with the one-`.tes`-plus-one-README layout, the anchor rows and the
construct sets above, so none of them is review-enforced only. The parser (WP A)
replaces that text-level reading with a semantic one.
