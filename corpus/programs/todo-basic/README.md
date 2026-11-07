# todo-basic

`basic.tes` — minimal todo program covering the §4 skeleton: `state` tables, a
`create` rule with `new t: T where ...`, a `complete` rule with `or fail` lookup and
snapshot `unless` write, a `forall t in tasks` law, an idempotence `prove`, and an
`example ... fails NotFound;` scenario (fresh store ⇒ both tables empty ⇒ lookup fails).

## Issue anchors

| Issue | Anchor (resolution link) | Used? |
|---|---|---|
| #5 keyword list / `return` reserved | [#5 comment](https://github.com/developerz-ai/tessera/issues/5#issuecomment-6571502968) | NONE |
| #6 keywords without productions | [#6 comment](https://github.com/developerz-ai/tessera/issues/6#issuecomment-6571732818) | NONE |
| #7 statement separator | [#7 comment](https://github.com/developerz-ai/tessera/issues/7#issuecomment-6571620854) | NONE |
| #8 G4-fragment productions | [#8 comment](https://github.com/developerz-ai/tessera/issues/8#issuecomment-6571764343) | NONE |
| #9 parameterized capability grammar | [#9 comment](https://github.com/developerz-ai/tessera/issues/9#issuecomment-6571798479) | NONE |
| #10 literal collisions / id-literal typing | [#10 comment](https://github.com/developerz-ai/tessera/issues/10#issuecomment-6571629208) | YES — `#1 : Id<User>` / `#12 : Id<Task>` ascribed id literals |
| #15 example fixture origin | [#15 comment](https://github.com/developerz-ai/tessera/issues/15#issuecomment-6572277019) | YES — `fixture bob: Id<User> = #1 : Id<User>;` and fixture-only example args |
