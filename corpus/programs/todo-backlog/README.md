# todo-backlog

`backlog.tes` — every anchor in one program, one construct each: `or fail` (#7),
Date literal and id-literal typing (#10), fixture-only example args (#15), `return`
as reserved keyword (#5), `where` constraint chain at admission (#6), the G4
partitioned-store production (#8) — and #9 precedence across `||`, `=>`, `in`, and
`with` in one `forall` law. Guarded payload access in `archive` follows the §5
positive-fact ordering (`t.due is Some` then `t.due.value`), not a falsy `||` arm:
the stale-due check reads the payload only where it is already positively guarded,
per docs/spec/v0.1.md:546-547. Proves the seven resolutions compose rather than
conflict.

## Issue anchors

| Issue | Anchor (resolution link) | Used? |
|---|---|---|
| #5 keyword list / `return` reserved | [#5 comment](https://github.com/developerz-ai/tessera/issues/5#issuecomment-6571502968) | YES |
| #6 keywords without productions (constraints) | [#6 comment](https://github.com/developerz-ai/tessera/issues/6#issuecomment-6571732818) | YES |
| #7 statement separator | [#7 comment](https://github.com/developerz-ai/tessera/issues/7#issuecomment-6571620854) | YES |
| #8 G4-fragment productions | [#8 comment](https://github.com/developerz-ai/tessera/issues/8#issuecomment-6571764343) | YES |
| #9 parameterized capability grammar | [#9 comment](https://github.com/developerz-ai/tessera/issues/9#issuecomment-6571798479) | YES |
| #10 literal collisions / id-literal typing | [#10 comment](https://github.com/developerz-ai/tessera/issues/10#issuecomment-6571629208) | YES |
| #15 example fixture origin | [#15 comment](https://github.com/developerz-ai/tessera/issues/15#issuecomment-6572277019) | YES |
