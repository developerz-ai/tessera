//! Typed Core IR: the only input to interpreter, verifier and lowering (spec §9).
//!
//! - Work package: shared (B produces, C/D/G/J consume) ([spec §13](../../../docs/spec/v0.1.md#13-swarm-work-packages-and-milestones))
//! - Tier: 0 — may depend only on lower tiers ([crate map](../../../docs/architecture/crates.md))
//! - Frozen interface: Core IR schema
//! - Acceptance: IR round-trips through its JSON schema
//!
//! Status: scaffold only. Corpus and semantics cases land before code (swarm rule, spec §13).
