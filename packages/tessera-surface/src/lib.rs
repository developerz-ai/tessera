//! Surfaces: cli and http wiring from surface declarations to rule calls (spec §7).
//!
//! - Work package: F ([spec §13](../../../docs/spec/v0.1.md#13-swarm-work-packages-and-milestones))
//! - Tier: 5 — may depend only on lower tiers ([crate map](../../../docs/architecture/crates.md))
//! - Frozen interface: Surface trait
//! - Acceptance: end-to-end todo-app tests
//!
//! Status: scaffold only. Corpus and semantics cases land before code (swarm rule, spec §13).
