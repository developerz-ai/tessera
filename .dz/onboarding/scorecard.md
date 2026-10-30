# AI-first scorecard

> Written by the developerz.ai maintainer agent — established by dz-runner setup mode. Each onboarding run rewrites this file whole; the latest run wins, and the same card is stored on the platform.

- Run: `run_4bff362a800b4d1c98c039a0ca70d081`
- Computed: 2026-10-23T08:06:28.727Z
- Proof: **unprovable** — `bin/check` did not run green at head on this box. Nothing in the repository’s own code was changed to force it; the excerpt below is the record.
- Score: 9 of 15 items ok

## `policy` — ok

- .dz/maintainer/maintainer.yml established
- .dz/maintainer/reviewer.md established

## `brain` — ok

- CLAUDE.md already present, left alone

## `roster` — ok

- .claude/agents already present, left alone

## `skills` — ok

- .claude/skills/verify-change/SKILL.md established
- .claude/skills/crate-tiers/SKILL.md established
- .claude/skills/gate-run/SKILL.md established

## `scripts` — unprovable

- bin/setup already present, left alone
- bin/dev established
- bin/check already present, left alone
- bin/stop established
- down: ./bin/stop (bin)
- ./bin/setup at head: red, exit 1 (0.0s)

## `wiring` — ok

- .mcp.json established

## `commands` — ok

- .claude/commands already present, left alone

## `access` — gap

- asked for 0 credentials

## `layout` — gap

- migration to the house layout filed as tasks
- packages/tessera: 1 move(s) planned as one task
- packages/tessera-check: 1 move(s) planned as one task
- packages/tessera-dev: 1 move(s) planned as one task
- packages/tessera-diag: 1 move(s) planned as one task
- packages/tessera-interp: 1 move(s) planned as one task
- packages/tessera-ir: 1 move(s) planned as one task
- …and 8 more

## `smoke_test` — unprovable

- packages/tessera/tests/cli.rs in the repository
- ./bin/check at head: not_runnable, exit 127 (0.0s)

## `env_example` — gap

- nothing recorded

## `agents_md` — ok

- AGENTS.md already present, left alone

## `per_workspace_brains` — ok

- single-package repository: 0 workspaces
- CLAUDE.md already present, left alone

## `docs_readme` — ok

- docs/README.md already present, left alone

## `gate_proved` — unprovable

- ./bin/check at head: not_runnable, exit 127 (0.0s)
