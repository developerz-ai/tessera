# auth-login

`login.tes` — an auth `login` rule with `or fail NotFound` on both the table
lookup and the password check. Uses the #15 fixture-origin rule twice: an
`Id<User>` fixture (`bob`) for the rule's `who: Id<User>` parameter and a `Text`
fixture (`wrong_password`) for its `attempt: Text` parameter, so each `example`
(`login(bob, wrong_password)`, `login(missing_user, wrong_password)`) passes
args whose declared types match `login(who: Id<User>, attempt: Text)` and
exercises both anchor paths in one rule — and never literals, which #15 forbids
as example arguments. Every fixture declared (`bob`, `missing_user`,
`wrong_password`) is consumed by an example, and none is left unreferenced.

The table lookup uses the anchored `users[who] or fail NotFound;` shape (docs/spec/v0.1.md:540),
keyed on the rule's `who` parameter rather than a fixture (per #15,
no fixture enters a rule body). The token is drawn from the declared `Random`
capability — `rng.token(32)` (docs/spec/v0.1.md:693, seeded from the log so it
replays identically) — and never from the stored password: a session token that
reuses `u.password` would publish the credential as the bearer token in every
ground-truth case that records it. The `new s: Session where s.user == u.id,
s.token == rng.token(32), s.opened == clock.now;` constraints determine every
field `Session` declares (`user`, `token`, `opened`), leaving no derived field
undetermined. `clock` and `rng` are both declared in the rule's `needs` clause,
so the module reads no ambient effect (§1 P3).

`law session_belongs_to_a_user` states an invariant `login` actually preserves:
every session row names a user the `users` table holds. The rule reads
`users[who] or fail NotFound;` before it creates the row and constrains
`s.user == u.id`, so a committed `login` can only insert a session for a stored
user, and it writes no other table.

It deliberately does not state "one session per user": `sessions` is keyed by
`token` (`:7`) and v0.1 addresses a `Table<T>` row only through its declared
key, whose write forms are place assignments that commit at once
(`docs/spec/v0.1.md:471`, `docs/spec/v0.1.md:641`) — no statement of the rule
body enumerates or removes a row. So `login` cannot see, replace or remove the
caller's earlier session: a second `login` by `bob` leaves two rows, each with
the `rng.token(32)` it was issued (`docs/spec/v0.1.md:640`), and a law
`one_session_per_user` would be falsified by this rule on its own second call —
so the module states only what the rule preserves.

## Issue anchors

| Issue | Anchor (resolution link) | Used? |
|---|---|---|
| #5 keyword list | [#5 comment](https://github.com/developerz-ai/tessera/issues/5#issuecomment-6571502968) | YES — `return` at statement position (`:20`) |
| #6 keywords without productions | [#6 comment](https://github.com/developerz-ai/tessera/issues/6#issuecomment-6571732818) | NONE |
| #7 statement separator | [#7 comment](https://github.com/developerz-ai/tessera/issues/7#issuecomment-6571620854) | YES — `;` terminates every statement, including the `or fail` forms at :17, :18 |
| #8 G4-fragment productions | [#8 comment](https://github.com/developerz-ai/tessera/issues/8#issuecomment-6571764343) | NONE |
| #9 parameterized capability grammar | [#9 comment](https://github.com/developerz-ai/tessera/issues/9#issuecomment-6571798479) | YES — `needs` clause (`:9`) |
| #10 literal collisions / id-literal typing | [#10 comment](https://github.com/developerz-ai/tessera/issues/10#issuecomment-6571629208) | YES — ascribed id literal :12, :13 |
| #15 example fixture origin | [#15 comment](https://github.com/developerz-ai/tessera/issues/15#issuecomment-6572277019) | YES — fixture-only example args; `users[who] or fail NotFound` reads the `who` parameter |
