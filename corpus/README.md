# Corpus

Ground truth, written **before** implementation (spec §13). M0 gate: 20 example programs plus the semantics cases, signed off by a human.

| Path | Holds | Consumed by |
|---|---|---|
| `programs/<name>/*.tes` | complete example programs with `law` and `example` declarations; every example input is a `fixture` declaration (spec §4) whose value is a closed literal (spec §2–§3) or a reference to another fixture of the same module, and whose declared type matches the type of that value | parser round-trip (A), type checker (B), interpreter (C), parity (G, H) |
| `programs/README.md` | the M0 program index: the 20 programs, their semantics cases, and the spec readings the corpus adopts | maintainers (veto), reviewers |
| `negative/<code>/*.tes` | programs that MUST fail with diagnostic `<code>` | WP B's 200 negative tests |
| `semantics/*.json` | semantics cases: initial state, call sequence, expected state and outputs — the only place seeded state lives | interpreter (C), every backend |
| `seeded-bugs/*.tes` | rules with a known law violation | verifier (D): must be refuted, never proven |

The 20 example programs, their per-program semantics cases and the seven issue-resolution links live in [`programs/README.md`](programs/README.md).

Rules:

- A program here is only added once its syntax is unambiguous — open grammar gaps are in [docs/spec/open-questions.md](../docs/spec/open-questions.md). Don't invent syntax to get an example to parse.
- Files are in `tessera fmt` canonical form once WP A lands; until then, match the spec's examples.
- Example inputs: every value an `example` passes is declared first as a `fixture` (spec §4) — `fixture <name>: <type_expr> = <value_expr>;` — module-local, deterministic, and its value is either a closed literal (spec §2–§3) or a reference to another fixture of the same module; its declared type matches the type of that value. No capability use, no rule or view calls, no state reads, and no reference cycles between fixtures.
- Example state: each `example` runs against a fresh store — every declared `state` table empty. A fixture is a module-level value, not a store row: it is bound to its declared value. v0.1 has no source form for seeding state.
- Seeded-state scenarios live in `semantics/*.json` (initial state, call sequence, expected state). The semantics case format is unchanged — a frozen interface (fourth in the freeze order); propose changes as a versioned PR.
