//! Lowering: Core IR to Dataflow IR to Low IR, C emission, AOT build (spec §9).
//!
//! - Work package: G ([spec §13](../../../docs/spec/v0.1.md#13-swarm-work-packages-and-milestones))
//! - Tier: 3 — may depend only on lower tiers ([crate map](../../../docs/architecture/crates.md))
//! - Frozen interface: Low IR schema
//! - Acceptance: parity with the interpreter on all examples
//!
//! Status: scaffold only. Corpus and semantics cases land before code (swarm rule, spec §13).
