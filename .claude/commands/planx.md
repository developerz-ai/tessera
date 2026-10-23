---
description: Write a concise, self-contained multi-file plan to docs/plans/<YYYY>/<MM>/<DD>/<1NN>-<slug>/ for another agent to execute
argument-hint: [the feature or change to plan]
allowed-tools: Write, Read, Glob, Grep, Agent, Bash
---

# /planx

Plan only. No edits outside the plan dir. The executor needs nothing but the plan and the files it cites.

## Goal
$ARGUMENTS

## Steps

1. **Resolve path.** `date +%Y`, `date +%m`, `date +%d`. `Glob docs/plans/<YYYY>/<MM>/<DD>/1*` → highest `1NN-*` + 1, else `101`. Slug kebab-case, ≤5 words.
2. **Explore (read-only).** `Agent` (Explore, very thorough): the spec sections that govern the change (`docs/spec/v0.1.md` §N), the owning crate and tier (`docs/architecture/crates.md`), files to touch (`file:line`), existing tests and corpus cases, frozen interfaces crossed, open questions it hits (`docs/spec/open-questions.md`). An unresolved open question in the path → the plan's first slice is `/spec-issue`, not code. **No worktrees.**
3. **Write multiple files**, never one `plan.md`:
   - `overview.md` — Goal (1–2 sentences) · Context (spec §§, crates, reference patterns at `file:line`) · Plan files in execution order, one-line hook each · Done when (verifiable: commands, corpus cases) · Risks / open questions.
   - `<NN>-<aspect>.md` per slice — `> Part of overview.md. Depends on: <NN|none>` · Files to change · Steps · Tests (corpus cases first, then unit; the exact `cargo test -p …`) · Done when.
   - **One slice = one crate where possible** — that's the file-set lock `/feature` uses, and the PR.
4. **`status.yml`:**
   ```yaml
   plan: <1NN>-<slug>
   title: <title>
   status: not_started   # not_started | in_progress | blocked | complete | superseded
   created_by: <git config user.name>
   worked_by: ""
   owner: <git config user.name>
   percent: 0
   current_focus: ""
   slices:
     - { file: 01-<aspect>.md, status: not_started, percent: 0 }
   evidence: []
   last_updated: <YYYY-MM-DD>
   ```

## Rules
- Fragments over sentences. Reference `file:line`, never paste code. No checkboxes — `status.yml` is the only tracker.
- Respect `CLAUDE.md` (tiers, frozen interfaces, determinism); don't restate it.
- Tests and corpus before implementation in every slice.

## Output
```
✓ docs/plans/<YYYY>/<MM>/<DD>/<1NN>-<slug>/overview.md + 01-…, 02-…, status.yml
Next: /feature docs/plans/<…>/overview.md
```
