# todo-archive

`archive.tes` — the status-transition + stream-append composition over already-
anchored resolutions: a `status` transition rule, the `output ledger += e;` append
shape, a `forall` law, and a three-fixture example. `created` is a `Date`-typed
field (docs/spec/v0.1.md:104 — v0.1 has no `Instant` literal; `Instant` comes only
from `clock.now`), so the law's `t.created < 2026-10-22` compares `Date` to `Date`.
No fresh construct appears; the #9 anchor gives the law's parenthesized precedence.

## Issue anchors

| Issue | Anchor (resolution link) | Used? |
|---|---|---|
| #5 keyword list | [#5 comment](https://github.com/developerz-ai/tessera/issues/5#issuecomment-6571502968)    | NONE |
| #6 keywords without productions | [#6 comment](https://github.com/developerz-ai/tessera/issues/6#issuecomment-6571732818)    | NONE |
| #7 statement separator | [#7 comment](https://github.com/developerz-ai/tessera/issues/7#issuecomment-6571620854) | YES — `or fail` paths |
| #8 G4-fragment productions | [#8 comment](https://github.com/developerz-ai/tessera/issues/8#issuecomment-6571764343)    | NONE |
| #9 parameterized capability grammar | [#9 comment](https://github.com/developerz-ai/tessera/issues/9#issuecomment-6571798479)    | YES |
| #10 literal collisions / Date-literal typing | [#10 comment](https://github.com/developerz-ai/tessera/issues/10#issuecomment-6571629208) | YES — `opened_on: Date` fixture, the on-comparison, Date-typed `created` field |
| #15 example fixture origin | [#15 comment](https://github.com/developerz-ai/tessera/issues/15#issuecomment-6572277019) | YES — three-fixture example args |
