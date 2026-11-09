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

| Issue | Used? |
|---|---|
| #5 keyword list | NONE |
| #6 keywords without productions | NONE |
| #7 statement separator | YES — `;` terminates every statement, including the `or fail` forms at :17, :18 |
| #8 G4-fragment productions | NONE |
| #9 parameterized capability grammar | YES — `needs` clause (`:11`) |
| #10 literal collisions / id-literal typing | YES — ascribed id literal :13, :14 |
| #15 example fixture origin | YES — fixture-only example args (`bob, alice_task` at :25) |
