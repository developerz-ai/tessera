//! Verifier: SMT encoding of laws, counterexamples, fast and full modes (spec §10).
//!
//! - Work package: D ([spec §13](../../../docs/spec/v0.1.md#13-swarm-work-packages-and-milestones))
//! - Tier: 3 — may depend only on lower tiers ([crate map](../../../docs/architecture/crates.md))
//! - Frozen interface: diagnostics JSON
//! - Acceptance: seeded-bad rules refuted, good ones proven, zero false proofs
//!
//! Status: scaffold only. Corpus and semantics cases land before code (swarm rule, spec §13).
