//! Optimizer: fusion, e-graph rewriting, index selection (spec §9).
//!
//! - Work package: I ([spec §13](../../../docs/spec/v0.1.md#13-swarm-work-packages-and-milestones))
//! - Tier: 4 — may depend only on lower tiers ([crate map](../../../docs/architecture/crates.md))
//! - Frozen interface: rewrite rule format
//! - Acceptance: parity kept, benchmarks improve
//!
//! Status: scaffold only. Corpus and semantics cases land before code (swarm rule, spec §13).
