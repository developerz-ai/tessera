//! Lexer, parser, AST and canonical formatter (spec §2).
//!
//! - Work package: A ([spec §13](../../../docs/spec/v0.1.md#13-swarm-work-packages-and-milestones))
//! - Tier: 1 — may depend only on lower tiers ([crate map](../../../docs/architecture/crates.md))
//! - Frozen interface: AST JSON schema
//! - Acceptance: round-trip on corpus; `tessera fmt` idempotent
//!
//! Status: scaffold only. Corpus and semantics cases land before code (swarm rule, spec §13).
