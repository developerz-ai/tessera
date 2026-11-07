# Corpus

Examples and tests for language behavior, kept separately from the toolchain crates.

| Directory | Content | Format |
|---|---|---|
| [`programs/`](programs/README.md) | Domain programs (todo, auth) — spec-conformant examples of §2–§7 syntax, written before any parser exists. One `.tes` + one README per program. | prose + `.tes` |
| [`semantics/`](../packages/tessera-interp/tests/semantics_schema.rs) | Interpreter semantics cases (success, failure, nondeterministic inputs), validated by `packages/tessera-interp` schema tests. | `.json` per `packages/tessera-interp/schema/semantics-case.schema.json` |

Schedule: the schema (`packages/tessera-interp/schema/semantics-case.schema.json`)
is a frozen M0 interface; the programs (§2–§7) become parser and interpreter test
inputs when WP A/C land.
