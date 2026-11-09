# todo-backlog

`backlog.tes` — a `require` chain over a guarded `Maybe` payload: the stale-due
check is the §3.1 ordered conjunction `(t.due is Some) && (t.due.value < on)`, so
the payload projection is evaluated only after the positive test that grants it
(docs/spec/v0.1.md:546-547). Anchors: `or fail` (#7), Date literal and id-literal
typing (#10), fixture-only example args (#15), `return` as reserved keyword (#5)
— and #9 precedence across `=>`, `in` and `with` in one `forall` law. The only
`&&` in the module is the rule's stale-due `require` at `:21`, not a law. Neither
`named_bob` nor `archived_only_when_some` conjoins with `&&`: they compose `=>`,
`in` and the `with (...)` guard.

This module does not use the #6 `where`-chain or the #8 partitioned-store shape:
there is no `new ... where` rule and no shard discriminator here — those two
anchors are exercised by `corpus/programs/auth-segments/` (`segments.tes`), the
README's cross-reference for both. Proves the five resolutions used here
compose rather than conflict.

## Issue anchors

| Issue | Anchor (resolution link) | Used? |
|---|---|---|
| #5 keyword list / `return` reserved | [#5 comment](https://github.com/developerz-ai/tessera/issues/5#issuecomment-6571502968) | YES — `return` at statement position (`:24`) |
| #6 keywords without productions (constraints) | [#6 comment](https://github.com/developerz-ai/tessera/issues/6#issuecomment-6571732818) | NONE — the `where`-chain anchor is exercised by `auth-segments/segments.tes`, not here |
| #7 statement separator | [#7 comment](https://github.com/developerz-ai/tessera/issues/7#issuecomment-6571620854) | YES — `;` terminates every statement, including the `or fail` forms at :19, :20, :21 |
| #8 G4-fragment productions | [#8 comment](https://github.com/developerz-ai/tessera/issues/8#issuecomment-6571764343) | NONE — the partitioned-store anchor is exercised by `auth-segments/segments.tes`, not here |
| #9 parameterized capability grammar | [#9 comment](https://github.com/developerz-ai/tessera/issues/9#issuecomment-6571798479) | YES — `needs` clause (`:12`) |
| #10 literal collisions / id-literal typing | [#10 comment](https://github.com/developerz-ai/tessera/issues/10#issuecomment-6571629208) | YES — ascribed id literal :14, :15; `Date` literal :16 |
| #15 example fixture origin | [#15 comment](https://github.com/developerz-ai/tessera/issues/15#issuecomment-6572277019) | YES |
