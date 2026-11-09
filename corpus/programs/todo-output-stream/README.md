# todo-output-stream

`stream.tes` — pins the §4 `output ledger: Stream<Entry>;` declaration, the
`output ledger += e;` rule-body append shape, and the §5 atomic-abort discard of
buffered stream elements (the rule also fails `NotFound` on lookup, so the buffered
`ledger += entry` is retained-but-discarded on abort). The appended value is an
`Entry`-typed value — a locally bound `Entry` value flowed into `+=` — not a
record literal: v0.1 has no record-literal expression, so the append form takes
the stream's element type only. The `or fail` paths here ride the #7 record, and
#9's capability text lists what the rule may use.

## Issue anchors

| Issue | Anchor (resolution link) | Used? |
|---|---|---|
| #5 keyword list | [#5 comment](https://github.com/developerz-ai/tessera/issues/5#issuecomment-6571502968) | NONE |
| #6 keywords without productions | [#6 comment](https://github.com/developerz-ai/tessera/issues/6#issuecomment-6571732818) | NONE |
| #7 statement separator | [#7 comment](https://github.com/developerz-ai/tessera/issues/7#issuecomment-6571620854) | YES — `;` terminates every statement, including the `or fail` forms at :17, :18 |
| #8 G4-fragment productions | [#8 comment](https://github.com/developerz-ai/tessera/issues/8#issuecomment-6571764343) | NONE |
| #9 parameterized capability grammar | [#9 comment](https://github.com/developerz-ai/tessera/issues/9#issuecomment-6571798479) | YES — `needs` clause (`:11`) |
| #10 literal collisions / id-literal typing | [#10 comment](https://github.com/developerz-ai/tessera/issues/10#issuecomment-6571629208) | YES — ascribed id literal :13, :14 |
| #15 example fixture origin | [#15 comment](https://github.com/developerz-ai/tessera/issues/15#issuecomment-6572277019) | YES — fixture-only example args |
