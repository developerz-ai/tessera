//! Reference interpreter: indexed tables, atomic rule commit, input log, replay (spec §5).
//!
//! - Work package: C ([spec §13](../../../docs/spec/v0.1.md#13-swarm-work-packages-and-milestones))
//! - Tier: 3 — may depend only on lower tiers ([crate map](../../../docs/architecture/crates.md))
//! - Frozen interface: semantics test suite (JSON cases)
//! - Acceptance: all spec examples pass; replay is bit-identical
//!
//! Status: scaffold only. Corpus and semantics cases land before code (swarm rule, spec §13).
