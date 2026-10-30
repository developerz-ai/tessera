---
description: How to set up a fresh checkout and prove a change with the gate. Use for bin/setup, bin/check, gate, CI, fresh clone, prove.
applies: [rust]
---

> House skill — established by dz-runner setup mode from this repo's own evidence, then owned by its maintainers. Correct it in place; setup never overwrites a skill that exists.

1. On a fresh box install rustup if missing: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain none`, then put `~/.cargo/bin` on PATH.
2. Run `bin/setup` — idempotent; installs the pinned 1.98.0 toolchain, fetches deps and builds the workspace. Fail (rustup/git missing) -> install the tool it names and re-run.
3. Make the change: corpus case or failing test first, then code.
4. Format with `bin/fmt` — the only formatting decision anyone makes.
5. Run `bin/check`: fmt check, clippy -D warnings (all targets), workspace tests, `cargo xtask all`. It is exactly what CI runs. Any red step -> fix that step, re-run the whole gate.
6. Run the narrow test while iterating: `cargo test -p <crate> -- <name>`; the full `bin/check` once, last, before push.
