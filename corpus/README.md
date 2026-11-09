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

## Semantics cases: which programs they reach, and the gap that is deferred

A semantics case is scoped to one module by name, so a case names a program:
[`semantics/auth-login-success.json`](semantics/auth-login-success.json) and
[`semantics/auth-login-failure.json`](semantics/auth-login-failure.json) are the two
outcomes of
[`programs/auth-login/login.tes`](programs/auth-login/login.tes) — so the case's
`initial_state` seeds the `users` row that a program's `example` can never see, since
an `example` runs against a fresh store. The success case takes the
`require u.password == attempt or fail NotFound;` guard and commits the
`new s: Session` row to the second table with the logged `clock.now`; the failure case
trips that same guard, so `NotFound` leaves the state unchanged and writes no row.

**Deferred: a case for a `todo`-domain program.** Every program in the todo domain
carries `status: Status` where `type Status = Open | Done(at: Instant);`, and the
provisional schema defines no encoding for a sum-type value:
`$defs/value` (`packages/tessera-interp/schema/semantics-case.schema.json:80`) admits
an integer, number, boolean, string, id literal, record and array — a closed list —
and `$defs/recordValue` (`:92`) keeps the `type`-tagged shape reserved for an id
literal, so a `Done(at)` value can be written as neither a record nor anything else. A
todo case would therefore have to invent a variant encoding, which the corpus forbids
(`No invented syntax`, above) and CLAUDE.md's "The spec is not guessed" forbids.

So this is not a missing file; it is a question the frozen schema does not answer.
Encoding a tagged union in `$defs/value` is a change to a frozen M0 interface, which
spec §13 requires to arrive as a versioned proposal with a green differential test —
not as a corpus edit here. The follow-up is a `spec:ambiguity` issue,
`spec(§3): how does a semantics case encode a sum-type value such as Done(at)?`
(`/spec-issue`); this corpus takes no reading until it lands, so the todo-domain case
pair arrives with that decision. Auth-domain cases need no such decision: every field
of `User` and `Session` is a `Text`, an `Instant` or an `Id<T>`, all of which the
schema already represents.
