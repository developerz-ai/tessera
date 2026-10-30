# Reviewer instructions — developerz-ai/tessera

> Read by the developerz.ai reviewer at EVERY review of this repository, at the reviewed
> commit, as this repo's own layer of the reviewer's prompt — established by dz-runner setup mode.
> It refines the platform rules (a finding is cited or withheld; the bot always discloses)
> and never overrides them. Edit freely; keep it under 12,000 characters — past that it is cut.

## Stack

Rust + Cargo (1.98.0, rustup). The gate is `bin/check` (`cargo fmt --all -- --check && cargo clippy --workspace --all-targets --locked -- -D warnings && cargo test --workspace --locked --quiet && cargo xtask all`): a change that does not pass it is not proven, whatever its description says.

## Review conventions
- Reject any `unwrap`/`expect`/`panic!`/`todo!`/`unimplemented!` outside tests — clippy denies them workspace-wide.
- Reject `unsafe` — `unsafe_code = "forbid"` in `[workspace.lints.rust]`.
- Reject nondeterminism: `HashMap`/`HashSet` iteration reaching output, IR, hashes or diagnostics (require `BTreeMap`/`BTreeSet` or a sort); wall clock, randomness or env reads in the compiler core; fast-math floats.
- Require a stable diagnostic code `E-<AREA>-NNN` for every user-facing failure, rendered identically as text and `--json`.
- Require every CLI command and diagnostic to support `--json`.
- Reject a crate depending on a same-or-higher tier (table in `xtask/src/tiers.rs`, map in `docs/architecture/crates.md`); a new crate must register in `xtask/src/tiers.rs`, the CLAUDE.md tier table, `docs/architecture/crates.md` and `[workspace.dependencies]`.
- Reject behavior added without a corpus case or a test that fails without the change — corpus/test first, then code.
- Reject silent interpretations of an ambiguous spec; the fix is an issue labelled `spec:ambiguity`, not code.
- Reject changes to frozen interfaces (AST schema, Core IR schema, diagnostics JSON, semantics format, Low IR schema, hash normalization) without a versioned proposal PR.
- Require backends to match the interpreter bit for bit; a PR that changes the interpreter to match a backend is backwards — flag it.
- Reject hand-written solvers or rewrites of the approved reuse list (`chumsky`/`logos`, Z3/cvc5, `egg`, `rusqlite`, `wasm-encoder`, `blake3`, `tower-lsp`); new third-party deps belong in `[workspace.dependencies]` with a stated reason.
- Require conventional commits scoped by crate (`feat(syntax): …`, scope = crate without the `tessera-` prefix), branch `<type>/<slug>`, one concern per PR with `Closes #N`.

## Gates that matter
- `bin/check` is the only merge gate and CI runs it verbatim: `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked`, `cargo xtask all` (tier boundaries, doc links, CLAUDE.md ≤ 16 KiB). A proven PR means this command green, stated in the PR.
- A red `xtask boundaries` means a tier violation — that alone blocks merge.
- From M4: interpreter-vs-backend parity on every corpus example is also a merge gate.

## Never flag
- Formatting — `bin/fmt` / rustfmt owns it (`rustfmt.toml`: edition 2024, max_width 100, field init shorthand); never ask for manual format changes.
- `unwrap`/`expect`/`panic` inside `#[cfg(test)]` or `crates/*/tests/` — explicitly allowed by `clippy.toml`.
- `module_name_repetitions` — allowed workspace-wide.
- Planned CLI commands answering `E-CLI-002` with their milestone — intended scaffold behavior, not a bug.
- The `AGENTS.md` symlink to `CLAUDE.md` and the `tmp/` directory `bin/setup` creates.
