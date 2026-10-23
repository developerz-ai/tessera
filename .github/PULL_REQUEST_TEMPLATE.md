## What & why

<!-- One line: what a Tessera user can do after this. Link the issue: Closes #N -->

## Spec

<!-- Which sections govern this (e.g. §4, §5.2)? Any reading the spec doesn't settle → filed as an issue, not decided here. -->

## Checklist

- [ ] Corpus case or test written first, and it fails without this change
- [ ] `bin/check` green locally
- [ ] No frozen interface changed (AST, Core IR, diagnostics JSON, semantics format, Low IR, hash normalization) — or a versioned proposal is linked
- [ ] Deterministic: no hash-ordered iteration into output, no clock/env/randomness in the core
- [ ] New diagnostics have stable codes and a `--json` form
- [ ] Docs/spec/open-questions updated if this made any of them wrong
- [ ] Parts written by an agent are noted; reviewed by someone/something other than the author
