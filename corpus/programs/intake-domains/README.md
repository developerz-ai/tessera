# intake-domains

`intake.tes` — a multi-domain composition over already-recorded anchors: `state`
tables across three types, `new t: Task where ...` with a `forall`-law check, the
`output ledger += e;` append inside the spanning rule against the declared
stream (`output ledger: Stream<Entry>;` — the referenced stream is the
declared `ledger`, not an undeclared name), a `new` constraint list that
asserts each field directly (`t.status is Open` — a Boolean constraint
expression per the `new t: T where` production, `docs/spec/v0.1.md:254`, using
the direct positive tag test `p is V` that §3.1 defines for a variant field),
and fixture-only example arguments.

The module declares the fixtures the example references only: the single
`example intake(raw_one, task_one) fails NotFound;` call passes exactly the two
arguments `intake(rid: Id<Raw>, tid: Id<Task>)` declares, both fixture-only per
#15. No `bob` or `received_on` fixture exists: an example argument is bound to
the declared rule parameter by position, so an `Id<User>` or a `Date` argument
could not type-check against `rid: Id<Raw>` / `tid: Id<Task>`, and §4 keeps
every fixture referenced by the example that uses it. The #10 anchor appears
here in its id-literal form — the ascribed `#3 : Id<Raw>` and `#12 : Id<Task>`
fixtures — while the Date-literal form of the same anchor lives in
[`todo-due-date/`](../todo-due-date/README.md), whose example passes the Date
fixture it declares.

## Issue anchors

| Issue | Anchor (resolution link) | Used? |
|---|---|---|
| #5 keyword list | [#5 comment](https://github.com/developerz-ai/tessera/issues/5#issuecomment-6571502968) | NONE |
| #6 keywords without productions | [#6 comment](https://github.com/developerz-ai/tessera/issues/6#issuecomment-6571732818)    | NONE |
| #7 statement separator | [#7 comment](https://github.com/developerz-ai/tessera/issues/7#issuecomment-6571620854) | YES — `;` terminates every statement, including the `or fail` forms at :20 |
| #8 G4-fragment productions | [#8 comment](https://github.com/developerz-ai/tessera/issues/8#issuecomment-6571764343)    | NONE |
| #9 parameterized capability grammar | [#9 comment](https://github.com/developerz-ai/tessera/issues/9#issuecomment-6571798479) | YES — `needs` clause (`:14`) |
| #10 literal collisions / Date literal typing | [#10 comment](https://github.com/developerz-ai/tessera/issues/10#issuecomment-6571629208) | YES — ascribed id literal :16, :17 |
| #15 example fixture origin | [#15 comment](https://github.com/developerz-ai/tessera/issues/15#issuecomment-6572277019) | YES — fixture-only example args across rules |
