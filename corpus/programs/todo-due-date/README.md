# todo-due-date

`due-date.tes` — a Date literal (`2026-10-22`) appearing inside `law` expressions
and as an inline term of the `new` constraint chain, per the #10 Date-literal lexer
table entry. The literal is inlined (`t.created == 2026-10-22` in the `new`
constraint chain, `t.due.value >= 2026-10-22` in the law) because §4 limits
fixture uses to `example` declarations and fixture initializers — the `created`
fixture appears only as an example argument. The `new` constraint list pins
`t.id == <the created task's Id>` beside `t.owner == owner.id`, `t.title == title`,
`t.due == due`, `t.status is Open` and `t.created == 2026-10-22`, so every field
`Task` declares (`id`, `owner`, `title`, `status`, `created`, `due`) is uniquely
determined, as `docs/spec/v0.1.md:254` requires — `title` included, bound by the
rule's own `title: Text` parameter and fed by the `task_title` fixture, which the
example consumes. That is the same all-fields-determined shape
[`auth-login/`](../auth-login/README.md) gives `Session`.

The rule opens with `let owner = users[by] or fail NotFound;` (§4,
`docs/spec/v0.1.md:638`), so `create` reads a table and owns a reachable
`NotFound`: on the fresh store both tables are empty, `users[bob]` misses, and
`example create(...) fails NotFound;` holds for exactly the reason the lookup
gives. Without that lookup the body would be a single `new` whose fields are all
bound by parameters and literals, and the declared failure could never occur.
`owner` is then projected (`t.owner == owner.id`) rather than the parameter read
directly, the same local-binding shape [`auth-login/`](../auth-login/login.tes)
uses for `s.user == u.id`.

No feature here depends on the still-open question of Date arithmetic
(`docs/spec/open-questions.md`).

## Issue anchors

| Issue | Used? |
|---|---|
| #5 keyword list | NONE |
| #6 keywords without productions | NONE |
| #7 untruncated statement boundaries | YES — `;` terminates every statement, including the `or fail` forms at :18 |
| #8 G4-fragment productions | NONE |
| #9 parameterized capability grammar | YES — `needs` clause (`:10`) |
| #10 literal collisions / Date literal precedence | YES — ascribed id literal :13, :14; `Date` literal :12, :19; the law's `Date` literal :22 |
| #15 example fixture origin | YES — fixture-only example args (`bob, alice_task, task_title, created` at :29) |
