# Crate map

One crate per work package (spec §13): **each agent owns one crate.** Dependencies point **down** a tier only — never sideways, never up. Enforced by `cargo xtask boundaries`; the executable table is [`xtask/src/tiers.rs`](../../xtask/src/tiers.rs) and must match the table in [`CLAUDE.md`](../../CLAUDE.md#crate-tiers).

```text
                     tessera (cli)                          tier 6
                          │
                  tessera-surface (F)                       tier 5
                          │
   tessera-store (E) · tessera-wasm (H) · tessera-opt (I) · tessera-dev (K)     tier 4
                          │
   tessera-interp (C) · tessera-verify (D) · tessera-lower (G) · tessera-pkg (J) tier 3
                          │
                  tessera-check (B)                         tier 2
                          │
                  tessera-syntax (A)                        tier 1
                          │
            tessera-diag · tessera-ir                       tier 0
```

| Crate | WP | Owns | Frozen interface | Depends on |
|---|---|---|---|---|
| `tessera-diag` | shared | diagnostic codes, spans, counterexamples, text + JSON rendering | diagnostics JSON | — |
| `tessera-ir` | shared | Core IR data types and JSON schema | Core IR schema | — |
| `tessera-syntax` | A | lexer, parser, AST, `tessera fmt` | AST JSON schema | diag |
| `tessera-check` | B | names, types, refinements, exhaustiveness, AST → Core IR | Core IR schema (produces) | diag, ir, syntax |
| `tessera-interp` | C | the semantic oracle: tables, atomic commit, input log, replay | semantics test suite | diag, ir |
| `tessera-verify` | D | SMT encoding, counterexamples, fast/full modes | diagnostics JSON | diag, ir |
| `tessera-lower` | G | Dataflow IR, Low IR, C emission | Low IR schema | diag, ir |
| `tessera-pkg` | J | hash normalization, store, lockfile, git resolver, audit | hash normalization spec | diag, ir, syntax |
| `tessera-store` | E | memory + SQLite stores, schema, checked migrations | Store trait | diag, ir, interp |
| `tessera-wasm` | H | Wasm backend, web runtime | Wasm ABI | diag, ir, lower |
| `tessera-opt` | I | fusion, e-graphs (`egg`), index selection | rewrite rule format | diag, ir, lower |
| `tessera-dev` | K | REPL, `why`, LSP, `fill` | diagnostics JSON | diag, ir, interp, verify |
| `tessera-surface` | F | cli + http surfaces | Surface trait | diag, ir, interp, store |
| `tessera` | — | the binary: argument parsing, dispatch | CLI contract (spec §11) | everything |

## Why Core IR sits at tier 0

The interpreter, verifier, lowering and hashing read **only** Core IR (spec §9). Putting the IR types below the checker means C, D, G and J never see the AST — a backend cannot accidentally depend on surface syntax, and the "one input" rule is a compile error rather than a convention.

## Adding a crate

1. Pick its tier first. If it doesn't fit, the design is wrong — fix the design, don't widen the table.
2. Add it to `xtask/src/tiers.rs`, the `CLAUDE.md` tier table, this file, and `[workspace.dependencies]`.
3. `bin/check` — `boundaries` refuses any mismatch.
