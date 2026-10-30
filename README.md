<h1 align="center">Tessera</h1>

<p align="center"><strong>Declare types, rules and laws. The compiler proves the laws, or hands you a counterexample.</strong></p>

<div align="center">

[![CI](https://github.com/developerz-ai/tessera/actions/workflows/ci.yml/badge.svg)](https://github.com/developerz-ai/tessera/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![Status: pre-M0](https://img.shields.io/badge/status-pre--M0%20%C2%B7%20spec-orange.svg)](docs/spec/v0.1.md)
[![Rust 1.98](https://img.shields.io/badge/rust-1.98-black.svg?logo=rust)](rust-toolchain.toml)

[Spec](docs/spec/v0.1.md) ·
[Packages](docs/spec/packages.md) ·
[Open questions](docs/spec/open-questions.md) ·
[Crate map](docs/architecture/crates.md) ·
[Website](https://developerz-ai.github.io/tessera/) ·
[llms.txt](llms.txt)

</div>

> **If you are a coding agent, start with [`CLAUDE.md`](CLAUDE.md)** (also at [`AGENTS.md`](AGENTS.md)) and [`llms.txt`](llms.txt).

## What it is

Tessera is a declarative, deterministic language for the **logic layer** of applications — the part where bugs cost money. You write three kinds of thing:

- **types** that are constraints, not just shapes (`Int[1..100]`, `Text matching email`, `Secret<Text>`)
- **rules** — atomic state transitions: preconditions, then declared changes
- **laws** — invariants that must hold after every rule

The compiler derives the implementation, runs an SMT solver over your laws, and either **proves** them or returns a **concrete failing state and call**. Same state and inputs give the same outputs, bit for bit, on every backend.

```tessera
rule complete(by: Principal, id: Id<Task>) {
  let t = tasks[id] or fail NotFound
  require t.owner == by.id or fail NotFound
  tasks[id].status = Done(clock.now) unless t.status is Done
}

law done_once: forall t in tasks. t.status is Done => t.status.at >= t.created
prove complete(by, id) is idempotent
example complete(bob, alice_task) fails NotFound
```

Drop the `require` line and an isolation law (only an owner changes their tasks) fails with a counterexample, as JSON an agent can act on:

```json
{ "code": "E-LAW-001", "law": "todo.isolation", "rule": "todo.complete",
  "counterexample": { "state": { "tasks": [ { "id": 1, "owner": "alice" } ] },
                      "call": "complete(by: bob, id: 1)" },
  "hint": "add `require t.owner == by.id or fail NotFound`" }
```

## Why

AI writes most new code now. Reviewing it line by line doesn't scale; **verifying it does**. Tessera moves human review to where text is short and errors are expensive — types, laws and examples — and lets machines write rule bodies that a solver checks.

| Principle | What it buys |
|---|---|
| Deterministic on every backend | a solver can verify AI-written code instead of trusting it |
| Three concepts: type, rule, law | a small grammar models get right |
| No ambient effects — time, randomness, IO are declared `needs` | power is visible in a header; every run replays from its input log |
| Errors are counterexamples, also as JSON | an agent gets something to act on, not a vague message |
| One canonical form, one content hash per definition | exact caching, semantic diffs, reproducible builds |
| The interpreter defines the semantics | one oracle; every backend is differential-tested against it |

Full rationale: [spec §1](docs/spec/v0.1.md#1-purpose-and-principles).

## How it's built

```text
source ─▶ AST ─▶ Core IR (typed, hashed) ─┬─▶ Interpreter — the semantic oracle, the dev loop
                                          ├─▶ Verifier    — Z3/cvc5: proven · refuted · unknown (= fail)
                                          └─▶ Dataflow IR ─▶ Low IR ─▶ C · Wasm · LLVM IR
```

Rust, one crate per work package, imports only flow down a tier ([crate map](docs/architecture/crates.md)). Stores (SQLite first) and surfaces (`cli`, `http`, later `web`) are generated wiring — no business logic outside the domain module. Packages are **git URLs plus tags**, pinned by commit and IR hash, with no central registry ([packages](docs/spec/packages.md)).

## Status and roadmap

**Pre-M0: the spec and the scaffold exist; nothing parses yet.** Nothing has been compiled or benchmarked, and performance expectations are hypotheses to test in M4 and M6.

| Milestone | Delivers | Gate |
|---|---|---|
| **M0** Spec and corpus | 20 example programs, semantics cases, open questions resolved | human sign-off on laws and examples |
| **M1** Interpreter | lexer, parser, formatter, type checker, interpreter — todo runs in the REPL | all examples pass |
| **M2** Verifier | SMT encoding, counterexamples, fast + full modes | zero false proofs on a seeded-bug suite |
| **M3** Store, CLI, HTTP | todo app with auth on SQLite | end-to-end tests, store parity |
| **M4** AOT via C | lowering, C emission | parity on every example; first benchmark |
| **M5** Wasm and web | Wasm backend, web surface | browser build passes the same examples |
| **M6** Optimizer and packages | fusion, e-graphs, index selection; git-native packages | parity kept, benchmarks improve, hash stability |

Tracked as [GitHub milestones](https://github.com/developerz-ai/tessera/milestones).

## Try the scaffold

```sh
git clone https://github.com/developerz-ai/tessera && cd tessera
bin/setup                       # pinned toolchain, build
bin/check                       # the gate: fmt, clippy, tests, repo guards
cargo run -q -p tessera -- help # every command, with the milestone that ships it
```

## Repository

```text
crates/      one crate per work package
packages/    house-layout crates: the `tessera` binary, `tessera-diag`, `tessera-ir`, `tessera-syntax`, `tessera-check`, `tessera-interp`, `tessera-lower`, `tessera-pkg`, `tessera-opt`, `tessera-dev`, `tessera-surface` (migration in flight)
xtask/       repo guards: crate tiers, doc links, CLAUDE.md size
corpus/      example programs, negative tests, semantics cases — written before code
docs/spec/   the language spec, amendments, open questions
site/        the project website
```

## Contributing

The best contribution right now is **M0 work**: example programs, semantics cases, and settling [open questions](docs/spec/open-questions.md). Spec ambiguities are filed as issues, never guessed in code. Start with [CONTRIBUTING.md](CONTRIBUTING.md).

This repo is built AI-first: agents write most of the code, a different agent reviews it, and humans review laws, examples and PRs. The setup lives in [`CLAUDE.md`](CLAUDE.md) and [`.claude/`](.claude/).

## License

Dual-licensed under either of [Apache License 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option. Unless you state otherwise, any contribution intentionally submitted for inclusion in this work, as defined in the Apache-2.0 license, shall be dual-licensed as above, without any additional terms or conditions.
