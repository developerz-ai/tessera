# Corpus

Examples and tests for language behavior, kept separately from the toolchain crates.

| Directory | Content | Format |
|---|---|---|
| [`programs/`](programs/README.md) | Domain programs (todo, auth) — spec-conformant examples of §2–§7 syntax, written before any parser exists. One `.tes` + one README per program. | prose + `.tes` |
| `negative/<code>/*.tes` | Negative tests: one directory per diagnostic code (`negative/E-LIT-001/`, …), each program MUST fail with that `E-<AREA>-NNN` code and with no other. Lands with WP B ([`CONTRIBUTING.md`](../CONTRIBUTING.md)). | `.tes`, grouped by code |
| `seeded-bugs/*.tes` | Verifier soundness cases: each program carries a deliberately broken law and MUST be **refuted**, never proven (`.claude/agents/test-author.md`). Lands with the verifier. | `.tes` |
| [`semantics/`](../packages/tessera-interp/tests/semantics_schema.rs) | Interpreter semantics cases (success, failure, nondeterministic inputs), validated by `packages/tessera-interp` schema tests. Seeded state lives here and nowhere else. | `.json` per `packages/tessera-interp/schema/semantics-case.schema.json` |

## Rules every corpus file follows

- **Fixture-only example arguments.** Every argument of an `example` call is the
  name of a `fixture` declared in the same module — never a literal, never a
  computed expression (§4, `docs/spec/v0.1.md:630`, issue #15). A `Date`
  argument therefore needs a `fixture at: Date = 2026-10-22;`, not an inline
  `2026-10-22`.
- **Fresh-store semantics.** Every `example` runs against a fresh store: each
  declared `state` table is empty and a fixture is a value, not a row
  (`docs/spec/v0.1.md:632`). A declared `fails <Name>` therefore needs a
  reachable failure site in the rule body — a bare `new` whose fields are all
  parameters and literals can never fail, so the example binds its table read
  first: `let x = <table>[<key>] or fail NotFound;`.
- **No invented syntax.** A construct the spec does not define is a
  `spec:ambiguity` issue (CLAUDE.md, "The spec is not guessed"), never
  corpus-local syntax: don't invent a form to get an example to parse.
- **Canonical formatting.** A `.tes` file is written in the form `tessera fmt`
  emits — the only accepted canonical form (§2, `docs/spec/v0.1.md:152`) — and
  follows `.editorconfig` (two-space indent, LF, final newline, no trailing
  whitespace). `bin/fmt` covers Rust only, so it neither formats nor checks
  `.tes` files.
- **Seeded state lives in `semantics/`.** v0.1 defines no source form for
  seeding state; stateful scenarios are the `corpus/semantics/*.json` cases
  (`packages/tessera-interp/schema/semantics-case.contract.md:49`), and their
  format is unchanged.

Schedule: the schema (`packages/tessera-interp/schema/semantics-case.schema.json`)
is a frozen M0 interface; the programs (§2–§7) become parser and interpreter test
inputs when WP A/C land, the `negative/` suite when WP B lands.
