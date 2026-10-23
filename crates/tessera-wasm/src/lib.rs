//! Wasm backend and web surface runtime (spec §7, §9).
//!
//! - Work package: H ([spec §13](../../../docs/spec/v0.1.md#13-swarm-work-packages-and-milestones))
//! - Tier: 4 — may depend only on lower tiers ([crate map](../../../docs/architecture/crates.md))
//! - Frozen interface: Wasm ABI
//! - Acceptance: parity under wasmtime
//!
//! Status: scaffold only. Corpus and semantics cases land before code (swarm rule, spec §13).
