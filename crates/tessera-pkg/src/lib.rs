//! Hash normalization, content store, lockfile, git-native packages, audit (spec §8, packages.md).
//!
//! - Work package: J ([spec §13](../../../docs/spec/v0.1.md#13-swarm-work-packages-and-milestones))
//! - Tier: 3 — may depend only on lower tiers ([crate map](../../../docs/architecture/crates.md))
//! - Frozen interface: hash normalization spec
//! - Acceptance: hash stable across renames and reformatting
//!
//! Status: scaffold only. Corpus and semantics cases land before code (swarm rule, spec §13).
