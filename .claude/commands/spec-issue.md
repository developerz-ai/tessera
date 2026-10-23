---
description: File a spec ambiguity or decision as a GitHub issue instead of guessing
argument-hint: <the ambiguity, with the spec section>
allowed-tools: Read, Grep, Bash, Edit
---

Swarm rule (spec §13): **spec ambiguity is filed as an issue to the spec owner, never guessed.**

## Ambiguity
$ARGUMENTS

1. Confirm it is real: quote the spec lines (`docs/spec/v0.1.md` §N) that conflict or are silent. Check `docs/spec/open-questions.md` and `gh issue list --label spec:ambiguity --state all` — if it's already filed, link that issue and stop.
2. File it:
   ```bash
   gh issue create --title "spec(§N): <one-line question>" \
     --label "spec:ambiguity" --milestone "M0 Spec and corpus" \
     --body "<quoted spec lines> · <the readings, each with a 2-line example> · <what blocks on it>"
   ```
   A product decision rather than a gap → label `spec:decision`.
3. Add a row to `docs/spec/open-questions.md` with the issue's ID.
4. In the code you were writing: take no reading. Leave the case unimplemented behind a diagnostic naming the issue (`E-…`, "blocked on #N"), or stop the slice and report.
