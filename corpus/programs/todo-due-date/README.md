# todo-due-date

`due-date.tes` — a Date literal (`2026-10-22`) appearing inside `law` expressions
combining `||` and parenthesized `forall` bodies, per the #10 Date-literal lexer
table entry. The literal is inlined (`t.created == 2026-10-22` in the `new`
constraint chain, `t.due.value >= 2026-10-22` in the law) because §4 limits
fixture uses to `example` declarations and fixture initializers — the `created`
fixture appears only as an example argument. The `new` constraint list pins
`t.id == <the created task's Id>` beside `t.owner == by`, `t.title == title`,
`t.due == due`, `t.status is Open` and `t.created == 2026-10-22`, so every field
`Task` declares (`id`, `owner`, `title`, `status`, `created`, `due`) is uniquely
determined, as `docs/spec/v0.1.md:254` requires — `title` included, bound by the
rule's own `title: Text` parameter and fed by the `task_title` fixture, which the
example consumes. That is the same all-fields-determined shape
[`auth-login/`](../auth-login/README.md) gives `Session`. No feature here depends
on the still-open question of Date arithmetic
(`docs/spec/open-questions.md`).

## Issue anchors

| Issue | Anchor (resolution link) | Used? |
|---|---|---|
| #5 keyword list | [#5 comment](https://github.com/developerz-ai/tessera/issues/5#issuecomment-6571502968) | NONE |
| #6 keywords without productions | [#6 comment](https://github.com/developerz-ai/tessera/issues/6#issuecomment-6571732818) | NONE |
| #7 untruncated statement boundaries | [#7 comment](https://github.com/developerz-ai/tessera/issues/7#issuecomment-6571620854) | NONE |
| #8 G4-fragment productions | [#8 comment](https://github.com/developerz-ai/tessera/issues/8#issuecomment-6571764343) | NONE |
| #9 parameterized capability grammar | [#9 comment](https://github.com/developerz-ai/tessera/issues/9#issuecomment-6571798479) | NONE |
| #10 literal collisions / Date literal precedence | [#10 comment](https://github.com/developerz-ai/tessera/issues/10#issuecomment-6571629208) | YES — `2026-10-22` Date literal lexed before comparison operators, inlined in the law and `new` chain |
| #15 example fixture origin | [#15 comment](https://github.com/developerz-ai/tessera/issues/15#issuecomment-6572277019) | YES — fixture-only example args; fixtures stay out of law and rule bodies |
