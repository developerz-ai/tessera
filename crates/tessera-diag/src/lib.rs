//! Diagnostics: stable codes, spans, counterexamples, human and JSON forms (spec §11).
//!
//! - Work package: shared (all WPs) ([spec §13](../../../docs/spec/v0.1.md#13-swarm-work-packages-and-milestones))
//! - Tier: 0 — may depend only on lower tiers ([crate map](../../../docs/architecture/crates.md))
//! - Frozen interface: diagnostics JSON
//! - Acceptance: every diagnostic renders identically as text and `--json`
//!
//! Status: scaffold only. Corpus and semantics cases land before code (swarm rule, spec §13).
