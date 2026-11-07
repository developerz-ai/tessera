# todo-precedence

`precedence.tes` — pins the #9 expression-precedence reading: `forall t in tasks`
mixes `||`, `=>`, `in` ({...} membership), `&&` conjunction and `or fail`-independent
`or` in one expression, every law hand-checked against a fresh-empty-store state.
The `guarded_owner` guard is part of the predicate — written as an `&&` conjunct
beside the positive test, because the claim grammar (§2.1: a claim is a quantifier
and a predicate) has no `with (...)` production — and it sits to the right of the
positive test so the §5 ordered-conjunction rule carries the fact inward.

## Issue anchors

| Issue | Anchor (resolution link) | Used? |
|---|---|---|
| #5 keyword list | [#5 comment](https://github.com/developerz-ai/tessera/issues/5#issuecomment-6571502968) | NONE |
| #6 keywords without productions | [#6 comment](https://github.com/developerz-ai/tessera/issues/6#issuecomment-6571732818) | NONE |
| #7 statement separator | [#7 comment](https://github.com/developerz-ai/tessera/issues/7#issuecomment-6571620854) | NONE |
| #8 G4-fragment productions | [#8 comment](https://github.com/developerz-ai/tessera/issues/8#issuecomment-6571764343) | NONE |
| #9 parameterized capability grammar (`in` precedence) | [#9 comment](https://github.com/developerz-ai/tessera/issues/9#issuecomment-6571798479) | YES |
| #10 literal collisions / id-literal typing | [#10 comment](https://github.com/developerz-ai/tessera/issues/10#issuecomment-6571629208) | NONE |
| #15 example fixture origin | [#15 comment](https://github.com/developerz-ai/tessera/issues/15#issuecomment-6572277019) | YES — fixture-only example args |
