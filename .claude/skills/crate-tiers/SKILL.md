---
description: How to add a new workspace crate under the tier rules. Use for new crate, add crate, tier, boundaries, workspace member.
applies: [rust]
---

> House skill — established by dz-runner setup mode from this repo's own evidence, then owned by its maintainers. Correct it in place; setup never overwrites a skill that exists.

1. Pick the tier first from the table in docs/architecture/crates.md; if the crate fits no tier, stop and fix the design — do not widen the table.
2. Create crates/<name>/ with a Cargo.toml using workspace inheritance (copy the shape of crates/tessera-diag/Cargo.toml: `[lints] workspace = true`, `version.workspace = true`, etc.).
3. Register it in ALL of: `xtask/src/tiers.rs`, the tier table in `CLAUDE.md`, `docs/architecture/crates.md`, and `[workspace.dependencies]` in the root `Cargo.toml`.
4. Declare dependencies only on strictly lower tiers, via `dep.workspace = true`.
5. Run `cargo xtask boundaries`; any finding means a mismatch between the four registrations — fix and re-run.
6. Run `bin/check`; green before the PR. Fail -> do not push.
