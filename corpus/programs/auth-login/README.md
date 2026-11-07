# auth-login

`login.tes` — an auth `login` rule with `or fail NotFound` on both the table
lookup and the password check. Uses the #15 fixture-origin rule twice: a `Text`
fixture (`bob_password`, `wrong_password`) alongside `Id<T>` fixtures, so an
`example` password check (`login(bob_password, wrong_password)`) exercises both
anchor paths in one rule.

The table lookup does not invent a `find(forall p in users. …)` call — the rule
body reads a fixture-declared `Id<User>` from `users` (`users[bob] or fail
NotFound;`) — and the `new s: Session where s.user == u.id, s.token ==
u.password, s.opened == clock.now;` constraints determine every field `Session`
declares (`user`, `token`, `opened`), leaving no derived field undetermined.

## Issue anchors

| Issue | Anchor (resolution link) | Used? |
|---|---|---|
| #5 keyword list | [#5 comment](https://github.com/developerz-ai/tessera/issues/5#issuecomment-6571502968) | NONE |
| #6 keywords without productions | [#6 comment](https://github.com/developerz-ai/tessera/issues/6#issuecomment-6571732818) | NONE |
| #7 statement separator | [#7 comment](https://github.com/developerz-ai/tessera/issues/7#issuecomment-6571620854) | NONE |
| #8 G4-fragment productions | [#8 comment](https://github.com/developerz-ai/tessera/issues/8#issuecomment-6571764343) | NONE |
| #9 parameterized capability grammar | [#9 comment](https://github.com/developerz-ai/tessera/issues/9#issuecomment-6571798479) | NONE |
| #10 literal collisions / id-literal typing | [#10 comment](https://github.com/developerz-ai/tessera/issues/10#issuecomment-6571629208) | NONE |
| #15 example fixture origin | [#15 comment](https://github.com/developerz-ai/tessera/issues/15#issuecomment-6572277019) | YES — fixture-only example args |
