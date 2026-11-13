# M0 program index

The M0 corpus in one page: the 20 example programs, the semantics case that drives each one, and the spec readings the corpus rests on.

A program is `corpus/programs/<dir>/<dir>.tes`; its module name is the directory with `-` written `_` (`todo-basic` → `todo_basic`), and its case is `corpus/semantics/<dir>.json`.

## Programs

| Program | Module | Case | Construct |
|---|---|---|---|
| [auth-login](auth-login/auth-login.tes) | `auth_login` | [auth-login.json](../semantics/auth-login.json) | two tables, a cross-table write and a token refresh |
| [auth-owner-check](auth-owner-check/auth-owner-check.tes) | `auth_owner_check` | [auth-owner-check.json](../semantics/auth-owner-check.json) | an ownership guard: only the owner may update a record |
| [auth-session-expiry](auth-session-expiry/auth-session-expiry.tes) | `auth_session_expiry` | [auth-session-expiry.json](../semantics/auth-session-expiry.json) | an expiry guard: a session past its expiry day is refused |
| [todo-assume-law](todo-assume-law/todo-assume-law.tes) | `todo_assume_law` | [todo-assume-law.json](../semantics/todo-assume-law.json) | admission marker: `assume` immediately before the semicolon of a law declaration |
| [todo-basic](todo-basic/todo-basic.tes) | `todo_basic` | [todo-basic.json](../semantics/todo-basic.json) | baseline lookup, `or fail`, count/total law |
| [todo-bools](todo-bools/todo-bools.tes) | `todo_bools` | [todo-bools.json](../semantics/todo-bools.json) | Bool column and Bool guard |
| [todo-counter](todo-counter/todo-counter.tes) | `todo_counter` | [todo-counter.json](../semantics/todo-counter.json) | increment assignment: `place += expr` |
| [todo-dates](todo-dates/todo-dates.tes) | `todo_dates` | [todo-dates.json](../semantics/todo-dates.json) | Date column and Date literal |
| [todo-decrement](todo-decrement/todo-decrement.tes) | `todo_decrement` | [todo-decrement.json](../semantics/todo-decrement.json) | decrement assignment: `place -= expr` |
| [todo-guard](todo-guard/todo-guard.tes) | `todo_guard` | [todo-guard.json](../semantics/todo-guard.json) | two named failures in one rule, in source order |
| [todo-ids](todo-ids/todo-ids.tes) | `todo_ids` | [todo-ids.json](../semantics/todo-ids.json) | ascribed and bare id literals |
| [todo-keyword-names](todo-keyword-names/todo-keyword-names.tes) | `todo_keyword_names` | [todo-keyword-names.json](../semantics/todo-keyword-names.json) | contextual words as field names |
| [todo-literals](todo-literals/todo-literals.tes) | `todo_literals` | [todo-literals.json](../semantics/todo-literals.json) | one fixture per literal form |
| [todo-needs-clock](todo-needs-clock/todo-needs-clock.tes) | `todo_needs_clock` | [todo-needs-clock.json](../semantics/todo-needs-clock.json) | needs binding at module level and a rule that requires it |
| [todo-output](todo-output/todo-output.tes) | `todo_output` | [todo-output.json](../semantics/todo-output.json) | declared output stream and an append statement |
| [todo-prove-bounds](todo-prove-bounds/todo-prove-bounds.tes) | `todo_prove_bounds` | [todo-prove-bounds.json](../semantics/todo-prove-bounds.json) | cost goal and a declared bound: a prove declaration with a limit goal and a bound list |
| [todo-prove-idempotent](todo-prove-idempotent/todo-prove-idempotent.tes) | `todo_prove_idempotent` | [todo-prove-idempotent.json](../semantics/todo-prove-idempotent.json) | idempotence goal: a prove declaration carrying the idempotent marker |
| [todo-quantifiers](todo-quantifiers/todo-quantifiers.tes) | `todo_quantifiers` | [todo-quantifiers.json](../semantics/todo-quantifiers.json) | law bodies: a conjunction and an implication over one quantifier |
| [todo-then-observation](todo-then-observation/todo-then-observation.tes) | `todo_then_observation` | [todo-then-observation.json](../semantics/todo-then-observation.json) | example observation: a `then` observation is carried, the outcome is the failure |
| [todo-unless](todo-unless/todo-unless.tes) | `todo_unless` | [todo-unless.json](../semantics/todo-unless.json) | `unless` suffix on an assignment statement |

## Spec readings the corpus adopts

The seven spec issues below are closed and their resolutions are merged in the spec; the corpus records the reading it adopted instead of reopening them. Each reading is stated as a comment on its issue, so a maintainer can veto it there and withdraw the programs that rest on it. Every comment body is the *adopted reading* of its row with its *spec anchor*, followed by

> The M0 corpus adopts this reading; veto here to withdraw the matching programs.

and the disclosure line

> 🤖 developerz-ai, an automated agent, not a human — sebastian-1 (developerz-ai[bot])

The seven bodies are reproduced verbatim [below](#the-seven-comment-bodies-verbatim).

| Issue | Title | Adopted reading | Spec anchor | Reading comment | Programs that depend on it |
|---|---|---|---|---|---|
| [#5](https://github.com/developerz-ai/tessera/issues/5) | spec(§2): words used as syntax but missing from the keyword list | `return`/`scope` reserved; `in`/`or`/`key`/`index`/`bound`/`as`/`fails` contextual; `key` and `index` usable as field names | [v0.1.md:65-102](../../docs/spec/v0.1.md#L65) | [comment](https://github.com/developerz-ai/tessera/issues/5#issuecomment-PENDING-5) | all 20 (`in`, `or`, `key` and `fails` in their keyword slots); `todo-keyword-names` also uses `key` and `index` as field names |
| [#6](https://github.com/developerz-ai/tessera/issues/6) | spec(§2): keywords with no grammar production | every keyword classified; `assume` is the trailing admission marker on type/law/prove; `hint` reserved and unsupported | [v0.1.md:102, 374-398](../../docs/spec/v0.1.md#L102) | [comment](https://github.com/developerz-ai/tessera/issues/6#issuecomment-PENDING-6) | `todo-assume-law` (`assume`); `hint` and the other reserved-unsupported words are used by none |
| [#7](https://github.com/developerz-ai/tessera/issues/7) | spec(§2): statement separator when whitespace is insignificant | every simple statement ends with an explicit `;`; newline, `}` and EOF never supply it; `unless`/`or` bind to the preceding expression | [v0.1.md:61, 173](../../docs/spec/v0.1.md#L61) | [comment](https://github.com/developerz-ai/tessera/issues/7#issuecomment-PENDING-7) | all 20 (every declaration and statement is terminated); `todo-unless` also uses the `unless` suffix |
| [#8](https://github.com/developerz-ai/tessera/issues/8) | spec(§2): missing productions for store, surface, app, use, formula, constraints, pat, place, bounds | G4 delivers `pat`, `place`, `bounds` and `formula`, which the corpus uses; `store`/`surface`/`app`/`use` stay out of corpus programs | [v0.1.md:214-262](../../docs/spec/v0.1.md#L214) | [comment](https://github.com/developerz-ai/tessera/issues/8#issuecomment-PENDING-8) | all 20 (`pat` in every `let`, `place` in every assignment, `formula` in every law); `todo-prove-bounds` also uses `bounds`; `constraints` is used by none (`new … where` waits on D3) |
| [#9](https://github.com/developerz-ai/tessera/issues/9) | spec(§2, §6): `needs` grammar can't express parameterized capabilities | `needs` is a binding plus a capability reference; a rule header selects bindings; parameterized-capability semantics stay undefined, so only bare references are used | [v0.1.md:200-206, 718-726](../../docs/spec/v0.1.md#L200) | [comment](https://github.com/developerz-ai/tessera/issues/9#issuecomment-PENDING-9) | `todo-needs-clock` (module-level `needs` binding and the rule-header `needs` clause) |
| [#10](https://github.com/developerz-ai/tessera/issues/10) | spec(§2): literal collisions and typing of id literals | longest-match lexer with date/duration/id candidates; an id literal needs a unique expected `Id<T>` or an explicit ascription | [v0.1.md:103-151, 505-524](../../docs/spec/v0.1.md#L103) | [comment](https://github.com/developerz-ai/tessera/issues/10#issuecomment-PENDING-10) | all 20 (an id literal in every fixture); `todo-ids` also uses a bare `#7`; `todo-dates` uses a date literal, `todo-literals` a date and a duration literal |
| [#15](https://github.com/developerz-ai/tessera/issues/15) | spec(§4): where do example fixtures like `bob` and `alice_task` come from? | every example argument is a module-local fixture whose value is a closed literal or another fixture | [v0.1.md:619-634](../../docs/spec/v0.1.md#L619) | [comment](https://github.com/developerz-ai/tessera/issues/15#issuecomment-PENDING-15) | all 20 (every example argument is a fixture of the same module) |

### The seven comment bodies, verbatim

One comment per issue: the paragraph headed `#N —` is the body of the comment for issue `#N`, and the disclosure line under it belongs to the same comment.

```text
#5 — `return`/`scope` reserved; `in`/`or`/`key`/`index`/`bound`/`as`/`fails` contextual; `key` and `index` usable as field names (spec anchor v0.1.md:65-102). The M0 corpus adopts this reading; veto here to withdraw the matching programs.
🤖 developerz-ai, an automated agent, not a human — sebastian-1 (developerz-ai[bot])

#6 — every keyword classified; `assume` is the trailing admission marker on type/law/prove; `hint` reserved and unsupported (spec anchor v0.1.md:102, 374-398). The M0 corpus adopts this reading; veto here to withdraw the matching programs.
🤖 developerz-ai, an automated agent, not a human — sebastian-1 (developerz-ai[bot])

#7 — every simple statement ends with an explicit `;`; newline, `}` and EOF never supply it; `unless`/`or` bind to the preceding expression (spec anchor v0.1.md:61, 173). The M0 corpus adopts this reading; veto here to withdraw the matching programs.
🤖 developerz-ai, an automated agent, not a human — sebastian-1 (developerz-ai[bot])

#8 — G4 delivers `pat`, `place`, `bounds` and `formula`, which the corpus uses; `store`/`surface`/`app`/`use` stay out of corpus programs (spec anchor v0.1.md:214-262). The M0 corpus adopts this reading; veto here to withdraw the matching programs.
🤖 developerz-ai, an automated agent, not a human — sebastian-1 (developerz-ai[bot])

#9 — `needs` is a binding plus a capability reference; a rule header selects bindings; parameterized-capability semantics stay undefined, so only bare references are used (spec anchor v0.1.md:200-206, 718-726). The M0 corpus adopts this reading; veto here to withdraw the matching programs.
🤖 developerz-ai, an automated agent, not a human — sebastian-1 (developerz-ai[bot])

#10 — longest-match lexer with date/duration/id candidates; an id literal needs a unique expected `Id<T>` or an explicit ascription (spec anchor v0.1.md:103-151, 505-524). The M0 corpus adopts this reading; veto here to withdraw the matching programs.
🤖 developerz-ai, an automated agent, not a human — sebastian-1 (developerz-ai[bot])

#15 — every example argument is a module-local fixture whose value is a closed literal or another fixture (spec anchor v0.1.md:619-634). The M0 corpus adopts this reading; veto here to withdraw the matching programs.
🤖 developerz-ai, an automated agent, not a human — sebastian-1 (developerz-ai[bot])
```

`PENDING-N` in the *reading comment* column is the placeholder for the comment id of issue `N`. The seven comments are posted by the runner — this checkout has no GitHub credential and `gh` is refused in its shell — and each id replaces `PENDING-N` in place, giving the permalink `https://github.com/developerz-ai/tessera/issues/N#issuecomment-<id>`. Every other cell is final.

## Invariants, exclusions, veto

Every program here is one `.tes` file whose first line is `module <name> @1;`, with `<name>` the directory name written in underscores; it declares at least one `law` and at least one `example`, every argument of an `example` call is the name of a `fixture` declared in the same module, and every example expects a failure the named rule can reach — on the fresh empty store the lookup misses, so `fails NotFound` holds (spec §4:632). Every declaration and statement ends with an explicit `;`; no program relies on a newline, `}` or EOF as a terminator (spec §2:61).

Each program has exactly one semantics case, `corpus/semantics/<dir>.json`: ids are tagged objects, dates are `YYYY-MM-DD`, instants are RFC 3339, no row carries a null or an undeclared field, and every seeded row and every post-state row satisfies that program's laws, so no case can abort with a law violation. The three older cases in `corpus/semantics` (`failed-call.json`, `nondeterministic-inputs.json`, `successful-commit.json`) run against the module `todo`; they are interpreter fixtures, not program cases.

Excluded from the corpus: `store`, `surface`, `app` and `use` (wiring — §7 stores, surfaces and the app manifest; `use` also pulls in package resolution and its lockfile, and D1, SQLite versus Postgres, is open), `view` and `comptime` (a case observes rule calls, state and outputs only, so a view's value or a compile-time generated declaration has nothing in the case to pin it down), type refinements and `new … where` (D3 open), a variant-typed state column (the frozen case format has no variant value), and `schedule`/`hint`/`foreign`/`plugin`/`never`/`old`/`exists`/`match` (reserved and unsupported in v0.1, [v0.1.md:98](../../docs/spec/v0.1.md#L98)); a `Duration` appears as a program fixture but never as a case row.

A maintainer who vetoes a reading in an issue thread withdraws the programs listed in that row's *Programs that depend on it* column from the corpus.
