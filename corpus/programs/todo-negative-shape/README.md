# todo-negative-shape

`boundary.tes` — the *negative* side of the #10 literal-typing boundary: the law
keeps `Id<Task>` and `Id<User>` id domains disjoint inside one rule, and no
law or rule treats them as interchangeable. The disjointness comparison is
`t.owner == u.id` — the `Id<User>` projection of the quantified `User` row — not
`t.owner == u`: comparing `Id<User>` against a `Task` row is an ordinary nominal
type mismatch (docs/spec/v0.1.md:524), not a domain-disjointness edge. The mirror
of `todo-ascribe-boundary` (the ascription side); the fixture block uses the same
`#1`/`#12` id shapes as that program.

## Issue anchors

| Issue | Anchor (resolution link) | Used? |
|---|---|---|
| #5 keyword list | [#5 comment](https://github.com/developerz-ai/tessera/issues/5#issuecomment-6571502968)    | NONE |
| #6 keywords without productions | [#6 comment](https://github.com/developerz-ai/tessera/issues/6#issuecomment-6571732818)    | NONE |
| #7 statement separator | [#7 comment](https://github.com/developerz-ai/tessera/issues/7#issuecomment-6571620854) | NONE |
| #8 G4-fragment productions | [#8 comment](https://github.com/developerz-ai/tessera/issues/8#issuecomment-6571764343)    | NONE |
| #9 parameterized capability grammar | [#9 comment](https://github.com/developerz-ai/tessera/issues/9#issuecomment-6571798479)    | NONE |
| #10 literal collisions / id-literal typing (disjoint domain side) | [#10 comment](https://github.com/developerz-ai/tessera/issues/10#issuecomment-6571629208) | YES |
| #15 example fixture origin | [#15 comment](https://github.com/developerz-ai/tessera/issues/15#issuecomment-6572277019) | YES — fixture-only example args |
