# todo-locked

`locked.tes` — exercises the #7 untruncated statement-boundary resolution: the
rule body mixes `require ... or fail Name;`, `let x = e or fail Name;`, an
`unless` assignment, `return e;`, and a parenthesized law. Statement separators
("`.`" per #7) end each statement instead of whitespace, so multi-line rule
bodies parse unambiguously.

## Issue anchors

| Issue | Anchor (resolution link) | Used? |
|---|---|---|
| #5 keyword list | [#5 comment](https://github.com/developerz-ai/tessera/issues/5#issuecomment-6571502968) | NONE |
| #6 keywords without productions | [#6 comment](https://github.com/developerz-ai/tessera/issues/6#issuecomment-6571732818) | NONE |
| #7 statement separator | [#7 comment](https://github.com/developerz-ai/tessera/issues/7#issuecomment-6571620854) | YES |
| #8 G4-fragment productions | [#8 comment](https://github.com/developerz-ai/tessera/issues/8#issuecomment-6571764343) | NONE |
| #9 parameterized capability grammar | [#9 comment](https://github.com/developerz-ai/tessera/issues/9#issuecomment-6571798479) | NONE |
| #10 literal collisions / id-literal typing | [#10 comment](https://github.com/developerz-ai/tessera/issues/10#issuecomment-6571629208) | NONE |
| #15 example fixture origin | [#15 comment](https://github.com/developerz-ai/tessera/issues/15#issuecomment-6572277019) | YES — fixture-only example args |
