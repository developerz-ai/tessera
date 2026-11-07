# todo-ascribe-boundary

`ascribe.tes` — pins the *ascription side* of the #10 literal-typing boundary:
`fixture alice_task: Id<Task> = #12 : Id<Task>;` is the id-literal-with-ascription
shape, and `fixture opened_on: Date = 2026-10-22;` is the Date-literal one. Per §4,
those fixtures are used only in the `example` declaration and its initializers;
the law and rule bodies inline the values (`t.id == #12 : Id<Task>` in the law,
`on >= 2026-10-22` in the rule body), so no fixture name crosses into a law or
rule body. The law compares the inline value to a table-typed row, so the boundary
through `Id<Task>` and `Date` is exercised together. The mirror edge is
`todo-negative-shape`.

## Issue anchors

| Issue | Anchor (resolution link) | Used? |
|---|---|---|
| #5 keyword list | [#5 comment](https://github.com/developerz-ai/tessera/issues/5#issuecomment-6571502968)    | NONE |
| #6 keywords without productions | [#6 comment](https://github.com/developerz-ai/tessera/issues/6#issuecomment-6571732818)    | NONE |
| #7 statement separator | [#7 comment](https://github.com/developerz-ai/tessera/issues/7#issuecomment-6571620854) | NONE |
| #8 G4-fragment productions | [#8 comment](https://github.com/developerz-ai/tessera/issues/8#issuecomment-6571764343)    | NONE |
| #9 parameterized capability grammar | [#9 comment](https://github.com/developerz-ai/tessera/issues/9#issuecomment-6571798479)    | NONE |
| #10 literal collisions / id-literal typing (ascription side) | [#10 comment](https://github.com/developerz-ai/tessera/issues/10#issuecomment-6571629208) | YES — inlined `#12 : Id<Task>` in the law, ascribed fixture in the example |
| #15 example fixture origin | [#15 comment](https://github.com/developerz-ai/tessera/issues/15#issuecomment-6572277019) | YES — fixture-only example args; law and rule bodies inline the values |
