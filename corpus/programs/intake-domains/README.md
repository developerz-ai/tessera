# intake-domains

`intake.tes` — a multi-domain composition over already-recorded anchors: `state`
tables across three types, `new t: Task where ...` with a `forall`-law check, the
`output ledger += e;` append inside the spanning rule against the declared
stream (`output ledger: Stream<Entry>;` — the referenced stream is the
declared `ledger`, not an undeclared name), a `new` constraint that asserts
the variant directly (`t.status == Open`, an equality on the declared variant
rather than an `is`-form query, per §5's requirement that `new` constraints be
equalities), and fixture-only example
arguments across several rules.

## Issue anchors

| Issue | Anchor (resolution link) | Used? |
|---|---|---|
| #5 keyword list | [#5 comment](https://github.com/developerz-ai/tessera/issues/5#issuecomment-6571502968)    | NONE |
| #6 keywords without productions | [#6 comment](https://github.com/developerz-ai/tessera/issues/6#issuecomment-6571732818)    | NONE |
| #7 statement separator | [#7 comment](https://github.com/developerz-ai/tessera/issues/7#issuecomment-6571620854) | YES — `or fail` on the lookup |
| #8 G4-fragment productions | [#8 comment](https://github.com/developerz-ai/tessera/issues/8#issuecomment-6571764343)    | NONE |
| #9 parameterized capability grammar | [#9 comment](https://github.com/developerz-ai/tessera/issues/9#issuecomment-6571798479)    | NONE |
| #10 literal collisions / Date literal typing | [#10 comment](https://github.com/developerz-ai/tessera/issues/10#issuecomment-6571629208) | YES — `fixture received_on: Date = 2026-10-22;` |
| #15 example fixture origin | [#15 comment](https://github.com/developerz-ai/tessera/issues/15#issuecomment-6572277019) | YES — fixture-only example args across rules |
