# CLAUDE.md

Guidance for Claude Code (and any agent — `AGENTS.md` is a symlink to this file) working in this repository.

## Tessera

A declarative, deterministic language for the logic layer of applications: you declare **types, rules and laws**; the compiler derives the implementation and **proves the laws or returns a counterexample**. Humans review types, laws and examples; machines write rule bodies. This repo is the toolchain: one Cargo workspace, one crate per work package, the `tessera` binary. **Status: pre-M0 — spec and scaffold only.** The spec is [`docs/spec/v0.1.md`](docs/spec/v0.1.md); known gaps are [`docs/spec/open-questions.md`](docs/spec/open-questions.md); `tessera help` lists every command with the milestone that ships it.

## Non-negotiables

| Rule | Detail |
|---|---|
| No `unwrap`/`expect`/`panic!`/`todo!` outside tests | Clippy denies them. A panic in the compiler is a crash with no diagnostic. |
| Typed errors only | An enum per crate; every user-facing failure maps to a stable diagnostic code (`E-<AREA>-NNN`). Codes are stable forever once shipped. |
| `--json` everywhere | On every command and every diagnostic. |
| No `unsafe` | `unsafe_code = "forbid"` workspace-wide. |
| Deterministic toolchain | No `HashMap`/`HashSet` iteration in anything that reaches output, IR, hashes or diagnostics — use `BTreeMap`/`BTreeSet` or sort. No wall clock, no randomness, no env reads in the compiler core. Floats: IEEE-754, no fast-math, canonical NaN. |
| Reuse, don't rewrite (spec §13) | `chumsky`/`logos`, Z3/cvc5, `egg`, `rusqlite`, `wasm-encoder`, `blake3`, `tower-lsp`. Never write a solver. New dependency → `[workspace.dependencies]` + a reason in the PR. |
| SRP | One file, one job. Target <300 LOC, hard ceiling ~500. |
| Tests next to code | `#[cfg(test)]` for units; `crates/<c>/tests/` for crate-level; `corpus/` for language behavior. A test that can't fail isn't a test. |

## Design axioms (override any instinct that conflicts)

From spec §1. Every change is judged against these.

| Axiom | Rule |
|---|---|
| Determinism (P1, P8) | Same state + inputs → same outputs on every backend, bit for bit. The interpreter is the oracle; a backend that disagrees is wrong. |
| Three concepts (P2) | Type, rule, law. A feature that needs a fourth concept is a spec change, not a PR. |
| No ambient effects (P3) | Time, randomness, IO, secrets are declared `needs` and logged inputs. |
| Errors are counterexamples (P5) | Every diagnostic: stable code, exact span, minimal counterexample when one exists, at most one fix, human + `--json`. |
| Local reasoning (P6) | No global mutable state in the toolchain either — no `static mut`, no `lazy_static` caches that change results. |
| Hashes are truth (P7) | Identity is the BLAKE3 hash of normalized Core IR. Names and versions are labels. |
| Unknown is not success | Solver timeout → build fails unless an explicit `assume`/`hint`. Never report a false proof. |
| The spec is not guessed | Ambiguity → issue labelled `spec:ambiguity` (`/spec-issue`), never a silent interpretation in code. |

## Response rules

Execute — no preamble, no restating the task; lead with the action or answer. Parallel tool calls when independent; read before speculating. Disagree when the user — or the spec — is wrong; state the correction. Terse: code, commands and paths stay exact; final summary 1–2 sentences.

## Commands

Run from the repo root.

| Task | Command |
|---|---|
| fresh clone → ready | `bin/setup` (idempotent) |
| **the gate** (≡ CI) | `bin/check` — fmt check, clippy `-D warnings`, tests, `cargo xtask all`. Green = mergeable |
| format | `bin/fmt` |
| build | `cargo build` |
| test | `cargo test --workspace` · one crate: `cargo test -p tessera-syntax` · one name: `cargo test -p tessera-syntax -- <name>` |
| lint | `cargo clippy --workspace --all-targets -- -D warnings` |
| guards | `cargo xtask boundaries` · `cargo xtask links` · `cargo xtask claude-md-size` · `cargo xtask all` |
| run the CLI | `cargo run -q -p tessera -- <args>` (e.g. `-- help`, `-- check --json`) |

Toolchain pinned in `rust-toolchain.toml` (Rust 1.98.0, edition 2024). `bin/check` is what CI runs, verbatim — a check that only lives in CI is a check agents can't run.

## Layout

```
crates/tessera-*/   one crate per work package (spec §13); see docs/architecture/crates.md
crates/tessera/     the `tessera` binary
xtask/              repo guards (`cargo xtask …`) — tiers, links, CLAUDE.md size
corpus/             example programs, negative tests, semantics cases — written BEFORE code
docs/spec/          the language spec (normative) + amendments + open questions
docs/architecture/  how the toolchain is built
docs/plans/         /planx output
site/               GitHub Pages landing page (deployed by .github/workflows/pages.yml)
bin/                setup, check, fmt
llms.txt            machine-readable repo map
```

## Crate tiers

A crate may depend only on strictly lower tiers. Enforced by `cargo xtask boundaries`; [`xtask/src/tiers.rs`](xtask/src/tiers.rs) is the executable copy and must agree with this table. Full map: [`docs/architecture/crates.md`](docs/architecture/crates.md). A new crate picks its tier first; if it fits no tier, fix the design — don't widen the table.

| Tier | Crates |
|---|---|
| 0 | `tessera-diag`, `tessera-ir` |
| 1 | `tessera-syntax` |
| 2 | `tessera-check` |
| 3 | `tessera-interp`, `tessera-verify`, `tessera-lower`, `tessera-pkg` |
| 4 | `tessera-store`, `tessera-wasm`, `tessera-opt`, `tessera-dev` |
| 5 | `tessera-surface` |
| 6 | `tessera` |

## Frozen interfaces

Freeze in this order (spec §13): **AST schema → Core IR schema → diagnostics JSON → semantics test-suite format → Low IR schema → hash normalization.** Once frozen, a change needs a versioned proposal PR and a green differential-test run. Schemas live next to the crate that owns them.

## Coding rules

| Rule | Detail |
|---|---|
| Think first | State assumptions. Two readings of the spec → file `/spec-issue`, don't pick one. |
| Simplicity | Minimum code for the stated problem. No speculative abstraction, no unrequested config. |
| Surgical | Every changed line traces to the task. No drive-by reformatting outside `bin/fmt`. |
| Goal-driven | Corpus case or test first, then code until it passes. Done = a command proves it. |
| Proactive | Fix bugs you see in files you're editing; ship the guard with the fix in the same PR. Report what you verified, not what you assume. |

## Preventive rules

These bite before any symptom appears.

| Rule | Detail |
|---|---|
| Parity is the merge gate (from M4) | Nothing merges unless interpreter vs backend parity is green on every example. |
| The interpreter defines semantics | When a backend and the interpreter disagree, fix the backend — or file a spec issue. Never change the interpreter to match a backend. |
| Fast mode may miss, never lie | A fast-mode shortcut that can turn Unknown into Proven is a soundness bug. |
| Hash the normalized IR, never the text | Formatting or renaming must not change a hash. |
| The designer doesn't review | An agent that built a feature does not review it — use a reviewer agent in `.claude/agents/`. |
| One checkout, one team | No stashes, no worktrees — work directly in this checkout with disjoint file sets. Only the top-level agent spawns subagents; a subagent that finds its scope too large reports back instead of delegating. |

## PR conventions

| Rule | Detail |
|---|---|
| Branches | `<type>/<slug>` (`feat/lexer-literals`, `spec/statement-separator`). Never commit to `main`. |
| Commits | Conventional: `feat(syntax): …`, `fix(interp): …`, `spec: …`, `docs: …`, `chore: …`. Scope = crate without the `tessera-` prefix. |
| One concern per PR | `Closes #N`; `bin/check` green before push; cap ~110 changed files. |
| One PR in flight at a time per area | Merge, fetch, then the next. |

## Where things live

| Need | Go to |
|---|---|
| what the language means | [`docs/spec/v0.1.md`](docs/spec/v0.1.md) |
| packages, lockfile, release | [`docs/spec/packages.md`](docs/spec/packages.md) |
| what is undecided | [`docs/spec/open-questions.md`](docs/spec/open-questions.md) |
| which crate owns X | [`docs/architecture/crates.md`](docs/architecture/crates.md) |
| corpus layout | [`corpus/README.md`](corpus/README.md) |
| milestones and gates | spec §13 · GitHub milestones M0–M6 |
| how to contribute | [`CONTRIBUTING.md`](CONTRIBUTING.md) |

## What NOT to build (v0.1 non-goals)

Raw pointers, custom allocators, hard real-time, pixel-level UI, general scripting, JIT, shared libraries, a custom machine-code backend, a hand-written SMT solver, a central package registry (rejected — [`docs/spec/packages.md`](docs/spec/packages.md)).
