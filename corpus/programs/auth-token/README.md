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

| Issue | Anchor (resolution link) | Used? |
|---|---|---|
| #5 keyword list / `return` reserved | [#5 comment](https://github.com/developerz-ai/tessera/issues/5#issuecomment-6571502968) | YES |
| #6 keywords without productions | [#6 comment](https://github.com/developerz-ai/tessera/issues/6#issuecomment-6571732818) | NONE |
| #7 statement separator | [#7 comment](https://github.com/developerz-ai/tessera/issues/7#issuecomment-6571620854) | NONE |
| #8 G4-fragment productions | [#8 comment](https://github.com/developerz-ai/tessera/issues/8#issuecomment-6571764343) | NONE |
| #9 parameterized capability grammar (`in` precedence) | [#9 comment](https://github.com/developerz-ai/tessera/issues/9#issuecomment-6571798479) | YES |
| #10 literal collisions / id-literal typing | [#10 comment](https://github.com/developerz-ai/tessera/issues/10#issuecomment-6571629208) | NONE |
| #15 example fixture origin | [#15 comment](https://github.com/developerz-ai/tessera/issues/15#issuecomment-6572277019) | YES — fixture-only example args; rule body reads the `who` parameter, not a fixture |
