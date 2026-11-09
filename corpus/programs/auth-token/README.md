# auth-token

`token.tes` — keys on the #5 keyword-list resolution: `return` is used as a
reserved keyword at statement position in a rule body, and the rule's writes go
through `state`+`output` in the fixed slots. The rule `issue` reads its own
parameter, not a fixture: the table lookup uses the anchored `users[who] or fail
NotFound;` shape (docs/spec/v0.1.md:540) keyed on the `who: Id<User>` parameter,
and the write goes to `users[u.id].token` via the found row — the declared
fixtures (`bob`, `token_value`) appear only as the `example`'s arguments
(`issue(bob, token_value)`), per #15's fixture-only-example rule. A second law
uses `in` (`u.name in {...}`) to pin #9's `in`-precedence reading twice over.

## Issue anchors

| Issue | Used? |
|---|---|
| #5 keyword list / `return` reserved | YES — `return` at statement position (`:18`) |
| #6 keywords without productions | NONE |
| #7 statement separator | YES — `;` terminates every statement, including the `or fail` forms at :15 |
| #8 G4-fragment productions | NONE |
| #9 parameterized capability grammar | YES — `needs` clause (`:9`) |
| #10 literal collisions / id-literal typing | YES — ascribed id literal :11 |
| #15 example fixture origin | YES — fixture-only example args (`bob, token_value` at :25) |
