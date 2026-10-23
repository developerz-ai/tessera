---
description: Idea or plan → merged PR. Understand, distrust the paperwork, explore, slice by crate, build test-first, gate, review by a different agent, PR, merge.
argument-hint: <the feature, or a docs/plans/…/overview.md path>
allowed-tools: Read, Write, Edit, Glob, Grep, Bash, Agent, SendMessage, TaskCreate, TaskUpdate, TaskList, Skill, WebFetch
---

# /feature

Read `CLAUDE.md` first. **Done means merged with `bin/check` green on CI — nothing less.** Report what you verified, not what you assume.

## Request
$ARGUMENTS

**Read autonomy from the prompt.** "Just ship it" → run end to end, decide, surface decisions in the PR body. Tentative → clarify the genuinely ambiguous, stop before merge. Always stop for a spec ambiguity (→ `/spec-issue`), a frozen-interface change without a proposal, or anything that weakens soundness (fast mode reporting a proof, Unknown treated as success).

## Flow

1. **Understand.** One line: what a user of Tessera can do after this. Name the spec §§ that govern it.
2. **Distrust the paperwork.** Check any plan's `status.yml`, `open-questions.md`, and docs against `git log` and the code. State which claims you falsified.
3. **Explore (parallel, read-only).** Explore agents over disjoint crates: what exists (`file:line`), corpus cases that cover it, frozen interfaces it crosses, tier constraints. Protect your context — don't read what an agent will report.
4. **Track.** `gh issue list` first. One issue per slice on the right milestone; `Closes #N` in the PR.
5. **Branch, then slice.** `git fetch origin && git status --short` (expect clean) → `git checkout -b <type>/<slug>`. **One crate per agent**, disjoint paths. Shared files (`Cargo.toml` workspace deps, `CLAUDE.md`, `docs/README.md`, `xtask/src/tiers.rs`) are edited **last, by you**.
6. **Build test-first.** Corpus case / failing test → code → green. Every brief carries: exclusive file set · who else holds which paths · evidence first, hypothesis labelled unverified · `CLAUDE.md` non-negotiables · checks scoped to its own crate (`cargo test -p <crate>`, `cargo clippy -p <crate> -- -D warnings`) · **no git operations** · never "ask me" — decide-and-flag or stop-and-report.
7. **Gate.** `bin/check` once, repo-wide, by you. Red → `/verify`.
8. **Review by a different agent.** The builder never reviews its own work (spec §13). Spawn the relevant reviewer from `.claude/agents/` (`spec-guardian`, `architecture-reviewer`, `determinism-auditor`, `test-author` for mutation checks) on the diff. Fix or rebut every finding.
9. **PR + merge.** Sweep scratch files. `git add <paths>` (never `-A`), read `git status --short`, commit (conventional, scoped), push, `gh pr create` (Summary · spec §§ · tests added · decisions made). Wait for checks to register **and** pass — 0 checks is not green. `gh pr merge --squash`. One PR in flight at a time.
10. **Leave the trail straight.** Update `status.yml`, delete resolved rows in `open-questions.md`, fix any doc the change made wrong — same PR.

## Hard rules
No git worktrees (no `isolation: worktree`, no `git worktree add`). Never `git stash`. Never `--force`. Only you spawn subagents; they never delegate further.

## Output
```
Shipped:   <crates / files>
Tests:     <corpus cases + unit tests added>
Gate:      bin/check <green/red>   CI <green/red>
Review:    <agent> — <n findings, n fixed, n rebutted>
Deferred:  <n> — <what, why>        [never omit]
Falsified: <paperwork claims that were wrong>
PR:        #NNN <merged/open>
```
