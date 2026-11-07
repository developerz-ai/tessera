# auth-session-expiry

`session-expiry.tes` — a clock-driven law: `expire` compares `s.expires < clock.today`
following #10's Date-literal reading, and the law `still_open_means_live` asserts
Date-boundary correctness over `sessions` (`forall s in sessions`) at the
post-state (§5). The rule body `new s: Session where s.user == u.id,
s.opened == opened, s.expires == expires;` determines every field `Session`
declares, because the session's identity is also constrained (`s.id == sid`,
matching the `sid` rule parameter) per §5's requirement that `new` constraints
uniquely determine all fields. Both `example` calls use only declared fixtures
(§4, #15).

## Issue anchors

| Issue | Anchor (resolution link) | Used? |
|---|---|---|
| #5 keyword list | [#5 comment](https://github.com/developerz-ai/tessera/issues/5#issuecomment-6571502968)    | NONE |
| #6 keywords without productions | [#6 comment](https://github.com/developerz-ai/tessera/issues/6#issuecomment-6571732818)    | NONE |
| #7 statement separator | [#7 comment](https://github.com/developerz-ai/tessera/issues/7#issuecomment-6571620854) | NONE |
| #8 G4-fragment productions | [#8 comment](https://github.com/developerz-ai/tessera/issues/8#issuecomment-6571764343)    | NONE |
| #9 parameterized capability grammar | [#9 comment](https://github.com/developerz-ai/tessera/issues/9#issuecomment-6571798479)    | NONE |
| #10 literal collisions / Date literal typing | [#10 comment](https://github.com/developerz-ai/tessera/issues/10#issuecomment-6571629208) | YES — `2026-10-22` / `2026-11-22` |
| #15 example fixture origin | [#15 comment](https://github.com/developerz-ai/tessera/issues/15#issuecomment-6572277019) | YES — fixture-only example args |
