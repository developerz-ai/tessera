# Corpus

Ground truth, written **before** implementation (spec §13). M0 gate: 20 example programs plus the semantics cases, signed off by a human.

| Path | Holds | Consumed by |
|---|---|---|
| `programs/<name>/*.tes` | complete example programs with `law` and `example` declarations | parser round-trip (A), type checker (B), interpreter (C), parity (G, H) |
| `negative/<code>/*.tes` | programs that MUST fail with diagnostic `<code>` | WP B's 200 negative tests |
| `semantics/*.json` | semantics cases: initial state, call sequence, expected state and outputs | interpreter (C), every backend |
| `seeded-bugs/*.tes` | rules with a known law violation | verifier (D): must be refuted, never proven |

Rules:

- A program here is only added once its syntax is unambiguous — open grammar gaps are in [docs/spec/open-questions.md](../docs/spec/open-questions.md). Don't invent syntax to get an example to parse.
- Files are in `tessera fmt` canonical form once WP A lands; until then, match the spec's examples.
- The semantics case format is a frozen interface (fourth in the freeze order); propose changes as a versioned PR.
