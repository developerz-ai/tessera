//! The executable copy of the crate tier table in `CLAUDE.md`. The two must agree;
//! `cargo xtask boundaries` fails when they drift.

/// `(crate, tier)`. A crate may depend only on crates in strictly lower tiers.
pub const TIERS: &[(&str, u8)] = &[
    ("tessera-diag", 0),
    ("tessera-ir", 0),
    ("tessera-syntax", 1),
    ("tessera-check", 2),
    ("tessera-interp", 3),
    ("tessera-verify", 3),
    ("tessera-lower", 3),
    ("tessera-pkg", 3),
    ("tessera-store", 4),
    ("tessera-wasm", 4),
    ("tessera-opt", 4),
    ("tessera-dev", 4),
    ("tessera-surface", 5),
    ("tessera", 6),
];

/// Tier of a workspace crate, if it has one.
pub fn tier(name: &str) -> Option<u8> {
    TIERS.iter().find(|(n, _)| *n == name).map(|(_, t)| *t)
}
