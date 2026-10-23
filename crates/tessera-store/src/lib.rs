//! Stores: in-memory and SQLite, schema and checked migration generation (spec §7).
//!
//! - Work package: E ([spec §13](../../../docs/spec/v0.1.md#13-swarm-work-packages-and-milestones))
//! - Tier: 4 — may depend only on lower tiers ([crate map](../../../docs/architecture/crates.md))
//! - Frozen interface: Store trait
//! - Acceptance: memory store and SQLite store agree
//!
//! Status: scaffold only. Corpus and semantics cases land before code (swarm rule, spec §13).
