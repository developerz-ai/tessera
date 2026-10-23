//! Name resolution, types, refinements, exhaustiveness; lowers AST to Core IR (spec §3).
//!
//! - Work package: B ([spec §13](../../../docs/spec/v0.1.md#13-swarm-work-packages-and-milestones))
//! - Tier: 2 — may depend only on lower tiers ([crate map](../../../docs/architecture/crates.md))
//! - Frozen interface: Typed Core IR schema
//! - Acceptance: 200 negative tests with expected diagnostic codes
//!
//! Status: scaffold only. Corpus and semantics cases land before code (swarm rule, spec §13).
