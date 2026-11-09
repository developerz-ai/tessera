# auth-session-expiry

`session-expiry.tes` — a clock-driven law: `expire` compares `s.expires < clock.today`
following #10's Date-literal reading, and the law `still_open_means_live` asserts
Date-boundary correctness over `sessions` (`forall s in sessions`) at the
post-state (§5). The rule body `new s: Session where s.id == sid, s.user == u.id,
s.opened == opened, s.expires == expires;` determines every field `Session`
declares, because the session's identity is also constrained (`s.id == sid`,
matching the `sid` rule parameter) per §5's requirement that `new` constraints
uniquely determine all fields. Both `example` calls use only declared fixtures
(§4, #15).

## Issue anchors

| Issue | Used? |
|---|---|
| #5 keyword list | NONE |
| #6 keywords without productions | NONE |
| #7 statement separator | YES — `;` terminates every statement, including the `or fail` forms at :19, :25, :26 |
| #8 G4-fragment productions | NONE |
| #9 parameterized capability grammar | YES — `needs` clause (`:11`) |
| #10 literal collisions / Date literal typing | YES — ascribed id literal :13, :14; `Date` literal :15, :16 |
| #15 example fixture origin | YES — fixture-only example args (`bob, alice_task, opened_at, expires_at` at :35, :37) |
