# Contributing to Tessera

Tessera is pre-M0: the spec exists, the code is a scaffold. That shapes what helps most.

## What helps right now

| Contribution | Where | Notes |
|---|---|---|
| Settle a spec question | [issues labelled `spec:ambiguity` / `spec:decision`](https://github.com/developerz-ai/tessera/issues?q=is%3Aopen+label%3Aspec%3Aambiguity%2Cspec%3Adecision) · [open-questions.md](docs/spec/open-questions.md) | propose a reading with 2–3 example programs; the spec owner decides |
| Example programs | `corpus/programs/` | complete modules with laws and examples; only once their syntax is unambiguous |
| Semantics cases | `corpus/semantics/` | initial state → calls → expected state; format freezes in M0 |
| Negative tests | `corpus/negative/<code>/` | programs that must fail with a given diagnostic code |
| Report a spec bug | [new issue](https://github.com/developerz-ai/tessera/issues/new/choose) | quote the section |

## Ground rules

- **Spec ambiguity is filed, never guessed.** If the spec doesn't say, open an issue — don't encode your reading in code or corpus.
- **Tests and corpus before implementation.** A PR adding behavior without a case that would fail without it is incomplete.
- **Determinism is not negotiable.** Same input, same bytes out, everywhere. No hash-ordered iteration into output, no clock, no env in the compiler core.
- **The interpreter is the oracle.** Backends match it bit for bit.
- **Frozen interfaces** (AST, Core IR, diagnostics JSON, semantics format, Low IR, hash normalization) change only through a versioned proposal PR.

## Workflow

```sh
bin/setup                  # once
git checkout -b feat/<slug>
# … corpus case / test first, then code …
bin/check                  # must be green before you push — CI runs exactly this
```

- Conventional commits, scoped by crate: `feat(syntax): lex duration literals`.
- One concern per PR; link the issue (`Closes #N`).
- Every PR gets an automated review and a human review. The author of a feature does not review it.

Repository conventions in full — crate tiers, error codes, non-negotiables — are in [CLAUDE.md](CLAUDE.md). They apply to humans too.

## AI-written contributions

Welcome, and expected. Same bar as everything else: `bin/check` green, tests that fail when the code breaks, no invented spec semantics. Say in the PR which parts an agent wrote.

## License

By contributing you agree your work is dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at the user's option.

## Conduct

[Code of Conduct](CODE_OF_CONDUCT.md).
