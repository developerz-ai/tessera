//! Developer loop: REPL, `why` derivations, LSP, typed-hole `fill` (spec §11).
//!
//! - Work package: K ([spec §13](../../../docs/spec/v0.1.md#13-swarm-work-packages-and-milestones))
//! - Tier: 4 — may depend only on lower tiers ([crate map](../../../docs/architecture/crates.md))
//! - Frozen interface: diagnostics JSON
//! - Acceptance: editor and agent loop demo
//!
//! Status: scaffold only. Corpus and semantics cases land before code (swarm rule, spec §13).
